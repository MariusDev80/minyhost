// Internet access through playit.gg (TanStack Query). Live updates arrive
// through the `tunnel-state` event, written into the same cache by
// `useServerEvents`.
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";
import { t } from "@/i18n";
import { showError } from "@/lib/errors";
import { playitUrls } from "@/lib/playit";
import { commands, openExternal } from "@/lib/tauri";

export const tunnelKey = ["tunnel"] as const;

/** While an address is pending, ask again: with no server running, the agent
 * is stopped and nothing else refreshes the state. */
const PENDING_REFRESH_MS = 5000;

export function useTunnel() {
  return useQuery({
    queryKey: tunnelKey,
    queryFn: commands.getTunnelState,
    refetchInterval: (query) =>
      Object.values(query.state.data?.tunnels ?? {}).some((t) => !t.address)
        ? PENDING_REFRESH_MS
        : false,
  });
}

/** Starts linking and opens the playit.gg approval page in the browser. */
export function useLinkPlayit() {
  return useMutation({
    mutationFn: commands.linkPlayit,
    onSuccess: (url) => openExternal(url),
    onError: showError,
  });
}

export function useCancelPlayitLink() {
  return useMutation({
    mutationFn: commands.cancelPlayitLink,
    onError: showError,
  });
}

export function useUnlinkPlayit() {
  return useMutation({
    mutationFn: commands.unlinkPlayit,
    // The agent stays on the account: point the user to where it is deleted.
    onSuccess: () =>
      toast.success(t.tunnel.settings.unlinked, {
        action: {
          label: t.tunnel.settings.openAgents,
          onClick: () => openExternal(playitUrls.agents),
        },
      }),
    onError: showError,
  });
}

export function useRecheckPlayitAgent() {
  return useMutation({
    mutationFn: commands.recheckPlayitAgent,
    onError: showError,
  });
}

export function useSetInternetAccess(id: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (enabled: boolean) =>
      enabled
        ? commands.enableInternetAccess(id)
        : commands.disableInternetAccess(id),
    onSettled: () => queryClient.invalidateQueries({ queryKey: tunnelKey }),
    onError: showError,
  });
}
