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
    help: "Help",
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
    openFolder: "Open the server folder",
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
  tunnel: {
    title: "Play with friends over the Internet",
    description:
      "So that friends who aren't at your place can join this server, MinyHost uses playit.gg, a free tunnel service. You need a playit.gg account (free too).",
    link: "Link playit.gg",
    linking: "Confirm the link in your browser…",
    linkingHint:
      "Log in to playit.gg (or create an account). The page then offers to claim a “self-managed” agent: click “Continue”, then “Claim Agent”. This access lets MinyHost create and delete the tunnel of each server by itself, nothing more.",
    reopen: "Open the page again",
    linked:
      "playit.gg account linked. You can close the playit.gg page in your browser.",
    enableHint:
      "Your server will be reachable from the Internet. Keep the whitelist on: only allowed players can join.",
    enable: "Open to the Internet",
    disable: "Close Internet access",
    address: "Address to give your friends",
    copy: "Copy the address",
    copied: "Address copied.",
    pendingAddress:
      "playit.gg is preparing your server's address, this can take a minute…",
    status: {
      stopped: "Start the server so that your friends can join it.",
      connecting: "Connecting to playit.gg…",
      online: "Open to the Internet.",
      error: "playit.gg can't be reached right now. Retrying…",
    },
    disabled: (reason: string) => `Disabled by playit.gg: ${reason}`,
    noticeLink: "Learn more",
    emailUnverified: {
      title: "playit.gg email address to verify",
      steps:
        "playit.gg doesn't send it automatically: on playit.gg, go to the account settings > Security > Verify email. Then click the link in the email you receive. Can't find it? Check your spam folder.",
      open: "Open my playit.gg account",
    },
    agentLimit: {
      title: "Too many agents on your playit.gg account",
      steps:
        "Each MinyHost link creates an agent, and a free account only accepts a limited number. On playit.gg, open your agent list and delete the old “MinyHost” agents: only keep the one shown in Settings > Internet access. Then click “Check again”.",
      open: "Open my playit.gg agents",
      recheck: "Check again",
    },
    linkErrors: {
      rejected: "The link to playit.gg was rejected.",
      expired: "The time to link playit.gg ran out. Try again.",
      revoked:
        "MinyHost no longer has access to your playit.gg account. Link it again to reopen your servers to the Internet.",
      failed: "Couldn't link playit.gg. Try again.",
    },
    settings: {
      title: "Internet access (playit.gg)",
      unlinkedDescription:
        "Link a playit.gg account so that friends away from home can join your servers.",
      linkedDescription:
        "Your playit.gg account is linked. A server is only open to the Internet if you turn it on from its page.",
      manage: "Manage my playit.gg account",
      unlink: "Unlink",
      unlinkTitle: "Unlink playit.gg?",
      unlinkDescription:
        "Your servers will no longer be reachable from the Internet and their addresses will be deleted. playit.gg doesn't let MinyHost delete its agent: delete it yourself on playit.gg afterwards (Agents), as a free account has a limited number of agents.",
      unlinked:
        "playit.gg unlinked. Remember to delete the MinyHost agent on playit.gg.",
      openAgents: "Open my agents",
      agent: "Agent used by MinyHost",
      agentId: (id: string) => `ID: ${id}`,
      agentHint:
        "It's the only agent to keep on playit.gg: the other “MinyHost” agents come from older links and can be deleted. Rely on the name and the ID, not on the “online” indicator, which may stay on for a while for an old agent.",
    },
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
    playitNotLinked: "Link a playit.gg account first.",
    playitLimit:
      "Your free playit.gg account can't open one more server. Delete a tunnel on playit.gg, then try again.",
    playitUnverified:
      "The email address of your playit.gg account isn't verified. On playit.gg, go to the account settings > Security > Verify email, then click the link in the email you receive (check your spam folder). Try again afterwards.",
    playitAgentLimit:
      "Your playit.gg account has reached its maximum number of agents. Delete the old “MinyHost” agents on playit.gg, then try again.",
    playit:
      "playit.gg couldn't handle the request. Check your connection, then try again.",
    unknown: "An unexpected error occurred.",
  },
  faq: {
    title: "Help",
    description: "Answers to the most common questions.",
    search: "Search a question…",
    noResult: "No question matches your search.",
    categories: [
      {
        title: "Getting started",
        items: [
          {
            q: "What is MinyHost?",
            a: "A free and open source app that creates and runs a Minecraft Java server on your PC, in a few clicks. You can play on it from this PC and, with playit.gg, open it to friends who aren't at your place.",
          },
          {
            q: "Do I need to install Java?",
            a: "No. MinyHost automatically downloads the right Java version for each server and keeps it in its own folder. The Java already installed on your PC is never used or changed.",
          },
          {
            q: "Which server type should I pick?",
            a: "• Paper: the fastest, supports plugins. The right choice in most cases.\n• Vanilla: Mojang's official server, unmodified.\n• Fabric: to play with mods. Players often need the same mods on their side.\nIn every case, players must use the same Minecraft version as the server.",
          },
          {
            q: "How much memory (RAM) should the server get?",
            a: "3 GB is enough for 2 to 10 players. Your PC runs the server and your game at the same time: keep enough memory for Minecraft (at least 4 more free GB is comfortable). With many mods, plan more.",
          },
          {
            q: "Why do I have to accept the Minecraft EULA?",
            a: "Mojang requires it to run a server. MinyHost never accepts it for you: it's the checkbox when you create the server. The “Read the EULA” link opens the official text.",
          },
        ],
      },
      {
        title: "Playing and managing the server",
        items: [
          {
            q: "How do I join my server from this PC?",
            a: "Start the server and wait for the “Online” status. In Minecraft: Multiplayer > Add Server, then enter the address shown on the server page (“Address (from this PC)”, for example localhost).",
          },
          {
            q: "A player sees “You are not white-listed on this server”. What should I do?",
            a: "The whitelist is on to protect your server: only players on the list can join. On the server page, Whitelist tab, add their exact Minecraft username. No restart needed.",
          },
          {
            q: "What are operators for?",
            a: "An operator can use every command in game (/gamemode, /tp, /ban…) and join even when not on the whitelist. Add yourself as an operator, and only add people you trust.",
          },
          {
            q: "How do I change the difficulty, the game mode or the game rules?",
            a: "Settings tab of the server page. Game rules (keep inventory, day/night cycle…) apply right away if the server is running. World settings (difficulty, PvP…) apply on the next start: a banner then offers to restart.",
          },
          {
            q: "Does the server keep running when I close MinyHost?",
            a: "No. When closing, MinyHost stops each server cleanly and saves the world. Your server is only reachable while MinyHost is open and your PC is on.",
          },
          {
            q: "The server stopped by itself. Why?",
            a: "Open the Console tab: the last lines often explain the problem. Common causes: not enough memory, a mod or plugin that doesn't match the version, or a damaged world. Try starting it again; if it happens again, look at the error message in the console.",
          },
          {
            q: "“This server's port is already in use”. What should I do?",
            a: "Another program already uses this port, often another Minecraft server. Stop the other servers (in MinyHost or elsewhere), then start this one again.",
          },
        ],
      },
      {
        title: "Playing with friends over the Internet (playit.gg)",
        items: [
          {
            q: "What is playit.gg and why does MinyHost use it?",
            a: "playit.gg is a free tunnel service: it gives your server a public address, without setting up your router or opening a port. MinyHost embeds the official playit.gg agent, nothing else to install. You need your own playit.gg account (free): MinyHost never shares an account.",
          },
          {
            q: "How do I link my playit.gg account?",
            a: "1. On a server page (or in Settings > Internet access), click “Link playit.gg”.\n2. Your browser opens playit.gg: log in or create an account.\n3. The page offers to claim a “self-managed” agent: click “Continue”, then “Claim Agent”.\n4. When MinyHost shows “playit.gg account linked”, you can close the playit.gg page.\nThe link expires after 15 minutes: if needed, click “Open the page again” or start over.",
          },
          {
            q: "Why does playit.gg ask for “self-managed” permissions?",
            a: "They let MinyHost create and delete the tunnel of each of your servers by itself. This access is limited to MinyHost's agent: it doesn't touch anything else on your account. You can remove it at any time (see “How do I unlink playit.gg?”).",
          },
          {
            q: "I'm asked to verify my email address. How?",
            a: "playit.gg doesn't send the verification email automatically. On playit.gg, go to the account settings > Security > Verify email, then click the link in the email you receive. Can't find it? Check your spam folder. Then come back to MinyHost and try again.",
          },
          {
            q: "How do I open my server to my friends?",
            a: "1. On the server page, click “Open to the Internet”.\n2. Wait for the address: playit.gg can take up to a minute to prepare it.\n3. Start the server and wait for the “Open to the Internet” status.\n4. Copy the address (button next to it) and send it to your friends.\n5. On their side: Multiplayer > Add Server, then paste the address.\nRemember to add them to the whitelist.",
          },
          {
            q: "My friends can't connect. What should I check?",
            a: "• The server is started and the card shows “Open to the Internet”.\n• They use exactly the address shown (copy it with the button).\n• Their username is on the whitelist.\n• They have the same Minecraft version as the server (and the same mods with Fabric).\n• For a brand new tunnel, wait a minute.\n• Try the address yourself from this PC: if it works for you but not for them, the problem is on their side (version, username).\n• As a last resort: restart the server, or close then reopen Internet access (the address will change).",
          },
          {
            q: "How do I know which playit.gg agent is MinyHost's?",
            a: "Go to Settings > Internet access: MinyHost shows its agent's name (“MinyHost <PC name> <date>”) and ID there. It's the only one to keep. The other “MinyHost” agents in your playit.gg list come from older links. Rely on the name and the ID rather than on the “online” indicator, which may stay on for a while for an old agent.",
          },
          {
            q: "“Too many agents on your playit.gg account”. What should I do?",
            a: "Each new MinyHost link creates an agent, and a free account only accepts a limited number.\n1. Click “Open my playit.gg agents”.\n2. Delete the old “MinyHost” agents, keeping the one shown in Settings > Internet access.\n3. Come back to MinyHost and click “Check again”.",
          },
          {
            q: "How do I unlink playit.gg?",
            a: "Settings > Internet access > Unlink. Your servers are no longer reachable from the Internet and their tunnels are deleted. playit.gg doesn't let MinyHost delete its agent: delete it yourself afterwards in your agent list on playit.gg.",
          },
          {
            q: "Will my server's address change?",
            a: "No, as long as Internet access stays open. It changes if you close then reopen Internet access, or if you link a playit.gg account again (a new tunnel is created).",
          },
          {
            q: "Is it safe to open my server to the Internet?",
            a: "MinyHost keeps the whitelist and Minecraft account checks on: only the players you added can join. Only servers that are started and open to the Internet are reachable, and nothing else on your PC is exposed. Still, avoid posting the address publicly, and close Internet access when you don't use it.",
          },
          {
            q: "Do I have to pay for playit.gg?",
            a: "No: the free plan is enough to play with friends with MinyHost. playit Premium is only useful for advanced options (custom address, regions…).",
          },
          {
            q: "My friends play on mobile or console (Bedrock). Can they join?",
            a: "No: MinyHost creates Minecraft Java Edition servers. Players must use Minecraft Java on a computer.",
          },
        ],
      },
      {
        title: "Files and backups",
        items: [
          {
            q: "Where are my servers' files?",
            a: "In the %APPDATA%\\MinyHost\\servers folder, one folder per server (world, settings, logs). The folder button on a server page opens it directly, and Settings > Files opens the folder of all servers.",
          },
          {
            q: "How do I back up my world?",
            a: "Automatic backups will come in a future version. Meanwhile: stop the server, open its folder, then copy it somewhere else (it holds the world and all the settings).",
          },
          {
            q: "What happens when I delete a server?",
            a: "Its folder is permanently deleted, world included, along with its playit.gg tunnel if it was open to the Internet. Copy the folder first if you want to keep the world.",
          },
        ],
      },
      {
        title: "App",
        items: [
          {
            q: "How do I change the language or the theme?",
            a: "In Settings > Appearance. By default, MinyHost follows the Windows language.",
          },
          {
            q: "I found a bug or have an idea. Where do I report it?",
            a: "On the project's GitHub page (github.com/MariusDev80/minyhost), Issues tab. Describe what you were doing and, if possible, copy the last lines of the server console.",
          },
        ],
      },
    ],
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
    files: "Files",
    filesDescription:
      "Each server (world, settings, logs) is stored in its own folder, on this PC.",
    openServersFolder: "Open the servers folder",
  },
};
