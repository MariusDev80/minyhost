//! Commands for the game settings (creation form and "Paramètres" tab).

use tauri::State;

use crate::core::game_settings::{self, Catalog, GameSettings, SaveOutcome};
use crate::core::instances;
use crate::error::AppResult;
use crate::state::AppState;

/// Settings available for a Minecraft version (types, defaults, limits).
#[tauri::command]
pub fn game_settings_catalog(mc_version: String) -> Catalog {
    game_settings::catalog(&mc_version)
}

/// Current values: world settings from server.properties, game rules from
/// the world (or the console, if the server runs).
#[tauri::command]
pub async fn get_game_settings(state: State<'_, AppState>, id: String) -> AppResult<GameSettings> {
    let instance = instances::load(&state.paths, &id)?;
    let live_rules = state.processes.live_rules(&id).await;
    game_settings::read(&state.paths.server_dir(&id), &instance, &live_rules)
}

/// Saves the settings the user changed (only those are sent).
#[tauri::command]
pub async fn save_game_settings(
    state: State<'_, AppState>,
    id: String,
    settings: GameSettings,
) -> AppResult<SaveOutcome> {
    game_settings::save(&state.paths, &state.processes, &id, &settings).await
}
