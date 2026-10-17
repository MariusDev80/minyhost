//! Commands for mods, plugins and datapacks (Modrinth, CurseForge).
//!
//! Changing the content of a server requires it to be stopped: a running
//! server keeps its jars open, and would only see the change after a restart.

use serde::Serialize;
use tauri::State;

use crate::core::content::modpack::{self, ModpackVersion};
use crate::core::content::{
    self, curseforge, ContentKind, InstallOutcome, InstalledContent, SearchPage, Source, Target,
};
use crate::core::instances;
use crate::core::process::ServerStatus;
use crate::error::{AppError, AppResult};
use crate::state::AppState;

/// Mirrored by `ContentSources` in `src/types/index.ts`.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentSources {
    /// Only offered when this build has a CurseForge API key.
    curseforge: bool,
}

#[tauri::command]
pub fn content_sources() -> ContentSources {
    ContentSources {
        curseforge: curseforge::is_available(),
    }
}

/// Content compatible with `server_id` (Minecraft version and server type),
/// or any modpack when `server_id` is `None`.
#[tauri::command]
pub async fn search_content(
    state: State<'_, AppState>,
    source: Source,
    kind: ContentKind,
    query: String,
    server_id: Option<String>,
    offset: u32,
) -> AppResult<SearchPage> {
    let target = match server_id {
        Some(id) => Some(Target::new(&instances::load(&state.paths, &id)?, kind)?),
        None => None,
    };
    content::search(&state.http, source, kind, &query, target.as_ref(), offset).await
}

/// Versions of a modpack, newest first.
#[tauri::command]
pub async fn list_modpack_versions(
    state: State<'_, AppState>,
    source: Source,
    project_id: String,
) -> AppResult<Vec<ModpackVersion>> {
    modpack::versions(&state.http, source, &project_id).await
}

/// Project icon as a data URL.
#[tauri::command]
pub async fn content_icon(state: State<'_, AppState>, url: String) -> AppResult<String> {
    content::icon_data_url(&state.http, &url).await
}

#[tauri::command]
pub async fn list_content(
    state: State<'_, AppState>,
    id: String,
) -> AppResult<Vec<InstalledContent>> {
    let instance = instances::load(&state.paths, &id)?;
    content::list_installed(&state.paths.server_dir(&id), instance.loader)
}

/// Installs the latest compatible version of a project and what it needs.
#[tauri::command]
pub async fn install_content(
    state: State<'_, AppState>,
    id: String,
    source: Source,
    kind: ContentKind,
    project_id: String,
) -> AppResult<InstallOutcome> {
    let instance = instances::load(&state.paths, &id)?;
    let target = Target::new(&instance, kind)?;
    let _lock = state.content_lock.lock().await;
    ensure_stopped(&state, &id).await?;
    content::install(
        &state.http,
        &state.paths.server_dir(&id),
        source,
        &project_id,
        &target,
    )
    .await
}

#[tauri::command]
pub async fn remove_content(
    state: State<'_, AppState>,
    id: String,
    kind: ContentKind,
    file_name: String,
) -> AppResult<()> {
    instances::load(&state.paths, &id)?;
    let _lock = state.content_lock.lock().await;
    ensure_stopped(&state, &id).await?;
    content::remove(&state.paths.server_dir(&id), kind, &file_name)
}

#[tauri::command]
pub async fn set_content_enabled(
    state: State<'_, AppState>,
    id: String,
    kind: ContentKind,
    file_name: String,
    enabled: bool,
) -> AppResult<()> {
    instances::load(&state.paths, &id)?;
    let _lock = state.content_lock.lock().await;
    ensure_stopped(&state, &id).await?;
    content::set_enabled(&state.paths.server_dir(&id), kind, &file_name, enabled)
}

async fn ensure_stopped(state: &AppState, id: &str) -> AppResult<()> {
    if state.processes.status(id).await == ServerStatus::Stopped {
        Ok(())
    } else {
        Err(AppError::MustBeStopped)
    }
}
