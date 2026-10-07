// Game settings: catalog per Minecraft version, and a server's values (TanStack Query).
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";
import { t } from "@/i18n";
import { showError } from "@/lib/errors";
import { commands } from "@/lib/tauri";
import type { GameSettings } from "@/types";
import { serversKey } from "./useServers";

/** Settings that exist in this Minecraft version. Never changes: cached forever. */
export function useSettingsCatalog(mcVersion: string | undefined) {
  return useQuery({
    queryKey: ["settingsCatalog", mcVersion],
    queryFn: () => commands.gameSettingsCatalog(mcVersion ?? ""),
    enabled: mcVersion !== undefined,
    staleTime: Infinity,
  });
}

export const gameSettingsKey = (serverId: string) =>
  ["gameSettings", serverId] as const;

export function useGameSettings(serverId: string) {
  return useQuery({
    queryKey: gameSettingsKey(serverId),
    queryFn: () => commands.getGameSettings(serverId),
  });
}

/**
 * Saves only the changed settings. Game rules apply right away if the server
 * runs; world settings need a restart (Rust then flags the server).
 */
export function useSaveGameSettings(serverId: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (changes: GameSettings) =>
      commands.saveGameSettings(serverId, changes),
    onSuccess: async (outcome) => {
      toast.success(
        outcome.needsRestart
          ? t.gameSettings.savedNeedsRestart
          : t.gameSettings.saved,
      );
      await Promise.all([
        queryClient.invalidateQueries({ queryKey: gameSettingsKey(serverId) }),
        queryClient.invalidateQueries({ queryKey: serversKey }),
      ]);
    },
    onError: showError,
  });
}
