import { useState } from "react";
import { ExternalLink } from "lucide-react";
import { ContentBrowser } from "@/components/content/ContentBrowser";
import { ContentIcon } from "@/components/content/ContentIcon";
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
import { useModpackVersions } from "@/hooks/useContent";
import { t } from "@/i18n";
import { openExternal } from "@/lib/tauri";
import type {
  ContentHit,
  Loader,
  ModpackVersion,
  NewModpackServer,
} from "@/types";
import { EulaField, Field, MemoryField } from "./CreateFields";

const MEMORY_OPTIONS_MB = [4096, 6144, 8192, 10240, 12288];
const DEFAULT_MEMORY_MB = 6144;
const MAX_NAME_LENGTH = 40; // Same limit as Rust (`core/create.rs`).

/** What the form submits, plus what the page needs to show the progress. */
export interface ModpackDraft {
  hit: ContentHit;
  input: NewModpackServer;
  version: ModpackVersion;
}

/** Pick a modpack (search), then its version, name and memory. */
export function ModpackForm({
  initial,
  onSubmit,
}: {
  initial?: ModpackDraft;
  onSubmit: (draft: ModpackDraft) => void;
}) {
  const [hit, setHit] = useState(initial?.hit);

  if (!hit) {
    return (
      <ContentBrowser
        kinds={["modpack"]}
        serverId={null}
        action={(result) => (
          <Button onClick={() => setHit(result)}>{t.create.pick}</Button>
        )}
      />
    );
  }
  return (
    <ModpackSettings
      // A new pack starts from a fresh form.
      key={`${hit.source}/${hit.projectId}`}
      hit={hit}
      initial={initial?.hit === hit ? initial : undefined}
      onChange={() => setHit(undefined)}
      onSubmit={onSubmit}
    />
  );
}

function ModpackSettings({
  hit,
  initial,
  onChange,
  onSubmit,
}: {
  hit: ContentHit;
  initial?: ModpackDraft;
  onChange: () => void;
  onSubmit: (draft: ModpackDraft) => void;
}) {
  const versions = useModpackVersions(hit.source, hit.projectId);
  const [pickedVersion, setPickedVersion] = useState(initial?.input.versionId);
  const [name, setName] = useState(
    initial?.input.name ?? hit.title.slice(0, MAX_NAME_LENGTH),
  );
  const [memoryMb, setMemoryMb] = useState(
    initial?.input.memoryMb ?? DEFAULT_MEMORY_MB,
  );
  const [eulaAccepted, setEulaAccepted] = useState(false);

  // Latest supported version by default.
  const version =
    versions.data?.find((v) => v.id === pickedVersion && v.supported) ??
    versions.data?.find((v) => v.supported);
  const canSubmit = name.trim() !== "" && version && eulaAccepted;

  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        if (!canSubmit) return;
        onSubmit({
          hit,
          version,
          input: {
            name: name.trim(),
            memoryMb,
            eulaAccepted,
            source: hit.source,
            projectId: hit.projectId,
            versionId: version.id,
          },
        });
      }}
    >
      <Card>
        <CardContent className="space-y-6">
          <div className="flex items-center gap-3">
            <ContentIcon url={hit.iconUrl} className="size-12" />
            <div className="min-w-0 flex-1">
              <p className="truncate font-medium">{hit.title}</p>
              <p className="truncate text-sm text-muted-foreground">
                {t.content.by(hit.author)} · {t.content.sources[hit.source]}
              </p>
            </div>
            <Button
              type="button"
              variant="ghost"
              size="icon-sm"
              aria-label={t.content.openPage}
              title={t.content.openPage}
              onClick={() => void openExternal(hit.pageUrl)}
            >
              <ExternalLink />
            </Button>
            <Button type="button" variant="outline" onClick={onChange}>
              {t.create.changeModpack}
            </Button>
          </div>

          <Field label={t.create.modpackVersion} htmlFor="modpack-version">
            {versions.isPending ? (
              <Skeleton className="h-8 w-72 rounded-lg" />
            ) : versions.isError ? (
              <div className="flex items-center gap-3 text-sm text-destructive">
                {t.create.versionsError}
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  onClick={() => void versions.refetch()}
                >
                  {t.actions.retry}
                </Button>
              </div>
            ) : !version ? (
              <p className="text-sm text-destructive">
                {t.create.modpackUnsupported}
              </p>
            ) : (
              <Select value={version.id} onValueChange={setPickedVersion}>
                <SelectTrigger id="modpack-version" className="w-full max-w-md">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {versions.data.map((v) => (
                    <SelectItem key={v.id} value={v.id} disabled={!v.supported}>
                      {versionLabel(v)}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            )}
          </Field>

          <Field label={t.create.name} htmlFor="server-name">
            <Input
              id="server-name"
              value={name}
              maxLength={MAX_NAME_LENGTH}
              placeholder={t.create.namePlaceholder}
              onChange={(e) => setName(e.target.value)}
            />
          </Field>

          <MemoryField
            options={MEMORY_OPTIONS_MB}
            value={memoryMb}
            onChange={setMemoryMb}
            hint={t.create.modpackMemoryHint}
          />

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

/** "Pack 1.2 — 1.21.1 · NeoForge" (+ "not supported" for Quilt…). */
function versionLabel(version: ModpackVersion): string {
  const details = [
    version.mcVersions.slice(0, 3).join(", "),
    version.loaders.map(loaderName).join(", "),
  ].filter(Boolean);
  const label = `${version.name} — ${details.join(" · ")}`;
  return version.supported ? label : `${label} (${t.create.notSupported})`;
}

/** "neoforge" -> "NeoForge"; loaders MinyHost does not host keep their name. */
function loaderName(loader: string): string {
  return loader in t.loaders
    ? t.loaders[loader as Loader].name
    : loader.charAt(0).toUpperCase() + loader.slice(1);
}
