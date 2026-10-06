//! Tauri commands exposed to the frontend.
//!
//! Keep these functions thin: validate input, call into `core`, map errors.
//! Each command must also be registered in `lib.rs` (`generate_handler!`) and
//! wrapped in `src/lib/tauri.ts` (`commands`).

pub mod process;
pub mod servers;
pub mod whitelist;
