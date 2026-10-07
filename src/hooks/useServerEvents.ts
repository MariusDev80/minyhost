// Listens to the events sent by Rust and dispatches them to the right place.
// Mounted once, in `App.tsx`.
import { useEffect } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";
import { t } from "@/i18n";
import { events } from "@/lib/tauri";
import { useConsoleStore } from "@/stores/console";
import { useUiStore } from "@/stores/ui";
import type { ServerInfo, TunnelState } from "@/types";
import { gameSettingsKey } from "./useGameSettings";
import { operatorsKey } from "./useOperators";
import { serversKey } from "./useServers";
import { tunnelKey } from "./useTunnel";
import { whitelistKey } from "./useWhitelist";

export function useServerEvents() {
  const queryClient = useQueryClient();
  const appendLine = useConsoleStore((state) => state.append);
  const clearConsole = useConsoleStore((state) => state.clear);
  const setClosing = useUiStore((state) => state.setClosing);

  // Status change -> update the cached server list.
  useEffect(
    () =>
      events.onServerStatus(({ id, status }) => {
        queryClient.setQueryData<ServerInfo[]>(serversKey, (servers) =>
          servers?.map((s) =>
            s.id === id
              ? {
                  ...s,
                  status,
                  // A fresh process runs with the latest settings.
                  needsRestart:
                    status === "starting" || status === "stopped"
                      ? false
                      : s.needsRestart,
                }
              : s,
          ),
        );
        // Each start begins with a fresh console.
        if (status === "starting") clearConsole(id);
      }),
    [queryClient, clearConsole],
  );

  // Console output -> console store.
  useEffect(
    () =>
      events.onConsoleLine(({ id, line, stream }) => {
        appendLine(id, line, stream);
        // e.g. "Added Steve to the whitelist", typed in the console.
        if (/whitelist/i.test(line))
          void queryClient.invalidateQueries({ queryKey: whitelistKey(id) });
        // e.g. "Made Steve a server operator" (after `op` / `deop`).
        if (/server operator/i.test(line))
          void queryClient.invalidateQueries({ queryKey: operatorsKey(id) });
        // e.g. "Game rule keep_inventory is now set to true", from anywhere.
        if (/game ?rule .* set to/i.test(line))
          void queryClient.invalidateQueries({ queryKey: gameSettingsKey(id) });
      }),
    [appendLine, queryClient],
  );

  // Crash -> notification.
  useEffect(
    () =>
      events.onServerCrashed(({ id }) => {
        const servers = queryClient.getQueryData<ServerInfo[]>(serversKey);
        const name = servers?.find((s) => s.id === id)?.name ?? id;
        toast.error(t.notifications.crashed(name), {
          description: t.notifications.crashedHint,
        });
      }),
    [queryClient],
  );

  // Internet access (playit.gg) -> cached state, plus a toast when linking ends.
  useEffect(
    () =>
      events.onTunnelState((state) => {
        const previous = queryClient.getQueryData<TunnelState>(tunnelKey);
        queryClient.setQueryData(tunnelKey, state);
        if (previous?.link.state === "linking") {
          if (state.link.state === "linked") toast.success(t.tunnel.linked);
          else if (state.linkError)
            toast.error(t.tunnel.linkErrors[state.linkError]);
        } else if (state.linkError === "revoked" && !previous?.linkError) {
          toast.error(t.tunnel.linkErrors.revoked);
        }
      }),
    [queryClient],
  );

  // Window closing -> "stopping servers" overlay.
  useEffect(() => events.onAppClosing(() => setClosing(true)), [setClosing]);
}
