import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Server } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import {
  Card,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Toaster } from "@/components/ui/sonner";
import { TooltipProvider } from "@/components/ui/tooltip";

const queryClient = new QueryClient();

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <TooltipProvider>
        <main className="flex min-h-screen items-center justify-center p-8">
          <Card className="w-full max-w-md">
            <CardHeader>
              <div className="flex items-center gap-3">
                <Server className="size-6 text-primary" />
                <CardTitle className="text-xl">MinyHost</CardTitle>
                <Badge variant="secondary">En développement</Badge>
              </div>
              <CardDescription>
                Héberge ton serveur Minecraft sur ton PC, en quelques clics.
              </CardDescription>
            </CardHeader>
          </Card>
        </main>
        <Toaster />
      </TooltipProvider>
    </QueryClientProvider>
  );
}

export default App;
