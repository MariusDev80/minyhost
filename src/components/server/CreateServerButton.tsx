import { Plus } from "lucide-react";
import { Button } from "@/components/ui/button";
import { t } from "@/i18n";
import { useUiStore } from "@/stores/ui";

export function CreateServerButton() {
  const navigate = useUiStore((state) => state.navigate);
  return (
    <Button size="lg" onClick={() => navigate({ name: "create-server" })}>
      <Plus data-icon="inline-start" />
      {t.actions.createServer}
    </Button>
  );
}
