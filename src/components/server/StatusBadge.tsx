import { cn } from "@/lib/utils";
import { t } from "@/i18n";
import type { ServerStatus } from "@/types";

const styles: Record<ServerStatus, { badge: string; dot: string }> = {
  starting: {
    badge: "bg-warning/15 text-warning",
    dot: "bg-warning animate-pulse",
  },
  running: { badge: "bg-success/15 text-success", dot: "bg-success" },
  stopping: {
    badge: "bg-warning/15 text-warning",
    dot: "bg-warning animate-pulse",
  },
  stopped: {
    badge: "bg-muted text-muted-foreground",
    dot: "bg-muted-foreground",
  },
};

/** Colored pill showing a server's lifecycle state. */
export function StatusBadge({
  status,
  className,
}: {
  status: ServerStatus;
  className?: string;
}) {
  const style = styles[status];
  return (
    <span
      className={cn(
        "inline-flex h-6 items-center gap-1.5 rounded-full px-2.5 text-xs font-medium",
        style.badge,
        className,
      )}
    >
      <span aria-hidden className={cn("size-1.5 rounded-full", style.dot)} />
      {t.status[status]}
    </span>
  );
}
