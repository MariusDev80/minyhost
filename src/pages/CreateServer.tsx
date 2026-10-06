import { useState } from "react";
import { AlertCircle, ArrowLeft, ExternalLink } from "lucide-react";
import { toast } from "sonner";
import { EmptyState } from "@/components/layout/EmptyState";
import { PageHeader } from "@/components/layout/PageHeader";
import { CreationProgress } from "@/components/server/CreationProgress";
import { loaders } from "@/components/server/loaders";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Checkbox } from "@/components/ui/checkbox";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Skeleton } from "@/components/ui/skeleton";
import { useCreateServer } from "@/hooks/useServers";
import { useVersions } from "@/hooks/useVersions";
import { errorMessage } from "@/lib/errors";
import { openExternal } from "@/lib/tauri";
import { cn } from "@/lib/utils";
import { t } from "@/i18n";
import { useUiStore } from "@/stores/ui";
import type { Loader, NewServer } from "@/types";

const MEMORY_OPTIONS_MB = [2048, 3072, 4096, 6144, 8192];
const DEFAULT_MEMORY_MB = 3072;
const MAX_NAME_LENGTH = 40; // Same limit as Rust (`core/create.rs`).
const EULA_URL = "https://aka.ms/MinecraftEULA";

/**
 * Three states, driven by the creation mutation:
 * form -> progress (pending) -> server page (success) or error card.
 */
export function CreateServerPage() {
  const navigate = useUiStore((state) => state.navigate);
  const create = useCreateServer();

  const submit = (input: NewServer) =>
    create.mutate(input, {
      onSuccess: (instance) => {
        toast.success(t.create.success(instance.name));
        navigate({ name: "server", id: instance.id });
      },
    });

  return (
    <div className="space-y-8">
      <div className="space-y-4">
        <Button
          variant="ghost"
          size="sm"
          disabled={create.isPending}
          onClick={() => navigate({ name: "servers" })}
        >
          <ArrowLeft data-icon="inline-start" />
          {t.server.back}
        </Button>
        <PageHeader title={t.create.title} description={t.create.description} />
      </div>

      {create.isPending ? (
        <CreationProgress />
      ) : create.isError ? (
        <EmptyState
          icon={AlertCircle}
          title={t.create.failed}
          description={errorMessage(create.error)}
          action={
            <div className="flex gap-2">
              <Button variant="outline" onClick={() => create.reset()}>
                {t.actions.edit}
              </Button>
              <Button
                onClick={() => create.variables && submit(create.variables)}
              >
                {t.actions.retry}
              </Button>
            </div>
          }
        />
      ) : (
        // `create.variables` restores the form after "Modifier".
        <CreateServerForm initial={create.variables} onSubmit={submit} />
      )}
    </div>
  );
}

function CreateServerForm({
  initial,
  onSubmit,
}: {
  initial?: NewServer;
  onSubmit: (input: NewServer) => void;
}) {
  const [name, setName] = useState(initial?.name ?? "");
  const [loader, setLoader] = useState<Loader>(initial?.loader ?? "paper");
  const [pickedVersion, setPickedVersion] = useState(initial?.mcVersion);
  const [memoryMb, setMemoryMb] = useState(
    initial?.memoryMb ?? DEFAULT_MEMORY_MB,
  );
  const [eulaAccepted, setEulaAccepted] = useState(false);

  const versions = useVersions(loader);
  // Latest version by default, or when the picked one does not exist for this type.
  const mcVersion =
    pickedVersion && versions.data?.includes(pickedVersion)
      ? pickedVersion
      : versions.data?.[0];

  const canSubmit = name.trim() !== "" && mcVersion && eulaAccepted;

  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        if (!canSubmit) return;
        onSubmit({
          name: name.trim(),
          loader,
          mcVersion,
          memoryMb,
          eulaAccepted,
        });
      }}
    >
      <Card>
        <CardContent className="space-y-6">
          <Field label={t.create.name} htmlFor="server-name">
            <Input
              id="server-name"
              value={name}
              maxLength={MAX_NAME_LENGTH}
              placeholder={t.create.namePlaceholder}
              onChange={(e) => setName(e.target.value)}
              autoFocus
            />
          </Field>

          <Field label={t.create.type}>
            <div
              role="radiogroup"
              aria-label={t.create.type}
              className="grid grid-cols-3 gap-2"
            >
              {loaders.map(({ value, icon: Icon, name, description }) => (
                <button
                  key={value}
                  type="button"
                  role="radio"
                  aria-checked={loader === value}
                  onClick={() => setLoader(value)}
                  className={cn(
                    "flex flex-col items-start gap-1 rounded-lg border p-3 text-left transition-colors outline-none hover:bg-muted focus-visible:ring-3 focus-visible:ring-ring/50",
                    loader === value &&
                      "border-primary bg-accent hover:bg-accent [&_svg]:text-primary",
                  )}
                >
                  <span className="flex items-center gap-2 text-sm font-medium">
                    <Icon className="size-4" />
                    {name}
                  </span>
                  <span className="text-xs text-muted-foreground">
                    {description}
                  </span>
                </button>
              ))}
            </div>
          </Field>

          <Field label={t.create.version} htmlFor="server-version">
            {versions.isPending ? (
              <Skeleton className="h-8 w-48 rounded-lg" />
            ) : versions.isError ? (
              <div className="flex items-center gap-3 text-sm text-destructive">
                {t.create.versionsError}
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => void versions.refetch()}
                >
                  {t.actions.retry}
                </Button>
              </div>
            ) : (
              <Select value={mcVersion} onValueChange={setPickedVersion}>
                <SelectTrigger id="server-version" className="w-48">
                  <SelectValue placeholder={t.create.versionPlaceholder} />
                </SelectTrigger>
                <SelectContent>
                  {versions.data.map((version) => (
                    <SelectItem key={version} value={version}>
                      {version}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            )}
          </Field>

          <Field label={t.create.memory} hint={t.create.memoryHint}>
            <div
              role="radiogroup"
              aria-label={t.create.memory}
              className="flex flex-wrap gap-2"
            >
              {MEMORY_OPTIONS_MB.map((mb) => (
                <Button
                  key={mb}
                  type="button"
                  role="radio"
                  aria-checked={memoryMb === mb}
                  variant={memoryMb === mb ? "default" : "outline"}
                  onClick={() => setMemoryMb(mb)}
                >
                  {t.units.memory(mb)}
                </Button>
              ))}
            </div>
          </Field>

          <div className="space-y-1 rounded-lg border p-3">
            <div className="flex items-center gap-3">
              <Checkbox
                id="server-eula"
                checked={eulaAccepted}
                onCheckedChange={(checked) => setEulaAccepted(checked === true)}
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
            <p className="pl-7 text-xs text-muted-foreground">
              {t.create.eulaHint}
            </p>
          </div>

          <div className="flex justify-end">
            <Button type="submit" size="lg" disabled={!canSubmit}>
              {t.create.submit}
            </Button>
          </div>
        </CardContent>
      </Card>
    </form>
  );
}

function Field({
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
