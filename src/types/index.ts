// Types shared with Rust. Keep them aligned with the Rust structs
// (`#[serde(rename_all = "camelCase")]`). The Rust file is noted on each type.

/** Server type. Rust: `core/providers/mod.rs` (`Loader`). */
export type Loader = "vanilla" | "paper" | "fabric" | "neoforge" | "forge";

/** Lifecycle of a server process. Rust: `core/process.rs` (`ServerStatus`). */
export type ServerStatus = "starting" | "running" | "stopping" | "stopped";

/** Content of `instance.json`. Rust: `core/instances.rs` (`Instance`). */
export interface Instance {
  id: string;
  name: string;
  mcVersion: string;
  loader: Loader;
  /** Paper build, Fabric loader, Forge or NeoForge version; `null` for Vanilla. */
  loaderVersion: string | null;
  javaVersion: number;
  memoryMb: number;
  port: number;
  /** ISO 8601, UTC. */
  createdAt: string;
  /** playit.gg tunnel, set while Internet access is on. */
  playitTunnelId?: string;
  /** Modpack the server was created from. */
  modpack?: ContentMeta;
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

/** Creation from a modpack. Rust: `core/create.rs` (`NewModpackServer`). */
export interface NewModpackServer {
  name: string;
  memoryMb: number;
  eulaAccepted: boolean;
  source: ContentSource;
  projectId: string;
  /** Modrinth version id, or CurseForge file id. */
  versionId: string;
}

/** Rust: `core/create.rs` (`ModpackCreated`). */
export interface ModpackCreated {
  instance: Instance;
  /** Files to download by hand before the server can start. */
  manual: ManualDownload[];
}

/** One version of a modpack. Rust: `core/content/modpack.rs` (`ModpackVersion`). */
export interface ModpackVersion {
  id: string;
  name: string;
  mcVersions: string[];
  loaders: string[];
  /** Made for a loader MinyHost can host. */
  supported: boolean;
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
  | "preparing"
  /** Modpacks only. */
  | "downloadingModpack"
  | "downloadingJava"
  | "downloadingServer"
  /** Forge and NeoForge only. */
  | "installingLoader"
  /** Modpacks only. */
  | "downloadingContent"
  | "finalizing";

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

/** Rust: `core/content/mod.rs` (`ContentKind`). */
export type ContentKind = "mod" | "plugin" | "datapack" | "modpack";

/** Rust: `core/content/mod.rs` (`Source`). */
export type ContentSource = "modrinth" | "curseforge";

/** Sources offered by this build. Rust: `commands/content.rs`. */
export interface ContentSources {
  /** Only when the app was built with a CurseForge API key. */
  curseforge: boolean;
}

/** One search result. Rust: `core/content/mod.rs` (`ContentHit`). */
export interface ContentHit {
  source: ContentSource;
  projectId: string;
  title: string;
  description: string;
  author: string;
  iconUrl: string | null;
  downloads: number;
  pageUrl: string;
}

/** Rust: `core/content/mod.rs` (`SearchPage`). */
export interface SearchPage {
  hits: ContentHit[];
  /** Results of the whole search. */
  total: number;
}

/** Origin of a file installed by MinyHost. Rust: `ContentMeta`. */
export interface ContentMeta {
  source: ContentSource;
  projectId: string;
  versionId: string;
  title: string;
  versionNumber: string;
  iconUrl?: string;
  pageUrl: string;
  /** Installed because another project needs it. */
  dependency: boolean;
}

/** A mod, plugin or datapack of a server. Rust: `InstalledContent`. */
export interface InstalledContent {
  kind: ContentKind;
  /** Without the `.disabled` suffix. */
  fileName: string;
  enabled: boolean;
  /** `null` for files added by hand. */
  meta: ContentMeta | null;
}

/** A file to download from CurseForge's website. Rust: `ManualDownload`. */
export interface ManualDownload {
  title: string;
  fileName: string;
  pageUrl: string;
}

/** Rust: `core/content/mod.rs` (`InstallOutcome`). */
export interface InstallOutcome {
  installed: string[];
  /** Dependencies with no version for this server. */
  unavailable: string[];
  manual: ManualDownload[];
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
  | "mustBeStopped"
  | "eulaNotAccepted"
  | "loaderInstall"
  | "missingFiles"
  | "invalidInput"
  | "invalidPlayerName"
  | "playerNotFound"
  | "playitNotLinked"
  | "playitLimit"
  | "playitUnverified"
  | "playitAgentLimit"
  | "playit"
  | "contentUnavailable"
  | "curseforgeUnavailable"
  | "modpackUnsupported"
  | "distributionBlocked";

export interface AppError {
  code: AppErrorCode;
  /** Technical message (English), for logs and bug reports. */
  detail: string;
}
