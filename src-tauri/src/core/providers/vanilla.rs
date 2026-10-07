//! Vanilla server jars (Mojang piston-meta).
//!
//! Also the source of truth for the Java version each Minecraft version needs,
//! whatever the server type (see `required_java`).

use serde::Deserialize;

use super::ServerJar;
use crate::download::{get_json, Checksum};
use crate::error::{AppError, AppResult};

const MANIFEST_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Deserialize)]
struct Manifest {
    versions: Vec<ManifestEntry>,
}

#[derive(Deserialize)]
struct ManifestEntry {
    id: String,
    #[serde(rename = "type")]
    kind: String,
    url: String,
}

/// The parts of a version file (`<version>.json`) we use.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct VersionDetails {
    java_version: Option<JavaVersion>,
    downloads: Downloads,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct JavaVersion {
    major_version: u32,
}

#[derive(Deserialize)]
struct Downloads {
    server: Option<Download>,
}

#[derive(Deserialize)]
struct Download {
    sha1: String,
    url: String,
}

/// Release versions (no snapshots), newest first.
pub async fn list_versions(http: &reqwest::Client) -> AppResult<Vec<String>> {
    let manifest: Manifest = get_json(http, MANIFEST_URL).await?;
    Ok(manifest
        .versions
        .into_iter()
        .filter(|version| version.kind == "release")
        .map(|version| version.id)
        .collect())
}

pub async fn server_jar(http: &reqwest::Client, mc_version: &str) -> AppResult<ServerJar> {
    let details = version_details(http, mc_version).await?;
    let server = details
        .downloads
        .server
        .ok_or_else(|| AppError::VersionUnavailable(mc_version.to_string()))?;
    Ok(ServerJar {
        url: server.url,
        checksum: Some(Checksum::Sha1(server.sha1)),
        loader_version: None,
    })
}

/// Java major version Mojang requires for `mc_version` (e.g. 21 for 1.21,
/// 25 for 26.1). Very old versions do not declare one: they run on Java 8.
pub async fn required_java(http: &reqwest::Client, mc_version: &str) -> AppResult<u32> {
    let details = version_details(http, mc_version).await?;
    Ok(details.java_version.map_or(8, |java| java.major_version))
}

async fn version_details(http: &reqwest::Client, mc_version: &str) -> AppResult<VersionDetails> {
    let manifest: Manifest = get_json(http, MANIFEST_URL).await?;
    let entry = manifest
        .versions
        .into_iter()
        .find(|version| version.id == mc_version)
        .ok_or_else(|| AppError::VersionUnavailable(mc_version.to_string()))?;
    get_json(http, &entry.url).await
}
