//! Paper server jars (PaperMC fill API v3, https://docs.papermc.io/misc/downloads-service).

use serde::Deserialize;

use super::ServerJar;
use crate::download::{get_json, Checksum};
use crate::error::{AppError, AppResult};

const API_URL: &str = "https://fill.papermc.io/v3/projects/paper";

#[derive(Deserialize)]
struct VersionList {
    versions: Vec<VersionEntry>,
}

#[derive(Deserialize)]
struct VersionEntry {
    version: VersionInfo,
}

#[derive(Deserialize)]
struct VersionInfo {
    id: String,
}

#[derive(Deserialize)]
struct Build {
    id: u32,
    channel: String,
    downloads: BuildDownloads,
}

#[derive(Deserialize)]
struct BuildDownloads {
    #[serde(rename = "server:default")]
    server: Option<BuildDownload>,
}

#[derive(Deserialize)]
struct BuildDownload {
    url: String,
    checksums: BuildChecksums,
}

#[derive(Deserialize)]
struct BuildChecksums {
    sha256: String,
}

/// Release versions (pre-releases and release candidates excluded), newest first.
pub async fn list_versions(http: &reqwest::Client) -> AppResult<Vec<String>> {
    let list: VersionList = get_json(http, &format!("{API_URL}/versions")).await?;
    Ok(list
        .versions
        .into_iter()
        .map(|entry| entry.version.id)
        .filter(|id| !id.contains('-'))
        .collect())
}

/// Latest stable build, or the latest build if the version has no stable one
/// yet (brand-new Minecraft versions start with beta builds only).
pub async fn server_jar(http: &reqwest::Client, mc_version: &str) -> AppResult<ServerJar> {
    // Builds are listed newest first.
    let builds: Vec<Build> =
        get_json(http, &format!("{API_URL}/versions/{mc_version}/builds")).await?;
    let unavailable = || AppError::VersionUnavailable(mc_version.to_string());

    let build = builds
        .iter()
        .find(|build| build.channel == "STABLE")
        .or_else(|| builds.first())
        .ok_or_else(unavailable)?;
    let download = build.downloads.server.as_ref().ok_or_else(unavailable)?;

    Ok(ServerJar {
        url: download.url.clone(),
        checksum: Some(Checksum::Sha256(download.checksums.sha256.clone())),
        loader_version: Some(build.id.to_string()),
    })
}
