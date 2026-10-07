import { Server } from "lucide-react";
import { EmptyState } from "@/components/layout/EmptyState";
import { PageHeader } from "@/components/layout/PageHeader";
import { CreateServerButton } from "@/components/server/CreateServerButton";
import { ServerGrid } from "@/components/server/ServerGrid";
import { t } from "@/i18n";

export function ServersPage() {
  return (
    <div className="space-y-8">
      <PageHeader
        title={t.servers.title}
        description={t.servers.description}
        actions={<CreateServerButton />}
      />
      <ServerGrid
        empty={
          <EmptyState
            icon={Server}
            title={t.servers.emptyTitle}
            description={t.servers.emptyDescription}
          />
        }
      />
    </div>
  );
}
