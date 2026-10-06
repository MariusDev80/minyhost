// Whitelist of a server, and player skins (TanStack Query).
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";
import { t } from "@/i18n";
import { showError } from "@/lib/errors";
import { loadSkin } from "@/lib/skin";
import { commands } from "@/lib/tauri";
import type { WhitelistEntry } from "@/types";

export const whitelistKey = (serverId: string) =>
  ["whitelist", serverId] as const;

export function useWhitelist(serverId: string) {
  return useQuery({
    queryKey: whitelistKey(serverId),
    queryFn: () => commands.listWhitelist(serverId),
  });
}

/** Looks the pseudo up at Mojang (Rust side), then adds the player. */
export function useAddToWhitelist(serverId: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (name: string) => commands.addToWhitelist(serverId, name),
    onSuccess: (entry) => {
      toast.success(t.whitelist.added(entry.name));
      return queryClient.invalidateQueries({
        queryKey: whitelistKey(serverId),
      });
    },
    onError: showError,
  });
}

export function useRemoveFromWhitelist(serverId: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (entry: WhitelistEntry) =>
      commands.removeFromWhitelist(serverId, entry.uuid),
    onSuccess: (_, entry) => {
      toast.success(t.whitelist.removed(entry.name));
      return queryClient.invalidateQueries({
        queryKey: whitelistKey(serverId),
      });
    },
    onError: showError,
  });
}

/** Player skin, ready to draw. Skins rarely change: fetched once per app session. */
export function usePlayerSkin(uuid: string) {
  return useQuery({
    queryKey: ["skin", uuid],
    queryFn: async () => loadSkin(await commands.playerSkin(uuid)),
    staleTime: Infinity,
    retry: 1,
  });
}
