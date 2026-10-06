//! Fabric server launcher (meta.fabricmc.net).
//!
//! Fabric provides a small launcher jar: on first start it downloads the
//! vanilla server from Mojang by itself, then starts Fabric on top of it.

use serde::Deserialize;

use super::ServerJar;
use crate::download::get_json;
use crate::error::{AppError, AppResult};

const API_URL: &str = "https://meta.fabricmc.net/v2";

/// Shape shared by the game, loader and installer version lists.
#[derive(Deserialize)]
struct VersionEntry {
    version: String,
    stable: bool,
}

/// Stable Minecraft versions supported by Fabric, newest first.
pub async fn list_versions(http: &reqwest::Client) -> AppResult<Vec<String>> {
    let versions: Vec<VersionEntry> = get_json(http, &format!("{API_URL}/versions/game")).await?;
    Ok(versions
        .into_iter()
        .filter(|version| version.stable)
        .map(|version| version.version)
        .collect())
}

/// Launcher jar for the latest stable loader and installer.
/// Fabric does not publish a hash for this generated jar.
pub async fn server_jar(http: &reqwest::Client, mc_version: &str) -> AppResult<ServerJar> {
    let loader = latest_stable(http, "loader").await?;
    let installer = latest_stable(http, "installer").await?;
    Ok(ServerJar {
        url: format!("{API_URL}/versions/loader/{mc_version}/{loader}/{installer}/server/jar"),
        checksum: None,
        loader_version: Some(loader),
    })
}

async fn latest_stable(http: &reqwest::Client, component: &str) -> AppResult<String> {
    let versions: Vec<VersionEntry> =
        get_json(http, &format!("{API_URL}/versions/{component}")).await?;
    versions
        .into_iter()
        .find(|version| version.stable)
        .map(|version| version.version)
        .ok_or_else(|| AppError::VersionUnavailable(format!("fabric {component}")))
}
