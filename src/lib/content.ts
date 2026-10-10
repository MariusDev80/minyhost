// Display helpers for mods, plugins and datapacks.
import type { ContentKind, InstalledContent, Loader } from "@/types";

/** What a server type can receive. Same rule as Rust (`core/content/mod.rs`, `kinds_for`). */
export function contentKinds(loader: Loader): ContentKind[] {
  switch (loader) {
    case "vanilla":
      return ["datapack"];
    case "paper":
      return ["plugin", "datapack"];
    case "fabric":
    case "neoforge":
    case "forge":
      return ["mod", "datapack"];
  }
}

/** Name shown for an installed file: its project title, else the file name. */
export function contentTitle(item: InstalledContent): string {
  return item.meta?.title ?? item.fileName;
}
