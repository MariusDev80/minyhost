//! Business logic, independent from Tauri (usable from a CLI or tests).

pub mod backup;
pub mod create;
pub mod eula;
pub mod game_rules;
pub mod game_settings;
pub mod instances;
pub mod java;
pub mod nbt;
pub mod operators;
pub mod players;
pub mod process;
pub mod properties;
pub mod providers;
pub mod tunnel;
pub mod version;
pub mod whitelist;

#[cfg(test)]
mod e2e_tests;
