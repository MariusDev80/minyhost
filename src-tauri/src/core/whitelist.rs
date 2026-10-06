//! Whitelist: the players allowed to join (`whitelist.json` in the server folder).
//!
//! We edit the file ourselves (works whether the server runs or not), then,
//! if the server is running, send `whitelist reload` so it applies at once.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::core::players;
use crate::core::process::{ProcessManager, ServerStatus};
use crate::error::AppResult;
use crate::paths::AppPaths;

const FILE: &str = "whitelist.json";

/// One entry of `whitelist.json`. Mirrored by `WhitelistEntry` in `src/types/index.ts`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WhitelistEntry {
    pub uuid: String,
    pub name: String,
}

/// Players in the whitelist (empty if the file does not exist yet).
pub fn load(server_dir: &Path) -> AppResult<Vec<WhitelistEntry>> {
    let file = server_dir.join(FILE);
    if !file.exists() {
        return Ok(Vec::new());
    }
    Ok(serde_json::from_str(&std::fs::read_to_string(file)?)?)
}

fn save(server_dir: &Path, entries: &[WhitelistEntry]) -> AppResult<()> {
    std::fs::write(
        server_dir.join(FILE),
        serde_json::to_string_pretty(entries)?,
    )?;
    Ok(())
}

/// Looks the pseudo up at Mojang, then adds the player.
pub async fn add_player(
    http: &reqwest::Client,
    paths: &AppPaths,
    processes: &ProcessManager,
    server_id: &str,
    name: &str,
) -> AppResult<WhitelistEntry> {
    let profile = players::lookup(http, name.trim()).await?;
    let entry = WhitelistEntry {
        uuid: profile.uuid,
        name: profile.name,
    };

    let dir = paths.server_dir(server_id);
    save(&dir, &with_player(load(&dir)?, entry.clone()))?;
    reload_if_running(processes, server_id).await;
    Ok(entry)
}

pub async fn remove_player(
    paths: &AppPaths,
    processes: &ProcessManager,
    server_id: &str,
    uuid: &str,
) -> AppResult<()> {
    let dir = paths.server_dir(server_id);
    save(&dir, &without_player(load(&dir)?, uuid))?;
    reload_if_running(processes, server_id).await;
    Ok(())
}

async fn reload_if_running(processes: &ProcessManager, server_id: &str) {
    if processes.status(server_id).await != ServerStatus::Stopped {
        // The file is already saved: if this fails, the server reads it on next start.
        let _ = processes.send_command(server_id, "whitelist reload").await;
    }
}

/// Adds the player, or refreshes their name if already there (renamed account).
fn with_player(mut entries: Vec<WhitelistEntry>, entry: WhitelistEntry) -> Vec<WhitelistEntry> {
    match entries
        .iter_mut()
        .find(|e| e.uuid.eq_ignore_ascii_case(&entry.uuid))
    {
        Some(existing) => existing.name = entry.name,
        None => entries.push(entry),
    }
    entries
}

fn without_player(entries: Vec<WhitelistEntry>, uuid: &str) -> Vec<WhitelistEntry> {
    entries
        .into_iter()
        .filter(|e| !e.uuid.eq_ignore_ascii_case(uuid))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(uuid: &str, name: &str) -> WhitelistEntry {
        WhitelistEntry {
            uuid: uuid.into(),
            name: name.into(),
        }
    }

    #[test]
    fn adding_twice_keeps_one_entry_with_latest_name() {
        let list = with_player(Vec::new(), entry("a-1", "Old"));
        let list = with_player(list, entry("A-1", "New"));
        assert_eq!(list, vec![entry("a-1", "New")]);
    }

    #[test]
    fn removes_by_uuid() {
        let list = vec![entry("a-1", "Alice"), entry("b-2", "Bob")];
        assert_eq!(without_player(list, "A-1"), vec![entry("b-2", "Bob")]);
    }

    #[test]
    fn reads_the_server_format() {
        let json = r#"[ { "uuid": "a99441c3-e100-4fa9-9b3a-c1d7142543b7", "name": "Marius80" } ]"#;
        let list: Vec<WhitelistEntry> = serde_json::from_str(json).unwrap();
        assert_eq!(list[0].name, "Marius80");
    }
}
