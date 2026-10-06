// French UI strings. Every user-facing text lives here so the app can be
// translated later without touching components.
export const fr = {
  app: {
    name: "MinyHost",
    tagline: "Héberge ton serveur Minecraft sur ton PC, en quelques clics.",
  },
  titleBar: {
    minimize: "Réduire",
    maximize: "Agrandir",
    restore: "Restaurer",
    close: "Fermer",
  },
  nav: {
    home: "Accueil",
    servers: "Serveurs",
    settings: "Paramètres",
    designSystem: "Design system",
  },
  status: {
    starting: "Démarrage…",
    running: "En ligne",
    stopping: "Arrêt…",
    stopped: "Arrêté",
  },
  actions: {
    createServer: "Créer un serveur",
    comingSoon: "Bientôt disponible",
  },
  home: {
    title: "Bienvenue",
    description: "Crée un serveur, lance-le et joue avec tes amis.",
    emptyTitle: "Aucun serveur pour l'instant",
    emptyDescription:
      "Crée ton premier serveur Minecraft : Java et le serveur sont installés automatiquement.",
  },
  servers: {
    title: "Serveurs",
    description: "Tous les serveurs hébergés sur ce PC.",
    emptyTitle: "Aucun serveur",
    emptyDescription: "Les serveurs que tu crées apparaîtront ici.",
  },
  settings: {
    title: "Paramètres",
    description: "Préférences de l'application.",
    appearance: "Apparence",
    theme: "Thème",
    themeDescription: "Choisis l'apparence de MinyHost.",
    themes: {
      dark: "Sombre",
      light: "Clair",
      system: "Système",
    },
  },
} as const;
