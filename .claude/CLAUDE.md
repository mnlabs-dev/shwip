<critical>
Ce projet herite des principes du CLAUDE.md global. TDD, Plan Mode, Read Before Mutate, conventions commits FR, et toutes les regles de ~/.claude/rules/ s'appliquent.
</critical>

# shwip

> Nettoyage Mac intelligent pour developpeurs. CLI Swift, zero cloud, zero telemetrie.

<!-- product.md et stack.md : charger a la demande uniquement (Read docs/product.md / docs/stack.md) -->

## Stack

| Cle | Valeur |
|-----|--------|
| Langage | Swift 6.2 |
| Runtime | macOS 14+ (Sonoma) |
| Tests | Swift Testing |
| PM | Swift Package Manager |
| Build | `swift build` |
| Test | `swift test` |
| Run | `swift run shwip scan` |
| Lint | `swift-format` |

## Architecture

Moteur de regles deterministes (90%) + LLM local optionnel via Ollama REST API (10%).
CLI-first. Menu bar SwiftUI futur.

## Connaissance projet

Accumulee dans `.claude/memory/` (auto-apprentissage, symlink vers `~/.claude/projects/<slug>/memory/` cree par `session-start-async.sh`). 4 types :
- feedback_*.md : pieges, corrections, patterns critiques
- project_*.md : vision, decisions, stack, structure, backlog
- user_*.md : profil, preferences
- reference_*.md : liens externes, ressources

Index : `.claude/memory/MEMORY.md` (auto-cree par hook, 1 ligne par fichier, sections par type)
