import { Home, Palette, Server, Settings, type LucideIcon } from "lucide-react";
import { cn } from "@/lib/utils";
import { t } from "@/i18n";
import { useUiStore, type Route } from "@/stores/ui";

/** Routes reachable from the sidebar: those without parameters. */
type SectionRoute = Exclude<Route, { id: string }>["name"];

interface NavItem {
  route: SectionRoute;
  label: string;
  icon: LucideIcon;
}

// Built at render time so the labels follow the current locale.
function mainItems(): NavItem[] {
  return [
    { route: "home", label: t.nav.home, icon: Home },
    { route: "servers", label: t.nav.servers, icon: Server },
  ];
}

function bottomItems(): NavItem[] {
  return [
    // Dev-only showcase of the design system components.
    ...(import.meta.env.DEV
      ? [
          {
            route: "design-system" as const,
            label: t.nav.designSystem,
            icon: Palette,
          },
        ]
      : []),
    { route: "settings", label: t.nav.settings, icon: Settings },
  ];
}

export function Sidebar() {
  return (
    <nav className="flex w-56 shrink-0 flex-col gap-1 border-r border-sidebar-border bg-sidebar p-3">
      {mainItems().map((item) => (
        <SidebarLink key={item.route} item={item} />
      ))}
      <div className="flex-1" />
      {bottomItems().map((item) => (
        <SidebarLink key={item.route} item={item} />
      ))}
      <p className="px-3 pt-2 text-xs text-muted-foreground">
        v{__APP_VERSION__}
      </p>
    </nav>
  );
}

/** Sidebar entry to highlight: sub-pages belong to their parent section. */
function sectionOf(route: Route): SectionRoute {
  if (route.name === "server" || route.name === "create-server")
    return "servers";
  return route.name;
}

function SidebarLink({ item }: { item: NavItem }) {
  const active = useUiStore((state) => sectionOf(state.route) === item.route);
  const navigate = useUiStore((state) => state.navigate);
  const Icon = item.icon;

  return (
    <button
      type="button"
      aria-current={active ? "page" : undefined}
      onClick={() => navigate({ name: item.route })}
      className={cn(
        "flex h-9 items-center gap-3 rounded-lg px-3 text-sm font-medium text-sidebar-foreground transition-colors outline-none focus-visible:ring-2 focus-visible:ring-sidebar-ring",
        "hover:bg-sidebar-accent hover:text-sidebar-accent-foreground",
        active &&
          "bg-sidebar-accent text-sidebar-accent-foreground [&_svg]:text-sidebar-primary",
      )}
    >
      <Icon className="size-4.5" />
      {item.label}
    </button>
  );
}
