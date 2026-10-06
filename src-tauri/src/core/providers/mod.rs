//! Server jar providers: where each server type gets its versions and jar.
//!
//! Each provider file exposes the same two functions:
//! - `list_versions` — Minecraft versions it supports, newest first,
//! - `server_jar` — where to download the jar for a given version.
//!
//! To add a server type: create `<name>.rs`, add a `Loader` variant and a
//! branch in the two `match` below.

pub mod fabric;
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
}

/// A server jar ready to be downloaded.
#[derive(Debug, Clone)]
pub struct ServerJar {
    pub url: String,
    /// `None` when the provider does not publish a hash (Fabric).
    pub checksum: Option<Checksum>,
    /// Paper build number or Fabric loader version (`None` for Vanilla).
    pub loader_version: Option<String>,
}

pub async fn list_versions(http: &reqwest::Client, loader: Loader) -> AppResult<Vec<String>> {
    match loader {
        Loader::Vanilla => vanilla::list_versions(http).await,
        Loader::Paper => paper::list_versions(http).await,
        Loader::Fabric => fabric::list_versions(http).await,
    }
}

pub async fn server_jar(
    http: &reqwest::Client,
    loader: Loader,
    mc_version: &str,
) -> AppResult<ServerJar> {
    match loader {
        Loader::Vanilla => vanilla::server_jar(http, mc_version).await,
        Loader::Paper => paper::server_jar(http, mc_version).await,
        Loader::Fabric => fabric::server_jar(http, mc_version).await,
    }
}
