import { Loader2, Trash2 } from "lucide-react";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { useDeleteServer } from "@/hooks/useServers";
import { t } from "@/i18n";
import { useUiStore } from "@/stores/ui";
import type { ServerInfo } from "@/types";

/** Trash button + confirmation. Only enabled while the server is stopped. */
export function DeleteServerDialog({ server }: { server: ServerInfo }) {
  const navigate = useUiStore((state) => state.navigate);
  const remove = useDeleteServer();
  const stopped = server.status === "stopped";

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button
          variant="ghost"
          size="icon-lg"
          disabled={!stopped}
          aria-label={t.actions.delete}
          title={t.actions.delete}
        >
          <Trash2 />
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t.server.deleteTitle(server.name)}</DialogTitle>
          <DialogDescription>{t.server.deleteDescription}</DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">{t.actions.cancel}</Button>
          </DialogClose>
          <Button
            variant="destructive"
            disabled={remove.isPending}
            onClick={() =>
              remove.mutate(server.id, {
                onSuccess: () => {
                  toast.success(t.server.deleted(server.name));
                  navigate({ name: "servers" });
                },
              })
            }
          >
            {remove.isPending ? (
              <Loader2 data-icon="inline-start" className="animate-spin" />
            ) : (
              <Trash2 data-icon="inline-start" />
            )}
            {t.server.deleteConfirm}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
