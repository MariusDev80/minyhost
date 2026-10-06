//! Events sent from Rust to the frontend.
//!
//! Every event name and payload is defined here. The frontend side lives in
//! `src/lib/tauri.ts` (`events`) and `src/types/index.ts`: keep them aligned.

use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::core::create::CreateProgress;
use crate::core::process::{ConsoleStream, ServerEvent, ServerStatus};

pub const CONSOLE_LINE: &str = "console-line";
pub const SERVER_STATUS: &str = "server-status";
pub const SERVER_CRASHED: &str = "server-crashed";
pub const CREATE_PROGRESS: &str = "create-progress";
pub const APP_CLOSING: &str = "app-closing";

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ConsoleLinePayload {
    id: String,
    line: String,
    stream: ConsoleStream,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ServerStatusPayload {
    id: String,
    status: ServerStatus,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ServerCrashedPayload {
    id: String,
    exit_code: Option<i32>,
}

/// Turns an event of the process manager into a frontend event.
pub fn forward_server_event(app: &AppHandle, event: ServerEvent) {
    let result = match event {
        ServerEvent::Console { id, line, stream } => {
            app.emit(CONSOLE_LINE, ConsoleLinePayload { id, line, stream })
        }
        ServerEvent::Status { id, status } => {
            app.emit(SERVER_STATUS, ServerStatusPayload { id, status })
        }
        ServerEvent::Crashed { id, exit_code } => {
            app.emit(SERVER_CRASHED, ServerCrashedPayload { id, exit_code })
        }
    };
    log_emit_error(result);
}

pub fn emit_create_progress(app: &AppHandle, progress: CreateProgress) {
    log_emit_error(app.emit(CREATE_PROGRESS, progress));
}

pub fn emit_app_closing(app: &AppHandle) {
    log_emit_error(app.emit(APP_CLOSING, ()));
}

fn log_emit_error(result: tauri::Result<()>) {
    if let Err(err) = result {
        eprintln!("failed to emit event: {err}");
    }
}
