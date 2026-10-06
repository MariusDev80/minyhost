//! Server instances CRUD (`servers/<id>/instance.json`).
//!
//! An instance is "a server folder that contains an `instance.json`". Folders
//! without it (e.g. an interrupted creation) are ignored.

use std::net::{Ipv4Addr, SocketAddr, TcpStream};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::core::providers::Loader;
use crate::error::{AppError, AppResult};
use crate::paths::{server_files, AppPaths};

/// Port used by Minecraft when nothing else is set.
pub const DEFAULT_PORT: u16 = 25565;

/// RAM bounds accepted for a server, in MB.
pub const MIN_MEMORY_MB: u32 = 1024;
pub const MAX_MEMORY_MB: u32 = 16384;

/// Content of `instance.json`. Mirrored by `Instance` in `src/types/index.ts`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    pub id: String,
    pub name: String,
    pub mc_version: String,
    pub loader: Loader,
    /// Paper build or Fabric loader version, `None` for Vanilla.
    pub loader_version: Option<String>,
    pub java_version: u32,
    pub memory_mb: u32,
    pub port: u16,
    /// ISO 8601, UTC (`2026-10-05T12:00:00Z`).
    pub created_at: String,
}

/// All instances, newest first.
pub fn list(paths: &AppPaths) -> AppResult<Vec<Instance>> {
    let dir = paths.servers_dir();
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut instances: Vec<Instance> = std::fs::read_dir(dir)?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| load(paths, &entry.file_name().to_string_lossy()).ok())
        .collect();
    instances.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(instances)
}

pub fn load(paths: &AppPaths, id: &str) -> AppResult<Instance> {
    let file = paths.server_dir(id).join(server_files::INSTANCE);
    let text =
        std::fs::read_to_string(file).map_err(|_| AppError::ServerNotFound(id.to_string()))?;
    Ok(serde_json::from_str(&text)?)
}

pub fn save(paths: &AppPaths, instance: &Instance) -> AppResult<()> {
    let file = paths.server_dir(&instance.id).join(server_files::INSTANCE);
    std::fs::write(file, serde_json::to_string_pretty(instance)?)?;
    Ok(())
}

/// Deletes the whole server folder (world included).
pub fn delete(paths: &AppPaths, id: &str) -> AppResult<()> {
    let dir = paths.server_dir(id);
    if !dir.join(server_files::INSTANCE).exists() {
        return Err(AppError::ServerNotFound(id.to_string()));
    }
    std::fs::remove_dir_all(dir)?;
    Ok(())
}

/// Folder-safe id derived from the name: "Survie entre potes" -> "survie-entre-potes".
/// A number is appended if the id is taken ("survie-entre-potes-2").
pub fn unique_id(name: &str, taken: &[String]) -> String {
    let base = slugify(name);
    let base = if base.is_empty() {
        "serveur".to_string()
    } else {
        base
    };

    let mut id = base.clone();
    let mut counter = 2;
    while taken.contains(&id) {
        id = format!("{base}-{counter}");
        counter += 1;
    }
    id
}

/// First port from 25565 that no other instance uses and that is free right now.
pub fn free_port(taken: &[u16]) -> u16 {
    (DEFAULT_PORT..u16::MAX)
        .find(|port| !taken.contains(port) && is_port_free(*port))
        .unwrap_or(DEFAULT_PORT)
}

/// A port is taken if something answers on it locally. We connect instead of
/// binding: binding on all interfaces would make Windows show a firewall
/// prompt for MinyHost itself.
pub fn is_port_free(port: u16) -> bool {
    let address = SocketAddr::from((Ipv4Addr::LOCALHOST, port));
    TcpStream::connect_timeout(&address, Duration::from_millis(300)).is_err()
}

/// Current time as ISO 8601 in UTC, without pulling a date library.
pub fn now_iso8601() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs());
    format_iso8601(seconds)
}

fn format_iso8601(unix_seconds: u64) -> String {
    let days = (unix_seconds / 86_400) as i64;
    let time = unix_seconds % 86_400;

    // Days since 1970-01-01 to a civil date (Howard Hinnant's algorithm).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let day_of_era = z.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let mp = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = year_of_era + era * 400 + i64::from(month <= 2);

    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        time / 3600,
        time % 3600 / 60,
        time % 60
    )
}

fn slugify(name: &str) -> String {
    let mut slug = String::new();
    for c in name.chars().flat_map(char::to_lowercase) {
        let c = fold_accent(c);
        if c.is_ascii_alphanumeric() {
            slug.push(c);
        } else if !slug.is_empty() && !slug.ends_with('-') {
            slug.push('-');
        }
    }
    slug.trim_end_matches('-').chars().take(40).collect()
}

fn fold_accent(c: char) -> char {
    match c {
        'à' | 'á' | 'â' | 'ä' | 'ã' | 'å' => 'a',
        'ç' => 'c',
        'è' | 'é' | 'ê' | 'ë' => 'e',
        'ì' | 'í' | 'î' | 'ï' => 'i',
        'ñ' => 'n',
        'ò' | 'ó' | 'ô' | 'ö' | 'õ' => 'o',
        'ù' | 'ú' | 'û' | 'ü' => 'u',
        'ý' | 'ÿ' => 'y',
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_is_derived_from_name() {
        assert_eq!(unique_id("Survie entre potes", &[]), "survie-entre-potes");
        assert_eq!(unique_id("  Été & Hiver !! ", &[]), "ete-hiver");
        assert_eq!(unique_id("???", &[]), "serveur");
    }

    #[test]
    fn id_avoids_existing_ones() {
        let taken = vec!["survie".to_string(), "survie-2".to_string()];
        assert_eq!(unique_id("Survie", &taken), "survie-3");
    }

    #[test]
    fn port_in_use_is_detected() {
        // Loopback only, so the test itself triggers no firewall prompt.
        let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let port = listener.local_addr().unwrap().port();
        assert!(!is_port_free(port));
        drop(listener);
        assert!(is_port_free(port));
    }

    #[test]
    fn iso8601_formatting() {
        assert_eq!(format_iso8601(0), "1970-01-01T00:00:00Z");
        assert_eq!(format_iso8601(1_791_201_600), "2026-10-05T12:00:00Z");
        assert_eq!(format_iso8601(951_782_400), "2000-02-29T00:00:00Z");
    }
}
