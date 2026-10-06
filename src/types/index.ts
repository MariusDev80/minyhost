// Types shared with Rust. Keep them aligned with the Rust structs
// (`#[serde(rename_all = "camelCase")]`).

/** Lifecycle of a server process (see CLAUDE.md section 5.3). */
export type ServerStatus = "starting" | "running" | "stopping" | "stopped";
