import {
  useAddOperator,
  useOperators,
  useRemoveOperator,
} from "@/hooks/useOperators";
import { t } from "@/i18n";
import { PlayerListPanel } from "./PlayerListPanel";

/** Players allowed to use every command (`ops.json`). */
export function OperatorsPanel({ serverId }: { serverId: string }) {
  return (
    <PlayerListPanel
      texts={t.operators}
      players={useOperators(serverId)}
      add={useAddOperator(serverId)}
      remove={useRemoveOperator(serverId)}
    />
  );
}
