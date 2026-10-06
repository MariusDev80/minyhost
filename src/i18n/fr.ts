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
    start: "Démarrer",
    stop: "Arrêter",
    delete: "Supprimer",
    cancel: "Annuler",
    retry: "Réessayer",
    edit: "Modifier",
    send: "Envoyer",
  },
  loaders: {
    vanilla: {
      name: "Vanilla",
      description: "Le Minecraft officiel, sans modification.",
    },
    paper: {
      name: "Paper",
      description: "Plus rapide, et compatible avec les plugins.",
    },
    fabric: {
      name: "Fabric",
      description: "Pour jouer avec des mods.",
    },
  },
  units: {
    memory: (mb: number) => `${mb / 1024} Go`,
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
    loadError: "Impossible de charger la liste des serveurs.",
  },
  create: {
    title: "Créer un serveur",
    description:
      "Choisis le type, la version et la mémoire. MinyHost s'occupe du reste.",
    name: "Nom du serveur",
    namePlaceholder: "Survie entre potes",
    type: "Type de serveur",
    version: "Version de Minecraft",
    versionPlaceholder: "Choisis une version",
    versionsError: "Impossible de charger les versions. Vérifie ta connexion.",
    memory: "Mémoire (RAM)",
    memoryHint:
      "3 Go suffisent pour 2 à 10 joueurs. Garde assez de mémoire pour ton jeu.",
    eula: "J'accepte le contrat de licence de Minecraft (EULA).",
    eulaLink: "Lire l'EULA",
    eulaHint: "Obligatoire pour héberger un serveur Minecraft.",
    submit: "Créer le serveur",
    progressTitle: "Création du serveur",
    progressHint: "La première fois, cela peut prendre quelques minutes.",
    steps: {
      preparing: "Préparation",
      downloadingJava: "Téléchargement de Java",
      downloadingServer: "Téléchargement du serveur",
      finalizing: "Finalisation",
    },
    failed: "La création a échoué",
    success: (name: string) => `« ${name} » est prêt !`,
  },
  server: {
    back: "Serveurs",
    notFound: "Ce serveur n'existe plus.",
    address: "Adresse (depuis ce PC)",
    version: "Version",
    memory: "Mémoire",
    java: "Java",
    console: "Console",
    consoleEmpty: "Démarre le serveur pour voir sa console ici.",
    commandPlaceholder: "Tape une commande, par ex. « list »",
    deleteTitle: (name: string) => `Supprimer « ${name} » ?`,
    deleteDescription:
      "Le monde et tous les fichiers de ce serveur seront définitivement supprimés.",
    deleteConfirm: "Supprimer définitivement",
    deleted: (name: string) => `« ${name} » a été supprimé.`,
  },
  whitelist: {
    tab: "Whitelist",
    title: "Joueurs autorisés",
    description:
      "Seuls ces joueurs peuvent rejoindre le serveur. Utilise leur pseudo Minecraft exact.",
    placeholder: "Pseudo Minecraft",
    add: "Ajouter",
    remove: (name: string) => `Retirer ${name}`,
    invalidName: "3 à 16 caractères : lettres, chiffres ou _",
    empty: "Personne n'est encore autorisé. Commence par t'ajouter !",
    added: (name: string) => `${name} peut maintenant rejoindre le serveur.`,
    removed: (name: string) => `${name} a été retiré de la whitelist.`,
    already: (name: string) => `${name} est déjà dans la whitelist.`,
  },
  notifications: {
    killed:
      "Le serveur ne répondait plus : il a été arrêté de force. Le monde a pu être endommagé.",
    crashed: (name: string) => `« ${name} » s'est arrêté de façon inattendue.`,
    crashedHint: "Regarde la console pour voir ce qui s'est passé.",
  },
  closing: {
    title: "Arrêt des serveurs…",
    description: "MinyHost sauvegarde les mondes avant de se fermer.",
  },
  // One message per `AppErrorCode` (see `src/types/index.ts`).
  errors: {
    network:
      "Impossible de joindre Internet. Vérifie ta connexion, puis réessaie.",
    io: "Impossible de lire ou d'écrire des fichiers sur ce PC. Vérifie l'espace disque disponible.",
    invalidData: "Un fichier reçu est invalide. Réessaie dans un instant.",
    hashMismatch: "Un fichier téléchargé est abîmé. Réessaie.",
    serverNotFound: "Ce serveur est introuvable. Il a peut-être été supprimé.",
    versionUnavailable:
      "Cette version n'est pas disponible pour ce type de serveur.",
    javaUnavailable: "Java n'est pas disponible pour ton système.",
    portInUse:
      "Le port de ce serveur est déjà utilisé. Un autre serveur tourne peut-être déjà sur ce PC.",
    alreadyRunning: "Ce serveur est déjà en marche.",
    notRunning: "Ce serveur n'est pas démarré.",
    eulaNotAccepted:
      "L'EULA de Minecraft doit être acceptée pour lancer un serveur.",
    missingFiles:
      "Des fichiers de ce serveur sont manquants. Supprime-le puis recrée-le.",
    invalidInput: "Les informations saisies ne sont pas valides.",
    invalidPlayerName:
      "Ce pseudo n'est pas valide : 3 à 16 caractères, lettres, chiffres ou _.",
    playerNotFound:
      "Aucun compte Minecraft ne porte ce pseudo. Vérifie l'orthographe.",
    unknown: "Une erreur inattendue est survenue.",
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
