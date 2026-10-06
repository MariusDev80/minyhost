import { User } from "lucide-react";
import { Skeleton } from "@/components/ui/skeleton";
import { usePlayerSkin } from "@/hooks/useWhitelist";
import { cn } from "@/lib/utils";

/**
 * Face of a player's skin (Steve if they have none), drawn from the full
 * skin image: no image processing, just CSS.
 *
 * A skin is a 64-pixel-wide texture: the face is the 8x8 square at (8, 8),
 * and the hat layer drawn over it is the 8x8 square at (40, 8). Scaling the
 * texture by `size / 8` and shifting it shows only that square.
 * Old 64x32 skins work too: the face is at the same place.
 */
export function PlayerHead({
  uuid,
  size = 32,
  className,
}: {
  uuid: string;
  size?: number;
  className?: string;
}) {
  const skin = usePlayerSkin(uuid);
  const box = { width: size, height: size };

  if (skin.isPending) {
    return <Skeleton style={box} className={cn("rounded-md", className)} />;
  }

  // Offline or Mojang unreachable: generic icon.
  if (skin.isError) {
    return (
      <div
        style={box}
        className={cn(
          "flex shrink-0 items-center justify-center rounded-md bg-muted text-muted-foreground",
          className,
        )}
      >
        <User className="size-1/2" />
      </div>
    );
  }

  const layer = (x: number): React.CSSProperties => ({
    backgroundImage: `url(${skin.data.url})`,
    backgroundSize: `${size * 8}px auto`,
    backgroundPosition: `-${x * (size / 8)}px -${size}px`,
    imageRendering: "pixelated",
  });

  return (
    <div
      aria-hidden
      style={box}
      className={cn("relative shrink-0 overflow-hidden rounded-md", className)}
    >
      <div className="absolute inset-0" style={layer(8)} />
      {skin.data.showHat && (
        <div className="absolute inset-0" style={layer(40)} />
      )}
    </div>
  );
}
