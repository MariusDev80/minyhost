import { useState } from "react";
import { Search } from "lucide-react";
import { Input } from "@/components/ui/input";
import { t } from "@/i18n";
import { settingText, valueOf } from "@/lib/gameSettings";
import type {
  GameRuleCategory,
  GameSettings,
  SettingDef,
  SettingValue,
  SettingsCatalog,
} from "@/types";
import { SettingField } from "./SettingField";

const categoryOrder: GameRuleCategory[] = [
  "players",
  "mobs",
  "world",
  "drops",
  "commands",
];

/**
 * World settings and game rules, with a search field.
 * Used by the creation form (`mode="create"`) and the "Paramètres" tab.
 */
export function GameSettingsForm({
  catalog,
  value,
  onChange,
  mode,
}: {
  catalog: SettingsCatalog;
  value: GameSettings;
  onChange: (value: GameSettings) => void;
  mode: "create" | "edit";
}) {
  const [query, setQuery] = useState("");
  const matches = (def: SettingDef) => {
    if (!query.trim()) return true;
    const { label, description = "" } = settingText(def.key);
    return normalize(`${label} ${description} ${def.key}`).includes(
      normalize(query),
    );
  };

  const properties = catalog.properties
    // At creation the server name is the message; seed & co. only matter then.
    .filter((def) =>
      mode === "create" ? def.key !== "motd" : !def.creationOnly,
    )
    .filter(matches);
  const ruleGroups = categoryOrder
    .map((category) => ({
      category,
      rules: catalog.gameRules.filter(
        (def) => def.category === category && matches(def),
      ),
    }))
    .filter((group) => group.rules.length > 0);

  const set =
    (section: keyof GameSettings) => (key: string, setting: SettingValue) =>
      onChange({ ...value, [section]: { ...value[section], [key]: setting } });
  const setProperty = set("properties");
  const setRule = set("gameRules");

  return (
    <div className="space-y-6">
      <div className="relative max-w-sm">
        <Search className="pointer-events-none absolute top-1/2 left-2.5 size-4 -translate-y-1/2 text-muted-foreground" />
        <Input
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder={t.gameSettings.search}
          aria-label={t.gameSettings.search}
          className="pl-8"
        />
      </div>

      {properties.length > 0 && (
        <Section
          title={t.gameSettings.world}
          description={t.gameSettings.worldDescription}
        >
          {properties.map((def) => (
            <SettingField
              key={def.key}
              def={def}
              value={valueOf(value.properties, def)}
              onChange={(setting) => setProperty(def.key, setting)}
            />
          ))}
        </Section>
      )}

      {catalog.gameRules.length === 0 ? (
        <p className="text-sm text-muted-foreground">
          {t.gameSettings.unsupported}
        </p>
      ) : (
        ruleGroups.length > 0 && (
          <div className="space-y-3">
            <div className="space-y-1">
              <h3 className="font-semibold">{t.gameSettings.rules}</h3>
              <p className="text-sm text-muted-foreground">
                {t.gameSettings.rulesDescription}
              </p>
            </div>
            {ruleGroups.map(({ category, rules }) => (
              <Section
                key={category}
                title={t.gameSettings.categories[category]}
              >
                {rules.map((def) => (
                  <SettingField
                    key={def.key}
                    def={def}
                    value={valueOf(value.gameRules, def)}
                    onChange={(setting) => setRule(def.key, setting)}
                  />
                ))}
              </Section>
            ))}
          </div>
        )
      )}

      {properties.length === 0 && ruleGroups.length === 0 && (
        <p className="text-sm text-muted-foreground">
          {t.gameSettings.noResult}
        </p>
      )}
    </div>
  );
}

function Section({
  title,
  description,
  children,
}: {
  title: string;
  description?: string;
  children: React.ReactNode;
}) {
  return (
    <section className="rounded-xl px-4 pt-3 ring-1 ring-foreground/10">
      <h4 className="text-sm font-semibold">{title}</h4>
      {description && (
        <p className="text-xs text-muted-foreground">{description}</p>
      )}
      <div className="divide-y">{children}</div>
    </section>
  );
}

/** Lowercase without accents, so "regen" finds "Régénération". */
function normalize(text: string) {
  return text
    .normalize("NFD")
    .replace(/\p{Diacritic}/gu, "")
    .toLowerCase();
}
