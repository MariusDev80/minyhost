import { useEffect } from "react";
import { setLocale, systemLocale, type Locale } from "@/i18n";
import { useUiStore } from "@/stores/ui";

/**
 * Switches `t` to the locale chosen in the settings and keeps `<html lang>`
 * in sync. The switch happens during render (it is idempotent) so the
 * children, rendered right after, already read the new texts.
 */
export function useApplyLocale(): Locale {
  const language = useUiStore((state) => state.language);
  const locale = language === "system" ? systemLocale() : language;
  setLocale(locale);
  useEffect(() => {
    document.documentElement.lang = locale;
  }, [locale]);
  return locale;
}
