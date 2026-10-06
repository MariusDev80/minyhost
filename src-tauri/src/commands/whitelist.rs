//! Commands for the whitelist tab and player heads.

use tauri::State;

use crate::core::whitelist::{self, WhitelistEntry};
use crate::core::{instances, players};
use crate::error::AppResult;
use crate::state::AppState;

#[tauri::command]
pub async fn list_whitelist(
    state: State<'_, AppState>,
    id: String,
) -> AppResult<Vec<WhitelistEntry>> {
    instances::load(&state.paths, &id)?;
    whitelist::load(&state.paths.server_dir(&id))
}

/// Looks the pseudo up at Mojang, then adds the player.
#[tauri::command]
pub async fn add_to_whitelist(
    state: State<'_, AppState>,
    id: String,
    name: String,
) -> AppResult<WhitelistEntry> {
    instances::load(&state.paths, &id)?;
    whitelist::add_player(&state.http, &state.paths, &state.processes, &id, &name).await
}

#[tauri::command]
pub async fn remove_from_whitelist(
    state: State<'_, AppState>,
    id: String,
    uuid: String,
) -> AppResult<()> {
    instances::load(&state.paths, &id)?;
    whitelist::remove_player(&state.paths, &state.processes, &id, &uuid).await
}

/// Skin PNG as a data URL (Steve if the player has no custom skin).
#[tauri::command]
pub async fn player_skin(state: State<'_, AppState>, uuid: String) -> AppResult<String> {
    players::skin_data_url(&state.http, &uuid).await
}
