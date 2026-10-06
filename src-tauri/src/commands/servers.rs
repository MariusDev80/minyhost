//! Commands for the server list, creation and deletion.

use serde::Serialize;
use tauri::{AppHandle, State};

use crate::core::create::{self, NewServer};
use crate::core::instances::{self, Instance};
use crate::core::process::ServerStatus;
use crate::core::providers::{self, Loader};
use crate::error::{AppError, AppResult};
use crate::events;
use crate::state::AppState;

/// An instance plus its live status. Mirrored by `ServerInfo` in `src/types/index.ts`.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    #[serde(flatten)]
    instance: Instance,
    status: ServerStatus,
}

#[tauri::command]
pub async fn list_servers(state: State<'_, AppState>) -> AppResult<Vec<ServerInfo>> {
    let statuses = state.processes.statuses().await;
    let servers = instances::list(&state.paths)?
        .into_iter()
        .map(|instance| {
            let status = statuses
                .get(&instance.id)
                .copied()
                .unwrap_or(ServerStatus::Stopped);
            ServerInfo { instance, status }
        })
        .collect();
    Ok(servers)
}

#[tauri::command]
pub async fn list_versions(state: State<'_, AppState>, loader: Loader) -> AppResult<Vec<String>> {
    providers::list_versions(&state.http, loader).await
}

/// Long-running: reports its progress through the `create-progress` event.
#[tauri::command]
pub async fn create_server(
    app: AppHandle,
    state: State<'_, AppState>,
    input: NewServer,
) -> AppResult<Instance> {
    create::create_server(&state.http, &state.paths, input, |progress| {
        events::emit_create_progress(&app, progress)
    })
    .await
}

#[tauri::command]
pub async fn delete_server(state: State<'_, AppState>, id: String) -> AppResult<()> {
    if state.processes.status(&id).await != ServerStatus::Stopped {
        return Err(AppError::AlreadyRunning);
    }
    // Deleting a world can take a while: run it off the async threads.
    let paths = state.paths.clone();
    tokio::task::spawn_blocking(move || instances::delete(&paths, &id))
        .await
        .map_err(|err| AppError::Io(std::io::Error::other(err)))?
}
