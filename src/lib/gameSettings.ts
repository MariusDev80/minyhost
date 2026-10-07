// Helpers for game settings values (see `core/game_settings.rs` on the Rust side).
import { t } from "@/i18n";
import type {
  GameSettings,
  SettingDef,
  SettingValue,
  SettingsCatalog,
} from "@/types";

export const emptySettings: GameSettings = { properties: {}, gameRules: {} };

/** Current value of a setting, or its default. */
export function valueOf(
  values: Record<string, SettingValue>,
  def: SettingDef,
): SettingValue {
  return values[def.key] ?? def.default;
}

/** Only the settings that differ from their default (sent at creation). */
export function onlyChanged(
  catalog: SettingsCatalog,
  settings: GameSettings,
): GameSettings {
  const pick = (defs: SettingDef[], values: Record<string, SettingValue>) =>
    Object.fromEntries(
      defs
        .filter((def) => def.key in values && values[def.key] !== def.default)
        .map((def) => [def.key, values[def.key]]),
    );
  return {
    properties: pick(catalog.properties, settings.properties),
    gameRules: pick(catalog.gameRules, settings.gameRules),
  };
}

/** What the user changed in `draft` compared to `base` (sent when saving). */
export function changesFrom(
  base: GameSettings,
  draft: GameSettings,
): GameSettings {
  const diff = (
    before: Record<string, SettingValue>,
    after: Record<string, SettingValue>,
  ) =>
    Object.fromEntries(
      Object.entries(after).filter(([key, value]) => before[key] !== value),
    );
  return {
    properties: diff(base.properties, draft.properties),
    gameRules: diff(base.gameRules, draft.gameRules),
  };
}

/**
 * New values arrived (e.g. a game rule changed in game) while the user was
 * editing: keep their edits, take the new values everywhere else.
 */
export function rebase(
  draft: GameSettings,
  base: GameSettings,
  latest: GameSettings,
): GameSettings {
  const merge = (
    edited: Record<string, SettingValue>,
    before: Record<string, SettingValue>,
    now: Record<string, SettingValue>,
  ) =>
    Object.fromEntries(
      Object.entries(now).map(([key, value]) => [
        key,
        edited[key] !== before[key] ? edited[key] : value,
      ]),
    );
  return {
    properties: merge(draft.properties, base.properties, latest.properties),
    gameRules: merge(draft.gameRules, base.gameRules, latest.gameRules),
  };
}

export function hasChanges(changes: GameSettings): boolean {
  return (
    Object.keys(changes.properties).length > 0 ||
    Object.keys(changes.gameRules).length > 0
  );
}

/** French label and description, or the technical name if there is none. */
export function settingText(key: string): {
  label: string;
  description?: string;
} {
  const labels: Record<string, { label: string; description?: string }> =
    t.gameSettings.labels;
  return labels[key] ?? { label: key };
}

/** A value as shown to the user ("activé", "Difficile", "10"). */
export function formatValue(def: SettingDef, value: SettingValue): string {
  if (typeof value === "boolean")
    return value ? t.gameSettings.on : t.gameSettings.off;
  if (def.kind === "choice") return choiceLabel(def.key, String(value));
  if (value === "") return "—";
  return String(value);
}

export function choiceLabel(key: string, choice: string): string {
  const choices: Record<string, Record<string, string>> = t.gameSettings
    .choices;
  return choices[key]?.[choice] ?? choice;
}
