<critical>
Ce projet herite des principes du CLAUDE.md global. TDD, Plan Mode, Read Before Mutate, conventions commits FR, et toutes les regles de ~/.claude/rules/ s'appliquent.
</critical>

# shwip

> Nettoyage Mac intelligent pour developpeurs. Tauri 2 (React + Rust), zero cloud, zero telemetrie.

## Stack

| Cle | Valeur |
|-----|--------|
| Frontend | React 19 + TypeScript + Tailwind CSS 4 |
| Desktop | Tauri 2 (menu bar only, ActivationPolicy::Accessory) |
| Backend | Rust (edition 2021) |
| Bundler | Vite 6 |
| Tests | Vitest (frontend) + cargo test (backend) |
| Lint | biome check (TS) + cargo clippy (Rust) |
| PM | bun |
| CLI | clap 4 derive (meme binaire) |
| Dev | `bun run tauri dev` |
| Build | `bun run tauri build` |
| Test Rust | `cd src-tauri && cargo test` |
| Test Front | `bun run test` |

## Architecture

```
src-tauri/src/
  main.rs             # CLI vs GUI detection (clap ou Tauri)
  lib.rs              # Tauri Builder, plugins, commands (scan, load/save_settings)
  cli.rs              # clap : scan, clean, report subcommands
  error.rs            # ShwipError enum
  models.rs           # ScanResult, Confidence, ScanConfig
  scanner.rs          # scan_all() async orchestration via scanners/
  scanners/           # 12 modules (trait EcosystemScanner)
  settings.rs         # tauri-plugin-store wrapper
  menu.rs             # tray menu + dock hidden
  trash.rs            # crate trash + undo log
  notifications.rs    # tauri-plugin-notification

src/
  App.tsx             # Router dashboard/settings
  types.ts            # Types miroir Rust
  components/         # Dashboard, ResultItem, CleanFlow, SettingsPanel
  hooks/              # useFilter (categories, tri)
  assets/             # Logos SVG light/dark
  index.css           # Design system mnlabs (Teal + Blue, beige/cream)
```

## Plugins Tauri actifs

tauri-plugin-shell, tauri-plugin-store, tauri-plugin-notification, tauri-plugin-autostart, tauri-plugin-dialog

## Design system

Palette mnlabs : Teal #38786e (primaire shwip), Blue #375da2 (CTA), fond beige #F4F0E8 light / #161514 dark. Badges : green #48783e (SAFE), orange #b87a20 (REVIEW). Fonts : Instrument Sans (UI) + Newsreader (titres). Ref : `/Users/maximenejad/Local/logo-inspirations/propositions/brand-colors.html`

## Avancement

| Phase | Statut | Branche |
|-------|--------|---------|
| P1 Fondations (async, plugins, tray, tests) | Terminee | feature/foundations |
| P2 Scanners (12 ecosystemes TDD) | Terminee | feature/foundations |
| P3 CLI + Frontend (clap, dashboard, brand) | Terminee | feature/foundations |
| P4 Polish (LLM, updater, dark toggle, CI) | A faire | -- |

Plan complet : `.claude/plans/keen-greeting-hippo.md`

## Connaissance projet

Accumulee dans `.claude/memory/` (symlink). Index : `.claude/memory/MEMORY.md`.
