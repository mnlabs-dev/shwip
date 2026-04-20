# Product : shwip

> Nettoyage Mac intelligent pour developpeurs. Zero cloud, zero telemetrie.

## Vision

Un Mac propre sans effort ni risque. Shwip comprend ton workflow de developpeur et nettoie ce que les outils generiques ne voient pas.

## Probleme

Les developpeurs accumulent des dizaines de Go de residus invisibles : caches de package managers (uv, npm, bun, cargo), anciennes versions NVM, modeles Ollama oublies, residus d'apps desinstallees, browsers Playwright obsoletes. CleanMyMac et AppCleaner ne comprennent pas ces ecosystemes. Le nettoyage manuel prend 30+ minutes et demande une expertise systeme.

Pour qui : developpeurs macOS (JS/TS, Python, Rust, Swift) utilisant des outils modernes.
Pourquoi maintenant : proliferation des runtimes IA locaux (Ollama, LM Studio), des version managers (NVM, uv, rustup), et des caches MCP.

## Utilisateurs cibles

| Persona | Description | Besoin principal |
|---------|-------------|------------------|
| Dev fullstack | JS/TS + Python, 2-5 projets actifs, NVM, bun, uv | Recuperer 20-50 Go sans casser son setup |
| Dev IA/ML | Ollama, modeles locaux, Docker, caches GPU | Savoir quels modeles garder, lesquels supprimer |
| Indie dev | Mac 256-512 Go, chaque Go compte | Nettoyage sur, reversible, sans expertise |

## Proposition de valeur

Shwip fait ce qu'un senior devops ferait manuellement en 30 minutes : cross-reference apps installees vs residus, verifie les process actifs et LaunchAgents, comprend les relations de supersession (qwen3:8b remplace qwen2.5-coder), et ne supprime jamais sans filet de securite (trash, dry-run, undo).

Difference vs existants :
- CleanMyMac : pattern matching statique, zero awareness dev toolchain
- AppCleaner : desinstallation seulement, pas de caches dev
- DevCleaner : Xcode only
- PureMac : generique, pas de contexte "app installee ?"
- Mole : CLI conservateur, pas d'intelligence

## Features cles

| Feature | Description | Priorite |
|---------|-------------|----------|
| Scan multi-ecosystem | NVM, npm, bun, pnpm, uv, cargo, ollama, playwright, docker | Must |
| Cross-reference residus | /Applications vs ~/Library/* + process check + LaunchAgents | Must |
| Safety net | Toujours trash (jamais rm), dry-run par defaut, undo natif | Must |
| Rapport structure | Par categorie, taille, confiance (SAFE/REVIEW/KEEP), explication | Must |
| CLI interface | `shwip scan`, `shwip clean`, `shwip report` | Must |
| Regles deterministes | 90% du raisonnement via regles codees (pas besoin de LLM) | Must |
| LLM local optionnel | Ollama pour cas ambigus et explications en langage naturel | Should |
| Menu bar app | SwiftUI, scan periodique, notifications | Should |
| Profils custom | Definir quels ecosystemes scanner, seuils | Could |
| Brewfile audit | Detecter les brew packages installes mais jamais utilises | Could |

## Metriques de succes

| Metrique | Objectif | Comment mesurer |
|----------|----------|-----------------|
| Espace recupere moyen | > 15 Go par premier scan | Telemetrie locale (opt-in) |
| Faux positifs | 0 suppression regrettee | Feedback utilisateur + undo logs |
| Temps de scan | < 30 secondes | Benchmark CI |
| Stars GitHub | 500 en 3 mois | GitHub |
| Zero incidents | Aucun rapport de perte de donnees | Issues GitHub |

## Ce que shwip N'EST PAS

- Pas un antivirus ou outil de securite
- Pas un gestionnaire d'apps (pas de desinstallation, Pearcleaner fait ca)
- Pas un outil cloud (zero telemetrie, zero compte, 100% local)
- Pas un remplacement de CleanMyMac pour les non-devs (focus workflow dev)
- Pas dependant d'un LLM (fonctionne a 100% sans Ollama, le LLM est un bonus)
