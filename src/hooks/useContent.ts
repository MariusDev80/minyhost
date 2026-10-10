// Mods, plugins and datapacks (TanStack Query). Rust does the searching,
// downloading and file handling (`core/content/`).
import {
  useInfiniteQuery,
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";
import { toast } from "sonner";
import { t } from "@/i18n";
import { showError } from "@/lib/errors";
import { commands } from "@/lib/tauri";
import type {
  ContentHit,
  ContentKind,
  ContentSource,
  InstalledContent,
  InstallOutcome,
} from "@/types";

export const contentKey = (serverId: string) => ["content", serverId] as const;

/** Sources offered by this build (CurseForge needs an API key). */
export function useContentSources() {
  return useQuery({
    queryKey: ["content-sources"],
    queryFn: commands.contentSources,
    staleTime: Infinity,
  });
}

/** Files found in the server's mods / plugins / datapacks folders. */
export function useInstalledContent(serverId: string) {
  return useQuery({
    queryKey: contentKey(serverId),
    queryFn: () => commands.listContent(serverId),
  });
}

/**
 * Search pages, loaded one after the other ("load more").
 * `serverId: null` searches modpacks.
 */
export function useContentSearch(params: {
  source: ContentSource;
  kind: ContentKind;
  query: string;
  serverId: string | null;
}) {
  return useInfiniteQuery({
    queryKey: ["content-search", params],
    queryFn: ({ pageParam }) =>
      commands.searchContent({ ...params, offset: pageParam }),
    initialPageParam: 0,
    getNextPageParam: (lastPage) => lastPage.nextOffset ?? undefined,
    // Results change slowly; keep them while browsing back and forth.
    staleTime: 10 * 60 * 1000,
  });
}

/** Versions of a modpack, newest first. */
export function useModpackVersions(source: ContentSource, projectId: string) {
  return useQuery({
    queryKey: ["modpack-versions", source, projectId],
    queryFn: () => commands.listModpackVersions(source, projectId),
    staleTime: 10 * 60 * 1000,
  });
}

/** Project icon, fetched by Rust once per app session. */
export function useContentIcon(url: string | null | undefined) {
  return useQuery({
    queryKey: ["content-icon", url],
    queryFn: () => commands.contentIcon(url ?? ""),
    enabled: Boolean(url),
    staleTime: Infinity,
    retry: false,
  });
}

/**
 * Installs a project and its dependencies. The caller shows what needs a
 * manual download (`InstallOutcome.manual`).
 */
export function useInstallContent(serverId: string, kind: ContentKind) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (hit: ContentHit) =>
      commands.installContent(serverId, hit.source, kind, hit.projectId),
    onSuccess: (outcome) => {
      notifyInstalled(outcome);
      return queryClient.invalidateQueries({ queryKey: contentKey(serverId) });
    },
    onError: showError,
  });
}

function notifyInstalled(outcome: InstallOutcome) {
  const [first, ...dependencies] = outcome.installed;
  if (first) toast.success(t.content.installed(first, dependencies.length));
  if (outcome.unavailable.length > 0)
    toast.warning(t.content.unavailable(outcome.unavailable.join(", ")));
}

export function useRemoveContent(serverId: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (item: InstalledContent) =>
      commands.removeContent(serverId, item.kind, item.fileName),
    onSuccess: () =>
      queryClient.invalidateQueries({ queryKey: contentKey(serverId) }),
    onError: showError,
  });
}

export function useSetContentEnabled(serverId: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({
      item,
      enabled,
    }: {
      item: InstalledContent;
      enabled: boolean;
    }) =>
      commands.setContentEnabled(serverId, item.kind, item.fileName, enabled),
    onSuccess: () =>
      queryClient.invalidateQueries({ queryKey: contentKey(serverId) }),
    onError: showError,
  });
}
