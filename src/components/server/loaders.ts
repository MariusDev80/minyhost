import { Pickaxe, Puzzle, Zap, type LucideIcon } from "lucide-react";
import { t } from "@/i18n";
import type { Loader } from "@/types";

const icons: Record<Loader, LucideIcon> = {
  paper: Zap,
  vanilla: Pickaxe,
  fabric: Puzzle,
};

/** Order in which server types are shown in the UI. */
const order: Loader[] = ["paper", "vanilla", "fabric"];

/** Display info for a server type. Texts are read at call time (current locale). */
export function loaderInfo(loader: Loader): {
  value: Loader;
  icon: LucideIcon;
  name: string;
  description: string;
} {
  const value = loader in icons ? loader : order[0];
  return { value, icon: icons[value], ...t.loaders[value] };
}

/** Every server type, in display order. */
export function loaderList() {
  return order.map(loaderInfo);
}
