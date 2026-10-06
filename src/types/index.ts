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
}

/** An instance plus its live status. Rust: `commands/servers.rs` (`ServerInfo`). */
export interface ServerInfo extends Instance {
  status: ServerStatus;
}

/** Creation form. Rust: `core/create.rs` (`NewServer`). */
export interface NewServer {
  name: string;
  loader: Loader;
  mcVersion: string;
  memoryMb: number;
  eulaAccepted: boolean;
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
  | "playerNotFound";

export interface AppError {
  code: AppErrorCode;
  /** Technical message (English), for logs and bug reports. */
  detail: string;
}
