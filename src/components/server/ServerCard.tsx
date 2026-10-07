import { Card } from "@/components/ui/card";
import { t } from "@/i18n";
import { useUiStore } from "@/stores/ui";
import type { ServerInfo } from "@/types";
import { loaderInfo } from "./loaders";
import { ServerControls } from "./ServerControls";
import { StatusBadge } from "./StatusBadge";

/** Server summary in the list. The whole card opens the server page. */
export function ServerCard({ server }: { server: ServerInfo }) {
  const navigate = useUiStore((state) => state.navigate);
  const loader = loaderInfo(server.loader);
  const Icon = loader.icon;

  return (
    <Card className="relative gap-4 p-4 transition-colors hover:bg-accent/50">
      <div className="flex items-start gap-3">
        <div className="flex size-10 shrink-0 items-center justify-center rounded-lg bg-accent text-primary">
          <Icon className="size-5" />
        </div>
        <div className="min-w-0 flex-1">
          {/* Stretched button: its ::after covers the card, so the whole card is clickable. */}
          <button
            type="button"
            onClick={() => navigate({ name: "server", id: server.id })}
            className="block w-full truncate text-left font-semibold outline-none after:absolute after:inset-0 after:rounded-xl focus-visible:after:ring-3 focus-visible:after:ring-ring/50"
          >
            {server.name}
          </button>
          <p className="text-sm text-muted-foreground">
            {loader.name} {server.mcVersion}
          </p>
        </div>
        <StatusBadge status={server.status} />
      </div>
      <div className="flex items-center justify-between">
        <span className="text-sm text-muted-foreground">
          {t.units.memory(server.memoryMb)}
        </span>
        {/* Above the stretched button so it stays clickable. */}
        <div className="relative z-10">
          <ServerControls server={server} />
        </div>
      </div>
    </Card>
  );
}
