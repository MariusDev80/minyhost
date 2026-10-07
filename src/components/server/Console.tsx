import { useEffect, useRef, useState } from "react";
import { SendHorizontal } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useSendCommand } from "@/hooks/useServers";
import { cn } from "@/lib/utils";
import { t } from "@/i18n";
import { useConsoleStore, type ConsoleEntry } from "@/stores/console";
import type { ServerInfo } from "@/types";

const noLines: ConsoleEntry[] = [];

/** Live server output, plus a field to send commands. */
export function Console({ server }: { server: ServerInfo }) {
  const lines = useConsoleStore((state) => state.lines[server.id] ?? noLines);
  const canSend = server.status === "starting" || server.status === "running";

  return (
    <div className="flex flex-col overflow-hidden rounded-xl ring-1 ring-foreground/10">
      <ConsoleOutput lines={lines} />
      <CommandInput serverId={server.id} disabled={!canSend} />
    </div>
  );
}

function ConsoleOutput({ lines }: { lines: ConsoleEntry[] }) {
  const ref = useRef<HTMLDivElement>(null);
  // Follow new lines, unless the user scrolled up to read something.
  const stickToBottom = useRef(true);

  useEffect(() => {
    const el = ref.current;
    if (el && stickToBottom.current) el.scrollTop = el.scrollHeight;
  }, [lines]);

  return (
    <div
      ref={ref}
      data-selectable
      onScroll={(e) => {
        const el = e.currentTarget;
        stickToBottom.current =
          el.scrollHeight - el.scrollTop - el.clientHeight < 24;
      }}
      className="h-96 overflow-y-auto bg-console p-3 font-mono text-xs leading-relaxed text-console-foreground"
    >
      {lines.length === 0 ? (
        <p className="text-muted-foreground">{t.server.consoleEmpty}</p>
      ) : (
        lines.map((line) => (
          <p
            key={line.key}
            className={cn("break-all whitespace-pre-wrap", lineColor(line))}
          >
            {line.text}
          </p>
        ))
      )}
    </div>
  );
}

function CommandInput({
  serverId,
  disabled,
}: {
  serverId: string;
  disabled: boolean;
}) {
  const [command, setCommand] = useState("");
  const send = useSendCommand(serverId);

  return (
    <form
      className="flex gap-2 border-t bg-card p-2"
      onSubmit={(e) => {
        e.preventDefault();
        const trimmed = command.trim();
        if (!trimmed) return;
        send.mutate(trimmed, { onSuccess: () => setCommand("") });
      }}
    >
      <Input
        value={command}
        onChange={(e) => setCommand(e.target.value)}
        placeholder={t.server.commandPlaceholder}
        disabled={disabled}
        className="font-mono"
        aria-label={t.server.commandPlaceholder}
      />
      <Button
        type="submit"
        size="icon"
        disabled={disabled || send.isPending}
        aria-label={t.actions.send}
        title={t.actions.send}
      >
        <SendHorizontal />
      </Button>
    </form>
  );
}

/** Color hint from the log level written by Minecraft in each line. */
function lineColor(line: ConsoleEntry) {
  if (line.kind === "input") return "text-primary";
  if (line.kind === "stderr" || /\b(ERROR|FATAL)\b/.test(line.text))
    return "text-destructive";
  if (/\bWARN\b/.test(line.text)) return "text-warning";
  return undefined;
}
