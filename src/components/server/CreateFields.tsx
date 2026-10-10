// Fields shared by the two creation forms (blank server, modpack).
import { ExternalLink } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Label } from "@/components/ui/label";
import { t } from "@/i18n";
import { openExternal } from "@/lib/tauri";

const EULA_URL = "https://aka.ms/MinecraftEULA";

export function Field({
  label,
  htmlFor,
  hint,
  children,
}: {
  label: string;
  htmlFor?: string;
  hint?: string;
  children: React.ReactNode;
}) {
  return (
    <div className="space-y-2">
      <Label htmlFor={htmlFor}>{label}</Label>
      {children}
      {hint && <p className="text-xs text-muted-foreground">{hint}</p>}
    </div>
  );
}

export function MemoryField({
  options,
  value,
  onChange,
  hint,
}: {
  options: number[];
  value: number;
  onChange: (mb: number) => void;
  hint: string;
}) {
  return (
    <Field label={t.create.memory} hint={hint}>
      <div
        role="radiogroup"
        aria-label={t.create.memory}
        className="flex flex-wrap gap-2"
      >
        {options.map((mb) => (
          <Button
            key={mb}
            type="button"
            role="radio"
            aria-checked={value === mb}
            variant={value === mb ? "default" : "outline"}
            onClick={() => onChange(mb)}
          >
            {t.units.memory(mb)}
          </Button>
        ))}
      </div>
    </Field>
  );
}

/** Mojang's EULA, accepted by the user only (never on their behalf). */
export function EulaField({
  checked,
  onChange,
}: {
  checked: boolean;
  onChange: (checked: boolean) => void;
}) {
  return (
    <div className="space-y-1 rounded-lg border p-3">
      <div className="flex items-center gap-3">
        <Checkbox
          id="server-eula"
          checked={checked}
          onCheckedChange={(value) => onChange(value === true)}
        />
        <Label htmlFor="server-eula">{t.create.eula}</Label>
        <Button
          type="button"
          variant="link"
          size="sm"
          className="ml-auto"
          onClick={() => void openExternal(EULA_URL)}
        >
          {t.create.eulaLink}
          <ExternalLink data-icon="inline-end" />
        </Button>
      </div>
      <p className="pl-7 text-xs text-muted-foreground">{t.create.eulaHint}</p>
    </div>
  );
}
