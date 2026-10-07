import {
  ArrowLeft,
  Crown,
  ServerOff,
  Settings2,
  Terminal,
  Users,
} from "lucide-react";
import { EmptyState } from "@/components/layout/EmptyState";
import { PageHeader } from "@/components/layout/PageHeader";
import { Console } from "@/components/server/Console";
import { DeleteServerDialog } from "@/components/server/DeleteServerDialog";
import { OperatorsPanel } from "@/components/server/OperatorsPanel";
import { RestartBanner } from "@/components/server/RestartBanner";
import { ServerSettingsPanel } from "@/components/server/settings/ServerSettingsPanel";
import { loaderInfo } from "@/components/server/loaders";
import { ServerControls } from "@/components/server/ServerControls";
import { StatusBadge } from "@/components/server/StatusBadge";
import { WhitelistPanel } from "@/components/server/WhitelistPanel";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useOperators } from "@/hooks/useOperators";
import { useServer } from "@/hooks/useServers";
import { useWhitelist } from "@/hooks/useWhitelist";
import { t } from "@/i18n";
import { useUiStore } from "@/stores/ui";
import type { ServerInfo } from "@/types";

export function ServerDetailPage({ id }: { id: string }) {
  const navigate = useUiStore((state) => state.navigate);
  const { data: server, isPending } = useServer(id);

  return (
    <div className="space-y-6">
      <Button
        variant="ghost"
        size="sm"
        onClick={() => navigate({ name: "servers" })}
      >
        <ArrowLeft data-icon="inline-start" />
        {t.server.back}
      </Button>

      {isPending ? (
        <Skeleton className="h-64 rounded-xl" />
      ) : server ? (
        <ServerDetail server={server} />
      ) : (
        <EmptyState icon={ServerOff} title={t.server.notFound} />
      )}
    </div>
  );
}

function ServerDetail({ server }: { server: ServerInfo }) {
  const loader = loaderInfo(server.loader);
  const whitelist = useWhitelist(server.id);
  const operators = useOperators(server.id);
  // Minecraft's default port can be omitted from the address.
  const address =
    server.port === 25565 ? "localhost" : `localhost:${server.port}`;

  return (
    <>
      <PageHeader
        title={server.name}
        description={`${loader.name} ${server.mcVersion}`}
        actions={
          <>
            <StatusBadge status={server.status} className="self-center" />
            <DeleteServerDialog server={server} />
            <ServerControls server={server} size="lg" />
          </>
        }
      />

      <div className="grid grid-cols-2 gap-3 md:grid-cols-4">
        <InfoTile label={t.server.address} value={address} selectable />
        <InfoTile label={t.server.version} value={server.mcVersion} />
        <InfoTile
          label={t.server.memory}
          value={t.units.memory(server.memoryMb)}
        />
        <InfoTile label={t.server.java} value={String(server.javaVersion)} />
      </div>

      <RestartBanner server={server} />

      <Tabs defaultValue="console">
        <TabsList>
          <TabsTrigger value="console">
            <Terminal />
            {t.server.console}
          </TabsTrigger>
          <TabsTrigger value="whitelist">
            <Users />
            {t.whitelist.tab}
            <TabCount count={whitelist.data?.length} />
          </TabsTrigger>
          <TabsTrigger value="operators">
            <Crown />
            {t.operators.tab}
            <TabCount count={operators.data?.length} />
          </TabsTrigger>
          <TabsTrigger value="settings">
            <Settings2 />
            {t.gameSettings.tab}
          </TabsTrigger>
        </TabsList>
        <TabsContent value="console" className="pt-2">
          <Console server={server} />
        </TabsContent>
        <TabsContent value="whitelist" className="pt-2">
          <WhitelistPanel serverId={server.id} />
        </TabsContent>
        <TabsContent value="operators" className="pt-2">
          <OperatorsPanel serverId={server.id} />
        </TabsContent>
        <TabsContent value="settings" className="pt-2">
          <ServerSettingsPanel server={server} />
        </TabsContent>
      </Tabs>
    </>
  );
}

/** Number of players shown next to a tab name. */
function TabCount({ count }: { count?: number }) {
  if (count === undefined) return null;
  return <span className="text-xs text-muted-foreground">{count}</span>;
}

function InfoTile({
  label,
  value,
  selectable = false,
}: {
  label: string;
  value: string;
  selectable?: boolean;
}) {
  return (
    <Card size="sm" className="gap-1 px-3">
      <p className="text-xs text-muted-foreground">{label}</p>
      <p
        data-selectable={selectable || undefined}
        className="truncate font-medium"
      >
        {value}
      </p>
    </Card>
  );
}
