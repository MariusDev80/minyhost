# CLAUDE.md

Ce fichier donne le contexte du projet à Claude (et à tout contributeur). À lire avant toute modification du code.

> Nom du projet : **MinyHost** (mini + mine + host : un petit serveur Minecraft hébergé chez soi).
> Sous-titre : « Héberge ton serveur Minecraft sur ton PC, en quelques clics. »

---

## 1. Le projet

Application **Windows** open-source qui permet à n'importe quel joueur de **créer et héberger un serveur Minecraft sur son propre PC en quelques clics**, puis d'y jouer directement avec ses amis depuis la même machine.

- **Public visé** : joueurs non techniques, groupes d'amis (2 à 10 joueurs).
- **Distribution** : gratuite, code source sur GitHub, installeur dans les releases GitHub.
- **Inspiration visuelle** : l'app Modrinth (thème sombre, barre latérale, cartes, design épuré). On s'inspire de l'esprit du design, **on ne copie ni son code, ni ses assets, ni sa marque**.

### Objectifs
1. **Zéro configuration technique** : l'utilisateur n'installe ni Java, ni serveur, ni outil réseau à la main.
2. **Fonctionnel d'abord** : un MVP qui crée, lance et pilote un serveur avant tout travail de finition.
3. **Beau et agréable** : interface moderne, cohérente et réactive.
4. **Léger** : l'app tourne à côté du jeu, elle doit consommer le minimum de ressources.
5. **Sûr par défaut** : whitelist activée, `online-mode=true`, rien d'exposé sans action explicite de l'utilisateur.

### Hors périmètre (pour l'instant)
- Hébergement distant / cloud.
- Support macOS et Linux (l'architecture doit cependant le permettre plus tard).
- Monétisation.

---

## 2. Stack technique

| Couche | Technologie | Rôle |
|---|---|---|
| Shell desktop | **Tauri 2** | Fenêtre native, installeur, pont entre l'UI et le système |
| Backend | **Rust** (`tokio`, `reqwest`, `serde`, `serde_json`, `zip`, `sha1`/`sha2`) | Téléchargements, gestion des processus Java, fichiers |
| Frontend | **React + TypeScript + Vite** | Interface |
| Style | **Tailwind CSS** + **shadcn/ui** (sur Radix UI) | Design system, composants accessibles |
| Icônes / police | **Lucide**, **Inter** | |
| Données asynchrones | **TanStack Query** | Cache et états de chargement des appels Rust / API |
| État UI | **Zustand** | État global de l'interface (sélection, console, thème) |
| Stockage | Fichiers **JSON** dans `%APPDATA%` (SQLite plus tard si besoin) | Configuration et instances |
| CI / Release | **GitHub Actions** + `tauri-action`, installeur **NSIS**, updater Tauri | Build et publication |

### Règles de répartition
- **Rust** : tout ce qui touche au système (processus, fichiers, réseau, téléchargements). Le frontend n'accède jamais directement au disque.
- **Frontend** : affichage et interactions uniquement. Il passe par des commandes Tauri (`invoke`) et écoute des événements (`listen`).
- **TanStack Query** pour toute donnée venant de Rust ou d'Internet. **Zustand** uniquement pour l'état propre à l'UI.
- Les composants shadcn/ui vivent dans `src/components/ui/` et peuvent être modifiés librement.

---

## 3. Arborescence

```
.
├── CLAUDE.md
├── README.md
├── ROADMAP.md
├── package.json
├── vite.config.ts
├── tailwind.config.ts
├── src/                          # Frontend React
│   ├── main.tsx
│   ├── App.tsx
│   ├── components/
│   │   ├── ui/                   # Composants shadcn/ui
│   │   ├── layout/               # Sidebar, TitleBar, PageHeader
│   │   └── server/               # ServerCard, Console, StatusBadge…
│   ├── pages/                    # Home, CreateServer, ServerDetail, Settings
│   ├── hooks/                    # useServers, useVersions… (TanStack Query)
│   ├── stores/                   # Stores Zustand
│   ├── lib/
│   │   ├── tauri.ts              # Wrappers typés autour de invoke/listen
│   │   ├── errors.ts             # Erreur Rust -> message lisible (i18n)
│   │   ├── skin.ts               # Lecture d'un skin (tête du joueur)
│   │   └── utils.ts
│   ├── i18n/fr.ts                # Tous les textes de l'interface
│   └── types/                    # Types partagés avec Rust
└── src-tauri/                    # Backend Rust
    ├── Cargo.toml
    ├── tauri.conf.json
    ├── capabilities/             # Permissions Tauri
    └── src/
        ├── main.rs
        ├── lib.rs                # Enregistrement des commandes
        ├── commands/             # Fonctions exposées au frontend (fines)
        ├── core/                 # Logique métier, indépendante de Tauri
        │   ├── instances.rs      # CRUD des serveurs
        │   ├── create.rs         # Pipeline de création (5.1)
        │   ├── java.rs           # Détection / téléchargement des JRE
        │   ├── providers/        # vanilla.rs, paper.rs, fabric.rs
        │   ├── process.rs        # Lancement, arrêt, console
        │   ├── properties.rs     # Lecture / écriture server.properties
        │   ├── eula.rs           # eula.txt
        │   ├── whitelist.rs      # whitelist.json
        │   ├── players.rs        # Profils Mojang (pseudo -> UUID, skins)
        │   ├── e2e_tests.rs      # Tests bout en bout (--ignored)
        │   └── backup.rs
        ├── events.rs             # Événements envoyés au frontend
        ├── state.rs              # État partagé des commandes
        ├── download.rs           # Téléchargement + vérification de hash
        ├── paths.rs              # Chemins AppData
        └── error.rs              # Type d'erreur commun
```

**Principe** : `commands/` ne contient que des fonctions courtes qui appellent `core/`. La logique métier dans `core/` ne dépend pas de Tauri, pour pouvoir un jour en faire une CLI ou la tester isolément.

---

## 4. Stockage des données

```
%APPDATA%/MinyHost/
├── settings.json                 # Préférences globales
├── java/
│   ├── 21/                       # JRE Temurin 21
│   └── 17/
└── servers/
    └── <id>/
        ├── instance.json         # Métadonnées de l'instance
        ├── server.jar
        ├── eula.txt
        ├── server.properties
        ├── world/
        ├── logs/
        └── backups/
```

Exemple de `instance.json` :
```json
{
  "id": "survie-entre-potes",
  "name": "Survie entre potes",
  "mcVersion": "1.21.1",
  "loader": "paper",
  "loaderVersion": "latest",
  "javaVersion": 21,
  "memoryMb": 4096,
  "port": 25565,
  "createdAt": "2026-10-05T12:00:00Z"
}
```

Chaque serveur est **autonome dans son dossier** : on doit pouvoir le copier ailleurs et le relancer.

---

## 5. Déroulement des processus

### 5.1 Création d'un serveur
1. L'utilisateur ouvre « Créer un serveur » et choisit : nom, type (Vanilla / Paper / Fabric), version Minecraft, RAM.
2. Le frontend charge les versions disponibles via `list_versions(loader)` (mis en cache par TanStack Query).
3. À la validation, appel de `create_server(config)` :
   1. Génération d'un `id` unique et création du dossier `servers/<id>/`.
   2. Détermination de la version de Java requise (voir 5.2).
   3. Téléchargement du JRE s'il est absent.
   4. Téléchargement du jar serveur auprès du bon fournisseur, **avec vérification du hash**.
   5. Écriture de `instance.json` et d'un `server.properties` par défaut (`white-list=true`, `online-mode=true`, port libre).
4. Rust émet des événements `create-progress` (étape + pourcentage) affichés dans une barre de progression.
5. **L'EULA de Mojang** est présentée à l'utilisateur avec un lien. `eula.txt` n'est écrit à `true` qu'après son acceptation explicite.

### 5.2 Choix de Java
La source de vérité est le champ `javaVersion.majorVersion` du fichier de version Mojang (`providers/vanilla.rs`, `required_java`), utilisé pour tous les types de serveur. Il est arrondi à la LTS Temurin supérieure (`java.rs`, `runtime_for`). À titre indicatif (vérifié en octobre 2026) :

| Version Minecraft | Java requis |
|---|---|
| 26.1 et plus | 25 |
| 1.20.5 à 1.21.x | 21 |
| 1.18 à 1.20.4 | 17 |
| 1.17.x | 16 (17 installé) |
| 1.16.5 et moins | 8 |

Les JRE sont téléchargés depuis l'API Adoptium (Temurin) et stockés dans `java/<version>/`. On n'utilise jamais le Java installé sur le système.

### 5.3 Démarrage
1. `start_server(id)` vérifie : port disponible, jar et JRE présents, EULA acceptée, RAM demandée raisonnable.
2. Lancement du processus :
   ```
   <java>/bin/java -Xms<ram> -Xmx<ram> -jar server.jar nogui
   ```
   avec le dossier du serveur comme répertoire de travail et sans fenêtre console (`CREATE_NO_WINDOW` sous Windows).
3. Rust lit `stdout` / `stderr` ligne par ligne et émet un événement `console-line` `{ id, line, stream }`.
4. Le statut passe par `starting` → `running` (détection de la ligne `Done (...)! For help, type "help"`) → `stopping` → `stopped`. Chaque changement émet `server-status`.
5. Un seul processus par instance. Le registre des processus actifs est tenu côté Rust (état partagé Tauri).

### 5.4 Console et commandes
- Le frontend garde un tampon limité de lignes (ex. 2 000) dans Zustand.
- `send_command(id, cmd)` écrit la commande dans le `stdin` du processus.

### 5.5 Arrêt
1. `stop_server(id)` envoie `stop` sur `stdin`.
2. Attente de la fin du processus (timeout ~30 s).
3. En cas de dépassement uniquement : kill forcé, avec avertissement à l'utilisateur (risque de corruption du monde).
4. À la fermeture de l'app, tous les serveurs actifs sont arrêtés proprement, ou l'utilisateur est prévenu.

### 5.6 Backups (phase 4)
1. Si le serveur tourne : `save-off`, puis `save-all flush`, attente de confirmation dans la console.
2. Archivage zip de `world/` (et des dimensions) dans `backups/<date>.zip`.
3. `save-on`.
4. Rotation : conserver les N derniers backups.

### 5.7 Accès des amis (phase 3)
- **LAN** : afficher l'IP locale + le port.
- **Pare-feu** : proposer d'ajouter une règle Windows (demande d'élévation explicite).
- **Internet** : intégration d'un tunnel (playit.gg ou Tailscale), après vérification de leurs conditions d'utilisation et de redistribution.
- Bouton « Copier l'adresse » dans tous les cas.

---

## 6. APIs externes

| Usage | Source |
|---|---|
| Versions Minecraft + jar vanilla | `https://piston-meta.mojang.com/mc/game/version_manifest_v2.json` |
| Paper | API PaperMC (`fill.papermc.io`, v3) |
| Fabric | `https://meta.fabricmc.net/v2/` |
| Java (Temurin) | `https://api.adoptium.net/v3/` |
| Joueurs (UUID, skins) | `api.mojang.com`, `sessionserver.mojang.com`, `textures.minecraft.net` |
| Mods / plugins (phase 4) | `https://api.modrinth.com/v2/` |

Règles :
- Toujours envoyer un **User-Agent** identifiant l'app (`MinyHost/<version> (github.com/<user>/<repo>)`), exigé notamment par Modrinth.
- Toujours **vérifier le hash** d'un fichier téléchargé quand l'API le fournit.
- Mettre en cache les réponses de listes de versions pour limiter les appels.
- Vérifier ces URLs dans la documentation officielle avant implémentation : elles évoluent.

---

## 7. Design

- **Thème sombre par défaut**, thème clair secondaire.
- Couleurs définies en variables CSS (convention shadcn/ui : `--background`, `--foreground`, `--primary`, `--muted`, `--border`…). Ne jamais coder une couleur en dur dans un composant.
- Une couleur d'accent unique (à définir, différente du vert Modrinth pour garder une identité propre).
- Layout : barre latérale gauche (Accueil, Serveurs, Paramètres) + zone principale.
- Coins arrondis généreux, ombres légères, transitions courtes (150-200 ms).
- Chaque action longue affiche une progression ; chaque erreur a un message compréhensible par un non-technicien et, si possible, une action pour la corriger.
- Interface en **français** au départ, textes centralisés pour permettre une traduction plus tard.

---

## 8. Roadmap

- **Phase 0 — Setup** : dépôt, licence, projet Tauri qui démarre, Tailwind + shadcn/ui installés.
- **Phase 1 — Design system** : palette, typo, layout avec sidebar, composants de base.
- **Phase 2 — MVP** : création Vanilla/Paper/Fabric, Java automatique, démarrage/arrêt, console, liste des serveurs, gestion d'erreurs.
- **Phase 3 — Accès réseau** : IP locale, pare-feu, tunnel, copie d'adresse.
- **Phase 4 — Confort** : backups, éditeur de propriétés, whitelist, joueurs connectés, mods/plugins via Modrinth, auto-update.
- **Phase 5 — Publication** : README, CI de release, signature de l'exécutable, CONTRIBUTING, templates d'issues.

Le détail des tâches est dans `ROADMAP.md`.

---

## 9. Commandes utiles

```bash
npm install               # Dépendances frontend
npm run tauri dev         # Lancer l'app en développement
npm run tauri build       # Construire l'installeur
npx shadcn@latest add <composant>
cargo test --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml -- --ignored   # Bout en bout (télécharge Java + serveurs)
cargo clippy --manifest-path src-tauri/Cargo.toml
npm run lint
```

---

## 10. Conventions

- **Code** : identifiants et commentaires en anglais ; textes d'interface en français.
- **TypeScript** : mode `strict`, pas de `any`. Les types échangés avec Rust sont définis dans `src/types/` et alignés sur les structs Rust (`#[serde(rename_all = "camelCase")]`).
- **Rust** : pas de `unwrap()` / `expect()` hors tests ; erreurs via un type commun (`thiserror`) converti en message lisible pour le frontend. `cargo fmt` et `clippy` sans avertissement.
- **Commits** : Conventional Commits (`feat:`, `fix:`, `refactor:`, `docs:`, `chore:`).
- **Branches** : `main` stable, une branche par fonctionnalité.
- **Tests** : la logique de `core/` (choix de Java, parsing, chemins) est testée unitairement.

---

## 11. Consignes pour Claude

- Respecter la répartition Rust / frontend décrite en section 2. Ne pas ajouter d'accès disque ou réseau côté frontend.
- Avancer **phase par phase** : ne pas implémenter une fonctionnalité d'une phase ultérieure sans demande explicite.
- Privilégier des changements petits et vérifiables ; expliquer brièvement les choix techniques non évidents.
- Ne pas ajouter de dépendance sans justifier son intérêt.
- Ne jamais accepter l'EULA Mojang à la place de l'utilisateur.
- Ne jamais désactiver `online-mode` ni la whitelist par défaut.
- Ne pas copier de code ou d'assets de Modrinth ou d'un autre projet sans vérifier la compatibilité de licence.
- En cas de doute sur une API externe, consulter sa documentation officielle plutôt que de supposer.
