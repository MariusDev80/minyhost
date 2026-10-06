import { useEffect, useSyncExternalStore } from "react";
import { useUiStore } from "@/stores/ui";

const darkQuery = "(prefers-color-scheme: dark)";

function subscribeToSystemTheme(onChange: () => void) {
  const media = window.matchMedia(darkQuery);
  media.addEventListener("change", onChange);
  return () => media.removeEventListener("change", onChange);
}

function systemPrefersDark() {
  return window.matchMedia(darkQuery).matches;
}

/** Resolves the user's theme preference to the theme actually displayed. */
export function useResolvedTheme(): "dark" | "light" {
  const theme = useUiStore((state) => state.theme);
  const systemDark = useSyncExternalStore(
    subscribeToSystemTheme,
    systemPrefersDark,
  );
  if (theme === "system") return systemDark ? "dark" : "light";
  return theme;
}

/** Keeps the `dark` class on <html> in sync with the resolved theme. */
export function useApplyTheme() {
  const resolved = useResolvedTheme();
  useEffect(() => {
    document.documentElement.classList.toggle("dark", resolved === "dark");
  }, [resolved]);
  return resolved;
}
