//! Game settings shown in the "Paramètres" tab and the creation form:
//! - world settings, stored in `server.properties` (catalog: `PROPERTIES`),
//! - game rules, read from the world and applied with `/gamerule` (see `game_rules.rs`).
//!
//! World settings only take effect when the server (re)starts; game rules
//! apply right away when the server runs.

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::core::game_rules::{self, Category};
use crate::core::instances::{self, Instance};
use crate::core::process::{ProcessManager, ServerStatus};
use crate::core::properties::Properties;
use crate::core::version;
use crate::error::{AppError, AppResult};
use crate::paths::{server_files, AppPaths};

/// A setting value, as exchanged with the frontend (`true`, `12`, `"easy"`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SettingValue {
    Bool(bool),
    Int(i64),
    Text(String),
}

impl fmt::Display for SettingValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(value) => write!(f, "{value}"),
            Self::Int(value) => write!(f, "{value}"),
            Self::Text(value) => write!(f, "{value}"),
        }
    }
}

/// Type, default value and allowed values of a setting.
#[derive(Debug, Clone, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum SettingKind {
    Bool {
        default: bool,
    },
    Int {
        default: i64,
        min: i64,
        max: Option<i64>,
    },
    Choice {
        default: &'static str,
        choices: &'static [&'static str],
    },
    Text {
        default: &'static str,
        max_length: usize,
    },
}

impl SettingKind {
    pub fn default_value(&self) -> SettingValue {
        match self {
            Self::Bool { default } => SettingValue::Bool(*default),
            Self::Int { default, .. } => SettingValue::Int(*default),
            Self::Choice { default, .. } | Self::Text { default, .. } => {
                SettingValue::Text(default.to_string())
            }
        }
    }

    /// Checks that `value` has the right type and is allowed.
    fn validate(&self, key: &str, value: &SettingValue) -> AppResult<()> {
        let valid = match (self, value) {
            (Self::Bool { .. }, SettingValue::Bool(_)) => true,
            (Self::Int { min, max, .. }, SettingValue::Int(n)) => {
                n >= min && max.is_none_or(|max| *n <= max)
            }
            (Self::Choice { choices, .. }, SettingValue::Text(text)) => {
                choices.contains(&text.as_str())
            }
            (Self::Text { max_length, .. }, SettingValue::Text(text)) => {
                text.chars().count() <= *max_length
            }
            _ => false,
        };
        if valid {
            Ok(())
        } else {
            Err(AppError::InvalidInput(format!("{key} = {value}")))
        }
    }
}

/// A `server.properties` entry shown to the user.
#[derive(Debug)]
struct PropertyDef {
    key: &'static str,
    kind: SettingKind,
    /// First version with this property.
    since: Option<&'static str>,
    /// First version without it (moved to a game rule, for example).
    removed_in: Option<&'static str>,
    /// Only useful before the world exists (seed, structures…).
    creation_only: bool,
}

const fn property(key: &'static str, kind: SettingKind) -> PropertyDef {
    PropertyDef {
        key,
        kind,
        since: None,
        removed_in: None,
        creation_only: false,
    }
}

const DIFFICULTIES: &[&str] = &["peaceful", "easy", "normal", "hard"];
const GAME_MODES: &[&str] = &["survival", "creative", "adventure", "spectator"];

/// Before 1.14, `difficulty` and `gamemode` were numbers (index in the list).
const NAMED_CHOICES_SINCE: &str = "1.14";

static PROPERTIES: &[PropertyDef] = &[
    property(
        "motd",
        SettingKind::Text {
            default: "A Minecraft Server",
            max_length: 100,
        },
    ),
    property(
        "difficulty",
        SettingKind::Choice {
            default: "easy",
            choices: DIFFICULTIES,
        },
    ),
    property(
        "gamemode",
        SettingKind::Choice {
            default: "survival",
            choices: GAME_MODES,
        },
    ),
    property("hardcore", SettingKind::Bool { default: false }),
    property(
        "max-players",
        SettingKind::Int {
            default: 20,
            min: 1,
            max: Some(1000),
        },
    ),
    property(
        "view-distance",
        SettingKind::Int {
            default: 10,
            min: 3,
            max: Some(32),
        },
    ),
    PropertyDef {
        since: Some("1.18"),
        ..property(
            "simulation-distance",
            SettingKind::Int {
                default: 10,
                min: 3,
                max: Some(32),
            },
        )
    },
    property(
        "spawn-protection",
        SettingKind::Int {
            default: 16,
            min: 0,
            max: Some(1000),
        },
    ),
    property("allow-flight", SettingKind::Bool { default: false }),
    // Became the `pvp`, `spawn_monsters` and `allow_entering_nether_using_portals`
    // game rules in 1.21.9.
    PropertyDef {
        removed_in: Some("1.21.9"),
        ..property("pvp", SettingKind::Bool { default: true })
    },
    PropertyDef {
        removed_in: Some("1.21.9"),
        ..property("spawn-monsters", SettingKind::Bool { default: true })
    },
    PropertyDef {
        removed_in: Some("1.21.9"),
        ..property("allow-nether", SettingKind::Bool { default: true })
    },
    PropertyDef {
        creation_only: true,
        ..property(
            "level-seed",
            SettingKind::Text {
                default: "",
                max_length: 100,
            },
        )
    },
    PropertyDef {
        creation_only: true,
        ..property("generate-structures", SettingKind::Bool { default: true })
    },
];

impl PropertyDef {
    fn is_available(&self, mc_version: &str) -> bool {
        self.since
            .is_none_or(|since| version::at_least(mc_version, since))
            && self
                .removed_in
                .is_none_or(|removed| !version::at_least(mc_version, removed))
    }

    /// Reads the value from the file text, falling back to the default.
    fn parse(&self, raw: Option<String>) -> SettingValue {
        let Some(raw) = raw else {
            return self.kind.default_value();
        };
        let raw = raw.trim();
        match &self.kind {
            SettingKind::Bool { default } => SettingValue::Bool(raw.parse().unwrap_or(*default)),
            SettingKind::Int { default, .. } => SettingValue::Int(raw.parse().unwrap_or(*default)),
            SettingKind::Choice { default, choices } => {
                // Named ("easy") or, before 1.14, numbered ("1").
                let name = choices
                    .iter()
                    .find(|choice| **choice == raw)
                    .or_else(|| raw.parse::<usize>().ok().and_then(|i| choices.get(i)))
                    .unwrap_or(default);
                SettingValue::Text(name.to_string())
            }
            SettingKind::Text { .. } => SettingValue::Text(raw.to_string()),
        }
    }

    /// Text written in `server.properties` for this version.
    fn format(&self, value: &SettingValue, mc_version: &str) -> String {
        match (&self.kind, value) {
            (SettingKind::Choice { choices, .. }, SettingValue::Text(name))
                if !version::at_least(mc_version, NAMED_CHOICES_SINCE) =>
            {
                choices
                    .iter()
                    .position(|choice| choice == name)
                    .unwrap_or(0)
                    .to_string()
            }
            _ => value.to_string(),
        }
    }
}

/// A setting as described to the UI. Mirrored by `SettingDef` in `src/types/index.ts`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingDef {
    pub key: &'static str,
    #[serde(flatten)]
    pub kind: SettingKind,
    /// Game rules only.
    pub category: Option<Category>,
    pub creation_only: bool,
}

/// Every setting available for a Minecraft version.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Catalog {
    pub properties: Vec<SettingDef>,
    pub game_rules: Vec<SettingDef>,
}

pub fn catalog(mc_version: &str) -> Catalog {
    Catalog {
        properties: PROPERTIES
            .iter()
            .filter(|def| def.is_available(mc_version))
            .map(|def| SettingDef {
                key: def.key,
                kind: def.kind.clone(),
                category: None,
                creation_only: def.creation_only,
            })
            .collect(),
        game_rules: game_rules::available(mc_version)
            .map(|rule| SettingDef {
                key: rule.key,
                kind: rule.kind.clone(),
                category: Some(rule.category),
                creation_only: false,
            })
            .collect(),
    }
}

/// Current values of a server's settings. Mirrored by `GameSettings` in `src/types/index.ts`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameSettings {
    pub properties: BTreeMap<String, SettingValue>,
    pub game_rules: BTreeMap<String, SettingValue>,
}

/// Reads the settings of an existing server.
///
/// Game rules, by priority: change waiting for the next start, live value
/// seen in the console (server running), value saved in the world, default.
pub fn read(
    server_dir: &Path,
    instance: &Instance,
    live_rules: &BTreeMap<String, SettingValue>,
) -> AppResult<GameSettings> {
    let file = Properties::load(&server_dir.join(server_files::PROPERTIES)).unwrap_or_default();
    let mc = &instance.mc_version;
    let world_rules = game_rules::read_world(&world_dir(server_dir, &file));

    let properties = PROPERTIES
        .iter()
        .filter(|def| def.is_available(mc))
        .map(|def| (def.key.to_string(), def.parse(file.get_text(def.key))))
        .collect();
    let game_rules = game_rules::available(mc)
        .map(|rule| {
            let value = instance
                .pending_game_rules
                .get(rule.key)
                .or_else(|| live_rules.get(rule.key))
                .or_else(|| world_rules.get(rule.key))
                .cloned()
                .unwrap_or_else(|| rule.kind.default_value());
            (rule.key.to_string(), value)
        })
        .collect();
    Ok(GameSettings {
        properties,
        game_rules,
    })
}

/// The world folder (`level-name` in server.properties, `world` by default).
fn world_dir(server_dir: &Path, properties: &Properties) -> PathBuf {
    let name = properties
        .get_text("level-name")
        .filter(|name| !name.trim().is_empty())
        .unwrap_or_else(|| "world".to_string());
    server_dir.join(name)
}

/// What `apply` changed.
#[derive(Debug, Default)]
pub struct Applied {
    /// `server.properties` changed: only read when the server starts.
    pub properties_changed: bool,
    /// Game rules to send to the server (now or at next start).
    pub game_rules: BTreeMap<String, SettingValue>,
}

/// Checks the settings, writes the world settings to `server.properties`
/// and returns the game rules to apply. Unknown keys are ignored.
/// `creating` also writes the settings that only matter before the world exists.
pub fn apply(
    server_dir: &Path,
    mc_version: &str,
    settings: &GameSettings,
    creating: bool,
) -> AppResult<Applied> {
    let properties_path = server_dir.join(server_files::PROPERTIES);
    let mut file = Properties::load(&properties_path).unwrap_or_default();
    let mut applied = Applied::default();

    for def in PROPERTIES.iter().filter(|def| def.is_available(mc_version)) {
        if def.creation_only && !creating {
            continue;
        }
        let Some(value) = settings.properties.get(def.key) else {
            continue;
        };
        def.kind.validate(def.key, value)?;
        let text = def.format(value, mc_version);
        if file.get(def.key) != Some(text.as_str()) {
            file.set(def.key, &text);
            applied.properties_changed = true;
        }
    }

    for rule in game_rules::available(mc_version) {
        if let Some(value) = settings.game_rules.get(rule.key) {
            rule.kind.validate(rule.key, value)?;
            applied
                .game_rules
                .insert(rule.key.to_string(), value.clone());
        }
    }

    if applied.properties_changed {
        file.save(&properties_path)?;
    }
    Ok(applied)
}

/// Result of `save`. Mirrored by `SaveOutcome` in `src/types/index.ts`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveOutcome {
    /// World settings changed while the server runs: restart to apply them.
    pub needs_restart: bool,
}

/// Saves the settings the user changed (`settings` only holds those).
///
/// - World settings: written to `server.properties`, used at next start.
/// - Game rules: sent right away if the server runs, otherwise kept in
///   `instance.json` and sent once it has started.
pub async fn save(
    paths: &AppPaths,
    processes: &ProcessManager,
    id: &str,
    settings: &GameSettings,
) -> AppResult<SaveOutcome> {
    let mut instance = instances::load(paths, id)?;
    let applied = apply(&paths.server_dir(id), &instance.mc_version, settings, false)?;

    if !applied.game_rules.is_empty() {
        if processes.accepts_commands(id).await {
            let commands =
                game_rules::commands(&instance.mc_version, instance.loader, &applied.game_rules);
            for command in commands {
                processes.send_command(id, &command).await?;
            }
            processes.record_live_rules(id, &applied.game_rules).await;
        } else {
            instance.pending_game_rules.extend(applied.game_rules);
            instances::save(paths, &instance)?;
        }
    }

    let running = processes.status(id).await != ServerStatus::Stopped;
    let needs_restart = applied.properties_changed && running;
    if needs_restart {
        processes.mark_needs_restart(id).await;
    }
    Ok(SaveOutcome { needs_restart })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::providers::Loader;

    fn instance(mc_version: &str) -> Instance {
        Instance {
            id: "test".into(),
            name: "Test".into(),
            mc_version: mc_version.into(),
            loader: Loader::Vanilla,
            loader_version: None,
            java_version: 21,
            memory_mb: 2048,
            port: 25565,
            created_at: String::new(),
            pending_game_rules: BTreeMap::new(),
            playit_tunnel_id: None,
        }
    }

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("minyhost-settings-{name}"));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn catalog_follows_the_version() {
        let keys =
            |mc: &str| -> Vec<&str> { catalog(mc).properties.iter().map(|d| d.key).collect() };
        assert!(keys("1.21.1").contains(&"pvp"));
        assert!(!keys("26.3").contains(&"pvp"));
        assert!(!keys("1.17.1").contains(&"simulation-distance"));
        assert!(catalog("26.3").game_rules.iter().any(|d| d.key == "pvp"));
    }

    #[test]
    fn values_are_validated() {
        let view = &PROPERTIES[5].kind; // view-distance, 3..=32
        assert!(view
            .validate("view-distance", &SettingValue::Int(12))
            .is_ok());
        assert!(view
            .validate("view-distance", &SettingValue::Int(64))
            .is_err());
        assert!(view
            .validate("view-distance", &SettingValue::Bool(true))
            .is_err());
    }

    #[test]
    fn apply_writes_properties_and_returns_rules() {
        let dir = temp_dir("apply");
        let settings = GameSettings {
            properties: BTreeMap::from([
                ("difficulty".into(), SettingValue::Text("hard".into())),
                ("level-seed".into(), SettingValue::Text("123".into())),
            ]),
            game_rules: BTreeMap::from([("keep_inventory".into(), SettingValue::Bool(true))]),
        };
        let applied = apply(&dir, "26.3", &settings, false).unwrap();

        let file = Properties::load(&dir.join(server_files::PROPERTIES)).unwrap();
        assert_eq!(file.get("difficulty"), Some("hard"));
        assert_eq!(file.get("level-seed"), None); // creation only
        assert!(applied.properties_changed);
        assert_eq!(applied.game_rules, settings.game_rules);

        // Same values again: nothing to restart for.
        assert!(
            !apply(&dir, "26.3", &settings, false)
                .unwrap()
                .properties_changed
        );
    }

    #[test]
    fn invalid_rule_value_is_rejected() {
        let dir = temp_dir("invalid");
        let settings = GameSettings {
            game_rules: BTreeMap::from([("random_tick_speed".into(), SettingValue::Bool(true))]),
            ..Default::default()
        };
        assert!(apply(&dir, "26.3", &settings, false).is_err());
    }

    #[test]
    fn read_prefers_pending_then_live_then_world() {
        use crate::core::nbt::test_writer::{compound, named};

        let dir = temp_dir("read");
        let data = dir.join("world").join("data").join("minecraft");
        std::fs::create_dir_all(&data).unwrap();
        let world = named(
            10,
            "",
            &compound(&[named(
                10,
                "data",
                &compound(&[
                    named(1, "minecraft:keep_inventory", &[1]),
                    named(3, "minecraft:random_tick_speed", &7i32.to_be_bytes()),
                    named(1, "minecraft:pvp", &[0]),
                ]),
            )]),
        );
        std::fs::write(data.join("game_rules.dat"), world).unwrap();

        let mut server = instance("26.3");
        server
            .pending_game_rules
            .insert("pvp".into(), SettingValue::Bool(true));
        let live = BTreeMap::from([("random_tick_speed".into(), SettingValue::Int(10))]);

        let rules = read(&dir, &server, &live).unwrap().game_rules;
        assert_eq!(rules["keep_inventory"], SettingValue::Bool(true)); // world
        assert_eq!(rules["random_tick_speed"], SettingValue::Int(10)); // live
        assert_eq!(rules["pvp"], SettingValue::Bool(true)); // pending
        assert_eq!(rules["fall_damage"], SettingValue::Bool(true)); // default
    }

    #[test]
    fn old_versions_use_numbered_choices() {
        let dir = temp_dir("legacy");
        let settings = GameSettings {
            properties: BTreeMap::from([(
                "gamemode".into(),
                SettingValue::Text("creative".into()),
            )]),
            ..Default::default()
        };
        apply(&dir, "1.12.2", &settings, false).unwrap();
        let file = Properties::load(&dir.join(server_files::PROPERTIES)).unwrap();
        assert_eq!(file.get("gamemode"), Some("1"));
        assert_eq!(
            read(&dir, &instance("1.12.2"), &BTreeMap::new())
                .unwrap()
                .properties["gamemode"],
            SettingValue::Text("creative".into())
        );
    }
}
