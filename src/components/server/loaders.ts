import { Pickaxe, Puzzle, Zap, type LucideIcon } from "lucide-react";
import { t } from "@/i18n";
import type { Loader } from "@/types";

/** Display info for each server type, in the order shown in the UI. */
export const loaders: {
  value: Loader;
  icon: LucideIcon;
  name: string;
  description: string;
}[] = [
  { value: "paper", icon: Zap, ...t.loaders.paper },
  { value: "vanilla", icon: Pickaxe, ...t.loaders.vanilla },
  { value: "fabric", icon: Puzzle, ...t.loaders.fabric },
];

export function loaderInfo(loader: Loader) {
  return loaders.find((l) => l.value === loader) ?? loaders[0];
}
