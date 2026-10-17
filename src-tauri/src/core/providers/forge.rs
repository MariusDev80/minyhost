//! Forge installers (files.minecraftforge.net).
//!
//! `promotions_slim.json` lists, for each Minecraft version, the latest and
//! (when there is one) the recommended Forge build: `"1.20.1-recommended":
//! "47.2.0"`. The installer lives on Forge's Maven under `<mc>-<forge>`.

use std::collections::BTreeMap;

use serde::Deserialize;

use super::{installer, JarKind, ServerJar};
use crate::core::version;
use crate::download::get_json;
use crate::error::{AppError, AppResult};

const PROMOTIONS_URL: &str =
    "https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json";
const MAVEN_URL: &str = "https://maven.minecraftforge.net/net/minecraftforge/forge";

/// Oldest Minecraft version offered: older installers rely on download
/// servers that no longer exist.
const OLDEST_VERSION: &str = "1.12.2";

#[derive(Deserialize)]
struct Promotions {
    promos: BTreeMap<String, String>,
}

/// Minecraft versions with a Forge build, newest first.
pub async fn list_versions(http: &reqwest::Client) -> AppResult<Vec<String>> {
    let promotions: Promotions = get_json(http, PROMOTIONS_URL).await?;
    let mut versions: Vec<String> = promotions
        .promos
        .keys()
        .filter_map(|key| key.split_once('-').map(|(mc, _)| mc))
        .filter(|mc| is_release(mc) && version::at_least(mc, OLDEST_VERSION))
        .map(String::from)
        .collect();
    versions.sort_by(|a, b| version::compare(b, a));
    versions.dedup();
    Ok(versions)
}

/// Installer of `loader_version`, or of the recommended build of
/// `mc_version` (the latest one if none is recommended yet).
pub async fn server_jar(
    http: &reqwest::Client,
    mc_version: &str,
    loader_version: Option<&str>,
) -> AppResult<ServerJar> {
    let forge = match loader_version {
        Some(version) => version.to_string(),
        None => {
            let promotions: Promotions = get_json(http, PROMOTIONS_URL).await?;
            let promo = |kind: &str| promotions.promos.get(&format!("{mc_version}-{kind}"));
            promo("recommended")
                .or_else(|| promo("latest"))
                .cloned()
                .ok_or_else(|| AppError::VersionUnavailable(mc_version.to_string()))?
        }
    };

    let full = format!("{mc_version}-{forge}");
    let url = format!("{MAVEN_URL}/{full}/forge-{full}-installer.jar");
    Ok(ServerJar {
        checksum: Some(installer::maven_checksum(http, &url).await?),
        url,
        loader_version: Some(forge),
        kind: JarKind::Installer,
    })
}

/// "1.20.1" yes, "1.7.10_pre4" no.
fn is_release(mc_version: &str) -> bool {
    mc_version.chars().all(|c| c.is_ascii_digit() || c == '.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pre_releases_are_skipped() {
        assert!(is_release("1.20.1"));
        assert!(is_release("26.1"));
        assert!(!is_release("1.7.10_pre4"));
    }
}
