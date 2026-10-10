import { Package } from "lucide-react";
import { useContentIcon } from "@/hooks/useContent";
import { cn } from "@/lib/utils";

/** Project icon (fetched by Rust), or a neutral placeholder. */
export function ContentIcon({
  url,
  className,
}: {
  url: string | null | undefined;
  className?: string;
}) {
  const icon = useContentIcon(url);
  return (
    <div
      className={cn(
        "flex size-10 shrink-0 items-center justify-center overflow-hidden rounded-lg bg-muted text-muted-foreground",
        className,
      )}
    >
      {icon.data ? (
        <img src={icon.data} alt="" className="size-full object-cover" />
      ) : (
        <Package className="size-1/2" />
      )}
    </div>
  );
}
