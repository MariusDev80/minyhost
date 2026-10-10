import {
  Anvil,
  Flame,
  Pickaxe,
  Puzzle,
  Zap,
  type LucideIcon,
} from "lucide-react";
import { t } from "@/i18n";
import type { Loader } from "@/types";

const icons: Record<Loader, LucideIcon> = {
  paper: Zap,
  vanilla: Pickaxe,
  fabric: Puzzle,
  neoforge: Flame,
  forge: Anvil,
};

/** Order in which server types are shown in the UI. */
const order: Loader[] = ["paper", "vanilla", "fabric", "neoforge", "forge"];

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

/** Server types whose creation runs an installer (extra progress step). */
export function usesInstaller(loader: Loader) {
  return loader === "neoforge" || loader === "forge";
}

/** Every server type, in display order. */
export function loaderList() {
  return order.map(loaderInfo);
}
