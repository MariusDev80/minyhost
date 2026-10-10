//! Modrinth API v2 (https://docs.modrinth.com/api/), no account needed.
//!
//! - search: `/search` with facets (project type, loader, Minecraft version,
//!   server side),
//! - install: `/project/<id>` for the title and icon, then
//!   `/project/<id>/version` filtered by loader and Minecraft version (newest
//!   first). Files come with their SHA-1, checked after download.

use std::collections::HashMap;

use serde::Deserialize;

use super::{
    ContentHit, ContentKind, ContentMeta, RemoteFile, Resolution, Resolved, SearchPage, Source,
    Target, PAGE_SIZE,
};
use crate::core::providers::Loader;
use crate::download::{get_json, Checksum};
use crate::error::{AppError, AppResult};

const API_URL: &str = "https://api.modrinth.com/v2";
const SITE_URL: &str = "https://modrinth.com";

#[derive(Deserialize)]
struct SearchResponse {
    hits: Vec<SearchHit>,
    total_hits: u64,
}

#[derive(Deserialize)]
struct SearchHit {
    project_id: String,
    slug: String,
    title: String,
    description: String,
    author: String,
    icon_url: Option<String>,
    downloads: u64,
}

#[derive(Deserialize)]
pub(super) struct Project {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub icon_url: Option<String>,
    /// "required", "optional", "unsupported" or "unknown".
    #[serde(default)]
    pub server_side: Option<String>,
}

/// A file found by its hash (`identify`).
pub(super) struct Identified {
    pub meta: ContentMeta,
    /// The project says it does not run on servers.
    pub client_only: bool,
}

#[derive(Deserialize)]
pub(super) struct Version {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub version_number: String,
    pub version_type: String,
    #[serde(default)]
    pub game_versions: Vec<String>,
    #[serde(default)]
    pub loaders: Vec<String>,
    pub files: Vec<VersionFile>,
    #[serde(default)]
    dependencies: Vec<Dependency>,
}

#[derive(Deserialize)]
pub(super) struct VersionFile {
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub hashes: FileHashes,
}

#[derive(Deserialize)]
pub(super) struct FileHashes {
    pub sha1: Option<String>,
}

#[derive(Deserialize)]
struct Dependency {
    project_id: Option<String>,
    version_id: Option<String>,
    dependency_type: String,
}

/// Search, filtered for `target` (modpacks: `None`, any supported version).
pub async fn search(
    http: &reqwest::Client,
    kind: ContentKind,
    query: &str,
    target: Option<&Target>,
    offset: u32,
) -> AppResult<SearchPage> {
    let mut facets = vec![vec![format!("project_type:{}", project_type(kind))]];
    let loaders = match target {
        Some(target) => loaders_for(target),
        None => modpack_loaders(),
    };
    if kind != ContentKind::Datapack {
        facets.push(loaders.iter().map(|l| format!("categories:{l}")).collect());
    }
    if let Some(target) = target {
        facets.push(vec![format!("versions:{}", target.mc_version)]);
    }
    if matches!(kind, ContentKind::Mod | ContentKind::Modpack) {
        // Client-only mods (minimaps, shaders…) would crash a server.
        facets.push(vec!["server_side!=unsupported".into()]);
    }

    let index = if query.trim().is_empty() {
        "downloads"
    } else {
        "relevance"
    };
    let url = reqwest::Url::parse_with_params(
        &format!("{API_URL}/search"),
        &[
            ("query", query.trim().to_string()),
            ("facets", serde_json::to_string(&facets)?),
            ("index", index.to_string()),
            ("offset", offset.to_string()),
            ("limit", PAGE_SIZE.to_string()),
        ],
    )
    .map_err(|err| AppError::InvalidInput(err.to_string()))?;
    let response: SearchResponse = get_json(http, url.as_str()).await?;

    Ok(SearchPage {
        total: response.total_hits,
        hits: response
            .hits
            .into_iter()
            .map(|hit| ContentHit {
                source: Source::Modrinth,
                page_url: page_url(&hit.slug),
                project_id: hit.project_id,
                title: hit.title,
                description: hit.description,
                author: hit.author,
                icon_url: hit.icon_url,
                downloads: hit.downloads,
            })
            .collect(),
    })
}

/// Latest version of `project_id` for `target`, release preferred.
pub async fn resolve(
    http: &reqwest::Client,
    project_id: &str,
    target: &Target,
) -> AppResult<Resolution> {
    let project = get_project(http, project_id).await?;
    let versions = versions(
        http,
        &project.id,
        &loaders_for(target),
        Some(&target.mc_version),
    )
    .await?;
    let Some(version) = versions
        .iter()
        .find(|version| version.version_type == "release")
        .or_else(|| versions.first())
    else {
        return Ok(Resolution::Unavailable {
            title: project.title,
        });
    };
    let file = primary_file(version)?;

    let mut dependencies = Vec::new();
    for dependency in &version.dependencies {
        if dependency.dependency_type != "required" {
            continue;
        }
        match (&dependency.project_id, &dependency.version_id) {
            (Some(project), _) => dependencies.push(project.clone()),
            (None, Some(version_id)) => {
                dependencies.push(get_version(http, version_id).await?.project_id);
            }
            (None, None) => {}
        }
    }

    Ok(Resolution::Found(Box::new(Resolved {
        meta: project_meta(&project, version),
        file: remote_file(file),
        dependencies,
    })))
}

pub(super) async fn get_project(http: &reqwest::Client, project_id: &str) -> AppResult<Project> {
    get_json(http, &format!("{API_URL}/project/{project_id}")).await
}

/// Versions of a project, newest first, filtered by loader (none if empty)
/// and Minecraft version.
pub(super) async fn versions(
    http: &reqwest::Client,
    project_id: &str,
    loaders: &[&str],
    mc_version: Option<&str>,
) -> AppResult<Vec<Version>> {
    let mut params = Vec::new();
    if !loaders.is_empty() {
        params.push(("loaders", serde_json::to_string(loaders)?));
    }
    if let Some(mc_version) = mc_version {
        params.push(("game_versions", serde_json::to_string(&[mc_version])?));
    }
    let url = reqwest::Url::parse_with_params(
        &format!("{API_URL}/project/{project_id}/version"),
        &params,
    )
    .map_err(|err| AppError::InvalidInput(err.to_string()))?;
    get_json(http, url.as_str()).await
}

pub(super) async fn get_version(http: &reqwest::Client, version_id: &str) -> AppResult<Version> {
    get_json(http, &format!("{API_URL}/version/{version_id}")).await
}

/// Projects and versions behind files, found by their SHA-1 (files of a
/// modpack only come with their hash). Files Modrinth does not know are left out.
pub(super) async fn identify(
    http: &reqwest::Client,
    sha1s: &[String],
) -> AppResult<HashMap<String, Identified>> {
    if sha1s.is_empty() {
        return Ok(HashMap::new());
    }
    let versions: HashMap<String, Version> = http
        .post(format!("{API_URL}/version_files"))
        .json(&serde_json::json!({ "hashes": sha1s, "algorithm": "sha1" }))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let mut ids: Vec<&str> = versions.values().map(|v| v.project_id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    let url = reqwest::Url::parse_with_params(
        &format!("{API_URL}/projects"),
        &[("ids", serde_json::to_string(&ids)?)],
    )
    .map_err(|err| AppError::InvalidInput(err.to_string()))?;
    let projects: Vec<Project> = get_json(http, url.as_str()).await?;
    let projects: HashMap<&str, &Project> = projects.iter().map(|p| (p.id.as_str(), p)).collect();

    Ok(versions
        .into_iter()
        .filter_map(|(sha1, version)| {
            let project = projects.get(version.project_id.as_str())?;
            let identified = Identified {
                meta: project_meta(project, &version),
                client_only: project.server_side.as_deref() == Some("unsupported"),
            };
            Some((sha1, identified))
        })
        .collect())
}

pub(super) fn project_meta(project: &Project, version: &Version) -> ContentMeta {
    ContentMeta {
        source: Source::Modrinth,
        project_id: project.id.clone(),
        version_id: version.id.clone(),
        title: project.title.clone(),
        version_number: version.version_number.clone(),
        icon_url: project.icon_url.clone(),
        page_url: page_url(&project.slug),
        dependency: false,
    }
}

/// The file to install: the one marked primary, else the first.
pub(super) fn primary_file(version: &Version) -> AppResult<&VersionFile> {
    version
        .files
        .iter()
        .find(|file| file.primary)
        .or_else(|| version.files.first())
        .ok_or_else(|| AppError::InvalidData(format!("no file in version {}", version.id)))
}

pub(super) fn remote_file(file: &VersionFile) -> RemoteFile {
    RemoteFile {
        url: Some(file.url.clone()),
        file_name: file.filename.clone(),
        checksum: file.hashes.sha1.clone().map(Checksum::Sha1),
    }
}

pub(super) fn page_url(slug: &str) -> String {
    format!("{SITE_URL}/project/{slug}")
}

/// Modrinth's `project_type` facet. A project offering several types (e.g.
/// a mod also published as a datapack) matches each of them.
fn project_type(kind: ContentKind) -> &'static str {
    match kind {
        ContentKind::Mod => "mod",
        ContentKind::Plugin => "plugin",
        ContentKind::Datapack => "datapack",
        ContentKind::Modpack => "modpack",
    }
}

/// Modrinth loaders whose files run on `target`.
fn loaders_for(target: &Target) -> Vec<&'static str> {
    match target.kind {
        // Paper runs Bukkit and Spigot plugins too.
        ContentKind::Plugin => vec!["paper", "spigot", "bukkit"],
        ContentKind::Datapack => vec!["datapack"],
        ContentKind::Mod | ContentKind::Modpack => vec![loader_name(target.loader)],
    }
}

/// Loaders of the modpacks MinyHost can host.
pub(super) fn modpack_loaders() -> Vec<&'static str> {
    vec!["fabric", "neoforge", "forge"]
}

fn loader_name(loader: Loader) -> &'static str {
    match loader {
        Loader::Vanilla => "minecraft",
        Loader::Paper => "paper",
        Loader::Fabric => "fabric",
        Loader::Neoforge => "neoforge",
        Loader::Forge => "forge",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paper_accepts_bukkit_plugins() {
        let target = Target {
            kind: ContentKind::Plugin,
            mc_version: "1.21.1".into(),
            loader: Loader::Paper,
        };
        assert_eq!(loaders_for(&target), vec!["paper", "spigot", "bukkit"]);
    }
}
