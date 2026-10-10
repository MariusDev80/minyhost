//! Server jar providers: where each server type gets its versions and jar.
//!
//! Each provider file exposes the same two functions:
//! - `list_versions` — Minecraft versions it supports, newest first,
//! - `server_jar` — where to download the jar for a given version.
//!
//! To add a server type: create `<name>.rs`, add a `Loader` variant and a
//! branch in the two `match` below.

pub mod fabric;
pub mod forge;
pub mod installer;
pub mod neoforge;
pub mod paper;
pub mod vanilla;

use serde::{Deserialize, Serialize};

use crate::download::Checksum;
use crate::error::AppResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Loader {
    Vanilla,
    Paper,
    Fabric,
    Neoforge,
    Forge,
}

/// What the downloaded jar is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JarKind {
    /// Started as is (`server.jar`).
    Server,
    /// Official installer, run once at creation (`installer.rs`): Forge, NeoForge.
    Installer,
}

/// A server jar ready to be downloaded.
#[derive(Debug, Clone)]
pub struct ServerJar {
    pub url: String,
    /// `None` when the provider does not publish a hash (Fabric).
    pub checksum: Option<Checksum>,
    /// Paper build, Fabric loader, Forge or NeoForge version (`None` for Vanilla).
    pub loader_version: Option<String>,
    pub kind: JarKind,
}

pub async fn list_versions(http: &reqwest::Client, loader: Loader) -> AppResult<Vec<String>> {
    match loader {
        Loader::Vanilla => vanilla::list_versions(http).await,
        Loader::Paper => paper::list_versions(http).await,
        Loader::Fabric => fabric::list_versions(http).await,
        Loader::Neoforge => neoforge::list_versions(http).await,
        Loader::Forge => forge::list_versions(http).await,
    }
}

/// `loader_version` pins the loader (modpacks need the exact one they were
/// made with); `None` takes the latest stable one.
pub async fn server_jar(
    http: &reqwest::Client,
    loader: Loader,
    mc_version: &str,
    loader_version: Option<&str>,
) -> AppResult<ServerJar> {
    match loader {
        Loader::Vanilla => vanilla::server_jar(http, mc_version).await,
        Loader::Paper => paper::server_jar(http, mc_version).await,
        Loader::Fabric => fabric::server_jar(http, mc_version, loader_version).await,
        Loader::Neoforge => neoforge::server_jar(http, mc_version, loader_version).await,
        Loader::Forge => forge::server_jar(http, mc_version, loader_version).await,
    }
}
