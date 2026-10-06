import { Loader2 } from "lucide-react";
import { t } from "@/i18n";
import { useUiStore } from "@/stores/ui";

/** Covers the app while running servers are stopped before closing. */
export function ClosingOverlay() {
  const closing = useUiStore((state) => state.closing);
  if (!closing) return null;

  return (
    <div className="fixed inset-0 z-50 flex flex-col items-center justify-center gap-3 bg-background/90 backdrop-blur-sm">
      <Loader2 className="size-8 animate-spin text-primary" />
      <p className="font-semibold">{t.closing.title}</p>
      <p className="text-sm text-muted-foreground">{t.closing.description}</p>
    </div>
  );
}
