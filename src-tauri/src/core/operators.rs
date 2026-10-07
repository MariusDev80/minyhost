//! Server operators: players allowed to use every command (`ops.json`).
//!
//! Unlike the whitelist, a running server cannot reload `ops.json` and would
//! overwrite our changes. So:
//! - server stopped -> we edit `ops.json` ourselves,
//! - server running -> we send `op <name>` / `deop <name>` and the server
//!   updates the file (the UI refreshes when the console confirms it).

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::core::process::{ProcessManager, ServerStatus};
use crate::core::{players, properties};
use crate::error::AppResult;
use crate::paths::{server_files, AppPaths};

const FILE: &str = "ops.json";

/// Permission level Minecraft gives operators by default (all commands).
const DEFAULT_LEVEL: u8 = 4;

/// One entry of `ops.json`. Mirrored by `Operator` in `src/types/index.ts`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operator {
    pub uuid: String,
    pub name: String,
    pub level: u8,
    pub bypasses_player_limit: bool,
}

/// Operators of the server (empty if the file does not exist yet).
pub fn load(server_dir: &Path) -> AppResult<Vec<Operator>> {
    let file = server_dir.join(FILE);
    if !file.exists() {
        return Ok(Vec::new());
    }
    Ok(serde_json::from_str(&std::fs::read_to_string(file)?)?)
}

fn save(server_dir: &Path, operators: &[Operator]) -> AppResult<()> {
    std::fs::write(
        server_dir.join(FILE),
        serde_json::to_string_pretty(operators)?,
    )?;
    Ok(())
}

/// Looks the pseudo up at Mojang, then makes the player an operator.
pub async fn add_player(
    http: &reqwest::Client,
    paths: &AppPaths,
    processes: &ProcessManager,
    server_id: &str,
    name: &str,
) -> AppResult<Operator> {
    let profile = players::lookup(http, name.trim()).await?;
    let dir = paths.server_dir(server_id);
    let operator = Operator {
        uuid: profile.uuid,
        name: profile.name,
        level: default_level(&dir),
        bypasses_player_limit: false,
    };

    if is_running(processes, server_id).await {
        processes
            .send_command(server_id, &format!("op {}", operator.name))
            .await?;
    } else {
        save(&dir, &with_operator(load(&dir)?, operator.clone()))?;
    }
    Ok(operator)
}

pub async fn remove_player(
    paths: &AppPaths,
    processes: &ProcessManager,
    server_id: &str,
    uuid: &str,
) -> AppResult<()> {
    let dir = paths.server_dir(server_id);
    let operators = load(&dir)?;
    let Some(operator) = operators
        .iter()
        .find(|op| op.uuid.eq_ignore_ascii_case(uuid))
    else {
        return Ok(()); // Already removed.
    };

    if is_running(processes, server_id).await {
        processes
            .send_command(server_id, &format!("deop {}", operator.name))
            .await
    } else {
        save(&dir, &without_operator(operators.clone(), uuid))
    }
}

async fn is_running(processes: &ProcessManager, server_id: &str) -> bool {
    processes.status(server_id).await != ServerStatus::Stopped
}

/// `op-permission-level` from server.properties, 4 if missing.
fn default_level(server_dir: &Path) -> u8 {
    properties::Properties::load(&server_dir.join(server_files::PROPERTIES))
        .ok()
        .and_then(|props| props.get("op-permission-level")?.trim().parse().ok())
        .unwrap_or(DEFAULT_LEVEL)
}

/// Adds the operator, or refreshes their name if already there.
fn with_operator(mut operators: Vec<Operator>, operator: Operator) -> Vec<Operator> {
    match operators
        .iter_mut()
        .find(|op| op.uuid.eq_ignore_ascii_case(&operator.uuid))
    {
        Some(existing) => existing.name = operator.name,
        None => operators.push(operator),
    }
    operators
}

fn without_operator(operators: Vec<Operator>, uuid: &str) -> Vec<Operator> {
    operators
        .into_iter()
        .filter(|op| !op.uuid.eq_ignore_ascii_case(uuid))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn operator(uuid: &str, name: &str) -> Operator {
        Operator {
            uuid: uuid.into(),
            name: name.into(),
            level: 4,
            bypasses_player_limit: false,
        }
    }

    #[test]
    fn reads_the_server_format() {
        let json = r#"[{ "uuid": "a99441c3-e100-4fa9-9b3a-c1d7142543b7", "name": "Marius80",
                         "level": 4, "bypassesPlayerLimit": false }]"#;
        let operators: Vec<Operator> = serde_json::from_str(json).unwrap();
        assert_eq!(operators[0].name, "Marius80");
        let written = serde_json::to_string(&operators[0]).unwrap();
        assert!(written.contains("\"bypassesPlayerLimit\":false"));
    }

    #[test]
    fn adding_twice_keeps_one_entry() {
        let list = with_operator(Vec::new(), operator("a-1", "Old"));
        let list = with_operator(list, operator("A-1", "New"));
        assert_eq!(list, vec![operator("a-1", "New")]);
    }

    #[test]
    fn removes_by_uuid() {
        let list = vec![operator("a-1", "Alice"), operator("b-2", "Bob")];
        assert_eq!(without_operator(list, "A-1"), vec![operator("b-2", "Bob")]);
    }
}
