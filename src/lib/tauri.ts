// Typed wrappers around Tauri `invoke` / `listen`.
// The frontend never touches the disk or network directly: everything goes through here.
import { getCurrentWindow } from "@tauri-apps/api/window";

/** Controls for the frameless main window (custom title bar). */
export const appWindow = {
  minimize: () => getCurrentWindow().minimize(),
  toggleMaximize: () => getCurrentWindow().toggleMaximize(),
  close: () => getCurrentWindow().close(),
  isMaximized: () => getCurrentWindow().isMaximized(),
  onResized: (handler: () => void) => getCurrentWindow().onResized(handler),
};
