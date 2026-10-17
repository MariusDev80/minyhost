//! Server creation pipeline (CLAUDE.md section 5.1).
//!
//! Steps, in order: check the form, find the Java version, create the folder,
//! download Java, download the server jar (and run the installer for Forge and
//! NeoForge), write the config files.
//! If anything fails, the folder is removed so no broken server is left behind.
//!
//! A server can also be created from a modpack (`create_from_modpack`): the
//! pack is downloaded first, as it decides the Minecraft version and loader,
//! and its files are installed once the loader is in place.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::core::content::modpack::{self, Prepared};
use crate::core::content::{ManualDownload, Source};
use crate::core::game_settings::{self, GameSettings};
use crate::core::properties::Properties;
use crate::core::providers::{self, installer, vanilla, JarKind, Loader};
use crate::core::{eula, instances, java};
use crate::download::download_file;
use crate::error::{AppError, AppResult};
use crate::paths::{server_files, AppPaths};

const MAX_NAME_LENGTH: usize = 40;

/// What the user filled in the creation form. Mirrored in `src/types/index.ts`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewServer {
    pub name: String,
    pub loader: Loader,
    pub mc_version: String,
    pub memory_mb: u32,
    /// Must be `true`: ticked by the user in the form.
    pub eula_accepted: bool,
    /// Optional world settings and game rules chosen in the form.
    #[serde(default)]
    pub settings: GameSettings,
}

/// Creation form of a server made from a modpack. Mirrored in `src/types/index.ts`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewModpackServer {
    pub name: String,
    pub memory_mb: u32,
    /// Must be `true`: ticked by the user in the form.
    pub eula_accepted: bool,
    pub source: Source,
    pub project_id: String,
    /// Modrinth version id, or CurseForge file id.
    pub version_id: String,
}

/// Mirrored by `ModpackCreated` in `src/types/index.ts`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModpackCreated {
    pub instance: instances::Instance,
    /// Files to download by hand before the server can start.
    pub manual: Vec<ManualDownload>,
}

/// Progress sent to the UI during creation (`create-progress` event).
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProgress {
    pub step: CreateStep,
    /// 0-100 during downloads, `None` for steps without a measurable progress.
    pub percent: Option<u8>,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CreateStep {
    Preparing,
    /// Modpacks only: the pack archive.
    DownloadingModpack,
    DownloadingJava,
    DownloadingServer,
    /// Forge and NeoForge only: their installer runs (1 to 3 minutes).
    InstallingLoader,
    /// Modpacks only: mods and other files of the pack.
    DownloadingContent,
    Finalizing,
}

/// What a new server is made of, from the form or from a modpack.
struct Setup {
    name: String,
    loader: Loader,
    mc_version: String,
    loader_version: Option<String>,
    memory_mb: u32,
    settings: GameSettings,
}

pub async fn create_server(
    http: &reqwest::Client,
    paths: &AppPaths,
    input: NewServer,
    on_progress: impl Fn(CreateProgress),
) -> AppResult<instances::Instance> {
    validate(&input.name, input.memory_mb, input.eula_accepted)?;
    on_progress(CreateProgress {
        step: CreateStep::Preparing,
        percent: None,
    });
    let setup = Setup {
        name: input.name,
        loader: input.loader,
        mc_version: input.mc_version,
        loader_version: None,
        memory_mb: input.memory_mb,
        settings: input.settings,
    };
    let (instance, _) = build(http, paths, setup, None, &on_progress).await?;
    Ok(instance)
}

pub async fn create_from_modpack(
    http: &reqwest::Client,
    paths: &AppPaths,
    input: NewModpackServer,
    on_progress: impl Fn(CreateProgress),
) -> AppResult<ModpackCreated> {
    validate(&input.name, input.memory_mb, input.eula_accepted)?;
    on_progress(CreateProgress {
        step: CreateStep::DownloadingModpack,
        percent: Some(0),
    });
    let pack = modpack::prepare(
        http,
        paths,
        input.source,
        &input.project_id,
        &input.version_id,
        |percent| {
            on_progress(CreateProgress {
                step: CreateStep::DownloadingModpack,
                percent: Some(percent),
            })
        },
    )
    .await?;

    let setup = Setup {
        name: input.name,
        loader: pack.loader,
        mc_version: pack.mc_version.clone(),
        loader_version: pack.loader_version.clone(),
        memory_mb: input.memory_mb,
        settings: GameSettings::default(),
    };
    let (instance, manual) = build(http, paths, setup, Some(&pack), &on_progress).await?;
    Ok(ModpackCreated { instance, manual })
}

async fn build(
    http: &reqwest::Client,
    paths: &AppPaths,
    setup: Setup,
    pack: Option<&Prepared>,
    on_progress: &impl Fn(CreateProgress),
) -> AppResult<(instances::Instance, Vec<ManualDownload>)> {
    let report = |step, percent| on_progress(CreateProgress { step, percent });

    let java_version = java::runtime_for(vanilla::required_java(http, &setup.mc_version).await?);
    let jar = providers::server_jar(
        http,
        setup.loader,
        &setup.mc_version,
        setup.loader_version.as_deref(),
    )
    .await?;

    let existing = instances::list(paths)?;
    let taken_ids: Vec<String> = existing.iter().map(|i| i.id.clone()).collect();
    let taken_ports: Vec<u16> = existing.iter().map(|i| i.port).collect();

    let mut instance = instances::Instance {
        id: instances::unique_id(&setup.name, &taken_ids),
        name: setup.name.trim().to_string(),
        mc_version: setup.mc_version.clone(),
        loader: setup.loader,
        loader_version: jar.loader_version.clone(),
        java_version,
        memory_mb: setup.memory_mb,
        port: instances::free_port(&taken_ports),
        created_at: instances::now_iso8601(),
        pending_game_rules: BTreeMap::new(),
        playit_tunnel_id: None,
        modpack: pack.map(|pack| pack.meta.clone()),
    };

    let dir = paths.server_dir(&instance.id);
    tokio::fs::create_dir_all(&dir).await?;

    let result = async {
        report(CreateStep::DownloadingJava, Some(0));
        let java = java::ensure_installed(http, paths, java_version, |percent| {
            report(CreateStep::DownloadingJava, Some(percent))
        })
        .await?;

        report(CreateStep::DownloadingServer, Some(0));
        let jar_file = match jar.kind {
            JarKind::Server => server_files::JAR,
            JarKind::Installer => installer::INSTALLER_JAR,
        };
        download_file(
            http,
            &jar.url,
            &dir.join(jar_file),
            jar.checksum.as_ref(),
            |percent| report(CreateStep::DownloadingServer, Some(percent)),
        )
        .await?;
        if jar.kind == JarKind::Installer {
            report(CreateStep::InstallingLoader, None);
            installer::run(&java, &dir).await?;
        }

        let mut manual = Vec::new();
        if let Some(pack) = pack {
            report(CreateStep::DownloadingContent, Some(0));
            manual = modpack::install(http, &dir, pack, |percent| {
                report(CreateStep::DownloadingContent, Some(percent))
            })
            .await?;
        }

        report(CreateStep::Finalizing, None);
        // A modpack may ship its own settings (world type…): they are kept,
        // except those that keep the server safe (CLAUDE.md).
        let properties_file = dir.join(server_files::PROPERTIES);
        let mut properties =
            Properties::load(&properties_file).unwrap_or_else(|_| Properties::default());
        properties.apply_defaults(instance.port, &instance.name);
        properties.save(&properties_file)?;
        // Game rules wait for the world to exist: sent at first start.
        instance.pending_game_rules =
            game_settings::apply(&dir, &instance.mc_version, &setup.settings, true)?.game_rules;
        eula::accept(&dir)?;
        // Written last: the server only shows up in the list once complete.
        instances::save(paths, &instance)?;
        Ok(manual)
    }
    .await;

    match result {
        Ok(manual) => Ok((instance, manual)),
        Err(err) => {
            let _ = tokio::fs::remove_dir_all(&dir).await;
            Err(err)
        }
    }
}

fn validate(name: &str, memory_mb: u32, eula_accepted: bool) -> AppResult<()> {
    let name = name.trim();
    if name.is_empty() || name.chars().count() > MAX_NAME_LENGTH {
        return Err(AppError::InvalidInput(format!("name: {name:?}")));
    }
    if !(instances::MIN_MEMORY_MB..=instances::MAX_MEMORY_MB).contains(&memory_mb) {
        return Err(AppError::InvalidInput(format!("memory: {memory_mb} MB")));
    }
    if !eula_accepted {
        return Err(AppError::EulaNotAccepted);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_form_passes() {
        assert!(validate("Survie", 4096, true).is_ok());
    }

    #[test]
    fn eula_must_be_accepted() {
        assert!(matches!(
            validate("Survie", 4096, false),
            Err(AppError::EulaNotAccepted)
        ));
    }

    #[test]
    fn blank_name_and_bad_memory_are_rejected() {
        assert!(validate("   ", 4096, true).is_err());
        assert!(validate("Survie", 64_000, true).is_err());
    }
}
