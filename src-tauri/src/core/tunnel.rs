//! Internet access through playit.gg (CLAUDE.md 5.7).
//!
//! MinyHost embeds the official playit agent (`playit-agent-core`, BSD-2-Clause):
//! no extra program, service or admin rights. Each user links their own
//! playit.gg account; MinyHost never shares access with anyone else.
//!
//! - Linking: MinyHost generates a claim code, the user approves it on
//!   `playit.gg/claim/<code>`, and the code is exchanged for an agent secret,
//!   stored in `playit.json`.
//! - One `minecraft-java` tunnel per server, created only when the user turns
//!   Internet access on. Its id lives in `instance.json` (`playitTunnelId`).
//! - The agent only runs while a server with Internet access is running, and
//!   only forwards the tunnels of those servers (to `127.0.0.1:<port>`).
//!
//! Like `ProcessManager`, `TunnelManager` does not know about Tauri: it reports
//! every state change through a `TunnelSink` callback.

use std::collections::{HashMap, HashSet};
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::time::Duration;

use playit_agent_core::agent_control::errors::SetupError;
use playit_agent_core::network::origin_lookup::{OriginLookup, OriginResource};
use playit_agent_core::network::tcp::tcp_settings::TcpSettings;
use playit_agent_core::network::udp::udp_settings::UdpSettings;
use playit_agent_core::playit_agent::{PlayitAgent, PlayitAgentSettings};
use playit_api_client::api::{
    AccountStatus, AccountTunnelOriginCreate, AgentOrigin, AgentRunDataV1, AgentTunnelAttr,
    AgentTunnelConfig, ApiError, ApiErrorNoFail, ApiResponseError, ApiResult, AuthError,
    ClaimAgentType, ClaimExchangeError, ClaimSetupError, ClaimSetupResponse,
    CreateTunnelAllocationRequest, ObjectId, PlayitHttpClient, PlayitNetwork, ProtoRegisterError,
    ReqAgentsRename, ReqClaimExchange, ReqClaimSetup, ReqTunnelsDelete, TunnelPortDetails,
    TunnelType, UseAllocRegion,
};
use playit_api_client::PlayitApi;
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, Notify};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::core::instances;
use crate::core::process::ServerStatus;
use crate::error::{AppError, AppResult};
use crate::paths::AppPaths;

const API_BASE: &str = "https://api.playit.gg";
/// Shown to the user on the playit.gg approval page.
const AGENT_VERSION_TEXT: &str = concat!("MinyHost ", env!("CARGO_PKG_VERSION"));
/// How long a claim code waits for the user's approval.
const CLAIM_TIMEOUT: Duration = Duration::from_secs(15 * 60);
const CLAIM_POLL: Duration = Duration::from_secs(2);
/// How often tunnel addresses and notices are refreshed while the agent runs.
const REFRESH_INTERVAL: Duration = Duration::from_secs(5);
/// Delay before reconnecting after the agent lost its connection.
const RECONNECT_DELAY: Duration = Duration::from_secs(10);
/// How long the agent stays connected right after linking: the playit.gg
/// page waits for the agent to come online before it shows the setup as done.
const WELCOME_CONNECTION: Duration = Duration::from_secs(20);
/// playit.gg rejects non-ASCII or long tunnel names.
const TUNNEL_NAME_MAX: usize = 32;

/// Everything the UI shows about Internet access. Mirrored by `TunnelState`
/// in `src/types/index.ts`.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TunnelState {
    pub link: LinkState,
    /// Why the last linking attempt failed, until the next attempt.
    pub link_error: Option<LinkError>,
    pub agent: AgentStatus,
    /// Messages from playit.gg about the account (English), most important first.
    pub notices: Vec<Notice>,
    /// Tunnel of each server with Internet access, by server id.
    pub tunnels: HashMap<String, ServerTunnel>,
    /// The email of the playit.gg account is not verified yet: some tunnels
    /// cannot be created until it is.
    pub email_unverified: bool,
    /// playit.gg refused to connect this agent: the account has reached its
    /// maximum number of agents (old ones must be deleted on playit.gg).
    pub agent_over_limit: bool,
    /// Agent used by MinyHost, to tell it apart on playit.gg (older MinyHost
    /// agents may still be listed there).
    pub agent_id: Option<String>,
    /// Name MinyHost gave it on playit.gg (`MinyHost <PC> <date>`).
    pub agent_name: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(tag = "state", rename_all = "camelCase")]
pub enum LinkState {
    #[default]
    Unlinked,
    /// Waiting for the user to approve MinyHost on this page.
    Linking {
        url: String,
    },
    Linked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LinkError {
    Rejected,
    Expired,
    /// The secret was revoked on playit.gg (agent deleted…).
    Revoked,
    Failed,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AgentStatus {
    /// No server with Internet access is running.
    #[default]
    Stopped,
    Connecting,
    Online,
    /// Could not reach playit.gg; retrying.
    Error,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Notice {
    pub message: String,
    pub link: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTunnel {
    /// Address to give to friends, once playit.gg has assigned it.
    pub address: Option<String>,
    /// Disabled by playit.gg (account limits…), with its reason.
    pub disabled_reason: Option<String>,
}

pub type TunnelSink = Arc<dyn Fn(TunnelState) + Send + Sync>;

/// Content of `playit.json`.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SecretFile {
    secret_key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    agent_name: Option<String>,
}

#[derive(Default)]
struct Inner {
    secret: Option<String>,
    state: TunnelState,
    /// Polls playit.gg while the user approves the claim code.
    claim_task: Option<JoinHandle<()>>,
    /// Running servers with Internet access: server id -> local port.
    active: HashMap<String, u16>,
    /// Running agent, if any server of `active` exists.
    agent: Option<AgentHandle>,
    /// Last state sent through the sink, to skip identical updates.
    emitted: Option<TunnelState>,
    /// Naming the agent was tried in this session (it is done only once).
    naming_tried: bool,
}

struct AgentHandle {
    cancel: CancellationToken,
    /// Wakes the agent loop to reload tunnels right away.
    refresh: Arc<Notify>,
}

#[derive(Clone)]
pub struct TunnelManager {
    paths: AppPaths,
    inner: Arc<Mutex<Inner>>,
    sink: TunnelSink,
}

impl TunnelManager {
    pub fn new(paths: AppPaths, sink: TunnelSink) -> Self {
        let file = load_secret_file(&paths);
        let secret = file.as_ref().map(|f| f.secret_key.clone());
        let state = TunnelState {
            link: if secret.is_some() {
                LinkState::Linked
            } else {
                LinkState::Unlinked
            },
            agent_name: file.and_then(|f| f.agent_name),
            ..TunnelState::default()
        };
        Self {
            paths,
            inner: Arc::new(Mutex::new(Inner {
                secret,
                state,
                ..Inner::default()
            })),
            sink,
        }
    }

    /// Current state, with tunnel addresses fetched from playit.gg when the
    /// agent is not running (it refreshes them by itself otherwise).
    pub async fn state(&self) -> TunnelState {
        let (secret, agent_running) = {
            let inner = self.inner.lock().await;
            (inner.secret.clone(), inner.agent.is_some())
        };
        if let (Some(secret), false) = (secret, agent_running) {
            let api = PlayitApi::create(API_BASE.to_string(), Some(secret));
            if let Err(err) = self.refresh(&api, None).await {
                eprintln!("playit: failed to load tunnels: {err}");
            }
        }
        self.inner.lock().await.state.clone()
    }

    /// Starts linking a playit.gg account and returns the page where the user
    /// approves it. Progress is reported through the sink.
    pub async fn start_link(&self) -> AppResult<String> {
        let mut inner = self.inner.lock().await;
        match &inner.state.link {
            LinkState::Linked => return Err(AppError::InvalidInput("already linked".into())),
            LinkState::Linking { url } => return Ok(url.clone()),
            LinkState::Unlinked => {}
        }

        // playit.gg expects a hex code; 16 random hex digits are plenty.
        let code: String = Uuid::new_v4()
            .simple()
            .to_string()
            .chars()
            .take(16)
            .collect();
        let url = format!("https://playit.gg/claim/{code}");
        inner.state.link = LinkState::Linking { url: url.clone() };
        inner.state.link_error = None;

        let manager = self.clone();
        inner.claim_task = Some(tokio::spawn(async move {
            let result = claim(&code).await;
            manager.finish_link(result).await;
        }));
        self.emit(&mut inner);
        Ok(url)
    }

    pub async fn cancel_link(&self) {
        let mut inner = self.inner.lock().await;
        if let Some(task) = inner.claim_task.take() {
            task.abort();
        }
        if matches!(inner.state.link, LinkState::Linking { .. }) {
            inner.state.link = LinkState::Unlinked;
            self.emit(&mut inner);
        }
    }

    async fn finish_link(&self, result: Result<String, LinkError>) {
        let mut inner = self.inner.lock().await;
        inner.claim_task = None;
        match result.and_then(|secret| {
            save_secret(&self.paths, &secret).map_err(|err| {
                eprintln!("playit: failed to save the secret: {err}");
                LinkError::Failed
            })?;
            Ok(secret)
        }) {
            Ok(secret) => {
                inner.secret = Some(secret.clone());
                inner.state.link = LinkState::Linked;
                inner.state.agent_name = None;
                inner.naming_tried = false;
                self.sync_agent(&mut inner);
                tokio::spawn(self.clone().welcome(secret));
            }
            Err(error) => {
                inner.state.link = LinkState::Unlinked;
                inner.state.link_error = Some(error);
            }
        }
        self.emit(&mut inner);
    }

    /// Right after linking: connects the agent for a moment, so that the
    /// playit.gg page sees it online, and loads its id and name.
    async fn welcome(self, secret: String) {
        let connection = self.ensure_connected(&secret).await;
        let api = PlayitApi::create(API_BASE.to_string(), Some(secret));
        if let Err(err) = self.refresh(&api, None).await {
            eprintln!("playit: failed to load the agent: {err}");
        }
        if let Ok(Some(_)) = &connection {
            tokio::time::sleep(WELCOME_CONNECTION).await;
        }
        drop(connection);
    }

    /// Checks again whether playit.gg accepts the agent (after the user
    /// deleted old agents): the "too many agents" warning goes away if so.
    pub async fn recheck(&self) -> AppResult<()> {
        let secret = self.secret().await?;
        let refresh = self
            .inner
            .lock()
            .await
            .agent
            .as_ref()
            .map(|agent| agent.refresh.clone());
        match refresh {
            // The running agent retries by itself; just wake it up.
            Some(refresh) => {
                refresh.notify_one();
                Ok(())
            }
            None => self.ensure_connected(&secret).await.map(drop),
        }
    }

    /// Forgets the playit.gg account: stops the agent, deletes the tunnels
    /// MinyHost created (best effort) and the stored secret.
    pub async fn unlink(&self) -> AppResult<()> {
        let secret = {
            let mut inner = self.inner.lock().await;
            if let Some(task) = inner.claim_task.take() {
                task.abort();
            }
            stop_agent(&mut inner);
            inner.secret.take()
        };

        if let Some(secret) = secret {
            let api = PlayitApi::create(API_BASE.to_string(), Some(secret));
            for mut instance in instances::list(&self.paths)? {
                let Some(tunnel_id) = instance.playit_tunnel_id.take() else {
                    continue;
                };
                delete_tunnel(&api, tunnel_id).await;
                instances::save(&self.paths, &instance)?;
            }
        }
        match std::fs::remove_file(self.paths.playit_file()) {
            Err(err) if err.kind() != std::io::ErrorKind::NotFound => return Err(err.into()),
            _ => {}
        }

        let mut inner = self.inner.lock().await;
        inner.state = TunnelState::default();
        self.emit(&mut inner);
        Ok(())
    }

    /// Opens a server to the Internet: creates its tunnel if needed.
    pub async fn enable(&self, id: &str, status: ServerStatus) -> AppResult<()> {
        let secret = self.secret().await?;
        let api = PlayitApi::create(API_BASE.to_string(), Some(secret.clone()));
        let mut instance = instances::load(&self.paths, id)?;

        let run_data = api.v1_agents_rundata().await.map_err(api_error_no_fail)?;
        // A tunnel deleted on playit.gg is created again.
        let exists = instance.playit_tunnel_id.is_some_and(|tunnel_id| {
            run_data.tunnels.iter().any(|t| t.id == tunnel_id)
                || run_data.pending.iter().any(|p| p.id == tunnel_id)
        });
        if !exists {
            // playit.gg only creates tunnels for an agent that has connected
            // (it registers its version then: "AgentVersionTooOld" otherwise).
            // The official plugin also connects first. Kept until the end.
            let _connection = self.ensure_connected(&secret).await?;
            let tunnel_id = create_tunnel(&api, run_data.agent_id, &tunnel_name(id)).await?;
            instance.playit_tunnel_id = Some(tunnel_id);
            instances::save(&self.paths, &instance)?;
        }

        {
            let mut inner = self.inner.lock().await;
            if status != ServerStatus::Stopped {
                inner.active.insert(id.to_string(), instance.port);
            }
            self.sync_agent(&mut inner);
        }
        self.refresh(&api, None).await
    }

    /// Closes a server to the Internet and deletes its tunnel.
    pub async fn disable(&self, id: &str) -> AppResult<()> {
        let mut instance = instances::load(&self.paths, id)?;
        let Some(tunnel_id) = instance.playit_tunnel_id.take() else {
            return Ok(());
        };
        instances::save(&self.paths, &instance)?;

        let secret = {
            let mut inner = self.inner.lock().await;
            inner.active.remove(id);
            inner.state.tunnels.remove(id);
            self.sync_agent(&mut inner);
            self.emit(&mut inner);
            inner.secret.clone()
        };
        if let Some(secret) = secret {
            let api = PlayitApi::create(API_BASE.to_string(), Some(secret));
            delete_tunnel(&api, tunnel_id).await;
        }
        Ok(())
    }

    /// Follows server statuses: the agent runs while a server with Internet
    /// access is running.
    pub async fn on_server_status(&self, id: &str, status: ServerStatus) {
        let port = match status {
            ServerStatus::Starting => match instances::load(&self.paths, id) {
                Ok(instance) if instance.playit_tunnel_id.is_some() => Some(instance.port),
                _ => None,
            },
            ServerStatus::Stopped => None,
            // Running / stopping: keep what `Starting` decided.
            _ => return,
        };
        let mut inner = self.inner.lock().await;
        let changed = match port {
            Some(port) => inner.active.insert(id.to_string(), port).is_none(),
            None => inner.active.remove(id).is_some(),
        };
        if changed {
            self.sync_agent(&mut inner);
        }
    }

    /// Stops the agent (app closing).
    pub async fn shutdown(&self) {
        let mut inner = self.inner.lock().await;
        stop_agent(&mut inner);
    }

    /// Connects a short-lived agent if the main one is not online, so that
    /// playit.gg knows this agent (see `enable`). Disconnects when dropped.
    async fn ensure_connected(&self, secret: &str) -> AppResult<Option<TemporaryAgent>> {
        if self.inner.lock().await.state.agent == AgentStatus::Online {
            return Ok(None);
        }
        let settings = PlayitAgentSettings {
            api_url: API_BASE.to_string(),
            secret_key: secret.to_string(),
            tcp_settings: TcpSettings::default(),
            udp_settings: UdpSettings::default(),
        };
        // Empty lookup: this agent forwards nothing.
        match PlayitAgent::new(settings, Arc::new(OriginLookup::default())).await {
            Ok(agent) => {
                self.set_over_limit(false).await;
                let cancel = agent.cancellation_token();
                // Running keeps the connection alive (it forwards nothing).
                tokio::spawn(agent.run());
                Ok(Some(TemporaryAgent(cancel)))
            }
            Err(err) if is_revoked(&err) => Err(AppError::PlayitNotLinked),
            Err(err) if is_over_limit(&err) => {
                self.set_over_limit(true).await;
                Err(AppError::PlayitAgentLimit)
            }
            Err(err) => {
                eprintln!("playit: failed to connect: {err:?}");
                Err(AppError::Playit(format!(
                    "agent connection failed: {err:?}"
                )))
            }
        }
    }

    async fn secret(&self) -> AppResult<String> {
        self.inner
            .lock()
            .await
            .secret
            .clone()
            .ok_or(AppError::PlayitNotLinked)
    }

    /// Starts or stops the agent so that it runs iff the account is linked and
    /// a server with Internet access is running. Otherwise asks for a refresh.
    fn sync_agent(&self, inner: &mut Inner) {
        let wanted = inner.secret.is_some() && !inner.active.is_empty();
        match (&inner.agent, wanted) {
            (Some(agent), true) => agent.refresh.notify_one(),
            (Some(_), false) => {
                stop_agent(inner);
                inner.state.agent = AgentStatus::Stopped;
                self.emit(inner);
            }
            (None, true) => {
                let Some(secret) = inner.secret.clone() else {
                    return;
                };
                let handle = AgentHandle {
                    cancel: CancellationToken::new(),
                    refresh: Arc::new(Notify::new()),
                };
                tokio::spawn(self.clone().run_agent(
                    secret,
                    handle.cancel.clone(),
                    handle.refresh.clone(),
                ));
                inner.agent = Some(handle);
                inner.state.agent = AgentStatus::Connecting;
                self.emit(inner);
            }
            (None, false) => {}
        }
    }

    /// Agent loop: connects, forwards traffic, refreshes tunnels, reconnects.
    async fn run_agent(self, secret: String, cancel: CancellationToken, refresh: Arc<Notify>) {
        let api = PlayitApi::create(API_BASE.to_string(), Some(secret.clone()));
        let lookup = Arc::new(OriginLookup::default());

        while !cancel.is_cancelled() {
            self.set_agent_status(&cancel, AgentStatus::Connecting)
                .await;
            if let Err(err) = self.refresh(&api, Some(&lookup)).await {
                eprintln!("playit: failed to load tunnels: {err}");
            }

            let settings = PlayitAgentSettings {
                api_url: API_BASE.to_string(),
                secret_key: secret.clone(),
                tcp_settings: TcpSettings::default(),
                udp_settings: UdpSettings::default(),
            };
            let agent = tokio::select! {
                _ = cancel.cancelled() => return,
                agent = PlayitAgent::new(settings, lookup.clone()) => agent,
            };
            let agent = match agent {
                Ok(agent) => agent,
                Err(err) if is_revoked(&err) => {
                    eprintln!("playit: the agent secret is no longer valid: {err:?}");
                    self.revoke(&cancel).await;
                    return;
                }
                Err(err) => {
                    eprintln!("playit: failed to connect: {err:?}");
                    // Retried: the user may delete old agents on playit.gg meanwhile.
                    self.set_over_limit(is_over_limit(&err)).await;
                    self.set_agent_status(&cancel, AgentStatus::Error).await;
                    tokio::select! {
                        _ = cancel.cancelled() => return,
                        _ = tokio::time::sleep(RECONNECT_DELAY) => continue,
                    }
                }
            };

            let agent_cancel = agent.cancellation_token();
            let mut run = tokio::spawn(agent.run());
            self.set_over_limit(false).await;
            self.set_agent_status(&cancel, AgentStatus::Online).await;

            loop {
                tokio::select! {
                    _ = cancel.cancelled() => {
                        agent_cancel.cancel();
                        let _ = run.await;
                        return;
                    }
                    _ = &mut run => break,
                    _ = refresh.notified() => {}
                    _ = tokio::time::sleep(REFRESH_INTERVAL) => {}
                }
                if let Err(err) = self.refresh(&api, Some(&lookup)).await {
                    eprintln!("playit: failed to refresh tunnels: {err}");
                }
            }

            eprintln!("playit: connection lost, reconnecting");
            self.set_agent_status(&cancel, AgentStatus::Error).await;
            tokio::select! {
                _ = cancel.cancelled() => return,
                _ = tokio::time::sleep(RECONNECT_DELAY) => {}
            }
        }
    }

    /// Loads tunnels and notices from playit.gg into the state and, for a
    /// running agent, points the tunnels of running servers to their port.
    async fn refresh(&self, api: &PlayitApi, lookup: Option<&OriginLookup>) -> AppResult<()> {
        let run_data = match api.v1_agents_rundata().await {
            Ok(data) => data,
            Err(ApiErrorNoFail::ApiError(ApiResponseError::Auth(
                AuthError::InvalidAgentKey | AuthError::NoLongerValid,
            ))) => {
                self.revoke(&CancellationToken::new()).await;
                return Err(AppError::PlayitNotLinked);
            }
            Err(err) => return Err(api_error_no_fail(err)),
        };

        // Tunnel id -> server id, from the instances.
        let servers: HashMap<Uuid, String> = instances::list(&self.paths)?
            .into_iter()
            .filter_map(|instance| Some((instance.playit_tunnel_id?, instance.id)))
            .collect();

        let name_agent = {
            let mut inner = self.inner.lock().await;
            let first_try = inner.state.agent_name.is_none() && !inner.naming_tried;
            inner.naming_tried = true;
            first_try
        };
        if name_agent {
            self.name_agent(api, run_data.agent_id).await;
        }

        let mut inner = self.inner.lock().await;
        inner.state.agent_id = Some(run_data.agent_id.to_string());
        if let Some(lookup) = lookup {
            lookup
                .update(origins(&run_data, &servers, &inner.active).into_iter())
                .await;
        }
        let tunnels = server_tunnels(&run_data, &servers);
        let mut notices = run_data.notices;
        notices.sort_by_key(|notice| notice.priority);
        inner.state.tunnels = tunnels;
        inner.state.email_unverified =
            run_data.permissions.account_status == AccountStatus::EmailNotVerified;
        inner.state.notices = notices
            .into_iter()
            .map(|notice| Notice {
                message: notice.message.to_string(),
                link: notice.resolve_link,
            })
            .collect();
        self.emit(&mut inner);
        Ok(())
    }

    /// Gives the agent a name the user can recognize on playit.gg, and keeps it
    /// in `playit.json`. Best effort: the id is shown anyway.
    async fn name_agent(&self, api: &PlayitApi, agent_id: Uuid) {
        let date = &instances::now_iso8601()[..10];
        let computer = std::env::var("COMPUTERNAME")
            .or_else(|_| std::env::var("HOSTNAME"))
            .unwrap_or_default();
        // A long PC name may be refused: fall back to a shorter name.
        for name in [agent_name(&computer, date), agent_name("", date)] {
            let renamed = api
                .agents_rename(ReqAgentsRename {
                    agent_id,
                    name: name.clone(),
                })
                .await;
            match renamed {
                Ok(()) => {
                    let mut inner = self.inner.lock().await;
                    if let Some(secret) = inner.secret.clone() {
                        if let Err(err) = save_secret_file(&self.paths, &secret, Some(&name)) {
                            eprintln!("playit: failed to save the agent name: {err}");
                        }
                    }
                    inner.state.agent_name = Some(name);
                    self.emit(&mut inner);
                    return;
                }
                Err(err) => eprintln!("playit: failed to rename the agent to {name:?}: {err}"),
            }
        }
    }

    /// The secret was revoked on playit.gg: back to "not linked".
    async fn revoke(&self, agent_cancel: &CancellationToken) {
        agent_cancel.cancel();
        let mut inner = self.inner.lock().await;
        stop_agent(&mut inner);
        inner.secret = None;
        if let Err(err) = std::fs::remove_file(self.paths.playit_file()) {
            eprintln!("playit: failed to delete the secret: {err}");
        }
        inner.state = TunnelState {
            link_error: Some(LinkError::Revoked),
            ..TunnelState::default()
        };
        self.emit(&mut inner);
    }

    async fn set_over_limit(&self, over_limit: bool) {
        let mut inner = self.inner.lock().await;
        inner.state.agent_over_limit = over_limit;
        self.emit(&mut inner);
    }

    async fn set_agent_status(&self, cancel: &CancellationToken, status: AgentStatus) {
        let mut inner = self.inner.lock().await;
        // A stopped agent must not overwrite the state of the next one.
        if cancel.is_cancelled() || inner.state.agent == status {
            return;
        }
        inner.state.agent = status;
        self.emit(&mut inner);
    }

    /// Sends the state through the sink, unless it did not change.
    fn emit(&self, inner: &mut Inner) {
        if inner.emitted.as_ref() != Some(&inner.state) {
            inner.emitted = Some(inner.state.clone());
            (self.sink)(inner.state.clone());
        }
    }
}

/// Short-lived agent connection, stopped when dropped.
struct TemporaryAgent(CancellationToken);

impl Drop for TemporaryAgent {
    fn drop(&mut self) {
        self.0.cancel();
    }
}

fn stop_agent(inner: &mut Inner) {
    if let Some(agent) = inner.agent.take() {
        agent.cancel.cancel();
    }
    inner.state.agent = AgentStatus::Stopped;
}

/// Waits for the user to approve the claim code, then returns the agent secret.
async fn claim(code: &str) -> Result<String, LinkError> {
    let api = PlayitApi::create(API_BASE.to_string(), None);
    let deadline = tokio::time::Instant::now() + CLAIM_TIMEOUT;

    loop {
        if tokio::time::Instant::now() > deadline {
            return Err(LinkError::Expired);
        }
        let setup = api
            .claim_setup(ReqClaimSetup {
                code: code.to_string(),
                agent_type: ClaimAgentType::SelfManaged,
                version: AGENT_VERSION_TEXT.to_string(),
            })
            .await;
        match setup {
            Ok(ClaimSetupResponse::UserAccepted) => break,
            Ok(ClaimSetupResponse::UserRejected) => return Err(LinkError::Rejected),
            Ok(ClaimSetupResponse::WaitingForUserVisit | ClaimSetupResponse::WaitingForUser) => {}
            Err(ApiError::Fail(ClaimSetupError::CodeExpired)) => return Err(LinkError::Expired),
            Err(ApiError::Fail(err)) => {
                eprintln!("playit: claim setup failed: {err:?}");
                return Err(LinkError::Failed);
            }
            // Network hiccup: keep waiting.
            Err(err) => eprintln!("playit: claim setup error: {err}"),
        }
        tokio::time::sleep(CLAIM_POLL).await;
    }

    loop {
        if tokio::time::Instant::now() > deadline {
            return Err(LinkError::Expired);
        }
        let exchange = api
            .claim_exchange(ReqClaimExchange {
                code: code.to_string(),
            })
            .await;
        match exchange {
            Ok(key) => return Ok(key.secret_key),
            Err(ApiError::Fail(ClaimExchangeError::UserRejected)) => {
                return Err(LinkError::Rejected)
            }
            Err(ApiError::Fail(ClaimExchangeError::CodeExpired)) => return Err(LinkError::Expired),
            // Not ready yet, or a network hiccup: try again.
            Err(err) => eprintln!("playit: claim exchange: {err}"),
        }
        tokio::time::sleep(CLAIM_POLL).await;
    }
}

/// Where the agent sends the traffic of each tunnel: only the tunnels of
/// running servers, to their local port. Other tunnels get no origin, so
/// their connections are dropped.
fn origins(
    run_data: &AgentRunDataV1,
    servers: &HashMap<Uuid, String>,
    active: &HashMap<String, u16>,
) -> Vec<OriginResource> {
    run_data
        .tunnels
        .iter()
        .filter_map(|tunnel| {
            let server = servers.get(&tunnel.id)?;
            let port = *active.get(server)?;
            // `from_agent_tunnel` reads the local port from the config, or
            // else from the address. playit.gg sends neither for our tunnels
            // (`pgsql-lds.tun.ply.gg`, config: `local_ip` only): without this
            // it returns `None` and every connection is dropped.
            let mut tunnel = tunnel.clone();
            tunnel.agent_config.fields = vec![
                AgentTunnelAttr {
                    name: "local_ip".into(),
                    value: Ipv4Addr::LOCALHOST.to_string(),
                },
                AgentTunnelAttr {
                    name: "local_port".into(),
                    value: port.to_string(),
                },
            ];
            OriginResource::from_agent_tunnel(&tunnel)
        })
        .collect()
}

/// Tunnel info of each server known to MinyHost.
fn server_tunnels(
    run_data: &AgentRunDataV1,
    servers: &HashMap<Uuid, String>,
) -> HashMap<String, ServerTunnel> {
    let pending: HashSet<Uuid> = run_data.pending.iter().map(|p| p.id).collect();
    let mut tunnels: HashMap<String, ServerTunnel> = servers
        .values()
        .map(|server| (server.clone(), ServerTunnel::default()))
        .collect();
    for tunnel in &run_data.tunnels {
        let Some(server) = servers.get(&tunnel.id) else {
            continue;
        };
        if pending.contains(&tunnel.id) {
            continue;
        }
        tunnels.insert(
            server.clone(),
            ServerTunnel {
                address: Some(tunnel.display_address.clone()),
                disabled_reason: tunnel.disabled_reason.as_ref().map(|r| r.to_string()),
            },
        );
    }
    tunnels
}

/// "MinyHost <server id>", limited to what playit.gg accepts.
fn tunnel_name(id: &str) -> String {
    format!("MinyHost {id}")
        .chars()
        .filter(|c| c.is_ascii() && !c.is_ascii_control())
        .take(TUNNEL_NAME_MAX)
        .collect()
}

async fn delete_tunnel(api: &PlayitApi, tunnel_id: Uuid) {
    // Best effort: an orphan tunnel has no origin, so it forwards nothing,
    // and the user can still delete it on playit.gg.
    if let Err(err) = api.tunnels_delete(ReqTunnelsDelete { tunnel_id }).await {
        eprintln!("playit: failed to delete tunnel {tunnel_id}: {err}");
    }
}

/// The account has more agents than its plan allows (agents cannot delete
/// themselves: `/agents/delete` refuses agent keys, checked in October 2026).
fn is_over_limit(err: &SetupError) -> bool {
    match err {
        SetupError::ApiFail(payload) => matches!(
            serde_json::from_str(payload),
            Ok(ProtoRegisterError::AgentDisabledOverLimit)
        ),
        _ => false,
    }
}

fn is_revoked(err: &SetupError) -> bool {
    matches!(
        err,
        SetupError::ApiError(ApiResponseError::Auth(
            AuthError::InvalidAgentKey | AuthError::NoLongerValid
        ))
    )
}

/// Body of `/v1/tunnels/create` as the API expects it (checked in October
/// 2026, same as the official Minecraft plugin). The request type of
/// `playit-api-client` v1.0.12 (`ReqTunnelsCreateV1`, with `ports` and
/// `alloc`) is outdated: the API answers "failed to parse body".
#[derive(Serialize)]
struct CreateTunnelRequest {
    name: String,
    /// `{ "type": "tunnel-type", "details": "minecraft-java" }`
    protocol: TunnelPortDetails,
    origin: AccountTunnelOriginCreate,
    /// `{ "type": "region", "details": { "region": "global", "port": null } }`
    endpoint: CreateTunnelAllocationRequest,
    enabled: bool,
}

fn create_request(agent_id: Uuid, name: &str) -> CreateTunnelRequest {
    CreateTunnelRequest {
        name: name.to_string(),
        protocol: TunnelPortDetails::TunnelType(TunnelType::MinecraftJava),
        origin: AccountTunnelOriginCreate::Agent(AgentOrigin {
            agent_id: Some(agent_id),
            // The local address is set by MinyHost (see `refresh`).
            config: AgentTunnelConfig { fields: Vec::new() },
        }),
        endpoint: CreateTunnelAllocationRequest::Region(UseAllocRegion {
            region: PlayitNetwork::Global,
            port: None,
        }),
        enabled: true,
    }
}

/// Creates a Minecraft Java tunnel for this agent and returns its id.
async fn create_tunnel(api: &PlayitApi, agent_id: Uuid, name: &str) -> AppResult<Uuid> {
    let result = api
        .get_client()
        // Failure codes are read as text: an unknown one must not hide the others.
        .call::<_, ObjectId, String>(
            std::panic::Location::caller(),
            "/v1/tunnels/create",
            create_request(agent_id, name),
        )
        .await;
    let error = match result {
        Ok(ApiResult::Success(created)) => return Ok(created.id),
        Ok(ApiResult::Fail(code)) => match code.as_str() {
            "RequiresPlayitPremium"
            | "RegionRequiresPlayitPremium"
            | "PublicPortRequiresPlayitPremium" => AppError::PlayitLimit,
            "RequiresVerifiedAccount" => AppError::PlayitUnverified,
            _ => AppError::Playit(format!("tunnel creation failed: {code}")),
        },
        Ok(ApiResult::Error(ApiResponseError::Auth(AuthError::EmailMustBeVerified))) => {
            AppError::PlayitUnverified
        }
        Ok(ApiResult::Error(ApiResponseError::Auth(
            AuthError::InvalidAgentKey | AuthError::NoLongerValid,
        ))) => AppError::PlayitNotLinked,
        Ok(ApiResult::Error(err)) => AppError::Playit(format!("tunnel creation failed: {err}")),
        Err(err) => AppError::Playit(format!("tunnel creation failed: {err:?}")),
    };
    eprintln!("playit: {error}");
    Err(error)
}

fn api_error_no_fail<C: std::fmt::Debug>(err: ApiErrorNoFail<C>) -> AppError {
    AppError::Playit(err.to_string())
}

/// `MinyHost <PC> <date>`, ASCII and short enough for playit.gg.
fn agent_name(computer: &str, date: &str) -> String {
    let computer: String = computer
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .take(15)
        .collect();
    if computer.is_empty() {
        format!("MinyHost {date}")
    } else {
        format!("MinyHost {computer} {date}")
    }
}

fn load_secret_file(paths: &AppPaths) -> Option<SecretFile> {
    let text = std::fs::read_to_string(paths.playit_file()).ok()?;
    let file: SecretFile = serde_json::from_str(&text).ok()?;
    Some(file).filter(|f| !f.secret_key.trim().is_empty())
}

fn save_secret(paths: &AppPaths, secret: &str) -> AppResult<()> {
    save_secret_file(paths, secret, None)
}

fn save_secret_file(paths: &AppPaths, secret: &str, agent_name: Option<&str>) -> AppResult<()> {
    let file = SecretFile {
        secret_key: secret.to_string(),
        agent_name: agent_name.map(str::to_string),
    };
    std::fs::create_dir_all(paths.root())?;
    std::fs::write(paths.playit_file(), serde_json::to_string_pretty(&file)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tunnel_name_is_ascii_and_short() {
        assert_eq!(tunnel_name("survie"), "MinyHost survie");
        let long = tunnel_name("un-nom-de-serveur-vraiment-tres-long");
        assert_eq!(long.len(), TUNNEL_NAME_MAX);
        assert!(tunnel_name("café-é").is_ascii());
    }

    /// Same shape as the body accepted by the API (see `CreateTunnelRequest`).
    #[test]
    fn create_request_matches_the_api() {
        let agent_id = Uuid::nil();
        let json = serde_json::to_value(create_request(agent_id, "MinyHost survie")).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "name": "MinyHost survie",
                "protocol": { "type": "tunnel-type", "details": "minecraft-java" },
                "origin": {
                    "type": "agent",
                    "data": { "agent_id": agent_id, "config": { "fields": [] } }
                },
                "endpoint": { "type": "region", "details": { "region": "global", "port": null } },
                "enabled": true
            })
        );
    }

    /// Run data of a real tunnel (October 2026): no `local_port`, no port in
    /// the address. The running server must still be its origin.
    #[tokio::test]
    async fn running_server_is_the_origin_of_its_tunnel() {
        let tunnel_id = Uuid::new_v4();
        let run_data: AgentRunDataV1 = serde_json::from_value(serde_json::json!({
            "agent_id": Uuid::nil(),
            "tunnels": [{
                "id": tunnel_id,
                "internal_id": 4864196,
                "name": "MinyHost survie",
                "display_address": "pgsql-lds.tun.ply.gg",
                "port_type": "tcp",
                "port_count": 1,
                "tunnel_type": "minecraft-java",
                "tunnel_type_display": "minecraft-java",
                "agent_config": { "fields": [{ "name": "local_ip", "value": "127.0.0.1" }] },
                "disabled_reason": null
            }],
            "pending": [],
            "notices": [],
            "permissions": {
                "is_self_managed": true,
                "has_premium": false,
                "account_status": "verified"
            }
        }))
        .unwrap();
        let servers = HashMap::from([(tunnel_id, "survie".to_string())]);

        // Stopped server: no origin, connections are dropped.
        assert!(origins(&run_data, &servers, &HashMap::new()).is_empty());

        let active = HashMap::from([("survie".to_string(), 25570)]);
        let found = origins(&run_data, &servers, &active);
        assert_eq!(found.len(), 1);
        let local = found[0].resolve_local(0).await;
        assert_eq!(local, Some("127.0.0.1:25570".parse().unwrap()));
    }

    #[test]
    fn agent_name_is_short_and_ascii() {
        assert_eq!(
            agent_name("DESKTOP-ABC123", "2026-10-07"),
            "MinyHost DESKTOP-ABC123 2026-10-07"
        );
        assert_eq!(agent_name("", "2026-10-07"), "MinyHost 2026-10-07");
        assert_eq!(
            agent_name("Pc de Léa", "2026-10-07"),
            "MinyHost PcdeLa 2026-10-07"
        );
        assert!(agent_name("A-VERY-LONG-COMPUTER-NAME", "2026-10-07").len() <= 35);
    }

    #[test]
    fn detects_the_agent_limit() {
        // As built by `playit-agent-core` from the API failure.
        let err = SetupError::ApiFail("\"AgentDisabledOverLimit\"".into());
        assert!(is_over_limit(&err));
        assert!(!is_over_limit(&SetupError::ApiFail(
            "\"AccountBanned\"".into()
        )));
        assert!(!is_over_limit(&SetupError::FailedToConnect));
    }

    #[test]
    fn secret_round_trip() {
        let dir = std::env::temp_dir().join(format!("minyhost-playit-{}", Uuid::new_v4()));
        let paths = AppPaths::new(dir.clone());
        assert!(load_secret_file(&paths).is_none());
        save_secret(&paths, "abc").unwrap();
        let file = load_secret_file(&paths).unwrap();
        assert_eq!(file.secret_key, "abc");
        assert_eq!(file.agent_name, None);
        save_secret_file(&paths, "abc", Some("MinyHost PC 2026-10-07")).unwrap();
        let file = load_secret_file(&paths).unwrap();
        assert_eq!(file.agent_name.as_deref(), Some("MinyHost PC 2026-10-07"));
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn state_serializes_for_the_frontend() {
        let state = TunnelState {
            link: LinkState::Linking {
                url: "https://playit.gg/claim/ab".into(),
            },
            ..TunnelState::default()
        };
        let json = serde_json::to_value(&state).unwrap();
        assert_eq!(json["link"]["state"], "linking");
        assert_eq!(json["link"]["url"], "https://playit.gg/claim/ab");
        assert_eq!(json["agent"], "stopped");
        assert_eq!(json["linkError"], serde_json::Value::Null);
        assert_eq!(json["emailUnverified"], false);
        assert_eq!(json["agentOverLimit"], false);
    }
}
