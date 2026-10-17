//! NeoForge installers (maven.neoforged.net).
//!
//! NeoForge versions encode the Minecraft version they target:
//! - up to 1.21.x: `<minor>.<patch>.<build>`, e.g. `21.1.209` for 1.21.1
//!   (`21.0.x` is 1.21),
//! - from 26.1: `<year>.<drop>.<hotfix>.<build>`, e.g. `26.1.2.115` for 26.1.2
//!   (`26.1.0.x` is 26.1).
//!
//! Versions ending in `-beta` are the only builds of a brand-new Minecraft
//! version, until NeoForge marks one stable.

use std::collections::BTreeMap;

use serde::Deserialize;

use super::{installer, JarKind, ServerJar};
use crate::core::version;
use crate::download::get_json;
use crate::error::{AppError, AppResult};

const VERSIONS_URL: &str =
    "https://maven.neoforged.net/api/maven/versions/releases/net/neoforged/neoforge";
const MAVEN_URL: &str = "https://maven.neoforged.net/releases/net/neoforged/neoforge";

#[derive(Deserialize)]
struct VersionList {
    versions: Vec<String>,
}

/// Minecraft versions with at least one NeoForge build, newest first.
pub async fn list_versions(http: &reqwest::Client) -> AppResult<Vec<String>> {
    let builds = builds_by_mc_version(http).await?;
    let mut versions: Vec<String> = builds.into_keys().collect();
    versions.sort_by(|a, b| version::compare(b, a));
    Ok(versions)
}

/// Installer of `loader_version`, or of the latest stable build of
/// `mc_version` (latest beta when there is no stable build yet).
pub async fn server_jar(
    http: &reqwest::Client,
    mc_version: &str,
    loader_version: Option<&str>,
) -> AppResult<ServerJar> {
    let neoforge = match loader_version {
        Some(version) => version.to_string(),
        None => {
            let mut builds = builds_by_mc_version(http)
                .await?
                .remove(mc_version)
                .unwrap_or_default();
            builds.sort_by_key(|build| build_key(build));
            builds
                .iter()
                .rev()
                .find(|build| !build.contains('-'))
                .or_else(|| builds.last())
                .cloned()
                .ok_or_else(|| AppError::VersionUnavailable(mc_version.to_string()))?
        }
    };

    let url = format!("{MAVEN_URL}/{neoforge}/neoforge-{neoforge}-installer.jar");
    Ok(ServerJar {
        checksum: Some(installer::maven_checksum(http, &url).await?),
        url,
        loader_version: Some(neoforge),
        kind: JarKind::Installer,
    })
}

async fn builds_by_mc_version(http: &reqwest::Client) -> AppResult<BTreeMap<String, Vec<String>>> {
    let list: VersionList = get_json(http, VERSIONS_URL).await?;
    let mut builds: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for build in list.versions {
        if let Some(mc) = mc_version_of(&build) {
            builds.entry(mc).or_default().push(build);
        }
    }
    Ok(builds)
}

/// Minecraft version targeted by a NeoForge version (`None` for oddities
/// such as April fools builds).
fn mc_version_of(neoforge: &str) -> Option<String> {
    let release = neoforge.split('-').next()?;
    let parts: Vec<u32> = release
        .split('.')
        .map(|part| part.parse().ok())
        .collect::<Option<_>>()?;
    match parts.as_slice() {
        [year, drop, 0, _] if *year >= 26 => Some(format!("{year}.{drop}")),
        [year, drop, hotfix, _] if *year >= 26 => Some(format!("{year}.{drop}.{hotfix}")),
        [minor @ 20..=25, 0, _] => Some(format!("1.{minor}")),
        [minor @ 20..=25, patch, _] => Some(format!("1.{minor}.{patch}")),
        _ => None,
    }
}

/// Numeric parts of a version, to sort builds ("21.1.99" < "21.1.209").
fn build_key(version: &str) -> Vec<u32> {
    let release = version.split('-').next().unwrap_or(version);
    release
        .split('.')
        .filter_map(|part| part.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_neoforge_to_minecraft_versions() {
        assert_eq!(mc_version_of("21.1.209").as_deref(), Some("1.21.1"));
        assert_eq!(mc_version_of("21.0.167").as_deref(), Some("1.21"));
        assert_eq!(mc_version_of("20.4.251").as_deref(), Some("1.20.4"));
        assert_eq!(mc_version_of("21.10.64").as_deref(), Some("1.21.10"));
        assert_eq!(mc_version_of("26.1.2.115").as_deref(), Some("26.1.2"));
        assert_eq!(mc_version_of("26.1.0.19-beta").as_deref(), Some("26.1"));
        assert_eq!(mc_version_of("0.25w14craftmine.5-beta"), None);
    }

    #[test]
    fn sorts_builds_numerically() {
        let mut builds = vec!["21.1.209", "21.1.99", "21.1.1-beta"];
        builds.sort_by_key(|build| build_key(build));
        assert_eq!(builds, vec!["21.1.1-beta", "21.1.99", "21.1.209"]);
    }
}
