import type { LucideIcon } from "lucide-react";
import { cn } from "@/lib/utils";

/** Placeholder shown when a list has nothing to display yet. */
export function EmptyState({
  icon: Icon,
  title,
  description,
  action,
  className,
}: {
  icon: LucideIcon;
  title: string;
  description?: string;
  action?: React.ReactNode;
  className?: string;
}) {
  return (
    <div
      className={cn(
        "flex flex-col items-center justify-center gap-3 rounded-xl border border-dashed px-6 py-12 text-center",
        className,
      )}
    >
      <div className="flex size-12 items-center justify-center rounded-xl bg-accent text-primary">
        <Icon className="size-6" />
      </div>
      <div className="space-y-1">
        <h2 className="font-semibold">{title}</h2>
        {description && (
          <p className="max-w-sm text-sm text-muted-foreground">
            {description}
          </p>
        )}
      </div>
      {action && <div className="pt-2">{action}</div>}
    </div>
  );
}
