//! Mods, plugins and datapacks from Modrinth and CurseForge.
//!
//! What a server can receive depends on its type (`kinds_for`): plugins for
//! Paper, mods for Fabric / NeoForge / Forge, datapacks for every type. Files
//! go where the server looks for them (`mods/`, `plugins/`,
//! `<world>/datapacks/`).
//!
//! The folders are the source of truth: a file added or deleted by hand shows
//! up (or disappears) like any other. `content.json` only remembers where the
//! files MinyHost installed come from (project, version, icon), keyed by their
//! path in the server folder.
//!
//! A file is disabled by renaming it `<name>.disabled`, which the server
//! ignores (mods and plugins only: a datapack folder would still load).

pub mod curseforge;
mod jar_info;
pub mod modpack;
pub mod modrinth;

use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

use base64::Engine;
use serde::{Deserialize, Serialize};

use crate::core::game_settings::world_dir;
use crate::core::instances::Instance;
use crate::core::properties::Properties;
use crate::core::providers::Loader;
use crate::download::{download_file, Checksum};
use crate::error::{AppError, AppResult};
use crate::paths::server_files;

const DISABLED_SUFFIX: &str = ".disabled";

/// Mirrored by `ContentKind` in `src/types/index.ts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentKind {
    Mod,
    Plugin,
    Datapack,
    /// Only searched when creating a server (`modpack.rs`), never installed in one.
    Modpack,
}

/// Mirrored by `ContentSource` in `src/types/index.ts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Source {
    Modrinth,
    Curseforge,
}

/// What a server of this type can receive, most useful first.
pub fn kinds_for(loader: Loader) -> &'static [ContentKind] {
    match loader {
        Loader::Vanilla => &[ContentKind::Datapack],
        Loader::Paper => &[ContentKind::Plugin, ContentKind::Datapack],
        Loader::Fabric | Loader::Neoforge | Loader::Forge => {
            &[ContentKind::Mod, ContentKind::Datapack]
        }
    }
}

/// What to look for: content compatible with one server.
#[derive(Debug, Clone)]
pub struct Target {
    pub kind: ContentKind,
    pub mc_version: String,
    pub loader: Loader,
}

impl Target {
    /// Fails if this server type cannot use `kind` (e.g. plugins on Fabric).
    pub fn new(instance: &Instance, kind: ContentKind) -> AppResult<Self> {
        if !kinds_for(instance.loader).contains(&kind) {
            return Err(AppError::InvalidInput(format!(
                "{kind:?} on a {:?} server",
                instance.loader
            )));
        }
        Ok(Self {
            kind,
            mc_version: instance.mc_version.clone(),
            loader: instance.loader,
        })
    }
}

/// One search result. Mirrored by `ContentHit` in `src/types/index.ts`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentHit {
    pub source: Source,
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub author: String,
    pub icon_url: Option<String>,
    pub downloads: u64,
    pub page_url: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchPage {
    pub hits: Vec<ContentHit>,
    /// Number of results for the whole search.
    pub total: u64,
    /// Where the next page starts, `None` on the last one. Not
    /// `offset + hits.len()`: some results of a page may be left out.
    pub next_offset: Option<u32>,
}

impl SearchPage {
    /// `next_offset` of the page starting at `offset`, out of `total`
    /// results (at most `limit` reachable).
    pub fn next_offset(offset: u32, total: u64, limit: u64) -> Option<u32> {
        let next = offset + PAGE_SIZE;
        (u64::from(next) < total.min(limit)).then_some(next)
    }
}

/// Results per search page.
pub const PAGE_SIZE: u32 = 20;

/// Where a file installed by MinyHost comes from (`content.json`).
/// Mirrored by `ContentMeta` in `src/types/index.ts`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentMeta {
    pub source: Source,
    pub project_id: String,
    pub version_id: String,
    pub title: String,
    pub version_number: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    pub page_url: String,
    /// Installed because another project needs it.
    #[serde(default)]
    pub dependency: bool,
}

/// A file to download, as found by a source.
#[derive(Debug, Clone)]
pub struct RemoteFile {
    /// `None` when the author forbids downloads outside of CurseForge.
    pub url: Option<String>,
    pub file_name: String,
    pub checksum: Option<Checksum>,
}

/// The version of a project to install on a server.
#[derive(Debug, Clone)]
pub struct Resolved {
    pub meta: ContentMeta,
    pub file: RemoteFile,
    /// Projects this version needs.
    pub dependencies: Vec<String>,
}

/// Result of looking a project up for a server.
pub enum Resolution {
    Found(Box<Resolved>),
    /// No version matches the server's Minecraft version or type.
    Unavailable {
        title: String,
    },
}

/// What `install` did. Mirrored by `InstallOutcome` in `src/types/index.ts`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallOutcome {
    /// Titles of the projects installed (requested one and dependencies).
    pub installed: Vec<String>,
    /// Dependencies with no version for this server.
    pub unavailable: Vec<String>,
    /// Files their author only allows to download from CurseForge's website.
    pub manual: Vec<ManualDownload>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManualDownload {
    pub title: String,
    pub file_name: String,
    pub page_url: String,
}

/// A file found in a content folder. Mirrored by `InstalledContent`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledContent {
    pub kind: ContentKind,
    /// Name on disk, without the `.disabled` suffix.
    pub file_name: String,
    pub enabled: bool,
    /// `None` for files added by hand.
    pub meta: Option<ContentMeta>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Manifest {
    /// By path in the server folder ("mods/sodium.jar", always with `/`).
    #[serde(default)]
    files: BTreeMap<String, ContentMeta>,
}

/// Folder of `kind` in a server folder.
pub fn content_dir(server_dir: &Path, kind: ContentKind) -> AppResult<PathBuf> {
    match kind {
        ContentKind::Mod => Ok(server_dir.join("mods")),
        ContentKind::Plugin => Ok(server_dir.join("plugins")),
        ContentKind::Datapack => {
            let properties = Properties::load(&server_dir.join(server_files::PROPERTIES))
                .unwrap_or_else(|_| Properties::parse(""));
            Ok(world_dir(server_dir, &properties).join("datapacks"))
        }
        ContentKind::Modpack => Err(AppError::InvalidInput("modpack folder".into())),
    }
}

/// Every mod, plugin and datapack of a server, sorted by name.
pub fn list_installed(server_dir: &Path, loader: Loader) -> AppResult<Vec<InstalledContent>> {
    let manifest = load_manifest(server_dir);
    let mut items = Vec::new();
    for &kind in kinds_for(loader) {
        let dir = content_dir(server_dir, kind)?;
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.filter_map(|entry| entry.ok()) {
            let name = entry.file_name().to_string_lossy().into_owned();
            let is_dir = entry.file_type().is_ok_and(|kind| kind.is_dir());
            let Some((file_name, enabled)) = recognize(kind, &name, is_dir) else {
                continue;
            };
            let meta = manifest
                .files
                .get(&manifest_key(server_dir, &dir.join(&file_name)))
                .cloned();
            items.push(InstalledContent {
                kind,
                file_name,
                enabled,
                meta,
            });
        }
    }
    items.sort_by_key(|item| display_name(item).to_lowercase());
    Ok(items)
}

fn display_name(item: &InstalledContent) -> String {
    item.meta
        .as_ref()
        .map_or_else(|| item.file_name.clone(), |meta| meta.title.clone())
}

/// `Some((name without .disabled, enabled))` if the entry is content of `kind`.
fn recognize(kind: ContentKind, name: &str, is_dir: bool) -> Option<(String, bool)> {
    if kind == ContentKind::Datapack {
        // Datapacks are zips or unpacked folders, and cannot be disabled.
        return (is_dir || name.ends_with(".zip")).then(|| (name.to_string(), true));
    }
    if is_dir {
        return None;
    }
    match name.strip_suffix(DISABLED_SUFFIX) {
        Some(base) if base.ends_with(".jar") => Some((base.to_string(), false)),
        None if name.ends_with(".jar") => Some((name.to_string(), true)),
        _ => None,
    }
}

/// Downloads `project` and the projects it needs into the server folder.
///
/// A project already installed is updated when asked for directly, and left
/// as is when it is only a dependency.
pub async fn install(
    http: &reqwest::Client,
    server_dir: &Path,
    source: Source,
    project_id: &str,
    target: &Target,
) -> AppResult<InstallOutcome> {
    let dir = content_dir(server_dir, target.kind)?;
    tokio::fs::create_dir_all(&dir).await?;
    let mut manifest = load_manifest(server_dir);
    let mut outcome = InstallOutcome::default();
    let mut seen = HashSet::new();
    let mut queue = vec![(project_id.to_string(), false)];

    while let Some((project, is_dependency)) = queue.pop() {
        if !seen.insert(project.clone()) {
            continue;
        }
        let installed = installed_files_of(&manifest, server_dir, source, &project);
        if is_dependency && !installed.is_empty() {
            continue;
        }

        let resolved = match resolve(http, source, &project, target).await? {
            Resolution::Found(resolved) => *resolved,
            Resolution::Unavailable { title } if is_dependency => {
                outcome.unavailable.push(title);
                continue;
            }
            Resolution::Unavailable { title } => return Err(AppError::ContentUnavailable(title)),
        };
        let Some(url) = &resolved.file.url else {
            outcome.manual.push(ManualDownload {
                title: resolved.meta.title.clone(),
                file_name: resolved.file.file_name.clone(),
                page_url: resolved.meta.page_url.clone(),
            });
            continue;
        };

        let file_name = safe_file_name(&resolved.file.file_name)?;
        let dest = dir.join(&file_name);
        download_file(http, url, &dest, resolved.file.checksum.as_ref(), |_| {}).await?;

        // Update: the previous version goes away once the new one is in place.
        for old in installed {
            if old != dest {
                remove_path(&old)?;
                manifest.files.remove(&manifest_key(server_dir, &old));
            }
        }
        let mut meta = resolved.meta;
        meta.dependency = is_dependency;
        outcome.installed.push(meta.title.clone());
        manifest.files.insert(manifest_key(server_dir, &dest), meta);
        queue.extend(
            resolved
                .dependencies
                .into_iter()
                .map(|dependency| (dependency, true)),
        );
    }

    save_manifest(server_dir, &manifest)?;
    Ok(outcome)
}

/// Searches `source`; `target` restricts results to one server (`None` for
/// modpacks, which create their own server).
pub async fn search(
    http: &reqwest::Client,
    source: Source,
    kind: ContentKind,
    query: &str,
    target: Option<&Target>,
    offset: u32,
) -> AppResult<SearchPage> {
    match source {
        Source::Modrinth => modrinth::search(http, kind, query, target, offset).await,
        Source::Curseforge => curseforge::search(http, kind, query, target, offset).await,
    }
}

/// Hosts project icons may come from: the UI cannot fetch anything itself.
const ICON_HOSTS: [&str; 2] = ["cdn.modrinth.com", "media.forgecdn.net"];
const MAX_ICON_BYTES: usize = 2 * 1024 * 1024;

/// A project icon as a `data:` URL, ready for the UI.
pub async fn icon_data_url(http: &reqwest::Client, url: &str) -> AppResult<String> {
    let parsed = reqwest::Url::parse(url).map_err(|err| AppError::InvalidInput(err.to_string()))?;
    let allowed = parsed.scheme() == "https"
        && parsed
            .host_str()
            .is_some_and(|host| ICON_HOSTS.contains(&host));
    if !allowed {
        return Err(AppError::InvalidInput(format!("icon host of {url}")));
    }

    let response = http.get(parsed).send().await?.error_for_status()?;
    let mime = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .filter(|mime| mime.starts_with("image/"))
        .unwrap_or("image/png")
        .to_string();
    let bytes = response.bytes().await?;
    if bytes.len() > MAX_ICON_BYTES {
        return Err(AppError::InvalidData(format!("icon too large: {url}")));
    }
    let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
    Ok(format!("data:{mime};base64,{encoded}"))
}

async fn resolve(
    http: &reqwest::Client,
    source: Source,
    project_id: &str,
    target: &Target,
) -> AppResult<Resolution> {
    match source {
        Source::Modrinth => modrinth::resolve(http, project_id, target).await,
        Source::Curseforge => curseforge::resolve(http, project_id, target).await,
    }
}

/// Files of `project` currently on disk (enabled or not).
fn installed_files_of(
    manifest: &Manifest,
    server_dir: &Path,
    source: Source,
    project_id: &str,
) -> Vec<PathBuf> {
    manifest
        .files
        .iter()
        .filter(|(_, meta)| meta.source == source && meta.project_id == project_id)
        .filter_map(|(key, _)| {
            let path = server_dir.join(key);
            let disabled = disabled_path(&path);
            [path, disabled].into_iter().find(|path| path.exists())
        })
        .collect()
}

/// Deletes a mod, plugin or datapack.
pub fn remove(server_dir: &Path, kind: ContentKind, file_name: &str) -> AppResult<()> {
    let path = content_dir(server_dir, kind)?.join(safe_file_name(file_name)?);
    let existing = [path.clone(), disabled_path(&path)]
        .into_iter()
        .find(|path| path.exists())
        .ok_or_else(|| AppError::MissingFiles(path.display().to_string()))?;
    remove_path(&existing)?;

    let mut manifest = load_manifest(server_dir);
    if manifest
        .files
        .remove(&manifest_key(server_dir, &path))
        .is_some()
    {
        save_manifest(server_dir, &manifest)?;
    }
    Ok(())
}

/// Turns a mod or plugin on or off (`<name>.disabled`).
pub fn set_enabled(
    server_dir: &Path,
    kind: ContentKind,
    file_name: &str,
    enabled: bool,
) -> AppResult<()> {
    if !matches!(kind, ContentKind::Mod | ContentKind::Plugin) {
        return Err(AppError::InvalidInput(format!("cannot disable a {kind:?}")));
    }
    let path = content_dir(server_dir, kind)?.join(safe_file_name(file_name)?);
    let disabled = disabled_path(&path);
    let (from, to) = if enabled {
        (disabled, path)
    } else {
        (path, disabled)
    };
    if to.exists() {
        return Ok(());
    }
    if !from.exists() {
        return Err(AppError::MissingFiles(from.display().to_string()));
    }
    std::fs::rename(from, to)?;
    Ok(())
}

fn disabled_path(path: &Path) -> PathBuf {
    let mut name = path.as_os_str().to_owned();
    name.push(DISABLED_SUFFIX);
    PathBuf::from(name)
}

fn remove_path(path: &Path) -> AppResult<()> {
    if path.is_dir() {
        std::fs::remove_dir_all(path)?;
    } else {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

/// A file name from an API or the UI, refused if it could escape its folder.
pub fn safe_file_name(name: &str) -> AppResult<String> {
    let valid = !name.is_empty()
        && name != "."
        && name != ".."
        && !name.contains(['/', '\\', ':'])
        && !name.chars().any(char::is_control);
    if valid {
        Ok(name.to_string())
    } else {
        Err(AppError::InvalidData(format!("file name {name:?}")))
    }
}

/// "mods/sodium.jar": path relative to the server folder, with `/`.
fn manifest_key(server_dir: &Path, path: &Path) -> String {
    let relative = path.strip_prefix(server_dir).unwrap_or(path);
    relative
        .components()
        .map(|part| part.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

/// Records where files installed by a modpack come from.
pub fn record(server_dir: &Path, files: Vec<(PathBuf, ContentMeta)>) -> AppResult<()> {
    if files.is_empty() {
        return Ok(());
    }
    let mut manifest = load_manifest(server_dir);
    for (path, meta) in files {
        manifest.files.insert(manifest_key(server_dir, &path), meta);
    }
    save_manifest(server_dir, &manifest)
}

fn load_manifest(server_dir: &Path) -> Manifest {
    std::fs::read_to_string(server_dir.join(server_files::CONTENT))
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

fn save_manifest(server_dir: &Path, manifest: &Manifest) -> AppResult<()> {
    std::fs::write(
        server_dir.join(server_files::CONTENT),
        serde_json::to_string_pretty(manifest)?,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_server(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("minyhost-content-{name}"));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("mods")).unwrap();
        dir
    }

    fn meta(title: &str) -> ContentMeta {
        ContentMeta {
            source: Source::Modrinth,
            project_id: "AANobbMI".into(),
            version_id: "v1".into(),
            title: title.into(),
            version_number: "1.0".into(),
            icon_url: None,
            page_url: "https://modrinth.com/project/sodium".into(),
            dependency: false,
        }
    }

    #[test]
    fn pages_follow_the_offset() {
        assert_eq!(SearchPage::next_offset(0, 45, u64::MAX), Some(20));
        assert_eq!(SearchPage::next_offset(40, 45, u64::MAX), None);
        assert_eq!(SearchPage::next_offset(9_960, 50_000, 10_000), Some(9_980));
        assert_eq!(SearchPage::next_offset(9_980, 50_000, 10_000), None);
    }

    #[test]
    fn kinds_follow_the_server_type() {
        assert_eq!(kinds_for(Loader::Vanilla), &[ContentKind::Datapack]);
        assert!(kinds_for(Loader::Paper).contains(&ContentKind::Plugin));
        assert!(!kinds_for(Loader::Paper).contains(&ContentKind::Mod));
        assert!(kinds_for(Loader::Neoforge).contains(&ContentKind::Mod));
    }

    #[test]
    fn recognizes_content_files() {
        assert_eq!(
            recognize(ContentKind::Mod, "a.jar", false),
            Some(("a.jar".into(), true))
        );
        assert_eq!(
            recognize(ContentKind::Mod, "a.jar.disabled", false),
            Some(("a.jar".into(), false))
        );
        assert_eq!(recognize(ContentKind::Mod, "notes.txt", false), None);
        assert_eq!(recognize(ContentKind::Plugin, "Essentials", true), None);
        assert_eq!(
            recognize(ContentKind::Datapack, "pack", true),
            Some(("pack".into(), true))
        );
    }

    #[test]
    fn unsafe_names_are_refused() {
        assert!(safe_file_name("sodium-0.6.jar").is_ok());
        for name in ["", "..", "../x.jar", "a/b.jar", "a\\b.jar", "C:x.jar"] {
            assert!(safe_file_name(name).is_err(), "{name}");
        }
    }

    #[test]
    fn lists_disables_and_removes() {
        let dir = temp_server("list");
        std::fs::write(dir.join("mods/sodium.jar"), b"").unwrap();
        std::fs::write(dir.join("mods/manual.jar"), b"").unwrap();
        let mut manifest = Manifest::default();
        manifest
            .files
            .insert("mods/sodium.jar".into(), meta("Sodium"));
        save_manifest(&dir, &manifest).unwrap();

        let items = list_installed(&dir, Loader::Fabric).unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].file_name, "manual.jar");
        assert!(items[0].meta.is_none());
        assert_eq!(items[1].meta.as_ref().unwrap().title, "Sodium");

        set_enabled(&dir, ContentKind::Mod, "sodium.jar", false).unwrap();
        assert!(dir.join("mods/sodium.jar.disabled").exists());
        let items = list_installed(&dir, Loader::Fabric).unwrap();
        assert!(!items[1].enabled);
        assert!(items[1].meta.is_some(), "metadata survives disabling");

        remove(&dir, ContentKind::Mod, "sodium.jar").unwrap();
        assert!(!dir.join("mods/sodium.jar.disabled").exists());
        assert!(load_manifest(&dir).files.is_empty());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn datapacks_live_in_the_world_folder() {
        let dir = temp_server("datapacks");
        std::fs::write(dir.join(server_files::PROPERTIES), "level-name=monde\n").unwrap();
        assert_eq!(
            content_dir(&dir, ContentKind::Datapack).unwrap(),
            dir.join("monde").join("datapacks")
        );
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
