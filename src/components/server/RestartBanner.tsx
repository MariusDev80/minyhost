import { Loader2, RefreshCw } from "lucide-react";
import { Button } from "@/components/ui/button";
import { useRestartServer } from "@/hooks/useServers";
import { t } from "@/i18n";
import type { ServerInfo } from "@/types";

/** Shown when settings were saved while the server was running. */
export function RestartBanner({ server }: { server: ServerInfo }) {
  const restart = useRestartServer();
  if (!server.needsRestart) return null;

  return (
    <div className="flex items-center gap-3 rounded-xl border border-warning/30 bg-warning/10 p-4 text-sm">
      <RefreshCw className="size-5 shrink-0 text-warning" />
      <div className="flex-1 space-y-0.5">
        <p className="font-medium">{t.restart.title}</p>
        <p className="text-muted-foreground">{t.restart.description}</p>
      </div>
      <Button
        disabled={restart.isPending || server.status !== "running"}
        onClick={() => restart.mutate(server.id)}
      >
        {restart.isPending ? (
          <Loader2 data-icon="inline-start" className="animate-spin" />
        ) : (
          <RefreshCw data-icon="inline-start" />
        )}
        {t.actions.restart}
      </Button>
    </div>
  );
}
