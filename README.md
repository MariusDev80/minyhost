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

## Licence

[MIT](LICENSE)
