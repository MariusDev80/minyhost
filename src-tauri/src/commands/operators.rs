//! Commands for the operators tab.

use tauri::State;

use crate::core::instances;
use crate::core::operators::{self, Operator};
use crate::error::AppResult;
use crate::state::AppState;

#[tauri::command]
pub async fn list_operators(state: State<'_, AppState>, id: String) -> AppResult<Vec<Operator>> {
    instances::load(&state.paths, &id)?;
    operators::load(&state.paths.server_dir(&id))
}

/// Looks the pseudo up at Mojang, then makes the player an operator.
#[tauri::command]
pub async fn add_operator(
    state: State<'_, AppState>,
    id: String,
    name: String,
) -> AppResult<Operator> {
    instances::load(&state.paths, &id)?;
    operators::add_player(&state.http, &state.paths, &state.processes, &id, &name).await
}

#[tauri::command]
pub async fn remove_operator(
    state: State<'_, AppState>,
    id: String,
    uuid: String,
) -> AppResult<()> {
    instances::load(&state.paths, &id)?;
    operators::remove_player(&state.paths, &state.processes, &id, &uuid).await
}
