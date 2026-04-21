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
  vite.config.ts          # Bundler + Tailwind CSS 4
  vitest.config.ts        # Tests frontend
  tsconfig.json           # TypeScript
  biome.json              # Lint TS/TSX
  index.html              # Entry point Vite
  src/                    # Frontend React
    main.tsx
    App.tsx               # Router dashboard/settings
    index.css             # Design system mnlabs (Tailwind @theme)
    types.ts              # Types miroir Rust
    vite-env.d.ts
    assets/               # Logos SVG light/dark
    components/
      Dashboard.tsx       # Layout principal (sidebar + stats + liste)
      ResultItem.tsx      # Ligne resultat avec badge SAFE/REVIEW/KEEP
      CleanFlow.tsx       # Modal preview + confirmation cleanup
      SettingsPanel.tsx   # Toggles ecosystemes, autostart, notifications
    hooks/
      useFilter.ts        # Filtrage categories + tri
    __tests__/
      App.test.tsx
      types.test.ts
      useFilter.test.ts
  src-tauri/              # Backend Rust + Tauri
    Cargo.toml
    tauri.conf.json
    src/
      main.rs             # CLI vs GUI detection
      lib.rs              # Tauri app + commands + plugins
      cli.rs              # clap subcommands (scan, clean, report)
      models.rs           # ScanResult, Confidence, ScanConfig
      error.rs            # ShwipError enum
      scanner.rs          # scan_all() async orchestration
      settings.rs         # tauri-plugin-store wrapper
      menu.rs             # Tray menu + ActivationPolicy::Accessory
      trash.rs            # TrashManager (crate trash)
      notifications.rs    # Notification post-scan
      scanners/
        mod.rs            # trait EcosystemScanner + registry
        app_residuals.rs  # Library/Application Support + process + LaunchAgents
        nvm.rs            # ~/.nvm/versions/node + .nvmrc cross-ref
        npm_cache.rs      # ~/.npm/_cacache (SAFE)
        bun_cache.rs      # ~/.bun/install/cache (SAFE)
        pnpm_cache.rs     # ~/Library/pnpm (SAFE)
        python.rs         # uv/pip cache + venvs orphelins
        cargo.rs          # ~/.cargo/registry + ~/.rustup/toolchains
        ollama.rs         # ~/.ollama/models + ollama list
        playwright.rs     # ms-playwright, garde latest
        docker.rs         # docker system df + ~/.orbstack
        xcode.rs          # DerivedData + simulateurs
        homebrew.rs       # brew cleanup --dry-run
    icons/                # App icons (PNG + ICNS)
  docs/                   # Documentation produit
    product.md            # Vision et features
    stack.md              # Choix techniques
    inventory.md          # Features detaillees + BDD
    setup.md              # Ce fichier
  .claude/                # Config Claude Code
    CLAUDE.md             # Instructions projet
    plans/                # Plans d'implementation
  README.md
  LICENSE                 # MIT
```

## Commandes

```bash
bun run tauri dev         # Dev mode (frontend + backend)
bun run tauri build       # Production build (.dmg)
bun run test              # Tests frontend (vitest)
bun run lint              # Lint frontend (biome)
cd src-tauri && cargo test   # Tests Rust (46 tests)
cd src-tauri && cargo clippy # Lint Rust

# CLI (apres cargo build --release)
./target/release/shwip scan              # Scan en mode CLI
./target/release/shwip scan --json       # Output JSON
./target/release/shwip clean --dry-run   # Preview
./target/release/shwip clean --confirm   # Nettoyer (trash)
./target/release/shwip report            # Rapport markdown
```

## Premiere session de dev

1. `bun run tauri dev` (verifier que l'app s'ouvre en mode menu bar)
2. Verifier le tray icon dans la barre des menus macOS
3. Cliquer "Scan Now" dans le tray menu
4. `cd src-tauri && cargo test` pour verifier les 46 tests
5. `bun run test` pour verifier les 14 tests frontend
