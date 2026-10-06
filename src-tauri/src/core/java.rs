//! Java runtime selection and Temurin JRE download (CLAUDE.md section 5.2).
//!
//! The required Java version comes from Mojang (`providers::vanilla::required_java`).
//! `runtime_for` maps it to a long-term-support Temurin release, then
//! `ensure_installed` downloads it once into `java/<version>/`.
//! The Java installed on the system is never used.

use std::fs::File;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::download::{download_file, get_json, Checksum};
use crate::error::{AppError, AppResult};
use crate::paths::AppPaths;

/// Temurin LTS releases we install, oldest first.
const LTS_RELEASES: [u32; 4] = [8, 17, 21, 25];

/// Smallest LTS release able to run code requiring `required` (e.g. 16 -> 17).
pub fn runtime_for(required: u32) -> u32 {
    LTS_RELEASES
        .into_iter()
        .find(|lts| *lts >= required)
        .unwrap_or(required)
}

/// Path to the `java` executable of an installed runtime.
pub fn executable(paths: &AppPaths, version: u32) -> PathBuf {
    let name = if cfg!(windows) { "java.exe" } else { "java" };
    paths.java_dir(version).join("bin").join(name)
}

/// Downloads and unpacks Java `version` if it is not installed yet, then
/// returns the path to its executable.
pub async fn ensure_installed(
    http: &reqwest::Client,
    paths: &AppPaths,
    version: u32,
    on_progress: impl FnMut(u8),
) -> AppResult<PathBuf> {
    let java = executable(paths, version);
    if java.exists() {
        return Ok(java);
    }

    let package = latest_package(http, version).await?;
    let install_dir = paths.java_dir(version);
    let parent = install_dir
        .parent()
        .ok_or_else(|| AppError::MissingFiles(install_dir.display().to_string()))?;
    tokio::fs::create_dir_all(parent).await?;

    let archive = parent.join(format!("{version}.zip"));
    let checksum = Checksum::Sha256(package.checksum);
    download_file(http, &package.link, &archive, Some(&checksum), on_progress).await?;

    // Unzipping is blocking work: keep it off the async threads.
    let staging = parent.join(format!("{version}.tmp"));
    let archive_clone = archive.clone();
    let staging_clone = staging.clone();
    tokio::task::spawn_blocking(move || extract_zip(&archive_clone, &staging_clone))
        .await
        .map_err(|err| AppError::Io(std::io::Error::other(err)))??;

    // Rename only once everything is unpacked: a half-installed runtime
    // never sits in `java/<version>/`.
    if install_dir.exists() {
        tokio::fs::remove_dir_all(&install_dir).await?;
    }
    tokio::fs::rename(&staging, &install_dir).await?;
    tokio::fs::remove_file(&archive).await?;

    if java.exists() {
        Ok(java)
    } else {
        Err(AppError::MissingFiles(java.display().to_string()))
    }
}

/// Adoptium asset (https://api.adoptium.net/q/swagger-ui).
#[derive(Deserialize)]
struct Asset {
    binary: Binary,
}

#[derive(Deserialize)]
struct Binary {
    package: Package,
}

#[derive(Deserialize)]
struct Package {
    link: String,
    checksum: String,
}

async fn latest_package(http: &reqwest::Client, version: u32) -> AppResult<Package> {
    let url = format!(
        "https://api.adoptium.net/v3/assets/latest/{version}/hotspot\
         ?architecture={}&image_type=jre&os={}&vendor=eclipse",
        adoptium_arch(),
        adoptium_os(),
    );
    let assets: Vec<Asset> = get_json(http, &url).await?;
    assets
        .into_iter()
        .next()
        .map(|asset| asset.binary.package)
        .ok_or(AppError::JavaUnavailable(version))
}

fn adoptium_os() -> &'static str {
    match std::env::consts::OS {
        "macos" => "mac",
        "linux" => "linux",
        _ => "windows",
    }
}

fn adoptium_arch() -> &'static str {
    match std::env::consts::ARCH {
        "aarch64" => "aarch64",
        _ => "x64",
    }
}

/// Unpacks a JRE zip into `dest`, dropping its top-level folder
/// (`jdk-21.0.5+11-jre/bin/java.exe` becomes `dest/bin/java.exe`).
fn extract_zip(archive: &Path, dest: &Path) -> AppResult<()> {
    if dest.exists() {
        std::fs::remove_dir_all(dest)?;
    }
    let mut zip = zip::ZipArchive::new(File::open(archive)?)?;

    for index in 0..zip.len() {
        let mut entry = zip.by_index(index)?;
        // `enclosed_name` rejects paths escaping the archive (`../`).
        let Some(name) = entry.enclosed_name() else {
            continue;
        };
        let relative: PathBuf = name.components().skip(1).collect();
        if relative.as_os_str().is_empty() {
            continue;
        }

        let target = dest.join(relative);
        if entry.is_dir() {
            std::fs::create_dir_all(&target)?;
        } else {
            if let Some(parent) = target.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::io::copy(&mut entry, &mut File::create(&target)?)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_matches_mojang_requirements() {
        assert_eq!(runtime_for(8), 8); // 1.16.5 and older
        assert_eq!(runtime_for(16), 17); // 1.17.x
        assert_eq!(runtime_for(17), 17); // 1.18 to 1.20.4
        assert_eq!(runtime_for(21), 21); // 1.20.5 to 1.21.x
        assert_eq!(runtime_for(25), 25); // 26.1 and newer
    }

    #[test]
    fn runtime_beyond_known_lts_is_used_as_is() {
        assert_eq!(runtime_for(29), 29);
    }
}
