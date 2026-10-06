import { AlertCircle } from "lucide-react";
import { EmptyState } from "@/components/layout/EmptyState";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { useServers } from "@/hooks/useServers";
import { t } from "@/i18n";
import { ServerCard } from "./ServerCard";

/** Server list with its loading, error and empty states. */
export function ServerGrid({ empty }: { empty: React.ReactNode }) {
  const { data: servers, isPending, isError, refetch } = useServers();

  if (isPending) {
    return (
      <div className="grid gap-4 sm:grid-cols-2">
        <Skeleton className="h-32 rounded-xl" />
        <Skeleton className="h-32 rounded-xl" />
      </div>
    );
  }

  if (isError) {
    return (
      <EmptyState
        icon={AlertCircle}
        title={t.servers.loadError}
        action={
          <Button variant="outline" onClick={() => void refetch()}>
            {t.actions.retry}
          </Button>
        }
      />
    );
  }

  if (servers.length === 0) return empty;

  return (
    <div className="grid gap-4 sm:grid-cols-2">
      {servers.map((server) => (
        <ServerCard key={server.id} server={server} />
      ))}
    </div>
  );
}
