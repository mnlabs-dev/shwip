<critical>
Ce projet herite des principes du CLAUDE.md global. TDD, Plan Mode, Read Before Mutate, conventions commits FR, et toutes les regles de ~/.claude/rules/ s'appliquent.
</critical>

# shwip

> Nettoyage Mac intelligent pour developpeurs. Tauri 2 (React + Rust), zero cloud, zero telemetrie.

<!-- product.md et stack.md : charger a la demande uniquement (Read docs/product.md / docs/stack.md) -->

## Stack

| Cle | Valeur |
|-----|--------|
| Frontend | React 19 + TypeScript |
| Desktop | Tauri 2 |
| Backend | Rust (edition 2021) |
| Bundler | Vite 6 |
| Tests | Vitest (frontend) + cargo test (backend) |
| PM | bun |
| Dev | `bun run tauri dev` |
| Build | `bun run tauri build` |
| Lint | `biome check` (TS) + `cargo clippy` (Rust) |

## Architecture

- Scan engine en Rust : regles deterministes (90%) + LLM optionnel (10%)
- Frontend React : affichage rapport, actions clean/undo
- LLM hybride : llama-cpp-rs embarque (tiny) OU Ollama REST API OU aucun
- CLI + GUI : meme binaire, `shwip scan` (CLI) ou `shwip` (ouvre Tauri)

## Connaissance projet

Accumulee dans `.claude/memory/` (auto-apprentissage, symlink vers `~/.claude/projects/<slug>/memory/` cree par `session-start-async.sh`). 4 types :
- feedback_*.md : pieges, corrections, patterns critiques
- project_*.md : vision, decisions, stack, structure, backlog
- user_*.md : profil, preferences
- reference_*.md : liens externes, ressources

Index : `.claude/memory/MEMORY.md` (auto-cree par hook, 1 ligne par fichier, sections par type)
