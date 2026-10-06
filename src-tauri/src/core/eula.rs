//! Mojang EULA (`eula.txt`).
//!
//! The EULA is only ever accepted by the user, through the checkbox of the
//! creation form. Never call `accept` without that explicit consent.

use std::path::Path;

use crate::error::AppResult;
use crate::paths::server_files;

pub const EULA_URL: &str = "https://aka.ms/MinecraftEULA";

/// Writes `eula=true` in the server folder.
pub fn accept(server_dir: &Path) -> AppResult<()> {
    let text = format!("# Accepted by the user in MinyHost ({EULA_URL}).\neula=true\n");
    std::fs::write(server_dir.join(server_files::EULA), text)?;
    Ok(())
}

pub fn is_accepted(server_dir: &Path) -> bool {
    std::fs::read_to_string(server_dir.join(server_files::EULA))
        .map(|text| text.lines().any(|line| line.trim() == "eula=true"))
        .unwrap_or(false)
}
