import {
  ExternalLink,
  Globe,
  Info,
  Link2,
  Loader2,
  RefreshCw,
  TriangleAlert,
  Unlink,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  useCancelPlayitLink,
  useLinkPlayit,
  useRecheckPlayitAgent,
  useTunnel,
  useUnlinkPlayit,
} from "@/hooks/useTunnel";
import { t } from "@/i18n";
import { playitUrls } from "@/lib/playit";
import { openExternal } from "@/lib/tauri";
import type { TunnelState } from "@/types";

/** "Link playit.gg" button, or the waiting state while the user approves it. */
export function PlayitLinkActions({ state }: { state: TunnelState }) {
  const link = useLinkPlayit();
  const cancel = useCancelPlayitLink();

  if (state.link.state === "linking") {
    const { url } = state.link;
    return (
      <div className="space-y-3">
        <div className="flex items-center gap-2 text-sm">
          <Loader2 className="size-4 animate-spin text-primary" />
          <span className="font-medium">{t.tunnel.linking}</span>
        </div>
        <p className="text-sm text-muted-foreground">{t.tunnel.linkingHint}</p>
        <div className="flex gap-2">
          <Button variant="outline" onClick={() => openExternal(url)}>
            <ExternalLink data-icon="inline-start" />
            {t.tunnel.reopen}
          </Button>
          <Button
            variant="ghost"
            disabled={cancel.isPending}
            onClick={() => cancel.mutate()}
          >
            {t.actions.cancel}
          </Button>
        </div>
      </div>
    );
  }

  return (
    <Button disabled={link.isPending} onClick={() => link.mutate()}>
      {link.isPending ? (
        <Loader2 data-icon="inline-start" className="animate-spin" />
      ) : (
        <Link2 data-icon="inline-start" />
      )}
      {t.tunnel.link}
    </Button>
  );
}

/** Messages about the account: what to fix on playit.gg, then its notices. */
export function PlayitNotices({ state }: { state: TunnelState }) {
  return (
    <>
      {state.agentOverLimit && (
        <AccountNotice
          title={t.tunnel.agentLimit.title}
          steps={t.tunnel.agentLimit.steps}
          action={t.tunnel.agentLimit.open}
          url={playitUrls.agents}
        >
          <RecheckButton />
        </AccountNotice>
      )}
      {state.emailUnverified && (
        <AccountNotice
          title={t.tunnel.emailUnverified.title}
          steps={t.tunnel.emailUnverified.steps}
          action={t.tunnel.emailUnverified.open}
          url={playitUrls.account}
        />
      )}
      <PlayitApiNotices state={state} />
    </>
  );
}

/** Something to fix on playit.gg, with the steps and a link to the page. */
function AccountNotice({
  title,
  steps,
  action,
  url,
  children,
}: {
  title: string;
  steps: string;
  action: string;
  url: string;
  /** Extra buttons, next to the link. */
  children?: React.ReactNode;
}) {
  return (
    <div className="flex items-start gap-3 rounded-xl border border-warning/30 bg-warning/10 p-3 text-sm">
      <TriangleAlert className="mt-0.5 size-4 shrink-0 text-warning" />
      <div className="flex-1 space-y-2">
        <p className="font-medium">{title}</p>
        <p className="text-muted-foreground">{steps}</p>
        <div className="flex flex-wrap gap-2">
          <Button variant="outline" size="sm" onClick={() => openExternal(url)}>
            <ExternalLink data-icon="inline-start" />
            {action}
          </Button>
          {children}
        </div>
      </div>
    </div>
  );
}

function RecheckButton() {
  const recheck = useRecheckPlayitAgent();
  return (
    <Button
      variant="ghost"
      size="sm"
      disabled={recheck.isPending}
      onClick={() => recheck.mutate()}
    >
      {recheck.isPending ? (
        <Loader2 data-icon="inline-start" className="animate-spin" />
      ) : (
        <RefreshCw data-icon="inline-start" />
      )}
      {t.tunnel.agentLimit.recheck}
    </Button>
  );
}

/** Which agent MinyHost uses, to keep the right one when cleaning up playit.gg. */
function AgentIdentity({ state }: { state: TunnelState }) {
  if (!state.agentId) return null;
  return (
    <div className="space-y-1 rounded-lg border bg-muted/50 px-3 py-2 text-sm">
      <p className="text-xs text-muted-foreground">{t.tunnel.settings.agent}</p>
      {state.agentName && (
        <p data-selectable className="font-medium">
          {state.agentName}
        </p>
      )}
      <p data-selectable className="font-mono text-xs text-muted-foreground">
        {t.tunnel.settings.agentId(state.agentId)}
      </p>
      <p className="pt-1 text-xs text-muted-foreground">
        {t.tunnel.settings.agentHint}
      </p>
    </div>
  );
}

/** Messages from playit.gg about the account (in English, from their API). */
function PlayitApiNotices({ state }: { state: TunnelState }) {
  if (state.notices.length === 0) return null;
  return (
    <ul className="space-y-2">
      {state.notices.map((notice) => (
        <li
          key={notice.message}
          lang="en"
          className="flex items-start gap-2 rounded-lg bg-muted p-3 text-sm"
        >
          <Info className="mt-0.5 size-4 shrink-0 text-muted-foreground" />
          <span className="flex-1">{notice.message}</span>
          {notice.link && (
            <Button
              variant="link"
              size="sm"
              className="h-auto p-0"
              onClick={() => notice.link && openExternal(notice.link)}
            >
              {t.tunnel.noticeLink}
            </Button>
          )}
        </li>
      ))}
    </ul>
  );
}

/** Account section of the Settings page. */
export function PlayitSettingsCard() {
  const { data: state } = useTunnel();
  if (!state) return null;
  const linked = state.link.state === "linked";

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Globe className="size-4 text-primary" />
          {t.tunnel.settings.title}
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        <p className="text-sm text-muted-foreground">
          {linked
            ? t.tunnel.settings.linkedDescription
            : t.tunnel.settings.unlinkedDescription}
        </p>
        {linked ? (
          <div className="flex gap-2">
            <Button
              variant="outline"
              onClick={() => openExternal(playitUrls.account)}
            >
              <ExternalLink data-icon="inline-start" />
              {t.tunnel.settings.manage}
            </Button>
            <UnlinkDialog />
          </div>
        ) : (
          <PlayitLinkActions state={state} />
        )}
        {linked && <AgentIdentity state={state} />}
        {linked && <PlayitNotices state={state} />}
      </CardContent>
    </Card>
  );
}

function UnlinkDialog() {
  const unlink = useUnlinkPlayit();

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="ghost">
          <Unlink data-icon="inline-start" />
          {t.tunnel.settings.unlink}
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t.tunnel.settings.unlinkTitle}</DialogTitle>
          <DialogDescription>
            {t.tunnel.settings.unlinkDescription}
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">{t.actions.cancel}</Button>
          </DialogClose>
          <Button
            variant="destructive"
            disabled={unlink.isPending}
            onClick={() => unlink.mutate()}
          >
            {unlink.isPending ? (
              <Loader2 data-icon="inline-start" className="animate-spin" />
            ) : (
              <Unlink data-icon="inline-start" />
            )}
            {t.tunnel.settings.unlink}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
