import { create } from "zustand";
import { persist } from "zustand/middleware";

export type Theme = "dark" | "light" | "system";

// In-app navigation. A desktop app with a handful of screens does not need
// URLs, so a typed route in the UI store replaces a router dependency.
// To add a page: add a route here, then a `case` in `App.tsx`.
export type Route =
  | { name: "home" }
  | { name: "servers" }
  | { name: "create-server" }
  | { name: "server"; id: string }
  | { name: "settings" }
  | { name: "design-system" };

interface UiState {
  route: Route;
  theme: Theme;
  /** True once the user closed the window and servers are being stopped. */
  closing: boolean;
  navigate: (route: Route) => void;
  setTheme: (theme: Theme) => void;
  setClosing: (closing: boolean) => void;
}

export const useUiStore = create<UiState>()(
  persist(
    (set) => ({
      route: { name: "home" },
      theme: "dark",
      closing: false,
      navigate: (route) => set({ route }),
      setTheme: (theme) => set({ theme }),
      setClosing: (closing) => set({ closing }),
    }),
    {
      name: "minyhost-ui",
      // Only the theme is a preference; the route always restarts at home.
      partialize: (state) => ({ theme: state.theme }),
    },
  ),
);
