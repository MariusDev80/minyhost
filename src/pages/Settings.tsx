import { Monitor, Moon, Sun, type LucideIcon } from "lucide-react";
import { PageHeader } from "@/components/layout/PageHeader";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { cn } from "@/lib/utils";
import { t } from "@/i18n";
import { useUiStore, type Theme } from "@/stores/ui";

const themes: { value: Theme; label: string; icon: LucideIcon }[] = [
  { value: "dark", label: t.settings.themes.dark, icon: Moon },
  { value: "light", label: t.settings.themes.light, icon: Sun },
  { value: "system", label: t.settings.themes.system, icon: Monitor },
];

export function SettingsPage() {
  const theme = useUiStore((state) => state.theme);
  const setTheme = useUiStore((state) => state.setTheme);

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
        <CardContent className="space-y-3">
          <div className="space-y-1">
            <p className="text-sm font-medium">{t.settings.theme}</p>
            <p className="text-sm text-muted-foreground">
              {t.settings.themeDescription}
            </p>
          </div>
          <div
            role="radiogroup"
            aria-label={t.settings.theme}
            className="grid max-w-md grid-cols-3 gap-2"
          >
            {themes.map(({ value, label, icon: Icon }) => (
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
                {label}
              </button>
            ))}
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
