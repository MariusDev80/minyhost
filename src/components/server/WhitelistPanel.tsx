import { useState } from "react";
import { Loader2, UserPlus, X } from "lucide-react";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Skeleton } from "@/components/ui/skeleton";
import {
  useAddToWhitelist,
  useRemoveFromWhitelist,
  useWhitelist,
} from "@/hooks/useWhitelist";
import { t } from "@/i18n";
import type { WhitelistEntry } from "@/types";
import { PlayerHead } from "./PlayerHead";

/** Same rule as Rust (`core/players.rs`, `is_valid_name`). */
const PSEUDO_PATTERN = /^[A-Za-z0-9_]{3,16}$/;

/** Players allowed on the server: add by pseudo, remove in one click. */
export function WhitelistPanel({ serverId }: { serverId: string }) {
  const whitelist = useWhitelist(serverId);

  return (
    <div className="space-y-4 rounded-xl p-4 ring-1 ring-foreground/10">
      <div className="space-y-1">
        <h3 className="font-semibold">{t.whitelist.title}</h3>
        <p className="text-sm text-muted-foreground">
          {t.whitelist.description}
        </p>
      </div>

      <AddPlayerForm serverId={serverId} players={whitelist.data ?? []} />

      {whitelist.isPending ? (
        <Skeleton className="h-12 rounded-lg" />
      ) : whitelist.isError ? null : whitelist.data.length === 0 ? (
        <p className="rounded-lg border border-dashed p-4 text-center text-sm text-muted-foreground">
          {t.whitelist.empty}
        </p>
      ) : (
        <ul className="grid gap-2 sm:grid-cols-2">
          {whitelist.data.map((player) => (
            <PlayerRow key={player.uuid} serverId={serverId} player={player} />
          ))}
        </ul>
      )}
    </div>
  );
}

function AddPlayerForm({
  serverId,
  players,
}: {
  serverId: string;
  players: WhitelistEntry[];
}) {
  const [name, setName] = useState("");
  const add = useAddToWhitelist(serverId);
  const trimmed = name.trim();
  const valid = PSEUDO_PATTERN.test(trimmed);

  return (
    <form
      className="space-y-1.5"
      onSubmit={(e) => {
        e.preventDefault();
        if (!valid) return;
        const existing = players.find(
          (p) => p.name.toLowerCase() === trimmed.toLowerCase(),
        );
        if (existing) {
          toast.info(t.whitelist.already(existing.name));
          return;
        }
        add.mutate(trimmed, { onSuccess: () => setName("") });
      }}
    >
      <div className="flex gap-2">
        <Input
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder={t.whitelist.placeholder}
          aria-label={t.whitelist.placeholder}
          aria-invalid={trimmed !== "" && !valid}
          maxLength={16}
          className="max-w-64"
        />
        <Button type="submit" disabled={!valid || add.isPending}>
          {add.isPending ? (
            <Loader2 data-icon="inline-start" className="animate-spin" />
          ) : (
            <UserPlus data-icon="inline-start" />
          )}
          {t.whitelist.add}
        </Button>
      </div>
      {trimmed !== "" && !valid && (
        <p className="text-xs text-destructive">{t.whitelist.invalidName}</p>
      )}
    </form>
  );
}

function PlayerRow({
  serverId,
  player,
}: {
  serverId: string;
  player: WhitelistEntry;
}) {
  const remove = useRemoveFromWhitelist(serverId);

  return (
    <li className="flex items-center gap-3 rounded-lg bg-muted/50 p-2">
      <PlayerHead uuid={player.uuid} size={32} />
      <span data-selectable className="min-w-0 flex-1 truncate font-medium">
        {player.name}
      </span>
      <Button
        variant="ghost"
        size="icon-sm"
        disabled={remove.isPending}
        onClick={() => remove.mutate(player)}
        aria-label={t.whitelist.remove(player.name)}
        title={t.whitelist.remove(player.name)}
      >
        {remove.isPending ? <Loader2 className="animate-spin" /> : <X />}
      </Button>
    </li>
  );
}
