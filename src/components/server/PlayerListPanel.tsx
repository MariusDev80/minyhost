import { useState } from "react";
import type { UseMutationResult, UseQueryResult } from "@tanstack/react-query";
import { Loader2, UserPlus, X } from "lucide-react";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Skeleton } from "@/components/ui/skeleton";
import { t } from "@/i18n";
import type { Player } from "@/types";
import { PlayerHead } from "./PlayerHead";

/** Same rule as Rust (`core/players.rs`, `is_valid_name`). */
const PSEUDO_PATTERN = /^[A-Za-z0-9_]{3,16}$/;

/**
 * A list of players with an "add by pseudo" field and a remove button.
 * Shared by the whitelist and operators tabs (see `WhitelistPanel`,
 * `OperatorsPanel`), which only provide their texts and data hooks.
 */
export function PlayerListPanel<T extends Player>({
  texts,
  players,
  add,
  remove,
}: {
  texts: {
    title: string;
    description: string;
    empty: string;
    already: (name: string) => string;
  };
  players: UseQueryResult<T[]>;
  add: UseMutationResult<T, unknown, string>;
  remove: UseMutationResult<void, unknown, T>;
}) {
  return (
    <div className="space-y-4 rounded-xl p-4 ring-1 ring-foreground/10">
      <div className="space-y-1">
        <h3 className="font-semibold">{texts.title}</h3>
        <p className="text-sm text-muted-foreground">{texts.description}</p>
      </div>

      <AddPlayerForm
        add={add}
        players={players.data ?? []}
        already={texts.already}
      />

      {players.isPending ? (
        <Skeleton className="h-12 rounded-lg" />
      ) : players.isError ? null : players.data.length === 0 ? (
        <p className="rounded-lg border border-dashed p-4 text-center text-sm text-muted-foreground">
          {texts.empty}
        </p>
      ) : (
        <ul className="grid gap-2 sm:grid-cols-2">
          {players.data.map((player) => (
            <PlayerRow
              key={player.uuid}
              player={player}
              removing={
                remove.isPending && remove.variables?.uuid === player.uuid
              }
              onRemove={() => remove.mutate(player)}
            />
          ))}
        </ul>
      )}
    </div>
  );
}

function AddPlayerForm<T extends Player>({
  add,
  players,
  already,
}: {
  add: UseMutationResult<T, unknown, string>;
  players: T[];
  already: (name: string) => string;
}) {
  const [name, setName] = useState("");
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
          toast.info(already(existing.name));
          return;
        }
        add.mutate(trimmed, { onSuccess: () => setName("") });
      }}
    >
      <div className="flex gap-2">
        <Input
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder={t.players.placeholder}
          aria-label={t.players.placeholder}
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
          {t.players.add}
        </Button>
      </div>
      {trimmed !== "" && !valid && (
        <p className="text-xs text-destructive">{t.players.invalidName}</p>
      )}
    </form>
  );
}

function PlayerRow({
  player,
  removing,
  onRemove,
}: {
  player: Player;
  removing: boolean;
  onRemove: () => void;
}) {
  return (
    <li className="flex items-center gap-3 rounded-lg bg-muted/50 p-2">
      <PlayerHead uuid={player.uuid} size={32} />
      <span data-selectable className="min-w-0 flex-1 truncate font-medium">
        {player.name}
      </span>
      <Button
        variant="ghost"
        size="icon-sm"
        disabled={removing}
        onClick={onRemove}
        aria-label={t.players.remove(player.name)}
        title={t.players.remove(player.name)}
      >
        {removing ? <Loader2 className="animate-spin" /> : <X />}
      </Button>
    </li>
  );
}
