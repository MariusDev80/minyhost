import { useQuery } from "@tanstack/react-query";
import { loadSkin } from "@/lib/skin";
import { commands } from "@/lib/tauri";

/** Player skin, ready to draw. Skins rarely change: fetched once per app session. */
export function usePlayerSkin(uuid: string) {
  return useQuery({
    queryKey: ["skin", uuid],
    queryFn: async () => loadSkin(await commands.playerSkin(uuid)),
    staleTime: Infinity,
    retry: 1,
  });
}
