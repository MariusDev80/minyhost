//! Commands that open MinyHost folders in the file explorer.
//! The frontend never gets a path to open itself: it asks for a folder by name.

use std::path::Path;

use tauri::State;

use crate::core::instances;
use crate::error::{AppError, AppResult};
use crate::state::AppState;

/// Where the servers live, shown in the Settings page.
#[tauri::command]
pub fn servers_folder_path(state: State<'_, AppState>) -> String {
    state.paths.servers_dir().display().to_string()
}

#[tauri::command]
pub fn open_servers_folder(state: State<'_, AppState>) -> AppResult<()> {
    let dir = state.paths.servers_dir();
    // Before the first server, the folder does not exist yet.
    std::fs::create_dir_all(&dir)?;
    open_folder(&dir)
}

#[tauri::command]
pub fn open_server_folder(state: State<'_, AppState>, id: String) -> AppResult<()> {
    // Only a real instance: `id` must not point anywhere else on the disk.
    let instance = instances::load(&state.paths, &id)?;
    open_folder(&state.paths.server_dir(&instance.id))
}

fn open_folder(dir: &Path) -> AppResult<()> {
    tauri_plugin_opener::open_path(dir, None::<&str>)
        .map_err(|err| AppError::Io(std::io::Error::other(err)))
}
