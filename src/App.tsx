import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { AppLayout } from "@/components/layout/AppLayout";
import { Toaster } from "@/components/ui/sonner";
import { TooltipProvider } from "@/components/ui/tooltip";
import { useServerEvents } from "@/hooks/useServerEvents";
import { useApplyLocale } from "@/hooks/useLocale";
import { useApplyTheme } from "@/hooks/useTheme";
import { CreateServerPage } from "@/pages/CreateServer";
import { DesignSystemPage } from "@/pages/DesignSystem";
import { HelpPage } from "@/pages/Help";
import { HomePage } from "@/pages/Home";
import { ServerDetailPage } from "@/pages/ServerDetail";
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
    case "create-server":
      return <CreateServerPage />;
    case "server":
      // `key` resets the page state when switching from one server to another.
      return <ServerDetailPage key={route.id} id={route.id} />;
    case "settings":
      return <SettingsPage />;
    case "help":
      return <HelpPage />;
    case "design-system":
      return <DesignSystemPage />;
  }
}

/** Global listeners; needs to live inside `QueryClientProvider`. */
function EventListeners() {
  useServerEvents();
  return null;
}

function App() {
  const theme = useApplyTheme();
  const locale = useApplyLocale();
  const route = useUiStore((state) => state.route);

  return (
    <QueryClientProvider client={queryClient}>
      <EventListeners />
      {/* `key` remounts the UI on a language change, so every text (memoized
          or not) is read again from the new locale. Data and stores are kept. */}
      <TooltipProvider key={locale}>
        <AppLayout>
          <CurrentPage route={route} />
        </AppLayout>
        <Toaster theme={theme} />
      </TooltipProvider>
    </QueryClientProvider>
  );
}

export default App;
