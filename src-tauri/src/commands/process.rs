//! Commands to start, stop and talk to a running server.
//! Status changes and console lines arrive through events (see `events.rs`).

use tauri::State;

use crate::core::process::StopOutcome;
use crate::error::AppResult;
use crate::state::AppState;

#[tauri::command]
pub async fn start_server(state: State<'_, AppState>, id: String) -> AppResult<()> {
    state.processes.start(&id).await
}

/// Resolves once the process has exited (up to ~30 s).
#[tauri::command]
pub async fn stop_server(state: State<'_, AppState>, id: String) -> AppResult<StopOutcome> {
    state.processes.stop(&id).await
}

/// Stops then starts the server, e.g. to apply new settings.
#[tauri::command]
pub async fn restart_server(state: State<'_, AppState>, id: String) -> AppResult<StopOutcome> {
    let outcome = state.processes.stop(&id).await?;
    state.processes.start(&id).await?;
    Ok(outcome)
}

#[tauri::command]
pub async fn send_command(
    state: State<'_, AppState>,
    id: String,
    command: String,
) -> AppResult<()> {
    state.processes.send_command(&id, &command).await
}
