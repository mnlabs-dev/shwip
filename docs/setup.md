# Setup developpement

## Prerequis

- Rust : `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Bun : `curl -fsSL https://bun.sh/install | bash`
- CommandLineTools : `xcode-select --install`

### Verification

```bash
cargo --version    # 1.88+
bun --version      # 1.3+
```

## Installation

```bash
cd ~/Developer/Apps/shwip
bun install
cd src-tauri && cargo check   # premiere compilation (~2 min)
```

## Structure du projet

```
shwip/
  package.json            # Frontend deps
  vite.config.ts          # Bundler
  tsconfig.json           # TypeScript
  index.html              # Entry point Vite
  src/                    # Frontend React
    main.tsx
    App.tsx
  src-tauri/              # Backend Rust + Tauri
    Cargo.toml
    tauri.conf.json
    src/
      main.rs             # Entry point binaire
      lib.rs              # Tauri app + commandes
      scanner.rs          # Moteur de scan
    icons/                # App icons (PNG)
  docs/                   # Documentation produit
    product.md            # Vision et features
    stack.md              # Choix techniques
    inventory.md          # Features detaillees + BDD
    setup.md              # Ce fichier
  .claude/                # Config Claude Code
    CLAUDE.md             # Instructions projet
  README.md
  LICENSE                 # MIT
```

## Commandes

```bash
bun run tauri dev         # Dev mode (frontend + backend)
bun run tauri build       # Production build (.dmg)
cd src-tauri && cargo test  # Tests Rust
```

## Premiere session de dev

1. `bun run tauri dev` (verifier que l'app s'ouvre)
2. Implementer les scanners par ecosysteme dans `src-tauri/src/scanner.rs`
3. Feature par feature : residus apps → NVM → caches npm/bun/uv → Ollama
