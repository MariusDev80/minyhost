// Dev-only page listing the design system building blocks. Not shipped in
// production builds (hidden from the sidebar), so its sample texts are not
// localized.
import { useState } from "react";
import { Play, Plus, Square, Trash2 } from "lucide-react";
import { toast } from "sonner";
import { PageHeader } from "@/components/layout/PageHeader";
import { StatusBadge } from "@/components/server/StatusBadge";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Progress } from "@/components/ui/progress";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Skeleton } from "@/components/ui/skeleton";
import { Switch } from "@/components/ui/switch";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

const swatches = [
  "bg-background",
  "bg-card",
  "bg-muted",
  "bg-accent",
  "bg-primary",
  "bg-success",
  "bg-warning",
  "bg-destructive",
  "bg-console",
];

function Section({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>{title}</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-wrap items-center gap-3">
        {children}
      </CardContent>
    </Card>
  );
}

export function DesignSystemPage() {
  const [progress, setProgress] = useState(42);

  return (
    <div className="space-y-6">
      <PageHeader
        title="Design system"
        description="Aperçu des composants de base (visible en dev uniquement)."
      />

      <Section title="Couleurs">
        {swatches.map((swatch) => (
          <div key={swatch} className="flex flex-col items-center gap-1.5">
            <div className={`size-12 rounded-lg border ${swatch}`} />
            <span className="text-xs text-muted-foreground">
              {swatch.replace("bg-", "")}
            </span>
          </div>
        ))}
      </Section>

      <Section title="Typographie">
        <div className="space-y-1">
          <p className="text-2xl font-semibold tracking-tight">Titre de page</p>
          <p className="text-base font-medium">Titre de carte</p>
          <p className="text-sm">Texte courant</p>
          <p className="text-sm text-muted-foreground">Texte secondaire</p>
          <p className="font-mono text-xs">
            [12:00:00] [Server thread/INFO]: Done (3.2s)! For help, type "help"
          </p>
        </div>
      </Section>

      <Section title="Boutons">
        <Button>
          <Play data-icon="inline-start" />
          Démarrer
        </Button>
        <Button variant="secondary">
          <Square data-icon="inline-start" />
          Arrêter
        </Button>
        <Button variant="outline">
          <Plus data-icon="inline-start" />
          Ajouter
        </Button>
        <Button variant="ghost">Annuler</Button>
        <Button variant="destructive">
          <Trash2 data-icon="inline-start" />
          Supprimer
        </Button>
        <Button disabled>Désactivé</Button>
      </Section>

      <Section title="Statuts et badges">
        <StatusBadge status="running" />
        <StatusBadge status="starting" />
        <StatusBadge status="stopping" />
        <StatusBadge status="stopped" />
        <Badge>Paper</Badge>
        <Badge variant="secondary">1.21.1</Badge>
        <Badge variant="outline">4 Go</Badge>
      </Section>

      <Section title="Formulaires">
        <div className="grid w-64 gap-2">
          <Label htmlFor="ds-name">Nom du serveur</Label>
          <Input id="ds-name" placeholder="Survie entre potes" />
        </div>
        <div className="grid w-48 gap-2">
          <Label>Type</Label>
          <Select defaultValue="paper">
            <SelectTrigger className="w-full">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="vanilla">Vanilla</SelectItem>
              <SelectItem value="paper">Paper</SelectItem>
              <SelectItem value="fabric">Fabric</SelectItem>
            </SelectContent>
          </Select>
        </div>
        <div className="flex items-center gap-2">
          <Switch id="ds-whitelist" defaultChecked />
          <Label htmlFor="ds-whitelist">Whitelist</Label>
        </div>
      </Section>

      <Section title="Progression et chargement">
        <div className="w-full space-y-2">
          <div className="flex justify-between text-sm">
            <span>Téléchargement de Java…</span>
            <span className="text-muted-foreground">{progress} %</span>
          </div>
          <Progress value={progress} />
          <Button
            variant="outline"
            size="sm"
            onClick={() => setProgress((p) => (p + 17) % 101)}
          >
            Avancer
          </Button>
        </div>
        <div className="flex w-full gap-3">
          <Skeleton className="h-20 flex-1 rounded-xl" />
          <Skeleton className="h-20 flex-1 rounded-xl" />
        </div>
      </Section>

      <Section title="Onglets, console et notifications">
        <Tabs defaultValue="console" className="w-full">
          <TabsList>
            <TabsTrigger value="console">Console</TabsTrigger>
            <TabsTrigger value="players">Joueurs</TabsTrigger>
          </TabsList>
          <TabsContent value="console">
            <div
              data-selectable
              className="rounded-lg bg-console p-3 font-mono text-xs text-console-foreground"
            >
              <p>[12:00:00] [Server thread/INFO]: Starting minecraft server</p>
              <p className="text-warning">
                [12:00:01] [Server thread/WARN]: Exemple d'avertissement
              </p>
              <p className="text-destructive">
                [12:00:02] [Server thread/ERROR]: Exemple d'erreur
              </p>
            </div>
          </TabsContent>
          <TabsContent value="players">
            <CardDescription>Aucun joueur connecté.</CardDescription>
          </TabsContent>
        </Tabs>
        <Button
          variant="outline"
          onClick={() => toast.success("Serveur démarré")}
        >
          Toast succès
        </Button>
        <Button
          variant="outline"
          onClick={() =>
            toast.error("Le port 25565 est déjà utilisé", {
              description: "Ferme l'autre serveur ou choisis un autre port.",
            })
          }
        >
          Toast erreur
        </Button>
      </Section>
    </div>
  );
}
