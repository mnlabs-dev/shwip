# Changelog

## v0.2.0 (2026-04-21)

### Fondations
- Architecture async Rust avec gestion d'erreurs structuree (ShwipError)
- Menu bar only (ActivationPolicy::Accessory), tray menu fonctionnel
- Plugins Tauri : store, notification, autostart, dialog, updater
- TrashManager avec crate trash (corbeille macOS native)

### Scanners (12 ecosystemes)
- Trait EcosystemScanner, registry dynamique par profil
- App residuals, NVM, npm cache, bun cache, pnpm cache
- uv/pip (venvs orphelins), cargo/rustup (toolchain active)
- Ollama models, Playwright browsers, Docker/OrbStack
- Xcode DerivedData/simulateurs, Homebrew cache

### CLI
- `shwip scan` (table + --json)
- `shwip clean --dry-run` / `--confirm`
- `shwip report --format md|json --explain`
- `shwip logs` (--clear, --lines)

### Frontend
- Dashboard React avec design system mnlabs (Teal + Blue, beige/cream)
- Sidebar categories, badges SAFE/REVIEW/KEEP, stats
- CleanFlow modal (preview, confirm, undo)
- SettingsPanel (profils, autostart, notifications, theme)
- Dark mode toggle (system/light/dark)
- Historique des 20 derniers scans

### Polish
- Scan parallele (tokio JoinSet, progress bar temps reel)
- LLM Ollama REST (explain items, fallback deterministe)
- Logging tracing (fichier ~/.shwip/logs/)
- Auto-updater GitHub Releases
- CI GitHub Actions (test + clippy + lint)
- CD release (tag v* -> .dmg)

### Tests
- 55 tests Rust (models, scanner, settings, trash, scanners, history, llm, logger)
- 14 tests frontend (App, useFilter, types)
- Clippy zero warnings, biome propre
