import { useEffect, useState } from "react";
import { Check, Circle, Loader2 } from "lucide-react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Progress } from "@/components/ui/progress";
import { cn } from "@/lib/utils";
import { events } from "@/lib/tauri";
import { t } from "@/i18n";
import type { CreateProgress, CreateStep } from "@/types";

const steps: CreateStep[] = [
  "preparing",
  "downloadingJava",
  "downloadingServer",
  "finalizing",
];

/** Step list shown while Rust creates the server (`create-progress` events). */
export function CreationProgress() {
  const [progress, setProgress] = useState<CreateProgress>({
    step: "preparing",
    percent: null,
  });
  useEffect(() => events.onCreateProgress(setProgress), []);

  const current = steps.indexOf(progress.step);

  return (
    <Card>
      <CardHeader>
        <CardTitle>{t.create.progressTitle}</CardTitle>
        <p className="text-sm text-muted-foreground">{t.create.progressHint}</p>
      </CardHeader>
      <CardContent>
        <ol className="space-y-4">
          {steps.map((step, index) => {
            const done = index < current;
            const active = index === current;
            return (
              <li key={step} className="space-y-2">
                <div
                  className={cn(
                    "flex items-center gap-3 text-sm",
                    !done && !active && "text-muted-foreground",
                  )}
                >
                  {done ? (
                    <Check className="size-4 text-success" />
                  ) : active ? (
                    <Loader2 className="size-4 animate-spin text-primary" />
                  ) : (
                    <Circle className="size-4" />
                  )}
                  <span className={cn(active && "font-medium")}>
                    {t.create.steps[step]}
                  </span>
                  {active && progress.percent !== null && (
                    <span className="ml-auto text-muted-foreground">
                      {progress.percent} %
                    </span>
                  )}
                </div>
                {active && progress.percent !== null && (
                  <Progress value={progress.percent} className="ml-7 w-auto" />
                )}
              </li>
            );
          })}
        </ol>
      </CardContent>
    </Card>
  );
}
