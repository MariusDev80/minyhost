import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { AppLayout } from "@/components/layout/AppLayout";
import { Toaster } from "@/components/ui/sonner";
import { TooltipProvider } from "@/components/ui/tooltip";
import { useApplyTheme } from "@/hooks/useTheme";
import { DesignSystemPage } from "@/pages/DesignSystem";
import { HomePage } from "@/pages/Home";
import { ServersPage } from "@/pages/Servers";
import { SettingsPage } from "@/pages/Settings";
import { useUiStore, type Route } from "@/stores/ui";

const queryClient = new QueryClient();

function CurrentPage({ route }: { route: Route }) {
  switch (route.name) {
    case "home":
      return <HomePage />;
    case "servers":
      return <ServersPage />;
    case "settings":
      return <SettingsPage />;
    case "design-system":
      return <DesignSystemPage />;
  }
}

function App() {
  const theme = useApplyTheme();
  const route = useUiStore((state) => state.route);

  return (
    <QueryClientProvider client={queryClient}>
      <TooltipProvider>
        <AppLayout>
          <CurrentPage route={route} />
        </AppLayout>
        <Toaster theme={theme} />
      </TooltipProvider>
    </QueryClientProvider>
  );
}

export default App;
