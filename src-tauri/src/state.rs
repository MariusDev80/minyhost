//! State shared by all Tauri commands (`State<'_, AppState>`).

use crate::core::process::ProcessManager;
use crate::paths::AppPaths;

pub struct AppState {
    pub paths: AppPaths,
    /// Shared HTTP client (keeps connections alive, sets the User-Agent).
    pub http: reqwest::Client,
    /// Registry of running servers.
    pub processes: ProcessManager,
}
