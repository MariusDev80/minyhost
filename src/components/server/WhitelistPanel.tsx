import {
  useAddToWhitelist,
  useRemoveFromWhitelist,
  useWhitelist,
} from "@/hooks/useWhitelist";
import { t } from "@/i18n";
import { PlayerListPanel } from "./PlayerListPanel";

/** Players allowed to join the server (`whitelist.json`). */
export function WhitelistPanel({ serverId }: { serverId: string }) {
  return (
    <PlayerListPanel
      texts={t.whitelist}
      players={useWhitelist(serverId)}
      add={useAddToWhitelist(serverId)}
      remove={useRemoveFromWhitelist(serverId)}
    />
  );
}
