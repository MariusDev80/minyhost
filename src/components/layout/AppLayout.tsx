import { ScrollArea } from "@/components/ui/scroll-area";
import { Sidebar } from "./Sidebar";
import { TitleBar } from "./TitleBar";

/** Window shell: title bar on top, sidebar on the left, scrollable page area. */
export function AppLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex h-screen flex-col overflow-hidden">
      <TitleBar />
      <div className="flex min-h-0 flex-1">
        <Sidebar />
        <ScrollArea className="min-w-0 flex-1">
          <main className="mx-auto w-full max-w-5xl p-8">{children}</main>
        </ScrollArea>
      </div>
    </div>
  );
}
