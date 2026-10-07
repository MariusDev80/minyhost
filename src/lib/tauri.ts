// Typed wrappers around Tauri `invoke` / `listen`.
// The frontend never touches the disk or network directly: everything goes through here.
//
// Rust side: commands in `src-tauri/src/commands/`, events in `src-tauri/src/events.rs`.
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { openUrl } from "@tauri-apps/plugin-opener";
import type {
  ConsoleLineEvent,
  CreateProgress,
  GameSettings,
  Instance,
  Loader,
  NewServer,
  SaveOutcome,
  Operator,
  ServerCrashedEvent,
  ServerInfo,
  ServerStatusEvent,
  SettingsCatalog,
  StopOutcome,
  TunnelState,
  WhitelistEntry,
} from "@/types";

/** Controls for the frameless main window (custom title bar). */
export const appWindow = {
  minimize: () => getCurrentWindow().minimize(),
  toggleMaximize: () => getCurrentWindow().toggleMaximize(),
  close: () => getCurrentWindow().close(),
  isMaximized: () => getCurrentWindow().isMaximized(),
  onResized: (handler: () => void) => getCurrentWindow().onResized(handler),
};

/** Rust commands. On failure they reject with an `AppError` (see `lib/errors.ts`). */
export const commands = {
  listServers: () => invoke<ServerInfo[]>("list_servers"),
  listVersions: (loader: Loader) =>
    invoke<string[]>("list_versions", { loader }),
  createServer: (input: NewServer) =>
    invoke<Instance>("create_server", { input }),
  deleteServer: (id: string) => invoke<void>("delete_server", { id }),
  serversFolderPath: () => invoke<string>("servers_folder_path"),
  /** Opens the folder in the file explorer. */
  openServersFolder: () => invoke<void>("open_servers_folder"),
  openServerFolder: (id: string) => invoke<void>("open_server_folder", { id }),
  startServer: (id: string) => invoke<void>("start_server", { id }),
  stopServer: (id: string) => invoke<StopOutcome>("stop_server", { id }),
  restartServer: (id: string) => invoke<StopOutcome>("restart_server", { id }),
  sendCommand: (id: string, command: string) =>
    invoke<void>("send_command", { id, command }),
  listWhitelist: (id: string) =>
    invoke<WhitelistEntry[]>("list_whitelist", { id }),
  addToWhitelist: (id: string, name: string) =>
    invoke<WhitelistEntry>("add_to_whitelist", { id, name }),
  removeFromWhitelist: (id: string, uuid: string) =>
    invoke<void>("remove_from_whitelist", { id, uuid }),
  listOperators: (id: string) => invoke<Operator[]>("list_operators", { id }),
  addOperator: (id: string, name: string) =>
    invoke<Operator>("add_operator", { id, name }),
  removeOperator: (id: string, uuid: string) =>
    invoke<void>("remove_operator", { id, uuid }),
  gameSettingsCatalog: (mcVersion: string) =>
    invoke<SettingsCatalog>("game_settings_catalog", { mcVersion }),
  getGameSettings: (id: string) =>
    invoke<GameSettings>("get_game_settings", { id }),
  saveGameSettings: (id: string, settings: GameSettings) =>
    invoke<SaveOutcome>("save_game_settings", { id, settings }),
  /** Skin PNG as a data URL (Steve when the player has no custom skin). */
  playerSkin: (uuid: string) => invoke<string>("player_skin", { uuid }),
  getTunnelState: () => invoke<TunnelState>("get_tunnel_state"),
  /** Returns the playit.gg page where the user approves MinyHost. */
  linkPlayit: () => invoke<string>("link_playit"),
  cancelPlayitLink: () => invoke<void>("cancel_playit_link"),
  unlinkPlayit: () => invoke<void>("unlink_playit"),
  /** Tries the agent again, e.g. after old agents were deleted on playit.gg. */
  recheckPlayitAgent: () => invoke<void>("recheck_playit_agent"),
  enableInternetAccess: (id: string) =>
    invoke<void>("enable_internet_access", { id }),
  disableInternetAccess: (id: string) =>
    invoke<void>("disable_internet_access", { id }),
};

/** Opens a web page in the user's browser. */
export const openExternal = (url: string) => openUrl(url).catch(() => {});

/**
 * Subscribes to a Rust event and returns the function that unsubscribes,
 * ready to be returned from a `useEffect`. Handles the case where the
 * component unmounts before `listen` has resolved.
 */
function subscribe<T>(event: string, handler: (payload: T) => void) {
  let disposed = false;
  let unlisten: (() => void) | undefined;
  listen<T>(event, (e) => handler(e.payload)).then(
    (fn) => {
      if (disposed) fn();
      else unlisten = fn;
    },
    () => {},
  );
  return () => {
    disposed = true;
    unlisten?.();
  };
}

/** Rust events. Usage: `useEffect(() => events.onConsoleLine((e) => ...), [])`. */
export const events = {
  onConsoleLine: (handler: (e: ConsoleLineEvent) => void) =>
    subscribe("console-line", handler),
  onServerStatus: (handler: (e: ServerStatusEvent) => void) =>
    subscribe("server-status", handler),
  onServerCrashed: (handler: (e: ServerCrashedEvent) => void) =>
    subscribe("server-crashed", handler),
  onCreateProgress: (handler: (e: CreateProgress) => void) =>
    subscribe("create-progress", handler),
  onAppClosing: (handler: () => void) => subscribe("app-closing", handler),
  onTunnelState: (handler: (e: TunnelState) => void) =>
    subscribe("tunnel-state", handler),
};
