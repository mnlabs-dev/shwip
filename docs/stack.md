# Stack : shwip

> Choix techniques argumentes.

## Stack retenue

| Couche | Technologie | Version |
|--------|-------------|---------|
| Langage | Swift | 6.2 |
| UI (future) | SwiftUI | macOS 14+ |
| Package manager | Swift Package Manager | natif |
| Tests | Swift Testing | natif |
| Lint/Format | swift-format | natif |
| LLM local | Ollama (optionnel) | API REST localhost:11434 |
| Distribution | Homebrew tap + GitHub Releases | - |

## Decisions argumentees

### Swift plutot que Rust ou TypeScript

Choix : Swift avec SPM
Raisons :
- Acces natif aux APIs macOS (NSWorkspace, FileManager, LaunchServices) sans FFI
- SwiftUI pour le menu bar app futur sans couche supplementaire
- Swift Testing integre, pas besoin de framework externe
- Le projet est macOS-only, pas de portabilite necessaire
- Swift 6.2 disponible via CommandLineTools (pas besoin de Xcode complet)

Alternatives ecartees :
- Rust : excellent pour CLI cross-platform, mais FFI complexe pour macOS APIs (NSWorkspace, app bundles). Le projet est macOS-only.
- TypeScript/Bun : rapide a prototyper, mais pas d'acces natif aux APIs macOS. Dependrait de `mdfind`, `lsappinfo` via shell.
- Go : meme probleme que Rust pour les APIs macOS natives.

### Architecture : regles deterministes + LLM optionnel

Choix : moteur de regles codees en Swift, LLM via Ollama REST API en option
Raisons :
- 90% des cas sont deterministes (cross-reference /Applications, check mtime, parse manifests)
- Le LLM ajoute : explications humaines, detection de supersession (qwen3 > qwen2.5), cas ambigus
- Zero dependance obligatoire sur un LLM : fonctionne offline, rapide, predictible
- Ollama REST API est triviale (POST localhost:11434/api/generate)

Alternatives ecartees :
- LLM-first (tout passe par le LLM) : lent, imprevisible, necessite un modele installe
- ML embeddings pour similarite : over-engineering pour ce use case

### CLI-first, UI ensuite

Choix : CLI comme produit principal, menu bar app comme extension future
Raisons :
- Les devs vivent dans le terminal
- Testable, scriptable, CI-friendly
- Le menu bar app reutilise le meme moteur via Process ou framework partage
- Plus rapide a shipper un CLI qu'une app signee/notariee

### Distribution : Homebrew tap

Choix : `brew install shwip/tap/shwip`
Raisons :
- Standard pour les CLIs macOS dans la communaute dev
- Pas besoin de signature Apple pour un CLI
- GitHub Releases pour les binaires universels (arm64 + x86_64)

## Infrastructure

| Service | Usage | Provider |
|---------|-------|----------|
| CI/CD | Tests + build + release | GitHub Actions |
| Distribution | Homebrew tap | GitHub (shwip/homebrew-tap) |
| Binaires | Universal (arm64+x86_64) | GitHub Releases |

## Commandes

```bash
# Dev
swift build

# Tests
swift test

# Run
swift run shwip scan
swift run shwip clean --dry-run
swift run shwip report

# Release build
swift build -c release
```
