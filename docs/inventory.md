# Inventory : shwip

> Features detaillees avec scenarios BDD.

## Ecosystemes supportes (v1)

| Ecosysteme | Ce qu'on scanne | Detection |
|------------|----------------|-----------|
| Apps macOS | ~/Library/Application Support, Containers, Group Containers, Preferences, Saved Application State | Cross-ref /Applications + process + LaunchAgents |
| NVM | ~/.nvm/versions/node/* | .nvmrc dans les projets, version courante |
| npm | ~/.npm/_cacache | `npm cache ls` |
| bun | ~/.bun/install/cache | `bun pm cache` |
| pnpm | ~/Library/pnpm | `pnpm store status` |
| uv | ~/.cache/uv | `uv cache info` |
| cargo | ~/.cargo/registry, ~/.rustup | `rustup show`, Cargo.toml dans projets |
| Ollama | ~/.ollama/models | `ollama list` (taille + derniere utilisation) |
| Playwright | ~/Library/Caches/ms-playwright | Versions multiples, garder la derniere |
| Docker/OrbStack | ~/.docker, ~/.orbstack, images | `docker images`, `docker system df` |
| Xcode | ~/Library/Developer | DerivedData, archives, simulateurs |
| Homebrew | $(brew --cache) | `brew cleanup --dry-run` |
| Python | ~/.cache/pip, virtualenvs orphelins | `uv`/`pip` installed, venvs sans projet |

## User Stories

### US1 : Scan complet [P1]
As a developer, I want to scan my Mac for reclaimable space, so that I know what can be safely cleaned.

| # | Given | When | Then |
|---|-------|------|------|
| S1 | Mac avec usage dev normal | `shwip scan` | Rapport par categorie avec taille et confiance |
| S2 | Aucun residu present | `shwip scan` | "Nothing to clean. Your Mac is tidy." |
| S3 | App encore installee avec gros cache | `shwip scan` | KEEP, pas SAFE (ne pas proposer suppression) |

Requirements: FR-001 (scan multi-ecosystem) | FR-002 (cross-reference)

### US2 : Nettoyage safe [P1]
As a developer, I want to clean identified items, so that I reclaim disk space without risk.

| # | Given | When | Then |
|---|-------|------|------|
| S1 | Scan avec 5 items SAFE | `shwip clean` | 5 items vers trash, recap avec taille |
| S2 | Scan avec items REVIEW | `shwip clean` | Seuls les SAFE sont nettoyes, REVIEW ignores |
| S3 | Premier lancement | `shwip clean` sans scan prealable | Scan automatique puis confirmation |

Requirements: FR-003 (trash only) | FR-004 (dry-run default)

### US3 : Dry run [P1]
As a developer, I want to preview what would be cleaned, so that I can verify before acting.

| # | Given | When | Then |
|---|-------|------|------|
| S1 | Items detectes | `shwip clean --dry-run` | Liste avec tailles, rien supprime |
| S2 | Dry run puis clean | `shwip clean --dry-run` puis `shwip clean --confirm` | Meme liste executee |

Requirements: FR-004

### US4 : Rapport detaille [P2]
As a developer, I want a detailed report, so that I understand what each item is and why it's safe to remove.

| # | Given | When | Then |
|---|-------|------|------|
| S1 | Scan termine | `shwip report` | Markdown avec categories, tailles, explications |
| S2 | Ollama disponible | `shwip report --explain` | Explications en langage naturel via LLM |
| S3 | Ollama absent | `shwip report --explain` | Fallback sur explications codees |

Requirements: FR-005 (rapport) | FR-006 (LLM optionnel)

### US5 : Ollama models audit [P2]
As a developer, I want to audit my Ollama models, so that I only keep the ones I need.

| # | Given | When | Then |
|---|-------|------|------|
| S1 | 5 modeles installes dont 2 non utilises 3+ mois | `shwip scan` | 2 modeles marques REVIEW avec derniere date |
| S2 | Modele supersede par version plus recente | `shwip scan` | Note "superseded by X" si LLM actif |

Requirements: FR-007

## Niveaux de confiance

| Niveau | Definition | Action par defaut |
|--------|-----------|-------------------|
| SAFE | App desinstallee + 0 process + 0 LaunchAgent, OU cache regenerable | `clean` supprime |
| REVIEW | Ancien mais potentiellement utilise (ex: NVM version sans .nvmrc) | `clean` ignore, `clean --include-review` supprime |
| KEEP | App installee, process actif, ou cache recemment utilise | Jamais propose |
