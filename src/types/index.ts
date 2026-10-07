// Types shared with Rust. Keep them aligned with the Rust structs
// (`#[serde(rename_all = "camelCase")]`). The Rust file is noted on each type.

/** Server type. Rust: `core/providers/mod.rs` (`Loader`). */
export type Loader = "vanilla" | "paper" | "fabric";

/** Lifecycle of a server process. Rust: `core/process.rs` (`ServerStatus`). */
export type ServerStatus = "starting" | "running" | "stopping" | "stopped";

/** Content of `instance.json`. Rust: `core/instances.rs` (`Instance`). */
export interface Instance {
  id: string;
  name: string;
  mcVersion: string;
  loader: Loader;
  /** Paper build or Fabric loader version, `null` for Vanilla. */
  loaderVersion: string | null;
  javaVersion: number;
  memoryMb: number;
  port: number;
  /** ISO 8601, UTC. */
  createdAt: string;
  /** playit.gg tunnel, set while Internet access is on. */
  playitTunnelId?: string;
}

/** An instance plus its live status. Rust: `commands/servers.rs` (`ServerInfo`). */
export interface ServerInfo extends Instance {
  status: ServerStatus;
  /** Settings changed while running: a restart is needed to apply them. */
  needsRestart: boolean;
}

/** Creation form. Rust: `core/create.rs` (`NewServer`). */
export interface NewServer {
  name: string;
  loader: Loader;
  mcVersion: string;
  memoryMb: number;
  eulaAccepted: boolean;
  /** Optional: only the settings that differ from their default. */
  settings?: GameSettings;
}

/** Value of a game setting. Rust: `core/game_settings.rs` (`SettingValue`). */
export type SettingValue = boolean | number | string;

/** Rust: `core/game_rules.rs` (`Category`). */
export type GameRuleCategory =
  "players" | "mobs" | "world" | "drops" | "commands";

/** A setting described by Rust. Rust: `core/game_settings.rs` (`SettingDef`). */
export type SettingDef = {
  key: string;
  /** Game rules only. */
  category: GameRuleCategory | null;
  /** Only useful before the world exists (seed…). */
  creationOnly: boolean;
} & (
  | { kind: "bool"; default: boolean }
  | { kind: "int"; default: number; min: number; max: number | null }
  | { kind: "choice"; default: string; choices: string[] }
  | { kind: "text"; default: string; maxLength: number }
);

/** Settings available for a Minecraft version. Rust: `Catalog`. */
export interface SettingsCatalog {
  /** World settings (`server.properties`). */
  properties: SettingDef[];
  gameRules: SettingDef[];
}

/** Values of a server's settings, by key. Rust: `GameSettings`. */
export interface GameSettings {
  properties: Record<string, SettingValue>;
  gameRules: Record<string, SettingValue>;
}

/** Result of saving settings. Rust: `core/game_settings.rs` (`SaveOutcome`). */
export interface SaveOutcome {
  /** World settings changed while running: restart to apply them. */
  needsRestart: boolean;
}

/** Rust: `core/create.rs` (`CreateStep`). */
export type CreateStep =
  "preparing" | "downloadingJava" | "downloadingServer" | "finalizing";

/** Payload of the `create-progress` event. */
export interface CreateProgress {
  step: CreateStep;
  /** 0-100 during downloads, `null` otherwise. */
  percent: number | null;
}

/** Rust: `core/process.rs` (`StopOutcome`). */
export type StopOutcome = "graceful" | "killed";

/** Payload of the `console-line` event. */
export interface ConsoleLineEvent {
  id: string;
  line: string;
  stream: "stdout" | "stderr";
}

/** Payload of the `server-status` event. */
export interface ServerStatusEvent {
  id: string;
  status: ServerStatus;
}

/** Payload of the `server-crashed` event. */
export interface ServerCrashedEvent {
  id: string;
  exitCode: number | null;
}

/** A Minecraft account, as stored in the server's player lists. */
export interface Player {
  /** With dashes. */
  uuid: string;
  name: string;
}

/** One player of `whitelist.json`. Rust: `core/whitelist.rs` (`WhitelistEntry`). */
export type WhitelistEntry = Player;

/** One player of `ops.json`. Rust: `core/operators.rs` (`Operator`). */
export interface Operator extends Player {
  /** Permission level, 4 = every command. */
  level: number;
  bypassesPlayerLimit: boolean;
}

/** Rust: `core/tunnel.rs` (`LinkState`). */
export type LinkState =
  | { state: "unlinked" }
  /** Waiting for the user to approve MinyHost on this playit.gg page. */
  | { state: "linking"; url: string }
  | { state: "linked" };

/** Rust: `core/tunnel.rs` (`LinkError`). */
export type LinkError = "rejected" | "expired" | "revoked" | "failed";

/** Rust: `core/tunnel.rs` (`AgentStatus`). */
export type AgentStatus = "stopped" | "connecting" | "online" | "error";

/** Tunnel of one server. Rust: `core/tunnel.rs` (`ServerTunnel`). */
export interface ServerTunnel {
  /** Address for friends, `null` until playit.gg has assigned it. */
  address: string | null;
  /** Disabled by playit.gg (account limits…). */
  disabledReason: string | null;
}

/** Internet access through playit.gg. Rust: `core/tunnel.rs` (`TunnelState`). */
export interface TunnelState {
  link: LinkState;
  linkError: LinkError | null;
  agent: AgentStatus;
  /** Messages from playit.gg (English), most important first. */
  notices: { message: string; link: string | null }[];
  /** By server id; a server is open to the Internet iff it has an entry. */
  tunnels: Record<string, ServerTunnel>;
  /** The playit.gg account email is not verified yet. */
  emailUnverified: boolean;
  /** playit.gg refused the agent: too many agents on the account. */
  agentOverLimit: boolean;
  /** Agent used by MinyHost, to tell it apart on playit.gg. */
  agentId: string | null;
  /** Name MinyHost gave it on playit.gg ("MinyHost <PC> <date>"). */
  agentName: string | null;
}

/** Error returned by every command. Rust: `error.rs` (`AppError::code`). */
export type AppErrorCode =
  | "network"
  | "io"
  | "invalidData"
  | "hashMismatch"
  | "serverNotFound"
  | "versionUnavailable"
  | "javaUnavailable"
  | "portInUse"
  | "alreadyRunning"
  | "notRunning"
  | "eulaNotAccepted"
  | "missingFiles"
  | "invalidInput"
  | "invalidPlayerName"
  | "playerNotFound"
  | "playitNotLinked"
  | "playitLimit"
  | "playitUnverified"
  | "playitAgentLimit"
  | "playit";

export interface AppError {
  code: AppErrorCode;
  /** Technical message (English), for logs and bug reports. */
  detail: string;
}
