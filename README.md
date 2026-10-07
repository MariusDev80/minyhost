# MinyHost

> Héberge ton serveur Minecraft sur ton PC, en quelques clics.

**Statut : en développement** — rien n'est encore utilisable.

MinyHost est une application Windows open-source qui permet de créer et d'héberger un serveur Minecraft (Vanilla, Paper, Fabric) sur son propre PC, sans rien installer à la main.

## Développement

Prérequis : Node.js LTS, Rust (toolchain stable MSVC), Visual Studio Build Tools 2022 (C++), WebView2.

```bash
npm install          # Dépendances frontend
npm run tauri dev    # Lancer l'app en développement
npm run tauri build  # Construire l'installeur
npm run lint         # ESLint
npm run format       # Prettier
cargo clippy --manifest-path src-tauri/Cargo.toml
```

## Publier une version

Le workflow [`release.yml`](.github/workflows/release.yml) compile l'installeur Windows sur GitHub Actions.

1. Mettre à jour la version dans `package.json`, `src-tauri/tauri.conf.json` et `src-tauri/Cargo.toml`.
2. Pousser un tag identique : `git tag v0.2.0 && git push origin v0.2.0`.
3. Une release **brouillon** est créée avec l'installeur `MinyHost_<version>_x64-setup.exe` : la relire sur GitHub puis la publier.

Pour tester une compilation sans release : onglet **Actions** > **Release** > **Run workflow**, l'installeur est téléchargeable dans les *Artifacts* du run.

## Licence

[MIT](LICENSE)
