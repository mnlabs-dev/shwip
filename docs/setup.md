# Setup developpement

## Prerequis

### Xcode (obligatoire)

Les CommandLineTools seuls ne suffisent pas pour Swift Package Manager + SwiftUI.

```bash
# Option 1 : Installer Xcode depuis l'App Store (gratuit, ~12 Go)
# Puis :
sudo xcode-select -s /Applications/Xcode.app/Contents/Developer

# Option 2 : Reinstaller les CommandLineTools (si pas besoin de SwiftUI)
sudo rm -rf /Library/Developer/CommandLineTools
xcode-select --install
```

Probleme actuel : Swift compiler 6.2.3 vs SDK 6.2 (mismatch). Xcode resout ce conflit.

### Verification

```bash
swift --version          # Swift 6.2+
swift build              # Doit compiler sans erreur
swift test               # Tests passent
```

## Structure du projet

```
shwip/
  Package.swift           # SPM manifest
  Sources/shwip/          # Code source
    main.swift            # Point d'entree CLI
  Tests/shwipTests/       # Tests
  docs/                   # Documentation produit
    product.md            # Vision et features
    stack.md              # Choix techniques
    inventory.md          # Features detaillees + BDD
    setup.md              # Ce fichier
  .claude/                # Config Claude Code
    CLAUDE.md             # Instructions projet
  README.md
  LICENSE                 # MIT
```

## Premiere session de dev

1. Installer Xcode ou mettre a jour CommandLineTools
2. `cd ~/Developer/Apps/shwip`
3. `swift build` (verifier que ca compile)
4. `swift test` (verifier que les tests passent)
5. Commencer par le scan des residus d'apps (feature la plus simple et la plus impactante)
