import { useQuery } from "@tanstack/react-query";
import { commands } from "@/lib/tauri";
import type { Loader } from "@/types";

/** Minecraft versions available for a server type, newest first. */
export function useVersions(loader: Loader) {
  return useQuery({
    queryKey: ["versions", loader],
    queryFn: () => commands.listVersions(loader),
    // Version lists change a few times a month: no need to refetch often.
    staleTime: 60 * 60 * 1000,
  });
}
