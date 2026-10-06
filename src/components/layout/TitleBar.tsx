import { useEffect, useState } from "react";
import { Copy, Minus, Square, X } from "lucide-react";
import { cn } from "@/lib/utils";
import { appWindow } from "@/lib/tauri";
import { t } from "@/i18n";
import { Logo } from "./Logo";

/** Custom title bar for the frameless window: drag region + window controls. */
export function TitleBar() {
  const [maximized, setMaximized] = useState(false);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    let disposed = false;
    const sync = () => {
      appWindow.isMaximized().then(setMaximized, () => {});
    };
    sync();
    appWindow.onResized(sync).then(
      (fn) => {
        if (disposed) fn();
        else unlisten = fn;
      },
      () => {},
    );
    return () => {
      disposed = true;
      unlisten?.();
    };
  }, []);

  return (
    <header
      data-tauri-drag-region
      className="flex h-10 shrink-0 items-center border-b border-sidebar-border bg-sidebar pl-3"
    >
      <div data-tauri-drag-region className="flex items-center gap-2">
        <Logo className="size-5" />
        <span
          data-tauri-drag-region
          className="text-sm font-semibold text-sidebar-foreground"
        >
          {t.app.name}
        </span>
      </div>
      <div data-tauri-drag-region className="flex-1 self-stretch" />
      <div className="flex self-stretch">
        <WindowButton label={t.titleBar.minimize} onClick={appWindow.minimize}>
          <Minus />
        </WindowButton>
        <WindowButton
          label={maximized ? t.titleBar.restore : t.titleBar.maximize}
          onClick={appWindow.toggleMaximize}
        >
          {maximized ? (
            <Copy className="size-3.5 -scale-x-100" />
          ) : (
            <Square className="size-3.5" />
          )}
        </WindowButton>
        <WindowButton label={t.titleBar.close} onClick={appWindow.close} danger>
          <X />
        </WindowButton>
      </div>
    </header>
  );
}

function WindowButton({
  label,
  onClick,
  danger = false,
  children,
}: {
  label: string;
  onClick: () => Promise<void>;
  danger?: boolean;
  children: React.ReactNode;
}) {
  return (
    <button
      type="button"
      aria-label={label}
      title={label}
      onClick={() => void onClick().catch(() => {})}
      className={cn(
        "inline-flex w-12 items-center justify-center text-sidebar-foreground transition-colors outline-none focus-visible:bg-sidebar-accent [&_svg:not([class*='size-'])]:size-4",
        danger
          ? "hover:bg-destructive hover:text-destructive-foreground"
          : "hover:bg-sidebar-accent hover:text-sidebar-accent-foreground",
      )}
    >
      {children}
    </button>
  );
}
