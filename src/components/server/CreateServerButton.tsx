import { Plus } from "lucide-react";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import { t } from "@/i18n";

// Server creation arrives in phase 2; the button already has its final look.
export function CreateServerButton() {
  return (
    <Button size="lg" onClick={() => toast.info(t.actions.comingSoon)}>
      <Plus data-icon="inline-start" />
      {t.actions.createServer}
    </Button>
  );
}
