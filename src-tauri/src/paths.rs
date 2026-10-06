//! Application data paths (`%APPDATA%/MinyHost`), see CLAUDE.md section 4.
//!
//! Every path the app writes to is built here, so the storage layout can be
//! read (and changed) in one place.

use std::path::{Path, PathBuf};

const APP_DIR_NAME: &str = "MinyHost";

#[derive(Debug, Clone)]
pub struct AppPaths {
    root: PathBuf,
}

impl AppPaths {
    /// Uses `%APPDATA%/MinyHost` on Windows (`~/.local/share/MinyHost` elsewhere).
    pub fn from_env() -> Self {
        let base = std::env::var_os("APPDATA")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|home| Path::new(&home).join(".local/share")))
            .unwrap_or_else(|| PathBuf::from("."));
        Self::new(base.join(APP_DIR_NAME))
    }

    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// `java/` — one folder per Java major version.
    pub fn java_dir(&self, version: u32) -> PathBuf {
        self.root.join("java").join(version.to_string())
    }

    /// `servers/` — one folder per server instance.
    pub fn servers_dir(&self) -> PathBuf {
        self.root.join("servers")
    }

    /// `servers/<id>/` — everything a server needs lives in this folder.
    pub fn server_dir(&self, id: &str) -> PathBuf {
        self.servers_dir().join(id)
    }
}

/// Files inside a server folder.
pub mod server_files {
    pub const INSTANCE: &str = "instance.json";
    pub const JAR: &str = "server.jar";
    pub const EULA: &str = "eula.txt";
    pub const PROPERTIES: &str = "server.properties";
}
