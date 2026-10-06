// Listens to the events sent by Rust and dispatches them to the right place.
// Mounted once, in `App.tsx`.
import { useEffect } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";
import { t } from "@/i18n";
import { events } from "@/lib/tauri";
import { useConsoleStore } from "@/stores/console";
import { useUiStore } from "@/stores/ui";
import type { ServerInfo } from "@/types";
import { serversKey } from "./useServers";
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
          servers?.map((s) => (s.id === id ? { ...s, status } : s)),
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

  // Window closing -> "stopping servers" overlay.
  useEffect(() => events.onAppClosing(() => setClosing(true)), [setClosing]);
}
