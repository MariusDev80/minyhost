//! Server process lifecycle: start, stop, console I/O (CLAUDE.md 5.3 to 5.5).
//!
//! `ProcessManager` keeps the registry of running servers (one process per
//! instance). It does not know about Tauri: it reports what happens through an
//! `EventSink` callback, which `lib.rs` turns into frontend events.
//!
//! Life of a server:
//!   start()  -> Starting -> (line "Done (...)! For help") -> Running
//!   stop()   -> Stopping -> process exits -> Stopped
//!   crash    -> process exits on its own  -> Stopped + Crashed event

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use serde::Serialize;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, Command};
use tokio::sync::{oneshot, watch, Mutex};

use crate::core::{eula, instances, java, properties};
use crate::error::{AppError, AppResult};
use crate::paths::{server_files, AppPaths};

/// How long `stop` waits for a clean shutdown before killing the process.
const STOP_TIMEOUT: Duration = Duration::from_secs(30);

/// Mirrored by `ServerStatus` in `src/types/index.ts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ServerStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ConsoleStream {
    Stdout,
    Stderr,
}

/// How a `stop` ended.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StopOutcome {
    /// The server saved the world and exited by itself.
    Graceful,
    /// The timeout was reached and the process was killed (world may be damaged).
    Killed,
}

/// Everything the manager reports to the outside world.
#[derive(Debug, Clone)]
pub enum ServerEvent {
    Console {
        id: String,
        line: String,
        stream: ConsoleStream,
    },
    Status {
        id: String,
        status: ServerStatus,
    },
    /// The process exited on its own with an error.
    Crashed {
        id: String,
        exit_code: Option<i32>,
    },
}

pub type EventSink = Arc<dyn Fn(ServerEvent) + Send + Sync>;

/// A server process we started and that has not exited yet.
struct RunningServer {
    status: ServerStatus,
    stdin: ChildStdin,
    /// Send `()` to kill the process (used after the stop timeout).
    kill: Option<oneshot::Sender<()>>,
    /// Becomes `true` once the process has exited.
    exited: watch::Receiver<bool>,
}

#[derive(Clone)]
pub struct ProcessManager {
    servers: Arc<Mutex<HashMap<String, RunningServer>>>,
    sink: EventSink,
}

impl ProcessManager {
    pub fn new(sink: EventSink) -> Self {
        Self {
            servers: Arc::default(),
            sink,
        }
    }

    /// Status of every server; servers not in the map are stopped.
    pub async fn statuses(&self) -> HashMap<String, ServerStatus> {
        let servers = self.servers.lock().await;
        servers
            .iter()
            .map(|(id, server)| (id.clone(), server.status))
            .collect()
    }

    pub async fn status(&self, id: &str) -> ServerStatus {
        let servers = self.servers.lock().await;
        servers
            .get(id)
            .map_or(ServerStatus::Stopped, |server| server.status)
    }

    /// Checks that the instance can start, then launches it (CLAUDE.md 5.3).
    pub async fn start(&self, paths: &AppPaths, id: &str) -> AppResult<()> {
        let instance = instances::load(paths, id)?;
        let dir = paths.server_dir(id);
        let java = java::executable(paths, instance.java_version);

        // Lock for the whole start so two clicks cannot launch two processes.
        let mut servers = self.servers.lock().await;
        if servers.contains_key(id) {
            return Err(AppError::AlreadyRunning);
        }
        check_can_start(&instance, &dir, &java)?;

        let mut command = Command::new(&java);
        command
            .arg(format!("-Xms{}M", instance.memory_mb))
            .arg(format!("-Xmx{}M", instance.memory_mb))
            // Console output in UTF-8 so accents (player names, chat) display correctly.
            .args([
                "-Dfile.encoding=UTF-8",
                "-Dstdout.encoding=UTF-8",
                "-Dstderr.encoding=UTF-8",
            ])
            .args(["-jar", server_files::JAR, "nogui"])
            .current_dir(&dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        #[cfg(windows)]
        {
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            command.creation_flags(CREATE_NO_WINDOW);
        }

        let mut child = command.spawn()?;
        let (Some(stdin), Some(stdout), Some(stderr)) =
            (child.stdin.take(), child.stdout.take(), child.stderr.take())
        else {
            let _ = child.kill().await;
            return Err(AppError::Io(std::io::Error::other("missing process pipes")));
        };

        let (kill_tx, kill_rx) = oneshot::channel();
        let (exited_tx, exited_rx) = watch::channel(false);
        servers.insert(
            id.to_string(),
            RunningServer {
                status: ServerStatus::Starting,
                stdin,
                kill: Some(kill_tx),
                exited: exited_rx,
            },
        );
        drop(servers);
        self.emit_status(id, ServerStatus::Starting);

        tokio::spawn(
            self.clone()
                .read_output(id.to_string(), stdout, ConsoleStream::Stdout),
        );
        tokio::spawn(
            self.clone()
                .read_output(id.to_string(), stderr, ConsoleStream::Stderr),
        );
        tokio::spawn(
            self.clone()
                .wait_for_exit(id.to_string(), child, kill_rx, exited_tx),
        );
        Ok(())
    }

    /// Sends a console command (CLAUDE.md 5.4).
    pub async fn send_command(&self, id: &str, command: &str) -> AppResult<()> {
        let is_stop = command.trim() == "stop";
        {
            let mut servers = self.servers.lock().await;
            let server = servers.get_mut(id).ok_or(AppError::NotRunning)?;
            server
                .stdin
                .write_all(format!("{}\n", command.trim()).as_bytes())
                .await?;
            server.stdin.flush().await?;
            if is_stop {
                server.status = ServerStatus::Stopping;
            }
        }
        // Typing `stop` in the console is a normal shutdown, not a crash.
        if is_stop {
            self.emit_status(id, ServerStatus::Stopping);
        }
        Ok(())
    }

    /// Stops the server cleanly, killing it only after `STOP_TIMEOUT` (CLAUDE.md 5.5).
    pub async fn stop(&self, id: &str) -> AppResult<StopOutcome> {
        let mut exited = {
            let mut servers = self.servers.lock().await;
            let server = servers.get_mut(id).ok_or(AppError::NotRunning)?;
            server.status = ServerStatus::Stopping;
            // If stdin is already closed the process is exiting anyway.
            let _ = server.stdin.write_all(b"stop\n").await;
            let _ = server.stdin.flush().await;
            server.exited.clone()
        };
        self.emit_status(id, ServerStatus::Stopping);

        if tokio::time::timeout(STOP_TIMEOUT, exited.wait_for(|done| *done))
            .await
            .is_ok()
        {
            return Ok(StopOutcome::Graceful);
        }

        // Timeout: kill the process and wait for it to be gone.
        if let Some(server) = self.servers.lock().await.get_mut(id) {
            if let Some(kill) = server.kill.take() {
                let _ = kill.send(());
            }
        }
        let _ = exited.wait_for(|done| *done).await;
        Ok(StopOutcome::Killed)
    }

    /// Stops every running server in parallel (used when the app closes).
    pub async fn stop_all(&self) {
        let ids: Vec<String> = self.servers.lock().await.keys().cloned().collect();
        let tasks: Vec<_> = ids
            .into_iter()
            .map(|id| {
                let manager = self.clone();
                tokio::spawn(async move { manager.stop(&id).await })
            })
            .collect();
        for task in tasks {
            let _ = task.await;
        }
    }

    /// Forwards each output line to the UI and detects the end of startup.
    async fn read_output(self, id: String, output: impl AsyncRead + Unpin, stream: ConsoleStream) {
        let mut reader = BufReader::new(output);
        let mut buffer = Vec::new();
        loop {
            buffer.clear();
            // Read raw bytes: a non-UTF-8 line must not stop the console.
            match reader.read_until(b'\n', &mut buffer).await {
                Ok(0) | Err(_) => break,
                Ok(_) => {}
            }
            let line = String::from_utf8_lossy(&buffer).trim_end().to_string();

            if is_ready_line(&line) {
                self.set_status(&id, ServerStatus::Starting, ServerStatus::Running)
                    .await;
            }
            (self.sink)(ServerEvent::Console {
                id: id.clone(),
                line,
                stream,
            });
        }
    }

    /// Waits for the process to end (or kills it on request), then cleans up.
    async fn wait_for_exit(
        self,
        id: String,
        mut child: Child,
        kill: oneshot::Receiver<()>,
        exited: watch::Sender<bool>,
    ) {
        let exit = tokio::select! {
            exit = child.wait() => exit,
            _ = kill => {
                let _ = child.kill().await;
                child.wait().await
            }
        };

        let previous = self.servers.lock().await.remove(&id).map(|s| s.status);
        let _ = exited.send(true);
        self.emit_status(&id, ServerStatus::Stopped);

        let success = exit.as_ref().is_ok_and(|status| status.success());
        if previous != Some(ServerStatus::Stopping) && !success {
            let exit_code = exit.ok().and_then(|status| status.code());
            (self.sink)(ServerEvent::Crashed { id, exit_code });
        }
    }

    /// Moves a server from `from` to `to`; does nothing if it is in another state.
    async fn set_status(&self, id: &str, from: ServerStatus, to: ServerStatus) {
        let changed = {
            let mut servers = self.servers.lock().await;
            match servers.get_mut(id) {
                Some(server) if server.status == from => {
                    server.status = to;
                    true
                }
                _ => false,
            }
        };
        if changed {
            self.emit_status(id, to);
        }
    }

    fn emit_status(&self, id: &str, status: ServerStatus) {
        (self.sink)(ServerEvent::Status {
            id: id.to_string(),
            status,
        });
    }
}

/// Pre-start checks: EULA, files, RAM, port.
fn check_can_start(
    instance: &instances::Instance,
    dir: &std::path::Path,
    java: &std::path::Path,
) -> AppResult<()> {
    if !eula::is_accepted(dir) {
        return Err(AppError::EulaNotAccepted);
    }
    for file in [dir.join(server_files::JAR), PathBuf::from(java)] {
        if !file.exists() {
            return Err(AppError::MissingFiles(file.display().to_string()));
        }
    }
    if !(instances::MIN_MEMORY_MB..=instances::MAX_MEMORY_MB).contains(&instance.memory_mb) {
        return Err(AppError::InvalidInput(format!(
            "memory: {} MB",
            instance.memory_mb
        )));
    }

    // The port the server will really use is the one in server.properties.
    let port = properties::Properties::load(&dir.join(server_files::PROPERTIES))
        .ok()
        .and_then(|props| props.get("server-port")?.trim().parse().ok())
        .unwrap_or(instance.port);
    if !instances::is_port_free(port) {
        return Err(AppError::PortInUse(port));
    }
    Ok(())
}

/// Minecraft prints `Done (3.2s)! For help, type "help"` once it accepts players.
fn is_ready_line(line: &str) -> bool {
    line.contains("Done (") && line.contains("For help, type")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_ready_line() {
        assert!(is_ready_line(
            r#"[12:00:00] [Server thread/INFO]: Done (3.214s)! For help, type "help""#
        ));
        assert!(!is_ready_line(
            "[12:00:00] [Server thread/INFO]: Preparing spawn area: 40%"
        ));
    }
}
