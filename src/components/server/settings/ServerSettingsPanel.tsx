import { useState } from "react";
import { Loader2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import {
  useGameSettings,
  useSaveGameSettings,
  useSettingsCatalog,
} from "@/hooks/useGameSettings";
import { t } from "@/i18n";
import { changesFrom, hasChanges, rebase } from "@/lib/gameSettings";
import type { GameSettings, ServerInfo, SettingsCatalog } from "@/types";
import { GameSettingsForm } from "./GameSettingsForm";

/** "Paramètres" tab: shows the server's real values, saves only what changed. */
export function ServerSettingsPanel({ server }: { server: ServerInfo }) {
  const catalog = useSettingsCatalog(server.mcVersion);
  const current = useGameSettings(server.id);

  if (catalog.isPending || current.isPending) {
    return <Skeleton className="h-64 rounded-xl" />;
  }
  if (catalog.isError || current.isError) {
    return (
      <p className="text-sm text-destructive">{t.gameSettings.loadError}</p>
    );
  }
  return (
    <SettingsEditor
      serverId={server.id}
      catalog={catalog.data}
      latest={current.data}
    />
  );
}

function SettingsEditor({
  serverId,
  catalog,
  latest,
}: {
  serverId: string;
  catalog: SettingsCatalog;
  /** Values on the server; refreshed when a game rule changes in game. */
  latest: GameSettings;
}) {
  // `base`: the values the draft was started from.
  const [base, setBase] = useState(latest);
  const [draft, setDraft] = useState(latest);
  if (latest !== base) {
    setBase(latest);
    setDraft(rebase(draft, base, latest));
  }

  const changes = changesFrom(base, draft);
  const save = useSaveGameSettings(serverId);

  return (
    <div className="space-y-4">
      <GameSettingsForm
        catalog={catalog}
        value={draft}
        onChange={setDraft}
        mode="edit"
      />
      {hasChanges(changes) && (
        <div className="sticky bottom-4 flex items-center gap-3 rounded-xl border bg-popover p-3 shadow-lg">
          <span className="flex-1 text-sm font-medium">
            {t.gameSettings.unsaved}
          </span>
          <Button variant="ghost" onClick={() => setDraft(base)}>
            {t.gameSettings.discard}
          </Button>
          <Button
            disabled={save.isPending}
            onClick={() => save.mutate(changes)}
          >
            {save.isPending && (
              <Loader2 data-icon="inline-start" className="animate-spin" />
            )}
            {t.gameSettings.save}
          </Button>
        </div>
      )}
    </div>
  );
}
