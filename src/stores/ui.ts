import { create } from "zustand";
import { persist } from "zustand/middleware";
import type { Locale } from "@/i18n";

export type Theme = "dark" | "light" | "system";
/** "system" follows the OS language. */
export type Language = Locale | "system";

// In-app navigation. A desktop app with a handful of screens does not need
// URLs, so a typed route in the UI store replaces a router dependency.
// To add a page: add a route here, then a `case` in `App.tsx`.
export type Route =
  | { name: "home" }
  | { name: "servers" }
  | { name: "create-server" }
  | { name: "server"; id: string }
  | { name: "settings" }
  | { name: "help" }
  | { name: "design-system" };

interface UiState {
  route: Route;
  theme: Theme;
  language: Language;
  /** True once the user closed the window and servers are being stopped. */
  closing: boolean;
  navigate: (route: Route) => void;
  setTheme: (theme: Theme) => void;
  setLanguage: (language: Language) => void;
  setClosing: (closing: boolean) => void;
}

export const useUiStore = create<UiState>()(
  persist(
    (set) => ({
      route: { name: "home" },
      theme: "dark",
      language: "system",
      closing: false,
      navigate: (route) => set({ route }),
      setTheme: (theme) => set({ theme }),
      setLanguage: (language) => set({ language }),
      setClosing: (closing) => set({ closing }),
    }),
    {
      name: "minyhost-ui",
      // Only theme and language are preferences; the route always restarts at home.
      partialize: (state) => ({
        theme: state.theme,
        language: state.language,
      }),
    },
  ),
);
