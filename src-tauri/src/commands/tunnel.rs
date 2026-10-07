//! Commands for Internet access through playit.gg (`core/tunnel.rs`).
//! State changes arrive through the `tunnel-state` event (see `events.rs`).

use tauri::State;

use crate::core::tunnel::TunnelState;
use crate::error::AppResult;
use crate::state::AppState;

#[tauri::command]
pub async fn get_tunnel_state(state: State<'_, AppState>) -> AppResult<TunnelState> {
    Ok(state.tunnels.state().await)
}

/// Returns the playit.gg page where the user approves MinyHost.
#[tauri::command]
pub async fn link_playit(state: State<'_, AppState>) -> AppResult<String> {
    state.tunnels.start_link().await
}

#[tauri::command]
pub async fn cancel_playit_link(state: State<'_, AppState>) -> AppResult<()> {
    state.tunnels.cancel_link().await;
    Ok(())
}

#[tauri::command]
pub async fn unlink_playit(state: State<'_, AppState>) -> AppResult<()> {
    state.tunnels.unlink().await
}

#[tauri::command]
pub async fn enable_internet_access(state: State<'_, AppState>, id: String) -> AppResult<()> {
    let status = state.processes.status(&id).await;
    state.tunnels.enable(&id, status).await
}

#[tauri::command]
pub async fn disable_internet_access(state: State<'_, AppState>, id: String) -> AppResult<()> {
    state.tunnels.disable(&id).await
}

/// Tries the agent again, e.g. after old agents were deleted on playit.gg.
#[tauri::command]
pub async fn recheck_playit_agent(state: State<'_, AppState>) -> AppResult<()> {
    state.tunnels.recheck().await
}
