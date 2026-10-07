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
    help: "Aide",
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
    openFolder: "Ouvrir le dossier du serveur",
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
  // Internet access through playit.gg (see `core/tunnel.rs`).
  tunnel: {
    title: "Jouer avec des amis via Internet",
    description:
      "Pour que des amis qui ne sont pas chez toi rejoignent ce serveur, MinyHost utilise playit.gg, un service de tunnel gratuit. Il te faut un compte playit.gg (gratuit lui aussi).",
    link: "Connecter playit.gg",
    linking: "Valide la connexion dans ton navigateur…",
    linkingHint:
      "Connecte-toi à playit.gg (ou crée un compte). La page propose alors de réclamer un agent « self-managed » : clique sur « Continue » puis « Claim Agent ». Cet accès permet à MinyHost de créer et supprimer lui-même le tunnel de chaque serveur, rien de plus.",
    reopen: "Rouvrir la page",
    linked:
      "Compte playit.gg connecté. Tu peux fermer la page playit.gg dans ton navigateur.",
    enableHint:
      "Ton serveur sera joignable depuis Internet. Garde la whitelist activée : seuls les joueurs autorisés pourront entrer.",
    enable: "Ouvrir sur Internet",
    disable: "Fermer l'accès Internet",
    address: "Adresse à donner à tes amis",
    copy: "Copier l'adresse",
    copied: "Adresse copiée.",
    pendingAddress:
      "playit.gg prépare l'adresse de ton serveur, cela peut prendre une minute…",
    status: {
      stopped: "Démarre le serveur pour que tes amis puissent le rejoindre.",
      connecting: "Connexion à playit.gg…",
      online: "Ouvert sur Internet.",
      error: "playit.gg est injoignable pour l'instant. Nouvel essai en cours…",
    },
    disabled: (reason: string) => `Désactivé par playit.gg : ${reason}`,
    noticeLink: "En savoir plus",
    emailUnverified: {
      title: "Adresse e-mail playit.gg à vérifier",
      steps:
        "playit.gg ne l'envoie pas automatiquement : sur playit.gg, va dans les paramètres du compte > Security > Verify email. Clique ensuite sur le lien de l'e-mail reçu. Tu ne le trouves pas ? Regarde dans tes spams.",
      open: "Ouvrir mon compte playit.gg",
    },
    agentLimit: {
      title: "Trop d'agents sur ton compte playit.gg",
      steps:
        "Chaque connexion de MinyHost crée un agent, et un compte gratuit en accepte un nombre limité. Sur playit.gg, ouvre la liste de tes agents et supprime les anciens agents « MinyHost » : garde seulement celui indiqué dans Paramètres > Accès Internet. Clique ensuite sur « Vérifier à nouveau ».",
      open: "Ouvrir mes agents playit.gg",
      recheck: "Vérifier à nouveau",
    },
    linkErrors: {
      rejected: "La connexion à playit.gg a été refusée.",
      expired: "Le délai pour connecter playit.gg est dépassé. Réessaie.",
      revoked:
        "MinyHost n'a plus accès à ton compte playit.gg. Reconnecte-le pour rouvrir tes serveurs sur Internet.",
      failed: "Impossible de connecter playit.gg. Réessaie.",
    },
    settings: {
      title: "Accès Internet (playit.gg)",
      unlinkedDescription:
        "Connecte un compte playit.gg pour que des amis hors de chez toi rejoignent tes serveurs.",
      linkedDescription:
        "Ton compte playit.gg est connecté. Un serveur n'est ouvert sur Internet que si tu l'actives sur sa page.",
      manage: "Gérer mon compte playit.gg",
      unlink: "Déconnecter",
      unlinkTitle: "Déconnecter playit.gg ?",
      unlinkDescription:
        "Tes serveurs ne seront plus accessibles depuis Internet et leurs adresses seront supprimées. playit.gg ne permet pas à MinyHost de supprimer son agent : supprime-le ensuite toi-même sur playit.gg (Agents), car un compte gratuit a un nombre d'agents limité.",
      unlinked:
        "playit.gg déconnecté. Pense à supprimer l'agent MinyHost sur playit.gg.",
      openAgents: "Ouvrir mes agents",
      agent: "Agent utilisé par MinyHost",
      agentId: (id: string) => `Identifiant : ${id}`,
      agentHint:
        "C'est le seul agent à garder sur playit.gg : les autres agents « MinyHost » viennent d'anciennes connexions et peuvent être supprimés. Fie-toi au nom et à l'identifiant, pas à l'indicateur « en ligne », qui peut rester affiché un moment pour un ancien agent.",
    },
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
    playitNotLinked: "Connecte d'abord un compte playit.gg.",
    playitLimit:
      "Ton compte playit.gg gratuit ne permet pas d'ouvrir un serveur de plus. Supprime un tunnel sur playit.gg, puis réessaie.",
    playitUnverified:
      "L'adresse e-mail de ton compte playit.gg n'est pas vérifiée. Sur playit.gg, va dans les paramètres du compte > Security > Verify email, puis clique sur le lien de l'e-mail reçu (pense à regarder dans tes spams). Réessaie ensuite.",
    playitAgentLimit:
      "Ton compte playit.gg a atteint son nombre maximal d'agents. Supprime les anciens agents « MinyHost » sur playit.gg, puis réessaie.",
    playit:
      "playit.gg n'a pas pu traiter la demande. Vérifie ta connexion, puis réessaie.",
    unknown: "Une erreur inattendue est survenue.",
  },
  // Help page. Answers use "\n" for line breaks (lists, steps).
  faq: {
    title: "Aide",
    description: "Les réponses aux questions les plus fréquentes.",
    search: "Rechercher une question…",
    noResult: "Aucune question ne correspond à ta recherche.",
    categories: [
      {
        title: "Premiers pas",
        items: [
          {
            q: "Qu'est-ce que MinyHost ?",
            a: "Une application gratuite et open source qui crée et fait tourner un serveur Minecraft Java sur ton PC, en quelques clics. Tu peux y jouer depuis ce PC et, avec playit.gg, l'ouvrir à des amis qui ne sont pas chez toi.",
          },
          {
            q: "Est-ce que je dois installer Java ?",
            a: "Non. MinyHost télécharge automatiquement la bonne version de Java pour chaque serveur et la range dans son propre dossier. Le Java déjà installé sur ton PC n'est jamais utilisé ni modifié.",
          },
          {
            q: "Quel type de serveur choisir ?",
            a: "• Paper : le plus rapide, compatible avec les plugins. Le bon choix dans la plupart des cas.\n• Vanilla : le serveur officiel de Mojang, sans modification.\n• Fabric : pour jouer avec des mods. Les joueurs ont souvent besoin des mêmes mods de leur côté.\nDans tous les cas, les joueurs doivent utiliser la même version de Minecraft que le serveur.",
          },
          {
            q: "Combien de mémoire (RAM) donner au serveur ?",
            a: "3 Go suffisent pour 2 à 10 joueurs. Ton PC fait tourner le serveur et ton jeu en même temps : garde assez de mémoire pour Minecraft (au moins 4 Go libres de plus est confortable). Avec beaucoup de mods, prévois davantage.",
          },
          {
            q: "Pourquoi dois-je accepter l'EULA de Minecraft ?",
            a: "Mojang l'exige pour faire tourner un serveur. MinyHost ne l'accepte jamais à ta place : c'est la case à cocher lors de la création du serveur. Le lien « Lire l'EULA » ouvre le texte officiel.",
          },
        ],
      },
      {
        title: "Jouer et gérer le serveur",
        items: [
          {
            q: "Comment rejoindre mon serveur depuis ce PC ?",
            a: "Démarre le serveur et attends le statut « En ligne ». Dans Minecraft : Multijoueur > Ajouter un serveur, puis entre l'adresse affichée sur la page du serveur (« Adresse (depuis ce PC) », par exemple localhost).",
          },
          {
            q: "Un joueur voit « You are not white-listed on this server ». Que faire ?",
            a: "La whitelist est activée pour protéger ton serveur : seuls les joueurs de la liste peuvent entrer. Sur la page du serveur, onglet Whitelist, ajoute son pseudo Minecraft exact. Pas besoin de redémarrer.",
          },
          {
            q: "À quoi servent les opérateurs ?",
            a: "Un opérateur peut utiliser toutes les commandes en jeu (/gamemode, /tp, /ban…) et entrer même s'il n'est pas dans la whitelist. Ajoute-toi en opérateur, et n'ajoute que des personnes de confiance.",
          },
          {
            q: "Comment changer la difficulté, le mode de jeu ou les règles du jeu ?",
            a: "Onglet Paramètres de la page du serveur. Les règles du jeu (garder l'inventaire, cycle jour/nuit…) s'appliquent tout de suite si le serveur tourne. Les réglages du monde (difficulté, PvP…) s'appliquent au prochain démarrage : un bandeau te propose alors de redémarrer.",
          },
          {
            q: "Le serveur continue-t-il de tourner quand je ferme MinyHost ?",
            a: "Non. À la fermeture, MinyHost arrête proprement chaque serveur en sauvegardant le monde. Ton serveur n'est donc accessible que tant que MinyHost est ouvert et que ton PC est allumé.",
          },
          {
            q: "Le serveur s'est arrêté tout seul. Pourquoi ?",
            a: "Ouvre l'onglet Console : les dernières lignes expliquent souvent le problème. Causes fréquentes : pas assez de mémoire, un mod ou un plugin incompatible avec la version, ou un monde abîmé. Essaie de le redémarrer ; si ça recommence, regarde le message d'erreur dans la console.",
          },
          {
            q: "« Le port de ce serveur est déjà utilisé ». Que faire ?",
            a: "Un autre programme utilise déjà ce port, souvent un autre serveur Minecraft. Arrête les autres serveurs (dans MinyHost ou ailleurs), puis relance celui-ci.",
          },
        ],
      },
      {
        title: "Jouer avec des amis via Internet (playit.gg)",
        items: [
          {
            q: "Qu'est-ce que playit.gg et pourquoi MinyHost l'utilise ?",
            a: "playit.gg est un service gratuit de tunnel : il donne à ton serveur une adresse publique, sans configurer ta box ni ouvrir de port. MinyHost intègre l'agent officiel de playit.gg, rien d'autre à installer. Il te faut ton propre compte playit.gg (gratuit) : MinyHost ne partage jamais de compte.",
          },
          {
            q: "Comment connecter mon compte playit.gg ?",
            a: "1. Sur la page d'un serveur (ou dans Paramètres > Accès Internet), clique sur « Connecter playit.gg ».\n2. Ton navigateur s'ouvre sur playit.gg : connecte-toi ou crée un compte.\n3. La page propose de réclamer un agent « self-managed » : clique sur « Continue » puis « Claim Agent ».\n4. Quand MinyHost affiche « Compte playit.gg connecté », tu peux fermer la page playit.gg.\nLe lien expire au bout de 15 minutes : si besoin, clique sur « Rouvrir la page » ou recommence.",
          },
          {
            q: "Pourquoi playit.gg demande-t-il des permissions « self-managed » ?",
            a: "Elles permettent à MinyHost de créer et de supprimer lui-même le tunnel de chacun de tes serveurs. Cet accès est limité à l'agent de MinyHost : il ne touche à rien d'autre sur ton compte. Tu peux le retirer à tout moment (voir « Comment déconnecter playit.gg ? »).",
          },
          {
            q: "On me demande de vérifier mon adresse e-mail. Comment faire ?",
            a: "playit.gg n'envoie pas l'e-mail de vérification automatiquement. Sur playit.gg, va dans les paramètres du compte > Security > Verify email, puis clique sur le lien de l'e-mail reçu. Tu ne le trouves pas ? Regarde dans tes spams. Reviens ensuite dans MinyHost et réessaie.",
          },
          {
            q: "Comment ouvrir mon serveur à mes amis ?",
            a: "1. Sur la page du serveur, clique sur « Ouvrir sur Internet ».\n2. Attends l'adresse : playit.gg peut mettre jusqu'à une minute à la préparer.\n3. Démarre le serveur et attends le statut « Ouvert sur Internet ».\n4. Copie l'adresse (bouton à côté) et envoie-la à tes amis.\n5. De leur côté : Multijoueur > Ajouter un serveur, puis coller l'adresse.\nN'oublie pas de les ajouter à la whitelist.",
          },
          {
            q: "Mes amis n'arrivent pas à se connecter. Que vérifier ?",
            a: "• Le serveur est démarré et la carte affiche « Ouvert sur Internet ».\n• Ils utilisent exactement l'adresse affichée (copie-la avec le bouton).\n• Leur pseudo est dans la whitelist.\n• Ils ont la même version de Minecraft que le serveur (et les mêmes mods avec Fabric).\n• Pour un tunnel tout neuf, attends une minute.\n• Teste toi-même l'adresse depuis ce PC : si ça marche chez toi mais pas chez eux, le problème vient de leur côté (version, pseudo).\n• En dernier recours : redémarre le serveur, ou ferme puis rouvre l'accès Internet (l'adresse changera).",
          },
          {
            q: "Comment savoir quel agent playit.gg est celui de MinyHost ?",
            a: "Va dans Paramètres > Accès Internet : MinyHost y affiche le nom de son agent (« MinyHost <nom du PC> <date> ») et son identifiant. C'est le seul à garder. Les autres agents « MinyHost » de ta liste playit.gg viennent d'anciennes connexions. Fie-toi au nom et à l'identifiant plutôt qu'à l'indicateur « en ligne », qui peut rester allumé un moment pour un ancien agent.",
          },
          {
            q: "« Trop d'agents sur ton compte playit.gg ». Que faire ?",
            a: "Chaque nouvelle connexion de MinyHost crée un agent, et un compte gratuit en accepte un nombre limité.\n1. Clique sur « Ouvrir mes agents playit.gg ».\n2. Supprime les anciens agents « MinyHost », en gardant celui indiqué dans Paramètres > Accès Internet.\n3. Reviens dans MinyHost et clique sur « Vérifier à nouveau ».",
          },
          {
            q: "Comment déconnecter playit.gg ?",
            a: "Paramètres > Accès Internet > Déconnecter. Tes serveurs ne sont plus accessibles depuis Internet et leurs tunnels sont supprimés. playit.gg ne permet pas à MinyHost de supprimer son agent : supprime-le ensuite toi-même dans ta liste d'agents sur playit.gg.",
          },
          {
            q: "L'adresse de mon serveur va-t-elle changer ?",
            a: "Non, tant que l'accès Internet reste ouvert. Elle change si tu fermes puis rouvres l'accès Internet, ou si tu reconnectes un compte playit.gg (un nouveau tunnel est créé).",
          },
          {
            q: "Est-ce que c'est sûr d'ouvrir mon serveur sur Internet ?",
            a: "MinyHost garde la whitelist et la vérification des comptes Minecraft activées : seuls les joueurs que tu as ajoutés peuvent entrer. Seuls les serveurs démarrés et ouverts sur Internet sont joignables, et rien d'autre sur ton PC n'est exposé. Évite quand même de publier l'adresse, et ferme l'accès Internet quand tu ne t'en sers plus.",
          },
          {
            q: "Faut-il payer playit.gg ?",
            a: "Non : l'offre gratuite suffit pour jouer entre amis avec MinyHost. playit Premium n'est utile que pour des options avancées (adresse personnalisée, régions…).",
          },
          {
            q: "Mes amis jouent sur mobile ou console (Bedrock). Peuvent-ils rejoindre ?",
            a: "Non : MinyHost crée des serveurs Minecraft Java Edition. Les joueurs doivent utiliser Minecraft Java sur ordinateur.",
          },
        ],
      },
      {
        title: "Fichiers et sauvegardes",
        items: [
          {
            q: "Où sont les fichiers de mes serveurs ?",
            a: "Dans le dossier %APPDATA%\\MinyHost\\servers, un dossier par serveur (monde, réglages, journaux). Le bouton dossier de la page d'un serveur l'ouvre directement, et Paramètres > Fichiers ouvre le dossier de tous les serveurs.",
          },
          {
            q: "Comment sauvegarder mon monde ?",
            a: "Les sauvegardes automatiques arriveront dans une prochaine version. En attendant : arrête le serveur, ouvre son dossier, puis copie-le ailleurs (il contient le monde et tous les réglages).",
          },
          {
            q: "Que se passe-t-il quand je supprime un serveur ?",
            a: "Son dossier est supprimé définitivement, monde compris, ainsi que son tunnel playit.gg s'il était ouvert sur Internet. Fais une copie du dossier avant si tu veux garder le monde.",
          },
        ],
      },
      {
        title: "Application",
        items: [
          {
            q: "Comment changer la langue ou le thème ?",
            a: "Dans Paramètres > Apparence. Par défaut, MinyHost suit la langue de Windows.",
          },
          {
            q: "J'ai trouvé un bug ou j'ai une idée. Où le signaler ?",
            a: "Sur la page GitHub du projet (github.com/MariusDev80/minyhost), onglet Issues. Décris ce que tu faisais et, si possible, copie les dernières lignes de la console du serveur.",
          },
        ],
      },
    ],
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
    files: "Fichiers",
    filesDescription:
      "Chaque serveur (monde, réglages, journaux) est rangé dans son propre dossier, sur ce PC.",
    openServersFolder: "Ouvrir le dossier des serveurs",
  },
} as const;
