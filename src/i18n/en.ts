// English UI strings. Same keys as `fr.ts`, the reference locale.
import type { Messages } from ".";

export const en: Messages = {
  app: {
    name: "MinyHost",
    tagline: "Host your Minecraft server on your PC, in a few clicks.",
  },
  titleBar: {
    minimize: "Minimize",
    maximize: "Maximize",
    restore: "Restore",
    close: "Close",
  },
  nav: {
    home: "Home",
    servers: "Servers",
    settings: "Settings",
    designSystem: "Design system",
  },
  status: {
    starting: "Starting…",
    running: "Online",
    stopping: "Stopping…",
    stopped: "Stopped",
  },
  actions: {
    createServer: "Create a server",
    start: "Start",
    stop: "Stop",
    delete: "Delete",
    cancel: "Cancel",
    retry: "Retry",
    edit: "Edit",
    send: "Send",
    restart: "Restart",
  },
  loaders: {
    vanilla: {
      name: "Vanilla",
      description: "The official Minecraft, unmodified.",
    },
    paper: {
      name: "Paper",
      description: "Faster, and supports plugins.",
    },
    fabric: {
      name: "Fabric",
      description: "To play with mods.",
    },
  },
  units: {
    memory: (mb: number) => `${mb / 1024} GB`,
  },
  home: {
    title: "Welcome",
    description: "Create a server, start it and play with your friends.",
    emptyTitle: "No server yet",
    emptyDescription:
      "Create your first Minecraft server: Java and the server are installed automatically.",
  },
  servers: {
    title: "Servers",
    description: "All the servers hosted on this PC.",
    emptyTitle: "No servers",
    emptyDescription: "The servers you create will show up here.",
    loadError: "Couldn't load the server list.",
  },
  create: {
    title: "Create a server",
    description:
      "Pick the type, the version and the memory. MinyHost takes care of the rest.",
    name: "Server name",
    namePlaceholder: "Survival with friends",
    type: "Server type",
    version: "Minecraft version",
    versionPlaceholder: "Pick a version",
    versionsError: "Couldn't load the versions. Check your connection.",
    memory: "Memory (RAM)",
    memoryHint:
      "3 GB is enough for 2 to 10 players. Keep enough memory for your game.",
    eula: "I accept the Minecraft End User License Agreement (EULA).",
    eulaLink: "Read the EULA",
    eulaHint: "Required to host a Minecraft server.",
    submit: "Create the server",
    progressTitle: "Creating the server",
    progressHint: "The first time, this can take a few minutes.",
    steps: {
      preparing: "Preparing",
      downloadingJava: "Downloading Java",
      downloadingServer: "Downloading the server",
      finalizing: "Finalizing",
    },
    failed: "The creation failed",
    success: (name: string) => `“${name}” is ready!`,
  },
  server: {
    back: "Servers",
    notFound: "This server no longer exists.",
    address: "Address (from this PC)",
    version: "Version",
    memory: "Memory",
    java: "Java",
    console: "Console",
    consoleEmpty: "Start the server to see its console here.",
    commandPlaceholder: "Type a command, e.g. “list”",
    deleteTitle: (name: string) => `Delete “${name}”?`,
    deleteDescription:
      "The world and all the files of this server will be permanently deleted.",
    deleteConfirm: "Delete permanently",
    deleted: (name: string) => `“${name}” has been deleted.`,
  },
  players: {
    placeholder: "Minecraft username",
    add: "Add",
    remove: (name: string) => `Remove ${name}`,
    invalidName: "3 to 16 characters: letters, digits or _",
  },
  whitelist: {
    tab: "Whitelist",
    title: "Allowed players",
    description:
      "Only these players can join the server. Use their exact Minecraft username.",
    empty: "Nobody is allowed yet. Start by adding yourself!",
    added: (name: string) => `${name} can now join the server.`,
    removed: (name: string) => `${name} was removed from the whitelist.`,
    already: (name: string) => `${name} is already on the whitelist.`,
  },
  operators: {
    tab: "Operators",
    title: "Operators",
    description:
      "Operators can use every command (/gamemode, /tp, /ban…) and join even when they are not on the whitelist. Only add people you trust.",
    empty: "No operators. Add yourself to use commands in game.",
    added: (name: string) => `${name} is now an operator.`,
    removed: (name: string) => `${name} is no longer an operator.`,
    already: (name: string) => `${name} is already an operator.`,
  },
  restart: {
    title: "Restart needed",
    description:
      "Some settings changed while the server was running. Restart it to apply them.",
  },
  gameSettings: {
    tab: "Settings",
    world: "World and server",
    worldDescription:
      "Saved in server.properties. Applied the next time the server starts.",
    rules: "Game rules",
    rulesDescription:
      "Values read from the world, including those changed in game. Server running: applied immediately. Server stopped: applied the next time it starts.",
    unsupported: "Game rules are not supported for versions older than 1.13.",
    search: "Search a setting…",
    noResult: "No setting matches your search.",
    reset: "Reset to default",
    defaultValue: (value: string) => `Default: ${value}`,
    on: "on",
    off: "off",
    unsaved: "Unsaved changes",
    discard: "Discard",
    save: "Save",
    saved: "Settings saved.",
    savedNeedsRestart:
      "Settings saved. Restart the server to apply the world settings.",
    loadError: "Couldn't read the settings of this server.",
    creationToggle: "Customize the world and the game rules",
    creationHint: "Optional: everything can be changed later.",
    categories: {
      players: "Players",
      mobs: "Mobs",
      world: "World",
      drops: "Items and loot",
      commands: "Commands and technical",
    },
    choices: {
      difficulty: {
        peaceful: "Peaceful",
        easy: "Easy",
        normal: "Normal",
        hard: "Hard",
      },
      gamemode: {
        survival: "Survival",
        creative: "Creative",
        adventure: "Adventure",
        spectator: "Spectator",
      },
    },
    labels: {
      // server.properties
      motd: {
        label: "Server message",
        description:
          "Text shown under the server name in the multiplayer list.",
      },
      difficulty: { label: "Difficulty" },
      gamemode: {
        label: "Game mode",
        description: "Game mode of the players who join the server.",
      },
      hardcore: {
        label: "Hardcore",
        description: "A single life: on death, the player becomes a spectator.",
      },
      "max-players": { label: "Max players" },
      "view-distance": {
        label: "View distance (chunks)",
        description:
          "Number of chunks sent around each player. The higher, the harder the server works.",
      },
      "simulation-distance": {
        label: "Simulation distance (chunks)",
        description:
          "Radius in chunks where the world stays active: mobs, crops, redstone.",
      },
      "spawn-protection": {
        label: "Spawn protection (blocks)",
        description:
          "Radius around the spawn point where only operators can build. 0 to disable.",
      },
      "allow-flight": {
        label: "Allow flight",
        description:
          "Prevents players from being kicked for flying (useful with some mods or plugins).",
      },
      pvp: {
        label: "Player versus player (PvP)",
        description: "Players can hurt each other.",
      },
      "spawn-monsters": {
        label: "Spawn monsters",
        description: "Hostile monsters spawn naturally.",
      },
      "allow-nether": {
        label: "Allow the Nether",
        description: "Portals can take players to the Nether.",
      },
      "level-seed": {
        label: "World seed",
        description: "Leave empty for a random seed.",
      },
      "generate-structures": {
        label: "Generate structures",
        description: "Villages, temples, fortresses…",
      },
      // Game rules: players
      keep_inventory: {
        label: "Keep inventory on death",
        description: "Players keep their items and experience when they die.",
      },
      natural_health_regeneration: {
        label: "Natural regeneration",
        description: "Health regenerates when the hunger bar is full enough.",
      },
      immediate_respawn: {
        label: "Immediate respawn",
        description: "No death screen: the player respawns right away.",
      },
      players_sleeping_percentage: {
        label: "Players required to sleep (%)",
        description:
          "Percentage of players who must sleep to skip the night. Above 100, the night can't be skipped.",
      },
      respawn_radius: {
        label: "Spawn radius (blocks)",
        description: "Area around the spawn point where players spawn.",
      },
      fall_damage: { label: "Fall damage" },
      fire_damage: { label: "Fire damage" },
      drowning_damage: { label: "Drowning damage" },
      freeze_damage: {
        label: "Freeze damage",
        description: "In powder snow.",
      },
      ender_pearls_vanish_on_death: {
        label: "Ender pearls vanish on death",
        description: "Thrown pearls disappear when their thrower dies.",
      },
      locator_bar: {
        label: "Locator bar",
        description: "Shows in which direction the other players are.",
      },
      show_death_messages: {
        label: "Death messages",
        description: "Announces player deaths in the chat.",
      },
      show_advancement_messages: {
        label: "Announce advancements",
        description: "Announces player advancements in the chat.",
      },
      limited_crafting: {
        label: "Limited crafting",
        description: "Only unlocked recipes can be crafted.",
      },
      reduced_debug_info: {
        label: "Reduced F3 screen",
        description:
          "Hides the coordinates and other info from the debug screen.",
      },
      allow_entering_nether_using_portals: {
        label: "Allow the Nether",
        description: "Portals can take players to the Nether.",
      },
      players_nether_portal_default_delay: {
        label: "Portal delay (ticks)",
        description:
          "Time to spend in a Nether portal before travelling, in survival. 20 ticks = 1 second.",
      },
      players_nether_portal_creative_delay: {
        label: "Portal delay in creative (ticks)",
      },
      spectators_generate_chunks: {
        label: "Spectators generate the world",
        description: "Players in spectator mode load new chunks.",
      },
      // Game rules: mobs
      spawn_mobs: {
        label: "Spawn mobs",
        description: "Animals and monsters spawn naturally.",
      },
      spawn_monsters: {
        label: "Spawn monsters",
        description: "Hostile monsters spawn naturally.",
      },
      spawn_phantoms: {
        label: "Spawn phantoms",
        description: "Phantoms attack players who don't sleep.",
      },
      spawn_patrols: { label: "Pillager patrols" },
      spawn_wandering_traders: { label: "Wandering traders" },
      spawn_wardens: {
        label: "Spawn wardens",
        description: "Sculk shriekers can summon the warden.",
      },
      raids: {
        label: "Raids",
        description: "Pillagers attack villages (Bad Omen effect).",
      },
      mob_griefing: {
        label: "Mob griefing",
        description:
          "Creepers that destroy blocks, endermen that pick them up, villagers that harvest…",
      },
      forgive_dead_players: {
        label: "Mobs forgive dead players",
        description: "Angry neutral mobs calm down when their target dies.",
      },
      universal_anger: {
        label: "Universal anger",
        description:
          "An angry neutral mob attacks every nearby player, not only its attacker.",
      },
      max_entity_cramming: {
        label: "Max entity cramming",
        description:
          "Mobs on the same block before they take damage. 0 to disable.",
      },
      spawner_blocks_work: {
        label: "Spawners work",
        description: "Monster spawners spawn mobs.",
      },
      // Game rules: world
      advance_time: {
        label: "Day/night cycle",
        description: "Time passes. When off, the time of day stays frozen.",
      },
      advance_weather: {
        label: "Weather cycle",
        description: "The weather changes. When off, it stays frozen.",
      },
      random_tick_speed: {
        label: "Random tick speed",
        description:
          "Speed at which crops and grass grow, leaves decay… 0 stops everything.",
      },
      fire_spread_radius_around_player: {
        label: "Fire spread (radius in blocks)",
        description:
          "Fire only spreads near players. 0 = it doesn't spread, -1 = everywhere.",
      },
      doFireTick: {
        label: "Fire spread",
        description: "Fire spreads and burns out naturally.",
      },
      allowFireTicksAwayFromPlayer: {
        label: "Fire away from players",
        description: "Fire spreads even far from any player.",
      },
      spread_vines: { label: "Vines spread" },
      water_source_conversion: {
        label: "Water source creation",
        description: "Two water sources next to each other form a new one.",
      },
      lava_source_conversion: {
        label: "Lava source creation",
        description: "Two lava sources next to each other form a new one.",
      },
      max_snow_accumulation_height: {
        label: "Max snow depth",
        description: "Snow layers that pile up when it snows (0 to 8).",
      },
      tnt_explodes: { label: "TNT explodes" },
      projectiles_can_break_blocks: {
        label: "Projectiles break blocks",
        description: "Some projectiles break blocks (decorated pots, chorus…).",
      },
      global_sound_events: {
        label: "Global sounds",
        description: "Some sounds (wither, dragon) are heard by every player.",
      },
      // Game rules: drops
      block_drops: {
        label: "Blocks drop items",
        description: "Broken blocks drop their items.",
      },
      mob_drops: {
        label: "Mob loot",
        description: "Mobs drop items when they die.",
      },
      entity_drops: {
        label: "Entity drops",
        description:
          "Minecarts, boats, item frames… drop their items when broken.",
      },
      block_explosion_drop_decay: {
        label: "Item loss (block explosions)",
        description:
          "Some of the blocks destroyed by a block explosion drop nothing.",
      },
      mob_explosion_drop_decay: {
        label: "Item loss (mob explosions)",
        description: "Some of the blocks destroyed by a creeper drop nothing.",
      },
      tnt_explosion_drop_decay: {
        label: "Item loss (TNT)",
        description: "Some of the blocks destroyed by TNT drop nothing.",
      },
      // Game rules: commands
      command_block_output: {
        label: "Command block messages",
        description: "Command blocks write to the operators' chat.",
      },
      command_blocks_work: { label: "Command blocks work" },
      send_command_feedback: {
        label: "Command feedback",
        description: "Shows the result of commands in the chat.",
      },
      log_admin_commands: {
        label: "Announce operator commands",
        description: "Other operators see the commands that are used.",
      },
      max_command_sequence_length: {
        label: "Max command chain length",
        description: "For functions and chained command blocks.",
      },
      max_command_forks: { label: "Max command forks" },
      max_block_modifications: {
        label: "Max blocks changed per command",
        description: "Limit for /fill, /clone…",
      },
      player_movement_check: {
        label: "Check player movement",
        description: "Anti-cheat: kicks players who move too fast.",
      },
      elytra_movement_check: {
        label: "Check elytra flight",
        description: "Anti-cheat for elytra.",
      },
    },
  },
  notifications: {
    killed:
      "The server stopped responding: it was force-stopped. The world may be damaged.",
    crashed: (name: string) => `“${name}” stopped unexpectedly.`,
    crashedHint: "Check the console to see what happened.",
  },
  closing: {
    title: "Stopping the servers…",
    description: "MinyHost is saving the worlds before closing.",
  },
  errors: {
    network: "Can't reach the Internet. Check your connection, then try again.",
    io: "Can't read or write files on this PC. Check the available disk space.",
    invalidData: "A received file is invalid. Try again in a moment.",
    hashMismatch: "A downloaded file is corrupted. Try again.",
    serverNotFound: "This server can't be found. It may have been deleted.",
    versionUnavailable: "This version isn't available for this server type.",
    javaUnavailable: "Java isn't available for your system.",
    portInUse:
      "This server's port is already in use. Another server may already be running on this PC.",
    alreadyRunning: "This server is already running.",
    notRunning: "This server isn't running.",
    eulaNotAccepted: "The Minecraft EULA must be accepted to start a server.",
    missingFiles:
      "Some files of this server are missing. Delete it, then create it again.",
    invalidInput: "The information entered is not valid.",
    invalidPlayerName:
      "This username isn't valid: 3 to 16 characters, letters, digits or _.",
    playerNotFound:
      "No Minecraft account has this username. Check the spelling.",
    unknown: "An unexpected error occurred.",
  },
  settings: {
    title: "Settings",
    description: "Application preferences.",
    appearance: "Appearance",
    theme: "Theme",
    themeDescription: "Choose how MinyHost looks.",
    themes: {
      dark: "Dark",
      light: "Light",
      system: "System",
    },
    language: "Language",
    languageDescription: "Language of the MinyHost interface.",
    languageSystem: (name: string) => `System (${name})`,
  },
};
