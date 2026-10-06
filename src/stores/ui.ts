import { create } from "zustand";
import { persist } from "zustand/middleware";

export type Theme = "dark" | "light" | "system";

// In-app navigation. A desktop app with a handful of screens does not need
// URLs, so a typed route in the UI store replaces a router dependency.
export type Route =
  | { name: "home" }
  | { name: "servers" }
  | { name: "settings" }
  | { name: "design-system" };

interface UiState {
  route: Route;
  theme: Theme;
  navigate: (route: Route) => void;
  setTheme: (theme: Theme) => void;
}

export const useUiStore = create<UiState>()(
  persist(
    (set) => ({
      route: { name: "home" },
      theme: "dark",
      navigate: (route) => set({ route }),
      setTheme: (theme) => set({ theme }),
    }),
    {
      name: "minyhost-ui",
      // Only the theme is a preference; the route always restarts at home.
      partialize: (state) => ({ theme: state.theme }),
    },
  ),
);
