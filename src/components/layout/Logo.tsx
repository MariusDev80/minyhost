import { Server } from "lucide-react";
import { cn } from "@/lib/utils";

/** Placeholder brand mark until a real logo is designed. */
export function Logo({ className }: { className?: string }) {
  return (
    <span
      aria-hidden
      className={cn(
        "inline-flex items-center justify-center rounded-md bg-primary text-primary-foreground",
        className,
      )}
    >
      <Server className="size-[60%]" />
    </span>
  );
}
