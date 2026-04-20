# Stack : shwip

> Choix techniques argumentes.

## Stack retenue

| Couche | Technologie | Version |
|--------|-------------|---------|
| Frontend | React + TypeScript | React 19, TS 5.7 |
| Bundler | Vite | 6.x |
| Desktop | Tauri 2 | 2.x |
| Backend | Rust | edition 2021 |
| Scan engine | Rust (sysinfo, dirs, fs) | - |
| LLM embarque | llama-cpp-rs (optionnel) | tiny model ~500Mo |
| LLM externe | Ollama REST API (optionnel) | localhost:11434 |
| Tests frontend | Vitest | 2.x |
| Tests backend | cargo test | natif |
| Lint/Format | Biome (TS) + clippy/rustfmt (Rust) | - |
| Distribution | Homebrew tap + GitHub Releases + .dmg | - |

## Decisions argumentees

### Tauri 2 plutot que SwiftUI ou Electron

Choix : Tauri 2 (React frontend + Rust backend)
Raisons :
- Equipe expertise JS/TS, pas Swift
- Binaire leger (~8 Mo vs ~150 Mo Electron)
- Rust pour le scan engine : acces filesystem rapide, safe, single binary
- Tauri v2 supporte tray icon (menu bar) natif macOS
- Cross-platform futur (Linux devs ont les memes problemes NVM/npm/cargo/Ollama)
- Pas besoin de Xcode

Alternatives ecartees :
- SwiftUI : Xcode obligatoire, expertise manquante, macOS-only, LLM C interop penible
- Electron : 150 Mo RAM idle, contradictoire pour un outil de nettoyage
- Go + TUI : pas de GUI native, APIs macOS limitees

### LLM hybride : embarque + externe

Choix : llama-cpp-rs pour tiny model embarque + Ollama REST API pour modeles plus capables
Raisons :
- llama-cpp-rs est mature en Rust, bindings stables
- Tiny model embarque (~500 Mo, qwen3:0.6b ou phi-3-mini) : zero dependance externe
- Ollama pour les users qui veulent un modele plus capable
- Fallback gracieux : explications codees si aucun LLM disponible

Architecture LLM :
```
User preference
  ├─ "embedded" → llama-cpp-rs + tiny model bundle
  ├─ "ollama"   → REST API localhost:11434
  └─ "none"     → explications deterministes (defaut)
```

### CLI + GUI

Choix : binaire unique avec sous-commandes CLI + fenetre Tauri
Raisons :
- `shwip scan` en CLI pour les power users et CI
- `shwip` sans argument ouvre la fenetre Tauri
- Meme moteur Rust, zero duplication

## Infrastructure

| Service | Usage | Provider |
|---------|-------|----------|
| CI/CD | Tests + build + release | GitHub Actions |
| Distribution | Homebrew tap + .dmg | GitHub Releases |
| Binaires | Universal macOS (arm64 + x86_64) | GitHub Actions |

## Commandes

```bash
# Dev
bun install
bun run tauri dev

# Tests
bun run test              # frontend
cd src-tauri && cargo test # backend

# Build
bun run tauri build

# CLI only (sans UI)
cd src-tauri && cargo build --release
./target/release/shwip scan
```
