# shwip

Intelligent Mac cleanup for developers. Zero cloud, zero telemetry.

shwip understands your developer workflow: it knows which apps are uninstalled, which NVM versions are superseded, which Ollama models haven't been used in months, and which caches are safely reclaimable.

## Install

Download the latest `.dmg` from [Releases](https://github.com/maximenejad/shwip/releases).

## Usage

### GUI (menu bar)

shwip runs as a menu bar app (no dock icon). Click the tray icon to scan, open the dashboard, or check settings.

### CLI

```bash
shwip scan                    # scan and report reclaimable space
shwip scan --json             # JSON output for scripting
shwip clean --dry-run         # preview what would be cleaned
shwip clean --confirm         # clean SAFE items (always uses trash)
shwip report                  # detailed markdown report
shwip report --explain        # LLM-enriched report (requires Ollama)
shwip logs                    # show recent logs
shwip logs --clear            # clear log files
```

## What it scans

12 ecosystems: NVM, npm, bun, pnpm, uv/pip, cargo/rustup, Ollama, Playwright, Docker/OrbStack, Xcode, Homebrew, app residuals.

## Safety

- Always `trash`, never `rm`
- Dry-run by default
- Three confidence levels: SAFE, REVIEW, KEEP
- Cross-references installed apps, running processes, and LaunchAgents

## Stack

| Layer | Technology |
|-------|-----------|
| Frontend | React 19 + TypeScript + Tailwind CSS 4 |
| Desktop | Tauri 2 (menu bar only) |
| Backend | Rust (async, tokio) |
| LLM | Ollama REST (optional, local) |
| Tests | Vitest + cargo test (55 Rust + 14 frontend) |

## Development

```bash
bun install
bun run tauri dev
```

```bash
cd src-tauri && cargo test     # Rust tests
bun run test                   # frontend tests
bun run lint                   # biome check
```

## License

MIT
