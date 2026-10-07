import { useQuery } from "@tanstack/react-query";
import { FolderOpen, Monitor, Moon, Sun, type LucideIcon } from "lucide-react";
import { PageHeader } from "@/components/layout/PageHeader";
import { PlayitSettingsCard } from "@/components/tunnel/PlayitAccount";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { showError } from "@/lib/errors";
import { commands } from "@/lib/tauri";
import { cn } from "@/lib/utils";
import { localeNames, locales, systemLocale, t } from "@/i18n";
import { useUiStore, type Language, type Theme } from "@/stores/ui";

const themeIcons: { value: Theme; icon: LucideIcon }[] = [
  { value: "dark", icon: Moon },
  { value: "light", icon: Sun },
  { value: "system", icon: Monitor },
];

export function SettingsPage() {
  return (
    <div className="space-y-8">
      <PageHeader
        title={t.settings.title}
        description={t.settings.description}
      />
      <Card>
        <CardHeader>
          <CardTitle>{t.settings.appearance}</CardTitle>
        </CardHeader>
        <CardContent className="space-y-6">
          <ThemeSetting />
          <LanguageSetting />
        </CardContent>
      </Card>
      <PlayitSettingsCard />
      <FilesCard />
    </div>
  );
}

function SettingHeading({
  id,
  title,
  description,
}: {
  id?: string;
  title: string;
  description: string;
}) {
  return (
    <div className="space-y-1">
      <p id={id} className="text-sm font-medium">
        {title}
      </p>
      <p className="text-sm text-muted-foreground">{description}</p>
    </div>
  );
}

function ThemeSetting() {
  const theme = useUiStore((state) => state.theme);
  const setTheme = useUiStore((state) => state.setTheme);

  return (
    <div className="space-y-3">
      <SettingHeading
        title={t.settings.theme}
        description={t.settings.themeDescription}
      />
      <div
        role="radiogroup"
        aria-label={t.settings.theme}
        className="grid max-w-md grid-cols-3 gap-2"
      >
        {themeIcons.map(({ value, icon: Icon }) => (
          <button
            key={value}
            type="button"
            role="radio"
            aria-checked={theme === value}
            onClick={() => setTheme(value)}
            className={cn(
              "flex flex-col items-center gap-2 rounded-lg border p-3 text-sm font-medium transition-colors outline-none hover:bg-muted focus-visible:ring-3 focus-visible:ring-ring/50",
              theme === value &&
                "border-primary bg-accent text-accent-foreground hover:bg-accent [&_svg]:text-primary",
            )}
          >
            <Icon className="size-5" />
            {t.settings.themes[value]}
          </button>
        ))}
      </div>
    </div>
  );
}

function LanguageSetting() {
  const language = useUiStore((state) => state.language);
  const setLanguage = useUiStore((state) => state.setLanguage);

  return (
    <div className="space-y-3">
      <SettingHeading
        id="setting-language"
        title={t.settings.language}
        description={t.settings.languageDescription}
      />
      <Select
        value={language}
        onValueChange={(value) => setLanguage(value as Language)}
      >
        <SelectTrigger aria-labelledby="setting-language" className="w-56">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="system">
            {t.settings.languageSystem(localeNames[systemLocale()])}
          </SelectItem>
          {locales.map((locale) => (
            <SelectItem key={locale} value={locale} lang={locale}>
              {localeNames[locale]}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </div>
  );
}

function FilesCard() {
  const path = useQuery({
    queryKey: ["servers-folder-path"],
    queryFn: commands.serversFolderPath,
    staleTime: Infinity,
  });

  return (
    <Card>
      <CardHeader>
        <CardTitle>{t.settings.files}</CardTitle>
      </CardHeader>
      <CardContent className="space-y-3">
        <p className="text-sm text-muted-foreground">
          {t.settings.filesDescription}
        </p>
        {path.data && (
          <p
            data-selectable
            className="truncate rounded-lg border bg-muted/50 px-3 py-2 font-mono text-sm"
            title={path.data}
          >
            {path.data}
          </p>
        )}
        <Button
          variant="outline"
          onClick={() => commands.openServersFolder().catch(showError)}
        >
          <FolderOpen data-icon="inline-start" />
          {t.settings.openServersFolder}
        </Button>
      </CardContent>
    </Card>
  );
}
