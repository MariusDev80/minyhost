import { CircleHelp, Copy, Globe, Loader2 } from "lucide-react";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { useSetInternetAccess, useTunnel } from "@/hooks/useTunnel";
import { t } from "@/i18n";
import { cn } from "@/lib/utils";
import { useUiStore } from "@/stores/ui";
import type { ServerInfo, ServerTunnel, TunnelState } from "@/types";
import { PlayitLinkActions, PlayitNotices } from "./PlayitAccount";

/** Opening a server to friends outside the local network, via playit.gg. */
export function InternetAccessCard({ server }: { server: ServerInfo }) {
  const { data: state } = useTunnel();
  const navigate = useUiStore((s) => s.navigate);

  return (
    <Card className="gap-4 p-4">
      <div className="flex items-start gap-3">
        <div className="flex size-10 shrink-0 items-center justify-center rounded-lg bg-accent text-primary">
          <Globe className="size-5" />
        </div>
        <div className="min-w-0 flex-1 space-y-3">
          <div className="flex items-center gap-2">
            <h2 className="flex-1 font-semibold">{t.tunnel.title}</h2>
            <Button
              variant="ghost"
              size="sm"
              onClick={() => navigate({ name: "help" })}
            >
              <CircleHelp data-icon="inline-start" />
              {t.nav.help}
            </Button>
          </div>
          {state ? (
            <InternetAccess server={server} state={state} />
          ) : (
            <Skeleton className="h-16" />
          )}
        </div>
      </div>
    </Card>
  );
}

function InternetAccess({
  server,
  state,
}: {
  server: ServerInfo;
  state: TunnelState;
}) {
  const setAccess = useSetInternetAccess(server.id);
  const tunnel = state.tunnels[server.id];

  if (state.link.state !== "linked") {
    return (
      <>
        <p className="text-sm text-muted-foreground">{t.tunnel.description}</p>
        <PlayitLinkActions state={state} />
      </>
    );
  }

  return (
    <>
      {tunnel ? (
        <TunnelDetails server={server} state={state} tunnel={tunnel} />
      ) : (
        <p className="text-sm text-muted-foreground">{t.tunnel.enableHint}</p>
      )}
      <Button
        variant={tunnel ? "ghost" : "default"}
        disabled={setAccess.isPending}
        onClick={() => setAccess.mutate(!tunnel)}
      >
        {setAccess.isPending ? (
          <Loader2 data-icon="inline-start" className="animate-spin" />
        ) : (
          <Globe data-icon="inline-start" />
        )}
        {tunnel ? t.tunnel.disable : t.tunnel.enable}
      </Button>
      <PlayitNotices state={state} />
    </>
  );
}

function TunnelDetails({
  server,
  state,
  tunnel,
}: {
  server: ServerInfo;
  state: TunnelState;
  tunnel: ServerTunnel;
}) {
  const { address } = tunnel;

  return (
    <div className="space-y-2">
      {address ? (
        <div className="flex items-center gap-2">
          <div className="min-w-0 flex-1 rounded-lg border bg-muted/50 px-3 py-2">
            <p className="text-xs text-muted-foreground">{t.tunnel.address}</p>
            <p data-selectable className="truncate font-mono font-medium">
              {address}
            </p>
          </div>
          <Button
            variant="outline"
            size="icon-lg"
            aria-label={t.tunnel.copy}
            title={t.tunnel.copy}
            onClick={() =>
              navigator.clipboard.writeText(address).then(
                () => toast.success(t.tunnel.copied),
                () => {},
              )
            }
          >
            <Copy />
          </Button>
        </div>
      ) : (
        <p className="flex items-center gap-2 text-sm text-muted-foreground">
          <Loader2 className="size-4 animate-spin" />
          {t.tunnel.pendingAddress}
        </p>
      )}
      <TunnelStatus server={server} state={state} tunnel={tunnel} />
    </div>
  );
}

/** One line telling whether friends can join right now, and why not. */
function TunnelStatus({
  server,
  state,
  tunnel,
}: {
  server: ServerInfo;
  state: TunnelState;
  tunnel: ServerTunnel;
}) {
  let text: string;
  let tone: "ok" | "wait" | "bad";
  if (tunnel.disabledReason) {
    text = t.tunnel.disabled(tunnel.disabledReason);
    tone = "bad";
  } else if (server.status === "stopped") {
    text = t.tunnel.status.stopped;
    tone = "wait";
  } else if (state.agent === "online") {
    text = t.tunnel.status.online;
    tone = "ok";
  } else if (state.agent === "error") {
    text = t.tunnel.status.error;
    tone = "bad";
  } else {
    text = t.tunnel.status.connecting;
    tone = "wait";
  }

  return (
    <p className="flex items-center gap-2 text-sm text-muted-foreground">
      <span
        className={cn(
          "size-2 shrink-0 rounded-full",
          tone === "ok" && "bg-success",
          tone === "wait" && "bg-muted-foreground",
          tone === "bad" && "bg-destructive",
        )}
      />
      {text}
    </p>
  );
}
