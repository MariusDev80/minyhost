// French UI strings: the reference locale. Every user-facing text lives here;
// other locales (`en.ts`…) must provide the same keys.
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
    restart: "Redémarrer",
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
  // Shared by the whitelist and operators tabs.
  players: {
    placeholder: "Pseudo Minecraft",
    add: "Ajouter",
    remove: (name: string) => `Retirer ${name}`,
    invalidName: "3 à 16 caractères : lettres, chiffres ou _",
  },
  whitelist: {
    tab: "Whitelist",
    title: "Joueurs autorisés",
    description:
      "Seuls ces joueurs peuvent rejoindre le serveur. Utilise leur pseudo Minecraft exact.",
    empty: "Personne n'est encore autorisé. Commence par t'ajouter !",
    added: (name: string) => `${name} peut maintenant rejoindre le serveur.`,
    removed: (name: string) => `${name} a été retiré de la whitelist.`,
    already: (name: string) => `${name} est déjà dans la whitelist.`,
  },
  operators: {
    tab: "Opérateurs",
    title: "Opérateurs",
    description:
      "Les opérateurs peuvent utiliser toutes les commandes (/gamemode, /tp, /ban…) et rejoindre même sans être dans la whitelist. N'ajoute que des personnes de confiance.",
    empty:
      "Aucun opérateur. Ajoute-toi pour pouvoir utiliser les commandes en jeu.",
    added: (name: string) => `${name} est maintenant opérateur.`,
    removed: (name: string) => `${name} n'est plus opérateur.`,
    already: (name: string) => `${name} est déjà opérateur.`,
  },
  restart: {
    title: "Redémarrage nécessaire",
    description:
      "Des paramètres ont changé pendant que le serveur tournait. Redémarre-le pour les appliquer.",
  },
  gameSettings: {
    tab: "Paramètres",
    world: "Monde et serveur",
    worldDescription:
      "Enregistrés dans server.properties. Appliqués au prochain démarrage du serveur.",
    rules: "Règles du jeu",
    rulesDescription:
      "Valeurs lues dans le monde, y compris celles changées en jeu. Serveur allumé : appliquées immédiatement. Serveur arrêté : appliquées à son prochain démarrage.",
    unsupported:
      "Les règles du jeu ne sont pas gérées pour les versions antérieures à 1.13.",
    search: "Rechercher un réglage…",
    noResult: "Aucun réglage ne correspond à ta recherche.",
    reset: "Remettre la valeur par défaut",
    defaultValue: (value: string) => `Par défaut : ${value}`,
    on: "activé",
    off: "désactivé",
    unsaved: "Modifications non enregistrées",
    discard: "Annuler",
    save: "Enregistrer",
    saved: "Paramètres enregistrés.",
    savedNeedsRestart:
      "Paramètres enregistrés. Redémarre le serveur pour appliquer les réglages du monde.",
    loadError: "Impossible de lire les paramètres de ce serveur.",
    creationToggle: "Personnaliser le monde et les règles du jeu",
    creationHint: "Facultatif : tout reste modifiable plus tard.",
    categories: {
      players: "Joueurs",
      mobs: "Créatures",
      world: "Monde",
      drops: "Objets et butin",
      commands: "Commandes et technique",
    },
    choices: {
      difficulty: {
        peaceful: "Paisible",
        easy: "Facile",
        normal: "Normale",
        hard: "Difficile",
      },
      gamemode: {
        survival: "Survie",
        creative: "Créatif",
        adventure: "Aventure",
        spectator: "Spectateur",
      },
    },
    // One entry per setting key (see `core/game_settings.rs` and `core/game_rules.rs`).
    // A setting without text here is shown with its technical name.
    labels: {
      // server.properties
      motd: {
        label: "Message du serveur",
        description:
          "Texte affiché sous le nom du serveur dans la liste multijoueur.",
      },
      difficulty: { label: "Difficulté" },
      gamemode: {
        label: "Mode de jeu",
        description: "Mode de jeu des joueurs qui rejoignent le serveur.",
      },
      hardcore: {
        label: "Hardcore",
        description: "Une seule vie : à sa mort, le joueur devient spectateur.",
      },
      "max-players": { label: "Joueurs maximum" },
      "view-distance": {
        label: "Distance d'affichage (chunks)",
        description:
          "Nombre de chunks envoyés autour de chaque joueur. Plus c'est haut, plus le serveur travaille.",
      },
      "simulation-distance": {
        label: "Distance de simulation (chunks)",
        description:
          "Rayon en chunks où le monde reste actif : créatures, cultures, redstone.",
      },
      "spawn-protection": {
        label: "Protection du spawn (blocs)",
        description:
          "Rayon autour du point d'apparition où seuls les opérateurs peuvent construire. 0 pour désactiver.",
      },
      "allow-flight": {
        label: "Autoriser le vol",
        description:
          "Évite que les joueurs soient expulsés pour vol (utile avec certains mods ou plugins).",
      },
      pvp: {
        label: "Combats entre joueurs (PvP)",
        description: "Les joueurs peuvent se blesser entre eux.",
      },
      "spawn-monsters": {
        label: "Apparition des monstres",
        description: "Les monstres hostiles apparaissent naturellement.",
      },
      "allow-nether": {
        label: "Autoriser le Nether",
        description: "Les portails permettent d'aller dans le Nether.",
      },
      "level-seed": {
        label: "Graine du monde (seed)",
        description: "Laisse vide pour une graine aléatoire.",
      },
      "generate-structures": {
        label: "Générer les structures",
        description: "Villages, temples, forteresses…",
      },
      // Game rules: players
      keep_inventory: {
        label: "Garder l'inventaire à la mort",
        description:
          "Les joueurs gardent leurs objets et leur expérience en mourant.",
      },
      natural_health_regeneration: {
        label: "Régénération naturelle",
        description:
          "La santé remonte quand la barre de faim est bien remplie.",
      },
      immediate_respawn: {
        label: "Réapparition immédiate",
        description:
          "Pas d'écran de mort : le joueur réapparaît tout de suite.",
      },
      players_sleeping_percentage: {
        label: "Joueurs devant dormir (%)",
        description:
          "Pourcentage des joueurs qui doivent dormir pour passer la nuit. Au-delà de 100, impossible de la passer.",
      },
      respawn_radius: {
        label: "Rayon d'apparition (blocs)",
        description:
          "Zone autour du point d'apparition où les joueurs apparaissent.",
      },
      fall_damage: { label: "Dégâts de chute" },
      fire_damage: { label: "Dégâts du feu" },
      drowning_damage: { label: "Dégâts de noyade" },
      freeze_damage: {
        label: "Dégâts du gel",
        description: "Dans la neige poudreuse.",
      },
      ender_pearls_vanish_on_death: {
        label: "Perles de l'Ender perdues à la mort",
        description:
          "Les perles lancées disparaissent quand leur lanceur meurt.",
      },
      locator_bar: {
        label: "Barre de localisation",
        description:
          "Montre dans quelle direction se trouvent les autres joueurs.",
      },
      show_death_messages: {
        label: "Messages de mort",
        description: "Annonce la mort des joueurs dans le chat.",
      },
      show_advancement_messages: {
        label: "Annoncer les progrès",
        description:
          "Annonce les progrès (advancements) des joueurs dans le chat.",
      },
      limited_crafting: {
        label: "Fabrication limitée",
        description: "Seules les recettes débloquées peuvent être fabriquées.",
      },
      reduced_debug_info: {
        label: "Écran F3 réduit",
        description:
          "Masque les coordonnées et d'autres infos de l'écran de débogage.",
      },
      allow_entering_nether_using_portals: {
        label: "Autoriser le Nether",
        description: "Les portails permettent d'aller dans le Nether.",
      },
      players_nether_portal_default_delay: {
        label: "Délai des portails (ticks)",
        description:
          "Temps à passer dans un portail du Nether avant le voyage, en survie. 20 ticks = 1 seconde.",
      },
      players_nether_portal_creative_delay: {
        label: "Délai des portails en créatif (ticks)",
      },
      spectators_generate_chunks: {
        label: "Les spectateurs génèrent le monde",
        description:
          "Les joueurs en spectateur font apparaître de nouveaux chunks.",
      },
      // Game rules: mobs
      spawn_mobs: {
        label: "Apparition des créatures",
        description: "Animaux et monstres apparaissent naturellement.",
      },
      spawn_monsters: {
        label: "Apparition des monstres",
        description: "Les monstres hostiles apparaissent naturellement.",
      },
      spawn_phantoms: {
        label: "Apparition des phantoms",
        description: "Les phantoms attaquent les joueurs qui ne dorment pas.",
      },
      spawn_patrols: { label: "Patrouilles de pillards" },
      spawn_wandering_traders: { label: "Marchands ambulants" },
      spawn_wardens: {
        label: "Apparition des wardens",
        description: "Les hurleurs sculk peuvent faire apparaître le warden.",
      },
      raids: {
        label: "Raids",
        description:
          "Les pillards attaquent les villages (effet Mauvais présage).",
      },
      mob_griefing: {
        label: "Les créatures modifient le monde",
        description:
          "Creepers qui détruisent, endermen qui prennent des blocs, villageois qui récoltent…",
      },
      forgive_dead_players: {
        label: "Les créatures pardonnent aux morts",
        description:
          "Les créatures neutres en colère se calment quand leur cible meurt.",
      },
      universal_anger: {
        label: "Colère générale",
        description:
          "Une créature neutre en colère attaque tous les joueurs proches, pas seulement son agresseur.",
      },
      max_entity_cramming: {
        label: "Entassement maximum",
        description:
          "Créatures sur un même bloc avant qu'elles subissent des dégâts. 0 pour désactiver.",
      },
      spawner_blocks_work: {
        label: "Les générateurs de monstres fonctionnent",
        description: "Les spawners font apparaître des créatures.",
      },
      // Game rules: world
      advance_time: {
        label: "Cycle jour/nuit",
        description: "Le temps passe. Désactivé, l'heure reste figée.",
      },
      advance_weather: {
        label: "Cycle de la météo",
        description: "La météo change. Désactivé, elle reste figée.",
      },
      random_tick_speed: {
        label: "Vitesse des ticks aléatoires",
        description:
          "Vitesse de pousse des cultures et de l'herbe, de chute des feuilles… 0 pour tout arrêter.",
      },
      fire_spread_radius_around_player: {
        label: "Propagation du feu (rayon en blocs)",
        description:
          "Le feu se propage seulement près des joueurs. 0 = il ne se propage pas, -1 = partout.",
      },
      doFireTick: {
        label: "Propagation du feu",
        description: "Le feu se propage et s'éteint naturellement.",
      },
      allowFireTicksAwayFromPlayer: {
        label: "Feu loin des joueurs",
        description: "Le feu se propage même loin de tout joueur.",
      },
      spread_vines: { label: "Pousse des lianes" },
      water_source_conversion: {
        label: "Création de sources d'eau",
        description: "Deux sources d'eau côte à côte en forment une nouvelle.",
      },
      lava_source_conversion: {
        label: "Création de sources de lave",
        description:
          "Deux sources de lave côte à côte en forment une nouvelle.",
      },
      max_snow_accumulation_height: {
        label: "Épaisseur de neige maximum",
        description:
          "Couches de neige qui s'accumulent quand il neige (0 à 8).",
      },
      tnt_explodes: { label: "La TNT explose" },
      projectiles_can_break_blocks: {
        label: "Les projectiles cassent des blocs",
        description:
          "Certains projectiles cassent des blocs (pots décorés, chorus…).",
      },
      global_sound_events: {
        label: "Sons globaux",
        description:
          "Certains sons (wither, dragon) sont entendus par tous les joueurs.",
      },
      // Game rules: drops
      block_drops: {
        label: "Les blocs donnent des objets",
        description: "Les blocs cassés lâchent leurs objets.",
      },
      mob_drops: {
        label: "Butin des créatures",
        description: "Les créatures lâchent des objets en mourant.",
      },
      entity_drops: {
        label: "Butin des entités",
        description:
          "Wagonnets, bateaux, cadres… lâchent leurs objets quand on les casse.",
      },
      block_explosion_drop_decay: {
        label: "Perte d'objets (explosions de blocs)",
        description:
          "Une partie des blocs détruits par une explosion de bloc ne donne rien.",
      },
      mob_explosion_drop_decay: {
        label: "Perte d'objets (explosions de créatures)",
        description:
          "Une partie des blocs détruits par un creeper ne donne rien.",
      },
      tnt_explosion_drop_decay: {
        label: "Perte d'objets (TNT)",
        description: "Une partie des blocs détruits par la TNT ne donne rien.",
      },
      // Game rules: commands
      command_block_output: {
        label: "Messages des blocs de commande",
        description:
          "Les blocs de commande écrivent dans le chat des opérateurs.",
      },
      command_blocks_work: { label: "Les blocs de commande fonctionnent" },
      send_command_feedback: {
        label: "Retour des commandes",
        description: "Affiche le résultat des commandes dans le chat.",
      },
      log_admin_commands: {
        label: "Annoncer les commandes des opérateurs",
        description: "Les autres opérateurs voient les commandes utilisées.",
      },
      max_command_sequence_length: {
        label: "Longueur max. des chaînes de commandes",
        description: "Pour les fonctions et les blocs de commande en chaîne.",
      },
      max_command_forks: { label: "Branches max. des commandes" },
      max_block_modifications: {
        label: "Blocs max. modifiés par commande",
        description: "Limite de /fill, /clone…",
      },
      player_movement_check: {
        label: "Vérifier les mouvements des joueurs",
        description:
          "Anti-triche : expulse les joueurs qui se déplacent trop vite.",
      },
      elytra_movement_check: {
        label: "Vérifier les vols en élytres",
        description: "Anti-triche pour les élytres.",
      },
    },
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
    language: "Langue",
    languageDescription: "Langue de l'interface de MinyHost.",
    languageSystem: (name: string) => `Système (${name})`,
  },
} as const;
