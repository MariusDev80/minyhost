//! Server creation pipeline (CLAUDE.md section 5.1).
//!
//! Steps, in order: check the form, find the Java version, create the folder,
//! download Java, download the server jar, write the config files.
//! If anything fails, the folder is removed so no broken server is left behind.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::core::game_settings::{self, GameSettings};
use crate::core::providers::{self, vanilla, Loader};
use crate::core::{eula, instances, java, properties};
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
    DownloadingJava,
    DownloadingServer,
    Finalizing,
}

pub async fn create_server(
    http: &reqwest::Client,
    paths: &AppPaths,
    input: NewServer,
    on_progress: impl Fn(CreateProgress),
) -> AppResult<instances::Instance> {
    validate(&input)?;
    let report = |step, percent| on_progress(CreateProgress { step, percent });
    report(CreateStep::Preparing, None);

    let java_version = java::runtime_for(vanilla::required_java(http, &input.mc_version).await?);
    let jar = providers::server_jar(http, input.loader, &input.mc_version).await?;

    let existing = instances::list(paths)?;
    let taken_ids: Vec<String> = existing.iter().map(|i| i.id.clone()).collect();
    let taken_ports: Vec<u16> = existing.iter().map(|i| i.port).collect();

    let mut instance = instances::Instance {
        id: instances::unique_id(&input.name, &taken_ids),
        name: input.name.trim().to_string(),
        mc_version: input.mc_version.clone(),
        loader: input.loader,
        loader_version: jar.loader_version.clone(),
        java_version,
        memory_mb: input.memory_mb,
        port: instances::free_port(&taken_ports),
        created_at: instances::now_iso8601(),
        pending_game_rules: BTreeMap::new(),
        playit_tunnel_id: None,
    };

    let dir = paths.server_dir(&instance.id);
    tokio::fs::create_dir_all(&dir).await?;

    let result = async {
        report(CreateStep::DownloadingJava, Some(0));
        java::ensure_installed(http, paths, java_version, |percent| {
            report(CreateStep::DownloadingJava, Some(percent))
        })
        .await?;

        report(CreateStep::DownloadingServer, Some(0));
        download_file(
            http,
            &jar.url,
            &dir.join(server_files::JAR),
            jar.checksum.as_ref(),
            |percent| report(CreateStep::DownloadingServer, Some(percent)),
        )
        .await?;

        report(CreateStep::Finalizing, None);
        properties::defaults(instance.port, &instance.name)
            .save(&dir.join(server_files::PROPERTIES))?;
        // Game rules wait for the world to exist: sent at first start.
        instance.pending_game_rules =
            game_settings::apply(&dir, &instance.mc_version, &input.settings, true)?.game_rules;
        eula::accept(&dir)?;
        // Written last: the server only shows up in the list once complete.
        instances::save(paths, &instance)
    }
    .await;

    if let Err(err) = result {
        let _ = tokio::fs::remove_dir_all(&dir).await;
        return Err(err);
    }
    Ok(instance)
}

fn validate(input: &NewServer) -> AppResult<()> {
    let name = input.name.trim();
    if name.is_empty() || name.chars().count() > MAX_NAME_LENGTH {
        return Err(AppError::InvalidInput(format!("name: {name:?}")));
    }
    if !(instances::MIN_MEMORY_MB..=instances::MAX_MEMORY_MB).contains(&input.memory_mb) {
        return Err(AppError::InvalidInput(format!(
            "memory: {} MB",
            input.memory_mb
        )));
    }
    if !input.eula_accepted {
        return Err(AppError::EulaNotAccepted);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn form() -> NewServer {
        NewServer {
            name: "Survie".into(),
            loader: Loader::Paper,
            mc_version: "1.21.1".into(),
            memory_mb: 4096,
            eula_accepted: true,
            settings: GameSettings::default(),
        }
    }

    #[test]
    fn valid_form_passes() {
        assert!(validate(&form()).is_ok());
    }

    #[test]
    fn eula_must_be_accepted() {
        let input = NewServer {
            eula_accepted: false,
            ..form()
        };
        assert!(matches!(validate(&input), Err(AppError::EulaNotAccepted)));
    }

    #[test]
    fn blank_name_and_bad_memory_are_rejected() {
        let blank = NewServer {
            name: "   ".into(),
            ..form()
        };
        assert!(validate(&blank).is_err());
        let too_much = NewServer {
            memory_mb: 64_000,
            ..form()
        };
        assert!(validate(&too_much).is_err());
    }
}
