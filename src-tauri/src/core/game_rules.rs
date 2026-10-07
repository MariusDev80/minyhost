//! Game rules catalog (`/gamerule`), and how to read and apply them.
//!
//! The world is the source of truth: values are read from its save files
//! (`read_world`), and while the server runs, from the console, which reports
//! every change, whoever made it (`parse_change`).
//!
//! A change made in MinyHost is sent with `/gamerule` right away if the
//! server runs. Otherwise (or at creation, before the world exists) it waits
//! in `instance.json` (`pendingGameRules`) and is sent once the server has
//! started (`commands`).
//!
//! Minecraft 1.21.11 renamed every rule (`keepInventory` -> `keep_inventory`)
//! and flipped a few (`disableRaids` -> `raids`). Each entry knows both names.
//! The values below were extracted from the server jars of 1.13.2 to 26.3
//! with Mojang's data generator (`--reports`), then checked on a live server.
//!
//! To add a rule: add a line to `GAME_RULES`, then its French text in
//! `src/i18n/fr.ts` (`gameSettings.gameRules`).

use std::collections::BTreeMap;
use std::path::Path;

use serde::Serialize;

use crate::core::game_settings::{SettingKind, SettingValue};
use crate::core::nbt::{self, Tag};
use crate::core::providers::Loader;
use crate::core::version;

/// First version using the new rule names.
const MODERN_NAMES_SINCE: &str = "1.21.11";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Category {
    Players,
    Mobs,
    World,
    Drops,
    Commands,
}

#[derive(Debug)]
pub struct GameRule {
    /// Stable id: the modern name, or the legacy one for rules that no longer exist.
    pub key: &'static str,
    /// Name since 1.21.11 (`None` if the rule was removed).
    modern: Option<&'static str>,
    /// Name up to 1.21.10 (`None` for rules added in 1.21.11 or later).
    legacy: Option<&'static str>,
    /// The legacy rule means the opposite (`disableRaids` vs `raids`).
    legacy_inverted: bool,
    /// First version where the legacy rule exists.
    since: &'static str,
    pub kind: SettingKind,
    pub category: Category,
}

const fn bool_rule(
    modern: &'static str,
    legacy: &'static str,
    since: &'static str,
    default: bool,
    category: Category,
) -> GameRule {
    GameRule {
        key: modern,
        modern: Some(modern),
        legacy: Some(legacy),
        legacy_inverted: false,
        since,
        kind: SettingKind::Bool { default },
        category,
    }
}

const fn int_rule(
    modern: &'static str,
    legacy: &'static str,
    since: &'static str,
    default: i64,
    (min, max): (i64, Option<i64>),
    category: Category,
) -> GameRule {
    GameRule {
        key: modern,
        modern: Some(modern),
        legacy: Some(legacy),
        legacy_inverted: false,
        since,
        kind: SettingKind::Int { default, min, max },
        category,
    }
}

/// A rule whose legacy version means the opposite.
const fn inverted(rule: GameRule) -> GameRule {
    GameRule {
        legacy_inverted: true,
        ..rule
    }
}

/// A rule that only exists up to 1.21.10.
const fn legacy_only(rule: GameRule) -> GameRule {
    GameRule {
        key: match rule.legacy {
            Some(name) => name,
            None => rule.key,
        },
        modern: None,
        ..rule
    }
}

/// A rule that only exists since 1.21.11.
const fn modern_only(rule: GameRule) -> GameRule {
    GameRule {
        legacy: None,
        since: MODERN_NAMES_SINCE,
        ..rule
    }
}

const NO_MAX: Option<i64> = None;

use Category::*;

#[rustfmt::skip]
pub static GAME_RULES: &[GameRule] = &[
    // Players
    bool_rule("keep_inventory", "keepInventory", "1.13.2", false, Players),
    bool_rule("natural_health_regeneration", "naturalRegeneration", "1.13.2", true, Players),
    bool_rule("immediate_respawn", "doImmediateRespawn", "1.15.2", false, Players),
    int_rule("players_sleeping_percentage", "playersSleepingPercentage", "1.17.1", 100, (0, NO_MAX), Players),
    bool_rule("pvp", "pvp", "1.21.9", true, Players),
    int_rule("respawn_radius", "spawnRadius", "1.13.2", 10, (0, NO_MAX), Players),
    bool_rule("fall_damage", "fallDamage", "1.15.2", true, Players),
    bool_rule("fire_damage", "fireDamage", "1.15.2", true, Players),
    bool_rule("drowning_damage", "drowningDamage", "1.15.2", true, Players),
    bool_rule("freeze_damage", "freezeDamage", "1.17.1", true, Players),
    bool_rule("ender_pearls_vanish_on_death", "enderPearlsVanishOnDeath", "1.20.2", true, Players),
    bool_rule("locator_bar", "locatorBar", "1.21.9", true, Players),
    bool_rule("show_death_messages", "showDeathMessages", "1.13.2", true, Players),
    bool_rule("show_advancement_messages", "announceAdvancements", "1.13.2", true, Players),
    bool_rule("limited_crafting", "doLimitedCrafting", "1.13.2", false, Players),
    bool_rule("reduced_debug_info", "reducedDebugInfo", "1.13.2", false, Players),
    bool_rule("allow_entering_nether_using_portals", "allowEnteringNetherUsingPortals", "1.21.9", true, Players),
    int_rule("players_nether_portal_default_delay", "playersNetherPortalDefaultDelay", "1.20.4", 80, (0, NO_MAX), Players),
    int_rule("players_nether_portal_creative_delay", "playersNetherPortalCreativeDelay", "1.20.4", 0, (0, NO_MAX), Players),
    bool_rule("spectators_generate_chunks", "spectatorsGenerateChunks", "1.13.2", true, Players),
    // Mobs
    bool_rule("spawn_mobs", "doMobSpawning", "1.13.2", true, Mobs),
    bool_rule("spawn_monsters", "spawnMonsters", "1.21.9", true, Mobs),
    bool_rule("spawn_phantoms", "doInsomnia", "1.15.2", true, Mobs),
    bool_rule("spawn_patrols", "doPatrolSpawning", "1.15.2", true, Mobs),
    bool_rule("spawn_wandering_traders", "doTraderSpawning", "1.15.2", true, Mobs),
    bool_rule("spawn_wardens", "doWardenSpawning", "1.19", true, Mobs),
    inverted(bool_rule("raids", "disableRaids", "1.14.4", true, Mobs)),
    bool_rule("mob_griefing", "mobGriefing", "1.13.2", true, Mobs),
    bool_rule("forgive_dead_players", "forgiveDeadPlayers", "1.16.5", true, Mobs),
    bool_rule("universal_anger", "universalAnger", "1.16.5", false, Mobs),
    int_rule("max_entity_cramming", "maxEntityCramming", "1.13.2", 24, (0, NO_MAX), Mobs),
    bool_rule("spawner_blocks_work", "spawnerBlocksEnabled", "1.21.9", true, Mobs),
    // World
    bool_rule("advance_time", "doDaylightCycle", "1.13.2", true, World),
    bool_rule("advance_weather", "doWeatherCycle", "1.13.2", true, World),
    int_rule("random_tick_speed", "randomTickSpeed", "1.13.2", 3, (0, NO_MAX), World),
    modern_only(int_rule("fire_spread_radius_around_player", "", "", 128, (-1, NO_MAX), World)),
    legacy_only(bool_rule("", "doFireTick", "1.13.2", true, World)),
    legacy_only(bool_rule("", "allowFireTicksAwayFromPlayer", "1.21.5", false, World)),
    bool_rule("spread_vines", "doVinesSpread", "1.19.4", true, World),
    bool_rule("water_source_conversion", "waterSourceConversion", "1.19.4", true, World),
    bool_rule("lava_source_conversion", "lavaSourceConversion", "1.19.4", false, World),
    int_rule("max_snow_accumulation_height", "snowAccumulationHeight", "1.19.4", 1, (0, Some(8)), World),
    bool_rule("tnt_explodes", "tntExplodes", "1.21.5", true, World),
    bool_rule("projectiles_can_break_blocks", "projectilesCanBreakBlocks", "1.20.4", true, World),
    bool_rule("global_sound_events", "globalSoundEvents", "1.19.4", true, World),
    // Drops
    bool_rule("block_drops", "doTileDrops", "1.13.2", true, Drops),
    bool_rule("mob_drops", "doMobLoot", "1.13.2", true, Drops),
    bool_rule("entity_drops", "doEntityDrops", "1.13.2", true, Drops),
    bool_rule("block_explosion_drop_decay", "blockExplosionDropDecay", "1.19.4", true, Drops),
    bool_rule("mob_explosion_drop_decay", "mobExplosionDropDecay", "1.19.4", true, Drops),
    bool_rule("tnt_explosion_drop_decay", "tntExplosionDropDecay", "1.19.4", false, Drops),
    // Commands
    bool_rule("command_block_output", "commandBlockOutput", "1.13.2", true, Commands),
    bool_rule("command_blocks_work", "commandBlocksEnabled", "1.21.9", true, Commands),
    bool_rule("send_command_feedback", "sendCommandFeedback", "1.13.2", true, Commands),
    bool_rule("log_admin_commands", "logAdminCommands", "1.13.2", true, Commands),
    int_rule("max_command_sequence_length", "maxCommandChainLength", "1.13.2", 65536, (0, NO_MAX), Commands),
    int_rule("max_command_forks", "maxCommandForkCount", "1.20.4", 65536, (0, NO_MAX), Commands),
    int_rule("max_block_modifications", "commandModificationBlockLimit", "1.19.4", 32768, (1, NO_MAX), Commands),
    inverted(bool_rule("player_movement_check", "disablePlayerMovementCheck", "1.21.5", true, Commands)),
    inverted(bool_rule("elytra_movement_check", "disableElytraMovementCheck", "1.13.2", true, Commands)),
];

impl GameRule {
    /// The name to use in `/gamerule` for this version, `None` if the rule
    /// does not exist in it.
    fn command_name(&self, mc_version: &str) -> Option<&'static str> {
        if version::at_least(mc_version, MODERN_NAMES_SINCE) {
            self.modern
        } else if version::at_least(mc_version, self.since) {
            self.legacy
        } else {
            None
        }
    }

    pub fn is_available(&self, mc_version: &str) -> bool {
        self.command_name(mc_version).is_some()
    }

    /// `/gamerule` value for this version (flipped for inverted legacy rules).
    fn command_value(&self, mc_version: &str, value: &SettingValue) -> String {
        let legacy = !version::at_least(mc_version, MODERN_NAMES_SINCE);
        match value {
            SettingValue::Bool(on) if legacy && self.legacy_inverted => (!on).to_string(),
            other => other.to_string(),
        }
    }
}

/// Rules that exist in this Minecraft version.
pub fn available(mc_version: &str) -> impl Iterator<Item = &'static GameRule> + '_ {
    GAME_RULES
        .iter()
        .filter(move |rule| rule.is_available(mc_version))
}

pub fn find(key: &str) -> Option<&'static GameRule> {
    GAME_RULES.iter().find(|rule| rule.key == key)
}

impl GameRule {
    /// Turns a raw value ("true", "3", or a legacy inverted rule) into the
    /// value shown to the user.
    fn parse_value(&self, raw: &str, legacy: bool) -> Option<SettingValue> {
        match self.kind {
            SettingKind::Bool { .. } => {
                let on: bool = raw.parse().ok()?;
                Some(SettingValue::Bool(on != (legacy && self.legacy_inverted)))
            }
            SettingKind::Int { .. } => raw.parse().ok().map(SettingValue::Int),
            _ => None,
        }
    }
}

/// Game rules saved in a world folder (`server_dir/world`).
///
/// Since 26.1 they live in `game_rules.dat` files (typed values,
/// new names); before, in `level.dat` → `Data.GameRules` (text values,
/// legacy names). Returns an empty map if the world does not exist yet.
pub fn read_world(world_dir: &Path) -> BTreeMap<String, SettingValue> {
    // Vanilla and Fabric keep one file for the whole world; Paper keeps one
    // per dimension (we show the overworld's).
    let modern_files = [
        world_dir.join("data/minecraft/game_rules.dat"),
        world_dir.join("dimensions/minecraft/overworld/data/minecraft/game_rules.dat"),
    ];
    if let Some(root) = modern_files
        .iter()
        .find_map(|file| nbt::read_file(file).ok())
    {
        let entries = root.get("data").and_then(Tag::as_compound);
        return entries
            .into_iter()
            .flatten()
            .filter_map(|(name, tag)| {
                let name = name.strip_prefix("minecraft:").unwrap_or(name);
                let rule = GAME_RULES.iter().find(|r| r.modern == Some(name))?;
                let value = match tag {
                    Tag::Byte(b) => SettingValue::Bool(*b != 0),
                    Tag::Int(n) => SettingValue::Int(i64::from(*n)),
                    _ => return None,
                };
                Some((rule.key.to_string(), value))
            })
            .collect();
    }

    let Ok(level) = nbt::read_file(&world_dir.join("level.dat")) else {
        return BTreeMap::new();
    };
    let entries = level
        .get("Data")
        .and_then(|data| data.get("GameRules"))
        .and_then(Tag::as_compound);
    entries
        .into_iter()
        .flatten()
        .filter_map(|(name, tag)| {
            let rule = GAME_RULES.iter().find(|r| r.legacy == Some(name))?;
            let Tag::String(raw) = tag else { return None };
            Some((rule.key.to_string(), rule.parse_value(raw, true)?))
        })
        .collect()
}

/// Detects a game rule change in a console line, wherever it came from:
/// the console, MinyHost, or a player in game.
///
/// - 26.x: `[..]: System chat: Game rule keep_inventory is now set to true`
/// - in game: `[..]: [Steve: Game rule keep_inventory is now set to true]`
/// - <= 1.21.10: `[..]: Gamerule keepInventory is now set to: true`
///
/// The message must follow the log prefix (`]: `), so a player typing the
/// same sentence in the chat (`]: <Steve> Game rule…`) is ignored.
pub fn parse_change(line: &str, mc_version: &str) -> Option<(String, SettingValue)> {
    let (_, message) = line.split_once("]: ")?;
    let message = message.strip_prefix("System chat: ").unwrap_or(message);
    let message = match message.strip_prefix('[') {
        // "[Steve: Game rule …]": a command run by a player.
        Some(rest) => rest.split_once(": ")?.1.trim_end_matches(']'),
        None => message,
    };

    let rest = message
        .strip_prefix("Game rule ")
        .or_else(|| message.strip_prefix("Gamerule "))?;
    let (name, rest) = rest.split_once(' ')?;
    let name = name.strip_prefix("minecraft:").unwrap_or(name);
    let raw = ["is now set to", "is already set to", "is currently set to"]
        .iter()
        .find_map(|phrase| rest.strip_prefix(phrase))?
        .trim_start_matches(':')
        .trim();

    let rule = GAME_RULES
        .iter()
        .find(|rule| rule.command_name(mc_version) == Some(name))?;
    let legacy = !version::at_least(mc_version, MODERN_NAMES_SINCE);
    Some((rule.key.to_string(), rule.parse_value(raw, legacy)?))
}

/// `/gamerule` console commands setting these rules.
///
/// Paper keeps separate rules for each dimension, so the command is sent in
/// each of them; Vanilla and Fabric share one set of rules for all.
pub fn commands(
    mc_version: &str,
    loader: Loader,
    rules: &BTreeMap<String, SettingValue>,
) -> Vec<String> {
    let dimensions: &[&str] = match loader {
        Loader::Paper => &[
            "minecraft:overworld",
            "minecraft:the_nether",
            "minecraft:the_end",
        ],
        Loader::Vanilla | Loader::Fabric => &[],
    };

    let mut commands = Vec::new();
    for (key, value) in rules {
        let Some(rule) = find(key) else { continue };
        let Some(name) = rule.command_name(mc_version) else {
            continue;
        };
        let gamerule = format!("gamerule {name} {}", rule.command_value(mc_version, value));
        if dimensions.is_empty() {
            commands.push(gamerule);
        } else {
            for dimension in dimensions {
                commands.push(format!("execute in {dimension} run {gamerule}"));
            }
        }
    }
    commands
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rules(entries: &[(&str, SettingValue)]) -> BTreeMap<String, SettingValue> {
        entries
            .iter()
            .map(|(key, value)| (key.to_string(), value.clone()))
            .collect()
    }

    #[test]
    fn keys_are_unique() {
        let mut keys: Vec<_> = GAME_RULES.iter().map(|rule| rule.key).collect();
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), GAME_RULES.len());
    }

    #[test]
    fn uses_the_right_name_for_each_version() {
        let keep = rules(&[("keep_inventory", SettingValue::Bool(true))]);
        assert_eq!(
            commands("26.3", Loader::Vanilla, &keep),
            ["gamerule keep_inventory true"]
        );
        assert_eq!(
            commands("1.21.10", Loader::Vanilla, &keep),
            ["gamerule keepInventory true"]
        );
    }

    #[test]
    fn flips_inverted_legacy_rules() {
        let no_raids = rules(&[("raids", SettingValue::Bool(false))]);
        assert_eq!(
            commands("1.20.1", Loader::Fabric, &no_raids),
            ["gamerule disableRaids true"]
        );
        assert_eq!(
            commands("26.1", Loader::Fabric, &no_raids),
            ["gamerule raids false"]
        );
    }

    #[test]
    fn skips_rules_missing_from_the_version() {
        let sleep = rules(&[("players_sleeping_percentage", SettingValue::Int(50))]);
        assert!(commands("1.16.5", Loader::Vanilla, &sleep).is_empty());
        let fire = rules(&[("doFireTick", SettingValue::Bool(false))]);
        assert!(commands("26.3", Loader::Vanilla, &fire).is_empty());
        assert_eq!(
            commands("1.20.1", Loader::Vanilla, &fire),
            ["gamerule doFireTick false"]
        );
    }

    #[test]
    fn paper_applies_rules_in_every_dimension() {
        let keep = rules(&[("keep_inventory", SettingValue::Bool(true))]);
        let commands = commands("26.3", Loader::Paper, &keep);
        assert_eq!(commands.len(), 3);
        assert_eq!(
            commands[1],
            "execute in minecraft:the_nether run gamerule keep_inventory true"
        );
    }

    #[test]
    fn detects_changes_in_the_console() {
        let change = |line: &str, mc: &str| parse_change(line, mc);
        assert_eq!(
            change(
                "[12:00:00] [Server thread/INFO]: System chat: Game rule keep_inventory is now set to true",
                "26.3"
            ),
            Some(("keep_inventory".into(), SettingValue::Bool(true)))
        );
        // Typed in game by an operator (Paper log format).
        assert_eq!(
            change(
                "[12:00:00 INFO]: [Steve: Game rule random_tick_speed is now set to 10]",
                "26.3"
            ),
            Some(("random_tick_speed".into(), SettingValue::Int(10)))
        );
        // Legacy name and wording, inverted rule.
        assert_eq!(
            change(
                "[12:00:00] [Server thread/INFO]: Gamerule disableRaids is now set to: true",
                "1.21.1"
            ),
            Some(("raids".into(), SettingValue::Bool(false)))
        );
    }

    #[test]
    fn ignores_the_same_sentence_typed_in_the_chat() {
        let chat =
            "[12:00:00] [Server thread/INFO]: <Steve> Game rule keep_inventory is now set to true";
        assert_eq!(parse_change(chat, "26.3"), None);
    }

    #[test]
    fn reads_both_world_formats() {
        use crate::core::nbt::test_writer::{compound, named, string};

        let dir = std::env::temp_dir().join("minyhost-world-rules");
        let _ = std::fs::remove_dir_all(&dir);

        // <= 1.21.10: level.dat, text values, legacy names.
        std::fs::create_dir_all(&dir).unwrap();
        let level = named(
            10,
            "",
            &compound(&[named(
                10,
                "Data",
                &compound(&[named(
                    10,
                    "GameRules",
                    &compound(&[
                        named(8, "keepInventory", &string("true")),
                        named(8, "disableRaids", &string("true")),
                        named(8, "randomTickSpeed", &string("7")),
                    ]),
                )]),
            )]),
        );
        std::fs::write(dir.join("level.dat"), level).unwrap();
        let rules = read_world(&dir);
        assert_eq!(rules["keep_inventory"], SettingValue::Bool(true));
        assert_eq!(rules["raids"], SettingValue::Bool(false));
        assert_eq!(rules["random_tick_speed"], SettingValue::Int(7));

        // 26.x: data/minecraft/game_rules.dat, typed values, new names.
        let data_dir = dir.join("data").join("minecraft");
        std::fs::create_dir_all(&data_dir).unwrap();
        let modern = named(
            10,
            "",
            &compound(&[named(
                10,
                "data",
                &compound(&[
                    named(1, "minecraft:keep_inventory", &[0]),
                    named(
                        3,
                        "minecraft:players_sleeping_percentage",
                        &50i32.to_be_bytes(),
                    ),
                ]),
            )]),
        );
        std::fs::write(data_dir.join("game_rules.dat"), modern).unwrap();
        let rules = read_world(&dir);
        assert_eq!(rules["keep_inventory"], SettingValue::Bool(false));
        assert_eq!(rules["players_sleeping_percentage"], SettingValue::Int(50));
    }
}
