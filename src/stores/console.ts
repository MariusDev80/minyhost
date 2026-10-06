import { create } from "zustand";

/** Lines kept per server; older lines are dropped (CLAUDE.md 5.4). */
const MAX_LINES = 2000;

export interface ConsoleEntry {
  /** Unique key for React lists. */
  key: number;
  text: string;
  /** `input` = a command typed by the user. */
  kind: "stdout" | "stderr" | "input";
}

interface ConsoleState {
  /** Console lines, by server id. */
  lines: Record<string, ConsoleEntry[]>;
  append: (id: string, text: string, kind: ConsoleEntry["kind"]) => void;
  clear: (id: string) => void;
}

let nextKey = 0;

export const useConsoleStore = create<ConsoleState>()((set) => ({
  lines: {},
  append: (id, text, kind) =>
    set((state) => {
      const previous = state.lines[id] ?? [];
      const entry: ConsoleEntry = { key: nextKey++, text, kind };
      return {
        lines: {
          ...state.lines,
          [id]: [...previous.slice(-(MAX_LINES - 1)), entry],
        },
      };
    }),
  clear: (id) => set((state) => ({ lines: { ...state.lines, [id]: [] } })),
}));
