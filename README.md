# shwip

Intelligent Mac cleanup for developers. Zero cloud, zero telemetry.

shwip understands your developer workflow: it knows which apps are uninstalled, which NVM versions are superseded, which Ollama models haven't been used in months, and which caches are safely reclaimable.

## Install

```bash
brew install shwip/tap/shwip
```

## Usage

```bash
shwip scan              # Scan and report reclaimable space
shwip clean --dry-run   # Preview what would be cleaned
shwip clean --confirm   # Clean SAFE items (always uses trash)
shwip report            # Detailed markdown report
```

## What it scans

NVM, npm, bun, pnpm, uv, cargo, Ollama, Playwright, Docker, Xcode, Homebrew, app residuals.

## Safety

- Always `trash`, never `rm`
- Dry-run by default
- Three confidence levels: SAFE, REVIEW, KEEP
- Cross-references installed apps, running processes, and LaunchAgents

## License

MIT
