import { useState } from "react";
import { ArrowLeft, Check, Info, Loader2, ServerOff } from "lucide-react";
import { ContentBrowser } from "@/components/content/ContentBrowser";
import { ManualDownloadsDialog } from "@/components/content/ManualDownloadsDialog";
import { EmptyState } from "@/components/layout/EmptyState";
import { PageHeader } from "@/components/layout/PageHeader";
import { loaderInfo } from "@/components/server/loaders";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { useInstallContent, useInstalledContent } from "@/hooks/useContent";
import { useServer } from "@/hooks/useServers";
import { t } from "@/i18n";
import { contentKinds } from "@/lib/content";
import { useUiStore } from "@/stores/ui";
import type {
  ContentHit,
  ContentKind,
  ManualDownload,
  ServerInfo,
} from "@/types";

/** Search and install mods, plugins or datapacks for one server. */
export function AddContentPage({ id }: { id: string }) {
  const navigate = useUiStore((state) => state.navigate);
  const { data: server, isPending } = useServer(id);

  return (
    <div className="space-y-6">
      <Button
        variant="ghost"
        size="sm"
        onClick={() => navigate({ name: "server", id })}
      >
        <ArrowLeft data-icon="inline-start" />
        {server?.name ?? t.server.back}
      </Button>

      {isPending ? (
        <Skeleton className="h-64 rounded-xl" />
      ) : server ? (
        <AddContent server={server} />
      ) : (
        <EmptyState icon={ServerOff} title={t.server.notFound} />
      )}
    </div>
  );
}

function AddContent({ server }: { server: ServerInfo }) {
  const [manual, setManual] = useState<{
    folder: string;
    downloads: ManualDownload[];
  }>({ folder: "", downloads: [] });
  const stopped = server.status === "stopped";
  const loader = loaderInfo(server.loader);

  return (
    <>
      <PageHeader
        title={t.content.addTitle}
        description={t.content.compatibleHint(
          `${loader.name} ${server.mcVersion}`,
        )}
      />

      {!stopped && (
        <p className="flex items-center gap-2 rounded-lg bg-muted/50 p-3 text-sm text-muted-foreground">
          <Info className="size-4 shrink-0" />
          {t.content.stopFirst}
        </p>
      )}

      <ContentBrowser
        kinds={contentKinds(server.loader)}
        serverId={server.id}
        action={(hit, kind) => (
          <InstallButton
            server={server}
            hit={hit}
            kind={kind}
            onManual={(downloads) =>
              setManual({ folder: t.content.folders[kind], downloads })
            }
          />
        )}
      />

      <ManualDownloadsDialog
        serverId={server.id}
        folder={manual.folder}
        downloads={manual.downloads}
        onClose={() => setManual({ folder: "", downloads: [] })}
      />
    </>
  );
}

function InstallButton({
  server,
  hit,
  kind,
  onManual,
}: {
  server: ServerInfo;
  hit: ContentHit;
  kind: ContentKind;
  onManual: (downloads: ManualDownload[]) => void;
}) {
  const installed = useInstalledContent(server.id);
  const install = useInstallContent(server.id, kind);
  const isInstalled = installed.data?.some(
    (item) =>
      item.meta?.source === hit.source && item.meta.projectId === hit.projectId,
  );

  return (
    <Button
      variant={isInstalled ? "outline" : "default"}
      disabled={server.status !== "stopped" || install.isPending}
      onClick={() =>
        install.mutate(hit, {
          onSuccess: (outcome) => {
            if (outcome.manual.length > 0) onManual(outcome.manual);
          },
        })
      }
    >
      {install.isPending ? (
        <Loader2 data-icon="inline-start" className="animate-spin" />
      ) : isInstalled ? (
        <Check data-icon="inline-start" />
      ) : null}
      {isInstalled ? t.content.update : t.content.install}
    </Button>
  );
}
