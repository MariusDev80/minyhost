import { useState } from "react";
import {
  AlertCircle,
  ArrowLeft,
  ChevronDown,
  SlidersHorizontal,
} from "lucide-react";
import { toast } from "sonner";
import { ManualDownloadsDialog } from "@/components/content/ManualDownloadsDialog";
import { EmptyState } from "@/components/layout/EmptyState";
import { PageHeader } from "@/components/layout/PageHeader";
import {
  EulaField,
  Field,
  MemoryField,
} from "@/components/server/CreateFields";
import { CreationProgress } from "@/components/server/CreationProgress";
import {
  loaderList,
  modpackSteps,
  serverSteps,
} from "@/components/server/loaders";
import {
  ModpackForm,
  type ModpackDraft,
} from "@/components/server/ModpackForm";
import { GameSettingsForm } from "@/components/server/settings/GameSettingsForm";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Skeleton } from "@/components/ui/skeleton";
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useCreateFromModpack, useCreateServer } from "@/hooks/useServers";
import { useSettingsCatalog } from "@/hooks/useGameSettings";
import { useVersions } from "@/hooks/useVersions";
import { errorMessage } from "@/lib/errors";
import { emptySettings, onlyChanged } from "@/lib/gameSettings";
import { cn } from "@/lib/utils";
import { t } from "@/i18n";
import { useUiStore } from "@/stores/ui";
import type {
  GameSettings,
  Instance,
  Loader,
  ManualDownload,
  NewServer,
} from "@/types";

const MEMORY_OPTIONS_MB = [2048, 3072, 4096, 6144, 8192];
const DEFAULT_MEMORY_MB = 3072;
const MAX_NAME_LENGTH = 40; // Same limit as Rust (`core/create.rs`).

type Mode = "blank" | "modpack";

/**
 * Two ways to create a server: the form (type, version…) or a modpack.
 * Each is driven by its mutation: form -> progress (pending) -> server page
 * (success) or error card.
 */
export function CreateServerPage() {
  const navigate = useUiStore((state) => state.navigate);
  const [mode, setMode] = useState<Mode>("blank");
  const create = useCreateServer();
  const createPack = useCreateFromModpack();
  const [draft, setDraft] = useState<ModpackDraft>();
  // Files to download by hand once a CurseForge pack is installed.
  const [manual, setManual] = useState<{
    instance: Instance;
    downloads: ManualDownload[];
  }>();
  const creation = mode === "blank" ? create : createPack;

  const created = (instance: Instance) => {
    toast.success(t.create.success(instance.name));
    navigate({ name: "server", id: instance.id });
  };
  const submit = (input: NewServer) =>
    create.mutate(input, { onSuccess: created });
  const submitPack = (pack: ModpackDraft) => {
    setDraft(pack);
    createPack.mutate(pack.input, {
      onSuccess: ({ instance, manual }) =>
        manual.length > 0
          ? setManual({ instance, downloads: manual })
          : created(instance),
    });
  };

  return (
    <div className="space-y-8">
      <div className="space-y-4">
        <Button
          variant="ghost"
          size="sm"
          disabled={creation.isPending}
          onClick={() => navigate({ name: "servers" })}
        >
          <ArrowLeft data-icon="inline-start" />
          {t.server.back}
        </Button>
        <PageHeader title={t.create.title} description={t.create.description} />
      </div>

      {creation.isPending ? (
        <CreationProgress
          steps={
            mode === "blank"
              ? serverSteps(create.variables?.loader ?? "paper")
              : modpackSteps(draft?.version.loaders ?? [])
          }
        />
      ) : creation.isError ? (
        <EmptyState
          icon={AlertCircle}
          title={t.create.failed}
          description={errorMessage(creation.error)}
          action={
            <div className="flex gap-2">
              <Button variant="outline" onClick={() => creation.reset()}>
                {t.actions.edit}
              </Button>
              <Button
                onClick={() =>
                  mode === "blank"
                    ? create.variables && submit(create.variables)
                    : draft && submitPack(draft)
                }
              >
                {t.actions.retry}
              </Button>
            </div>
          }
        />
      ) : (
        <div className="space-y-4">
          <Tabs value={mode} onValueChange={(value) => setMode(value as Mode)}>
            <TabsList>
              <TabsTrigger value="blank">{t.create.modes.blank}</TabsTrigger>
              <TabsTrigger value="modpack">
                {t.create.modes.modpack}
              </TabsTrigger>
            </TabsList>
          </Tabs>
          {mode === "blank" ? (
            // `create.variables` restores the form after "Modifier".
            <CreateServerForm initial={create.variables} onSubmit={submit} />
          ) : (
            <ModpackForm initial={draft} onSubmit={submitPack} />
          )}
        </div>
      )}

      {manual && (
        <ManualDownloadsDialog
          serverId={manual.instance.id}
          folder={t.content.folders.mod}
          downloads={manual.downloads}
          onClose={() => created(manual.instance)}
        />
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
  const [settings, setSettings] = useState<GameSettings>(
    initial?.settings ?? emptySettings,
  );
  const [showSettings, setShowSettings] = useState(false);

  const versions = useVersions(loader);
  // Latest version by default, or when the picked one does not exist for this type.
  const mcVersion =
    pickedVersion && versions.data?.includes(pickedVersion)
      ? pickedVersion
      : versions.data?.[0];
  const catalog = useSettingsCatalog(mcVersion);

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
          // Only what the user changed; the rest keeps Minecraft's defaults.
          settings: catalog.data && onlyChanged(catalog.data, settings),
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
              {loaderList().map(({ value, icon: Icon, name, description }) => (
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

          <MemoryField
            options={MEMORY_OPTIONS_MB}
            value={memoryMb}
            onChange={setMemoryMb}
            hint={t.create.memoryHint}
          />

          <div className="rounded-lg border">
            <button
              type="button"
              aria-expanded={showSettings}
              onClick={() => setShowSettings(!showSettings)}
              className="flex w-full items-center gap-3 rounded-lg p-3 text-left transition-colors outline-none hover:bg-muted/50 focus-visible:ring-3 focus-visible:ring-ring/50"
            >
              <SlidersHorizontal className="size-4 shrink-0 text-primary" />
              <span className="flex-1">
                <span className="block text-sm font-medium">
                  {t.gameSettings.creationToggle}
                </span>
                <span className="block text-xs text-muted-foreground">
                  {t.gameSettings.creationHint}
                </span>
              </span>
              <ChevronDown
                className={cn(
                  "size-4 shrink-0 transition-transform",
                  showSettings && "rotate-180",
                )}
              />
            </button>
            {showSettings && (
              <div className="border-t p-3">
                {catalog.data ? (
                  <GameSettingsForm
                    catalog={catalog.data}
                    value={settings}
                    onChange={setSettings}
                    mode="create"
                  />
                ) : (
                  <Skeleton className="h-32 rounded-lg" />
                )}
              </div>
            )}
          </div>

          <EulaField checked={eulaAccepted} onChange={setEulaAccepted} />

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
