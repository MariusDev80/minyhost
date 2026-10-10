import { ExternalLink, Info, Loader2, Plus, X } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { Switch } from "@/components/ui/switch";
import {
  useInstalledContent,
  useRemoveContent,
  useSetContentEnabled,
} from "@/hooks/useContent";
import { t } from "@/i18n";
import { contentKinds, contentTitle } from "@/lib/content";
import { openExternal } from "@/lib/tauri";
import { cn } from "@/lib/utils";
import { useUiStore } from "@/stores/ui";
import type { InstalledContent, ServerInfo } from "@/types";
import { ContentIcon } from "./ContentIcon";

/** Mods / plugins / datapacks of a server, grouped by kind. */
export function ContentPanel({ server }: { server: ServerInfo }) {
  const navigate = useUiStore((state) => state.navigate);
  const content = useInstalledContent(server.id);
  const stopped = server.status === "stopped";
  const kinds = contentKinds(server.loader);

  return (
    <div className="space-y-4 rounded-xl p-4 ring-1 ring-foreground/10">
      <div className="flex items-start justify-between gap-4">
        <div className="space-y-1">
          <h3 className="font-semibold">{t.content.title[server.loader]}</h3>
          <p className="text-sm text-muted-foreground">
            {t.content.description[server.loader]}
          </p>
        </div>
        <Button
          onClick={() => navigate({ name: "add-content", id: server.id })}
        >
          <Plus data-icon="inline-start" />
          {t.content.add}
        </Button>
      </div>

      {!stopped && (
        <p className="flex items-center gap-2 text-sm text-muted-foreground">
          <Info className="size-4 shrink-0" />
          {t.content.stopFirst}
        </p>
      )}

      {content.isPending ? (
        <Skeleton className="h-14 rounded-lg" />
      ) : content.isError ? null : content.data.length === 0 ? (
        <p className="rounded-lg border border-dashed p-4 text-center text-sm text-muted-foreground">
          {t.content.empty}
        </p>
      ) : (
        kinds.map((kind) => {
          const items = content.data.filter((item) => item.kind === kind);
          if (items.length === 0) return null;
          return (
            <section key={kind} className="space-y-2">
              <h4 className="text-sm font-medium text-muted-foreground">
                {t.content.kinds[kind]}{" "}
                <span className="text-xs">{items.length}</span>
              </h4>
              <ul className="space-y-2">
                {items.map((item) => (
                  <ContentRow
                    key={`${item.kind}/${item.fileName}`}
                    serverId={server.id}
                    item={item}
                    stopped={stopped}
                  />
                ))}
              </ul>
            </section>
          );
        })
      )}
    </div>
  );
}

function ContentRow({
  serverId,
  item,
  stopped,
}: {
  serverId: string;
  item: InstalledContent;
  stopped: boolean;
}) {
  const remove = useRemoveContent(serverId);
  const toggle = useSetContentEnabled(serverId);
  const title = contentTitle(item);
  // Datapacks cannot be disabled (see `core/content/mod.rs`).
  const canToggle = item.kind !== "datapack";

  return (
    <li className="flex items-center gap-3 rounded-lg bg-muted/50 p-2">
      <ContentIcon
        url={item.meta?.iconUrl}
        className={cn(!item.enabled && "opacity-50")}
      />
      <div className={cn("min-w-0 flex-1", !item.enabled && "opacity-60")}>
        <p className="flex items-center gap-2">
          <span className="truncate font-medium">{title}</span>
          {item.meta?.dependency && (
            <Badge variant="secondary">{t.content.dependency}</Badge>
          )}
          {!item.meta && <Badge variant="outline">{t.content.manual}</Badge>}
        </p>
        <p data-selectable className="truncate text-xs text-muted-foreground">
          {item.meta?.versionNumber ?? item.fileName}
        </p>
      </div>
      {item.meta && (
        <Button
          variant="ghost"
          size="icon-sm"
          aria-label={t.content.openPage}
          title={t.content.openPage}
          onClick={() => item.meta && void openExternal(item.meta.pageUrl)}
        >
          <ExternalLink />
        </Button>
      )}
      {canToggle && (
        <Switch
          checked={item.enabled}
          disabled={!stopped || toggle.isPending}
          aria-label={t.content.enable(title)}
          title={t.content.enable(title)}
          onCheckedChange={(enabled) => toggle.mutate({ item, enabled })}
        />
      )}
      <Button
        variant="ghost"
        size="icon-sm"
        disabled={!stopped || remove.isPending}
        aria-label={t.content.remove(title)}
        title={t.content.remove(title)}
        onClick={() => remove.mutate(item)}
      >
        {remove.isPending ? <Loader2 className="animate-spin" /> : <X />}
      </Button>
    </li>
  );
}
