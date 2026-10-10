//! CurseForge API (https://docs.curseforge.com/rest-api/).
//!
//! The API needs a key (`x-api-key` header), given to MinyHost by CurseForge
//! for the app. It is read from the `CURSEFORGE_API_KEY` environment variable
//! when the app is built and never committed: builds without it simply do not
//! offer CurseForge (`is_available`).
//!
//! Authors can forbid downloads outside of CurseForge (`allowModDistribution`):
//! the file then has no `downloadUrl`, and the user is sent to its page to
//! download it by hand. MinyHost never works around this choice.

use serde::de::DeserializeOwned;
use serde::Deserialize;

use super::{
    ContentHit, ContentKind, ContentMeta, RemoteFile, Resolution, Resolved, SearchPage, Source,
    Target, PAGE_SIZE,
};
use crate::core::providers::Loader;
use crate::download::Checksum;
use crate::error::{AppError, AppResult};

const API_URL: &str = "https://api.curseforge.com/v1";
const API_KEY: Option<&str> = option_env!("CURSEFORGE_API_KEY");
const MINECRAFT_GAME_ID: &str = "432";
/// `index + pageSize` may not exceed this.
const MAX_RESULTS: u32 = 10_000;
/// `ModsSearchSortField::Popularity`.
const SORT_BY_POPULARITY: &str = "2";
/// `FileRelationType::RequiredDependency`.
const REQUIRED_DEPENDENCY: u32 = 3;
/// `FileReleaseType::Release`.
const RELEASE: u32 = 1;
/// `HashAlgo::Sha1`.
const SHA1: u32 = 1;

/// Whether this build of MinyHost has a CurseForge API key.
pub fn is_available() -> bool {
    api_key().is_some()
}

fn api_key() -> Option<&'static str> {
    API_KEY.filter(|key| !key.trim().is_empty())
}

#[derive(Deserialize)]
struct Data<T> {
    data: T,
}

#[derive(Deserialize)]
struct SearchResponse {
    data: Vec<Mod>,
    pagination: Pagination,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Pagination {
    total_count: u64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Mod {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    summary: String,
    pub links: Links,
    pub logo: Option<Logo>,
    #[serde(default)]
    authors: Vec<Author>,
    #[serde(default)]
    download_count: f64,
    /// Category: mod, resource pack, datapack… (`class_id` below).
    #[serde(default)]
    pub class_id: Option<u32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Links {
    pub website_url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Logo {
    pub thumbnail_url: String,
}

#[derive(Deserialize)]
struct Author {
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct File {
    pub id: u32,
    pub mod_id: u32,
    pub display_name: String,
    pub file_name: String,
    pub download_url: Option<String>,
    #[serde(default)]
    hashes: Vec<FileHash>,
    /// Minecraft versions, loaders, and "Client" / "Server" when the author
    /// said where the file runs.
    #[serde(default)]
    pub game_versions: Vec<String>,
    #[serde(default)]
    dependencies: Vec<FileDependency>,
    release_type: u32,
    file_date: String,
    #[serde(default)]
    is_server_pack: Option<bool>,
    #[serde(default)]
    is_available: Option<bool>,
    /// Modpacks: the server pack made by the author for this version.
    #[serde(default)]
    pub server_pack_file_id: Option<u32>,
}

impl File {
    pub fn is_server_pack(&self) -> bool {
        self.is_server_pack == Some(true)
    }

    /// Tagged "Client" but not "Server": would crash a server.
    pub fn is_client_only(&self) -> bool {
        let tagged = |name: &str| self.game_versions.iter().any(|v| v == name);
        tagged("Client") && !tagged("Server")
    }
}

#[derive(Debug, Deserialize)]
struct FileHash {
    value: String,
    algo: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FileDependency {
    mod_id: u32,
    relation_type: u32,
}

pub(super) async fn get<T: DeserializeOwned>(
    http: &reqwest::Client,
    path: &str,
    params: &[(&str, String)],
) -> AppResult<T> {
    let key = api_key().ok_or(AppError::CurseforgeUnavailable)?;
    let url = reqwest::Url::parse_with_params(&format!("{API_URL}{path}"), params)
        .map_err(|err| AppError::InvalidInput(err.to_string()))?;
    let response = http
        .get(url)
        .header("x-api-key", key)
        .send()
        .await?
        .error_for_status()?;
    Ok(response.json().await?)
}

/// Several files at once, in no particular order.
pub(super) async fn files(http: &reqwest::Client, ids: &[u32]) -> AppResult<Vec<File>> {
    let response: Data<Vec<File>> =
        post(http, "/mods/files", serde_json::json!({ "fileIds": ids })).await?;
    Ok(response.data)
}

/// Several projects at once, in no particular order.
pub(super) async fn projects(http: &reqwest::Client, ids: &[u32]) -> AppResult<Vec<Mod>> {
    let response: Data<Vec<Mod>> =
        post(http, "/mods", serde_json::json!({ "modIds": ids })).await?;
    Ok(response.data)
}

pub(super) async fn file(http: &reqwest::Client, mod_id: u32, file_id: u32) -> AppResult<File> {
    let response: Data<File> = get(http, &format!("/mods/{mod_id}/files/{file_id}"), &[]).await?;
    Ok(response.data)
}

/// Files of a project, newest first (server packs left out).
pub(super) async fn project_files(http: &reqwest::Client, mod_id: &str) -> AppResult<Vec<File>> {
    let response: Data<Vec<File>> = get(
        http,
        &format!("/mods/{mod_id}/files"),
        &[("pageSize", "50".into())],
    )
    .await?;
    let mut files: Vec<File> = response
        .data
        .into_iter()
        .filter(|file| !file.is_server_pack())
        .collect();
    files.sort_by(|a, b| b.file_date.cmp(&a.file_date));
    Ok(files)
}

async fn post<T: DeserializeOwned>(
    http: &reqwest::Client,
    path: &str,
    body: serde_json::Value,
) -> AppResult<T> {
    let key = api_key().ok_or(AppError::CurseforgeUnavailable)?;
    let response = http
        .post(format!("{API_URL}{path}"))
        .header("x-api-key", key)
        .json(&body)
        .send()
        .await?
        .error_for_status()?;
    Ok(response.json().await?)
}

pub(super) async fn project(http: &reqwest::Client, mod_id: &str) -> AppResult<Mod> {
    let response: Data<Mod> = get(http, &format!("/mods/{mod_id}"), &[]).await?;
    Ok(response.data)
}

/// Search, filtered for `target` (modpacks: `None`).
pub async fn search(
    http: &reqwest::Client,
    kind: ContentKind,
    query: &str,
    target: Option<&Target>,
    offset: u32,
) -> AppResult<SearchPage> {
    let offset = offset.min(MAX_RESULTS - PAGE_SIZE);
    let mut params = vec![
        ("gameId", MINECRAFT_GAME_ID.to_string()),
        ("classId", class_id(kind).to_string()),
        ("sortField", SORT_BY_POPULARITY.to_string()),
        ("sortOrder", "desc".to_string()),
        ("index", offset.to_string()),
        ("pageSize", PAGE_SIZE.to_string()),
    ];
    if !query.trim().is_empty() {
        params.push(("searchFilter", query.trim().to_string()));
    }
    if let Some(target) = target {
        // Plugins are tagged with "1.21" rather than "1.21.1", and usually
        // keep working on newer versions: their version is chosen at install.
        if target.kind != ContentKind::Plugin {
            params.push(("gameVersion", target.mc_version.clone()));
        }
        if let Some(loader) = mod_loader_type(target) {
            params.push(("modLoaderType", loader.to_string()));
        }
    }
    let response: SearchResponse = get(http, "/mods/search", &params).await?;

    Ok(SearchPage {
        total: response.pagination.total_count.min(u64::from(MAX_RESULTS)),
        hits: response
            .data
            .into_iter()
            .map(|project| ContentHit {
                source: Source::Curseforge,
                project_id: project.id.to_string(),
                description: project.summary,
                author: project
                    .authors
                    .into_iter()
                    .next()
                    .map(|author| author.name)
                    .unwrap_or_default(),
                icon_url: project.logo.map(|logo| logo.thumbnail_url),
                downloads: project.download_count as u64,
                page_url: project.links.website_url,
                title: project.name,
            })
            .collect(),
    })
}

/// Latest file of `mod_id` for `target`, release preferred.
pub async fn resolve(
    http: &reqwest::Client,
    mod_id: &str,
    target: &Target,
) -> AppResult<Resolution> {
    let project = project(http, mod_id).await?;

    let mut candidates = Vec::new();
    for game_version in game_versions_to_try(target) {
        let mut params = vec![("pageSize", "50".to_string())];
        if let Some(version) = game_version {
            params.push(("gameVersion", version));
        }
        if let Some(loader) = mod_loader_type(target) {
            params.push(("modLoaderType", loader.to_string()));
        }
        let response: Data<Vec<File>> =
            get(http, &format!("/mods/{mod_id}/files"), &params).await?;
        candidates = response
            .data
            .into_iter()
            .filter(|file| !file.is_server_pack() && file.is_available != Some(false))
            .collect();
        if !candidates.is_empty() {
            break;
        }
    }
    // ISO 8601 dates sort as text.
    candidates.sort_by(|a, b| b.file_date.cmp(&a.file_date));
    let Some(file) = candidates
        .iter()
        .find(|file| file.release_type == RELEASE)
        .or_else(|| candidates.first())
    else {
        return Ok(Resolution::Unavailable {
            title: project.name,
        });
    };

    Ok(Resolution::Found(Box::new(Resolved {
        meta: meta(&project, file),
        file: remote_file(file),
        dependencies: file
            .dependencies
            .iter()
            .filter(|dependency| dependency.relation_type == REQUIRED_DEPENDENCY)
            .map(|dependency| dependency.mod_id.to_string())
            .collect(),
    })))
}

pub(super) fn meta(project: &Mod, file: &File) -> ContentMeta {
    ContentMeta {
        source: Source::Curseforge,
        project_id: project.id.to_string(),
        version_id: file.id.to_string(),
        title: project.name.clone(),
        version_number: file.display_name.clone(),
        icon_url: project.logo.as_ref().map(|logo| logo.thumbnail_url.clone()),
        page_url: project.links.website_url.clone(),
        dependency: false,
    }
}

pub(super) fn remote_file(file: &File) -> RemoteFile {
    RemoteFile {
        url: file.download_url.clone(),
        file_name: file.file_name.clone(),
        checksum: file
            .hashes
            .iter()
            .find(|hash| hash.algo == SHA1)
            .map(|hash| Checksum::Sha1(hash.value.clone())),
    }
}

/// Exact version first; plugins and datapacks are often only tagged with
/// "1.21", and plugins may have no tag for the newest versions at all.
fn game_versions_to_try(target: &Target) -> Vec<Option<String>> {
    let exact = Some(target.mc_version.clone());
    let major = Some(major_version(&target.mc_version));
    match target.kind {
        ContentKind::Plugin => vec![exact, major, None],
        ContentKind::Datapack => vec![exact, major],
        ContentKind::Mod | ContentKind::Modpack => vec![exact],
    }
}

/// "1.21.1" -> "1.21", "26.1.2" -> "26.1".
fn major_version(mc_version: &str) -> String {
    mc_version.split('.').take(2).collect::<Vec<_>>().join(".")
}

/// CurseForge category ("class") of each kind of content.
pub(super) fn class_id(kind: ContentKind) -> u32 {
    match kind {
        ContentKind::Mod => 6,
        ContentKind::Plugin => 5,
        ContentKind::Datapack => 6945,
        ContentKind::Modpack => 4471,
    }
}

/// `ModLoaderType` filter, for mods only.
fn mod_loader_type(target: &Target) -> Option<u32> {
    if target.kind != ContentKind::Mod {
        return None;
    }
    match target.loader {
        Loader::Forge => Some(1),
        Loader::Fabric => Some(4),
        Loader::Neoforge => Some(6),
        Loader::Vanilla | Loader::Paper => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn falls_back_to_major_versions() {
        let target = Target {
            kind: ContentKind::Plugin,
            mc_version: "1.21.1".into(),
            loader: Loader::Paper,
        };
        assert_eq!(
            game_versions_to_try(&target),
            vec![Some("1.21.1".into()), Some("1.21".into()), None]
        );
        assert_eq!(major_version("26.1.2"), "26.1");
    }

    #[test]
    fn files_without_download_url_are_manual() {
        let file: File = serde_json::from_str(
            r#"{"id":5,"modId":2,"displayName":"Mod 1.0","fileName":"mod-1.0.jar",
                "downloadUrl":null,"hashes":[{"value":"abc","algo":1}],
                "releaseType":1,"fileDate":"2026-01-01T00:00:00Z"}"#,
        )
        .unwrap();
        let remote = remote_file(&file);
        assert!(remote.url.is_none());
        assert!(matches!(remote.checksum, Some(Checksum::Sha1(ref hash)) if hash == "abc"));
    }
}
