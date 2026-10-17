import { ExternalLink, FolderOpen } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { t } from "@/i18n";
import { showError } from "@/lib/errors";
import { commands, openExternal } from "@/lib/tauri";
import type { ManualDownload } from "@/types";

/**
 * Files whose author only allows downloads from CurseForge's website: the
 * user downloads them and drops them into the server folder.
 */
export function ManualDownloadsDialog({
  serverId,
  folder,
  downloads,
  onClose,
}: {
  serverId: string;
  /** Folder the files go into, e.g. "mods". */
  folder: string;
  downloads: ManualDownload[];
  onClose: () => void;
}) {
  return (
    <Dialog
      open={downloads.length > 0}
      onOpenChange={(open) => !open && onClose()}
    >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t.content.manualTitle}</DialogTitle>
          <DialogDescription>
            {t.content.manualDescription(folder)}
          </DialogDescription>
        </DialogHeader>
        <ul className="space-y-2">
          {downloads.map((download) => (
            <li
              key={download.pageUrl}
              className="flex items-center gap-3 rounded-lg bg-muted/50 p-2"
            >
              <div className="min-w-0 flex-1">
                <p className="truncate text-sm font-medium">{download.title}</p>
                <p
                  data-selectable
                  className="truncate text-xs text-muted-foreground"
                >
                  {download.fileName}
                </p>
              </div>
              <Button
                variant="outline"
                size="sm"
                onClick={() => void openExternal(download.pageUrl)}
              >
                {t.content.openPage}
                <ExternalLink data-icon="inline-end" />
              </Button>
            </li>
          ))}
        </ul>
        <DialogFooter>
          <Button
            variant="outline"
            onClick={() => commands.openServerFolder(serverId).catch(showError)}
          >
            <FolderOpen data-icon="inline-start" />
            {t.server.openFolder}
          </Button>
          <Button onClick={onClose}>{t.actions.close}</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
