// Server data and actions (TanStack Query). Live status updates arrive
// through events and are written into the same cache by `useServerEvents`.
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";
import { t } from "@/i18n";
import { showError } from "@/lib/errors";
import { commands } from "@/lib/tauri";
import { useConsoleStore } from "@/stores/console";
import type { NewServer, ServerInfo } from "@/types";

export const serversKey = ["servers"] as const;

/** Every server with its live status, newest first. */
export function useServers() {
  return useQuery({ queryKey: serversKey, queryFn: commands.listServers });
}

/** One server, read from the same cached list (`undefined` if not found). */
export function useServer(id: string) {
  return useQuery({
    queryKey: serversKey,
    queryFn: commands.listServers,
    select: (servers: ServerInfo[]) => servers.find((s) => s.id === id),
  });
}

/** Long-running: follow its progress with `events.onCreateProgress`. */
export function useCreateServer() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (input: NewServer) => commands.createServer(input),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: serversKey }),
  });
}

export function useDeleteServer() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => commands.deleteServer(id),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: serversKey }),
    onError: showError,
  });
}

export function useStartServer() {
  return useMutation({
    mutationFn: (id: string) => commands.startServer(id),
    onError: showError,
  });
}

export function useStopServer() {
  return useMutation({
    mutationFn: (id: string) => commands.stopServer(id),
    onSuccess: (outcome) => {
      if (outcome === "killed") toast.warning(t.notifications.killed);
    },
    onError: showError,
  });
}

/** Stop then start, e.g. to apply new settings. */
export function useRestartServer() {
  return useMutation({
    mutationFn: (id: string) => commands.restartServer(id),
    onSuccess: (outcome) => {
      if (outcome === "killed") toast.warning(t.notifications.killed);
    },
    onError: showError,
  });
}

export function useSendCommand(id: string) {
  const append = useConsoleStore((state) => state.append);
  return useMutation({
    mutationFn: (command: string) => commands.sendCommand(id, command),
    // Echo the command in the console, like a terminal would.
    onSuccess: (_, command) => append(id, `> ${command}`, "input"),
    onError: showError,
  });
}
