mod commands;
mod core;
mod download;
mod error;
mod events;
mod paths;
mod state;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tauri::{Manager, Window, WindowEvent};

use crate::core::process::{ProcessManager, ServerEvent, ServerStatus};
use crate::core::tunnel::TunnelManager;
use crate::paths::AppPaths;
use crate::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let result = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let paths = AppPaths::from_env();

            let handle = app.handle().clone();
            let tunnels = TunnelManager::new(
                paths.clone(),
                Arc::new(move |state| events::emit_tunnel_state(&handle, state)),
            );

            // The tunnel manager follows server statuses, in order.
            let (status_tx, mut status_rx) =
                tokio::sync::mpsc::unbounded_channel::<(String, ServerStatus)>();
            let status_tunnels = tunnels.clone();
            tauri::async_runtime::spawn(async move {
                while let Some((id, status)) = status_rx.recv().await {
                    status_tunnels.on_server_status(&id, status).await;
                }
            });

            let handle = app.handle().clone();
            let processes = ProcessManager::new(
                paths.clone(),
                Arc::new(move |event| {
                    if let ServerEvent::Status { id, status } = &event {
                        // Only fails once the app is shutting down.
                        let _ = status_tx.send((id.clone(), *status));
                    }
                    events::forward_server_event(&handle, event)
                }),
            );
            app.manage(AppState {
                paths,
                http: download::http_client()?,
                processes,
                tunnels,
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::servers::list_servers,
            commands::servers::list_versions,
            commands::servers::create_server,
            commands::servers::delete_server,
            commands::folders::servers_folder_path,
            commands::folders::open_servers_folder,
            commands::folders::open_server_folder,
            commands::process::start_server,
            commands::process::stop_server,
            commands::process::restart_server,
            commands::process::send_command,
            commands::whitelist::list_whitelist,
            commands::whitelist::add_to_whitelist,
            commands::whitelist::remove_from_whitelist,
            commands::whitelist::player_skin,
            commands::operators::list_operators,
            commands::operators::add_operator,
            commands::operators::remove_operator,
            commands::settings::game_settings_catalog,
            commands::settings::get_game_settings,
            commands::settings::save_game_settings,
            commands::tunnel::get_tunnel_state,
            commands::tunnel::link_playit,
            commands::tunnel::cancel_playit_link,
            commands::tunnel::unlink_playit,
            commands::tunnel::recheck_playit_agent,
            commands::tunnel::enable_internet_access,
            commands::tunnel::disable_internet_access,
        ])
        .on_window_event(on_window_event)
        .run(tauri::generate_context!());

    if let Err(err) = result {
        eprintln!("error while running tauri application: {err}");
        std::process::exit(1);
    }
}

/// Set once the user asked to close, so a second click does not start over.
static CLOSING: AtomicBool = AtomicBool::new(false);

/// On close, stop every running server cleanly before the window goes away
/// (CLAUDE.md 5.5), then the playit agent. The UI shows an overlay meanwhile
/// (`app-closing` event).
fn on_window_event(window: &Window, event: &WindowEvent) {
    let WindowEvent::CloseRequested { api, .. } = event else {
        return;
    };
    api.prevent_close();
    if CLOSING.swap(true, Ordering::SeqCst) {
        return;
    }

    let window = window.clone();
    let processes = window.state::<AppState>().processes.clone();
    let tunnels = window.state::<AppState>().tunnels.clone();
    events::emit_app_closing(window.app_handle());
    tauri::async_runtime::spawn(async move {
        processes.stop_all().await;
        tunnels.shutdown().await;
        // `destroy` skips `CloseRequested`, so we do not end up here again.
        if let Err(err) = window.destroy() {
            eprintln!("failed to close the window: {err}");
            std::process::exit(0);
        }
    });
}
