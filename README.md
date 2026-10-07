# MinyHost

> Héberge ton serveur Minecraft sur ton PC, en quelques clics.

MinyHost est une application Windows gratuite et open-source qui crée et héberge un serveur Minecraft Java sur ton propre PC, pour jouer avec tes amis. Tu n'installes rien à la main : ni Java, ni serveur, ni outil réseau.

**Statut : en développement actif.** Les fonctionnalités ci-dessous sont utilisables ; d'autres arrivent (voir [Feuille de route](#feuille-de-route)).

## Télécharger

Télécharge `MinyHost_<version>_x64-setup.exe` dans la [dernière release](https://github.com/MariusDev80/minyhost/releases/latest), puis lance-le. L'installation ne demande pas de droits administrateur.

Configuration : Windows 10 ou 11 (64 bits).

## Fonctionnalités

- **Création en quelques clics** : Vanilla, Paper (plugins) ou Fabric (mods), toutes les versions de Minecraft proposées par ces projets.
- **Java automatique** : la bonne version de Java (Temurin) est téléchargée pour chaque serveur. Le Java installé sur ton PC n'est jamais utilisé ni modifié.
- **Téléchargements vérifiés** : chaque fichier téléchargé est contrôlé par son empreinte (hash) quand la source la fournit.
- **Démarrage, arrêt et console** : suis ce qui se passe sur le serveur et envoie des commandes.
- **Whitelist et opérateurs** : choisis qui peut rejoindre et qui peut utiliser les commandes.
- **Paramètres du monde et règles du jeu** : difficulté, mode de jeu, gamerules… sans éditer de fichier.
- **Jouer avec des amis à distance** : accès Internet via [playit.gg](https://playit.gg), avec ton propre compte playit.gg (gratuit).
- **Sûr par défaut** : whitelist activée et `online-mode=true` (seuls les comptes Minecraft officiels peuvent se connecter). Rien n'est ouvert sur Internet sans ton action.
- Interface en français et en anglais.

Chaque serveur vit dans son propre dossier (`%APPDATA%\MinyHost\servers\`) : tu peux le sauvegarder ou le copier comme tu veux.

L'EULA de Minecraft t'est présentée à la création de chaque serveur. MinyHost ne l'accepte jamais à ta place.

## Feuille de route

- Adresse locale (LAN) et règle de pare-feu Windows
- Sauvegardes automatiques du monde
- Mods et plugins depuis Modrinth
- Mises à jour automatiques de l'application

## Code signing policy

Free code signing provided by [SignPath.io](https://about.signpath.io), certificate by [SignPath Foundation](https://signpath.org).

Only binaries built from this repository by the [release workflow](.github/workflows/release.yml) on GitHub Actions are signed. Each signing request is approved manually by an approver.

### Team roles

- Committers and reviewers: [MariusDev80](https://github.com/MariusDev80)
- Approvers: [MariusDev80](https://github.com/MariusDev80)

### Privacy policy

This program will not transfer any information to other networked systems unless specifically requested by the user or the person installing or operating it.

MinyHost has no telemetry, no analytics and no account of its own. It only contacts the following services, as a direct result of user actions:

| Service                                                                         | When                                                                                           | Data sent                                                                                                                           |
| ------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Mojang (`piston-meta.mojang.com`)                                               | Listing Minecraft versions, creating a server                                                  | None (public downloads)                                                                                                             |
| PaperMC (`fill.papermc.io`), FabricMC (`meta.fabricmc.net`)                     | Creating a Paper or Fabric server                                                              | None (public downloads)                                                                                                             |
| Adoptium (`api.adoptium.net`)                                                   | Downloading Java for a server                                                                  | None (public downloads)                                                                                                             |
| Mojang (`api.mojang.com`, `sessionserver.mojang.com`, `textures.minecraft.net`) | Adding a player to the whitelist or the operators                                              | The Minecraft username entered                                                                                                      |
| playit.gg (`api.playit.gg`)                                                     | Only after the user links their own playit.gg account and enables Internet access for a server | Tunnel settings and the game traffic relayed for that server. See the [playit.gg privacy policy](https://playit.gg/privacy-policy). |

The Minecraft server itself, once started, connects to Mojang to authenticate players (`online-mode`), as any Minecraft server does.

## Développement

Prérequis : Node.js LTS, Rust (toolchain stable MSVC), Visual Studio Build Tools 2022 (C++), WebView2.

```bash
npm install          # Dépendances frontend
npm run tauri dev    # Lancer l'app en développement
npm run tauri build  # Construire l'installeur
npm run lint         # ESLint
npm run format       # Prettier
cargo clippy --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml
```

Stack : Tauri 2, Rust, React + TypeScript, Tailwind CSS, shadcn/ui.

## Publier une version

Le workflow [`release.yml`](.github/workflows/release.yml) compile l'installeur Windows sur GitHub Actions.

1. Mettre à jour la version dans `package.json`, `src-tauri/tauri.conf.json` et `src-tauri/Cargo.toml`.
2. Pousser un tag identique : `git tag v0.2.0 && git push origin v0.2.0`.
3. Une release **brouillon** est créée avec l'installeur `MinyHost_<version>_x64-setup.exe` : la relire sur GitHub puis la publier.

Pour tester une compilation sans release : onglet **Actions** > **Release** > **Run workflow**, l'installeur est téléchargeable dans les _Artifacts_ du run.

## Licence

[MIT](LICENSE). MinyHost intègre l'agent [playit.gg](https://github.com/playit-cloud/playit-agent) (BSD-2-Clause).

MinyHost n'est pas affilié à Mojang Studios ni à Microsoft. Minecraft est une marque de Mojang AB.
