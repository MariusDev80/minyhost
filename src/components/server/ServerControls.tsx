import { Loader2, Play, Square } from "lucide-react";
import { Button } from "@/components/ui/button";
import { useStartServer, useStopServer } from "@/hooks/useServers";
import { t } from "@/i18n";
import type { ServerInfo } from "@/types";

/** Start or Stop button, depending on the server's status. */
export function ServerControls({
  server,
  size = "default",
}: {
  server: ServerInfo;
  size?: "default" | "lg";
}) {
  const start = useStartServer();
  const stop = useStopServer();

  if (server.status === "stopped") {
    return (
      <Button
        size={size}
        disabled={start.isPending}
        onClick={() => start.mutate(server.id)}
      >
        {start.isPending ? (
          <Loader2 data-icon="inline-start" className="animate-spin" />
        ) : (
          <Play data-icon="inline-start" />
        )}
        {t.actions.start}
      </Button>
    );
  }

  const stopping = server.status === "stopping" || stop.isPending;
  return (
    <Button
      size={size}
      variant="secondary"
      disabled={stopping}
      onClick={() => stop.mutate(server.id)}
    >
      {stopping ? (
        <Loader2 data-icon="inline-start" className="animate-spin" />
      ) : (
        <Square data-icon="inline-start" />
      )}
      {t.actions.stop}
    </Button>
  );
}
