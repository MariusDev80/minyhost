// Operators of a server (TanStack Query).
//
// When the server is running, Rust sends `op` / `deop` and the server writes
// `ops.json` a moment later: the list is refreshed when the console confirms
// it (see `useServerEvents`).
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";
import { t } from "@/i18n";
import { showError } from "@/lib/errors";
import { commands } from "@/lib/tauri";
import type { Operator } from "@/types";

export const operatorsKey = (serverId: string) =>
  ["operators", serverId] as const;

export function useOperators(serverId: string) {
  return useQuery({
    queryKey: operatorsKey(serverId),
    queryFn: () => commands.listOperators(serverId),
  });
}

/** Looks the pseudo up at Mojang (Rust side), then makes the player an operator. */
export function useAddOperator(serverId: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (name: string) => commands.addOperator(serverId, name),
    onSuccess: (operator) => {
      toast.success(t.operators.added(operator.name));
      return queryClient.invalidateQueries({
        queryKey: operatorsKey(serverId),
      });
    },
    onError: showError,
  });
}

export function useRemoveOperator(serverId: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (operator: Operator) =>
      commands.removeOperator(serverId, operator.uuid),
    onSuccess: (_, operator) => {
      toast.success(t.operators.removed(operator.name));
      return queryClient.invalidateQueries({
        queryKey: operatorsKey(serverId),
      });
    },
    onError: showError,
  });
}
