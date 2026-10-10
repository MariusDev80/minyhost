import { useEffect, useState } from "react";
import { Download, ExternalLink, Search, SearchX } from "lucide-react";
import { EmptyState } from "@/components/layout/EmptyState";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Skeleton } from "@/components/ui/skeleton";
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useContentSearch, useContentSources } from "@/hooks/useContent";
import { t } from "@/i18n";
import { openExternal } from "@/lib/tauri";
import type { ContentHit, ContentKind, ContentSource } from "@/types";
import { ContentIcon } from "./ContentIcon";

const SEARCH_DELAY_MS = 300;

/**
 * Search Modrinth (and CurseForge when this build has a key) for one kind of
 * content. `serverId` keeps only what is compatible with that server;
 * `null` searches modpacks. `action` renders the button of each result.
 */
export function ContentBrowser({
  kinds,
  serverId,
  action,
}: {
  kinds: ContentKind[];
  serverId: string | null;
  action: (hit: ContentHit, kind: ContentKind) => React.ReactNode;
}) {
  const sources = useContentSources();
  const [kind, setKind] = useState(kinds[0]);
  const [source, setSource] = useState<ContentSource>("modrinth");
  const [input, setInput] = useState("");
  const query = useDebounced(input.trim(), SEARCH_DELAY_MS);
  const search = useContentSearch({ source, kind, query, serverId });
  const hits = search.data?.pages.flatMap((page) => page.hits) ?? [];

  const availableSources: ContentSource[] = sources.data?.curseforge
    ? ["modrinth", "curseforge"]
    : ["modrinth"];

  return (
    <div className="space-y-4">
      <div className="flex flex-wrap items-center gap-3">
        {kinds.length > 1 && (
          <Tabs
            value={kind}
            onValueChange={(value) => setKind(value as ContentKind)}
          >
            <TabsList>
              {kinds.map((value) => (
                <TabsTrigger key={value} value={value}>
                  {t.content.kinds[value]}
                </TabsTrigger>
              ))}
            </TabsList>
          </Tabs>
        )}
        {availableSources.length > 1 && (
          <Tabs
            value={source}
            onValueChange={(value) => setSource(value as ContentSource)}
          >
            <TabsList>
              {availableSources.map((value) => (
                <TabsTrigger key={value} value={value}>
                  {t.content.sources[value]}
                </TabsTrigger>
              ))}
            </TabsList>
          </Tabs>
        )}
        <div className="relative min-w-48 flex-1">
          <Search className="pointer-events-none absolute top-1/2 left-2.5 size-4 -translate-y-1/2 text-muted-foreground" />
          <Input
            value={input}
            onChange={(e) => setInput(e.target.value)}
            placeholder={t.content.searchPlaceholder(t.content.kinds[kind])}
            aria-label={t.content.searchPlaceholder(t.content.kinds[kind])}
            className="pl-8"
            autoFocus
          />
        </div>
      </div>

      {search.isPending ? (
        <div className="space-y-2">
          {Array.from({ length: 4 }, (_, index) => (
            <Skeleton key={index} className="h-20 rounded-xl" />
          ))}
        </div>
      ) : search.isError ? (
        <EmptyState
          icon={SearchX}
          title={t.content.searchError}
          action={
            <Button variant="outline" onClick={() => void search.refetch()}>
              {t.actions.retry}
            </Button>
          }
        />
      ) : hits.length === 0 ? (
        <EmptyState icon={SearchX} title={t.content.noResults} />
      ) : (
        <>
          <ul className="space-y-2">
            {hits.map((hit) => (
              <ResultRow
                key={`${hit.source}/${hit.projectId}`}
                hit={hit}
                action={action(hit, kind)}
              />
            ))}
          </ul>
          {search.hasNextPage && (
            <div className="flex justify-center">
              <Button
                variant="outline"
                disabled={search.isFetchingNextPage}
                onClick={() => void search.fetchNextPage()}
              >
                {t.content.loadMore}
              </Button>
            </div>
          )}
        </>
      )}
    </div>
  );
}

function ResultRow({
  hit,
  action,
}: {
  hit: ContentHit;
  action: React.ReactNode;
}) {
  return (
    <li className="flex items-center gap-3 rounded-xl p-3 ring-1 ring-foreground/10 transition-colors hover:bg-muted/30">
      <ContentIcon url={hit.iconUrl} className="size-12" />
      <div className="min-w-0 flex-1 space-y-0.5">
        <p className="truncate">
          <span className="font-medium">{hit.title}</span>
          {hit.author && (
            <span className="text-sm text-muted-foreground">
              {" "}
              {t.content.by(hit.author)}
            </span>
          )}
        </p>
        <p className="line-clamp-2 text-sm text-muted-foreground">
          {hit.description}
        </p>
        <p className="flex items-center gap-1 text-xs text-muted-foreground">
          <Download className="size-3" />
          {t.units.downloads(hit.downloads)}
        </p>
      </div>
      <div className="flex shrink-0 items-center gap-1">
        <Button
          variant="ghost"
          size="icon-sm"
          aria-label={t.content.openPage}
          title={t.content.openPage}
          onClick={() => void openExternal(hit.pageUrl)}
        >
          <ExternalLink />
        </Button>
        {action}
      </div>
    </li>
  );
}

/** `value`, once it has not changed for `delayMs`. */
function useDebounced<T>(value: T, delayMs: number): T {
  const [debounced, setDebounced] = useState(value);
  useEffect(() => {
    const timer = setTimeout(() => setDebounced(value), delayMs);
    return () => clearTimeout(timer);
  }, [value, delayMs]);
  return debounced;
}
