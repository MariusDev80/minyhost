import { Server } from "lucide-react";
import { EmptyState } from "@/components/layout/EmptyState";
import { PageHeader } from "@/components/layout/PageHeader";
import { CreateServerButton } from "@/components/server/CreateServerButton";
import { ServerGrid } from "@/components/server/ServerGrid";
import { t } from "@/i18n";

export function HomePage() {
  return (
    <div className="space-y-8">
      <PageHeader
        title={t.home.title}
        description={t.home.description}
        actions={<CreateServerButton />}
      />
      <ServerGrid
        empty={
          <EmptyState
            icon={Server}
            title={t.home.emptyTitle}
            description={t.home.emptyDescription}
            action={<CreateServerButton />}
          />
        }
      />
    </div>
  );
}
