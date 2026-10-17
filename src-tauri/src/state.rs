//! State shared by all Tauri commands (`State<'_, AppState>`).

use crate::core::process::ProcessManager;
use crate::core::tunnel::TunnelManager;
use crate::paths::AppPaths;

pub struct AppState {
    pub paths: AppPaths,
    /// Shared HTTP client (keeps connections alive, sets the User-Agent).
    pub http: reqwest::Client,
    /// Registry of running servers.
    pub processes: ProcessManager,
    /// Internet access through playit.gg.
    pub tunnels: TunnelManager,
    /// Held while mods, plugins or datapacks are being changed, so two
    /// installs never write `content.json` at the same time.
    pub content_lock: tokio::sync::Mutex<()>,
}
