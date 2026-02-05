Guide de développement de Waylestia
Configuration système requise

OS : Linux (Ubuntu 20.04+, Fedora 36+ ou équivalent)

Noyau : 5.14+ (pour le support Wayland)

RAM : 4 Go minimum, 8 Go recommandés

Disque : 10 Go d’espace libre

Dépendances d’exécution
Ubuntu / Debian
sudo apt-get install -y \
  build-essential \
  pkg-config \
  git \
  curl \
  libgtk-4-dev \
  libadwaita-1-dev \
  libglib2.0-dev \
  libssl-dev \
  pkg-config \
  libsystemd-dev \
  gjs \
  gir1.2-gtk-4.0 \
  gir1.2-adwaita-1
Fedora / RHEL
sudo dnf install -y \
  gcc \
  make \
  pkg-config \
  git \
  curl \
  gtk4-devel \
  libadwaita-devel \
  glib2-devel \
  openssl-devel \
  systemd-devel \
  gjs \
  gjs-devel \
  gobject-introspection-devel
Installation de Rust

Installez Rust avec rustup (si ce n’est pas déjà fait) :

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

Mettre Rust à jour :

rustup update stable

Vérifier l’installation :

rustc --version
cargo --version
Démarrage rapide
Cloner et compiler
# Cloner le dépôt
git clone https://github.com/yourusername/waylestia.git
cd waylestia


# Compiler tous les composants
make all


# Ou utiliser cargo directement
cargo build --release --workspace
Installation locale
# Installer dans ~/.local (recommandé pour le développement)
INSTALL_PREFIX=$HOME/.local make install


# Ou utiliser le script d’installation
./scripts/install.sh --user


# Démarrer les services
systemctl --user start waylestia-core
systemctl --user start waylestia-widgets
Vérifier l’installation
# Vérifier que les services systemd sont actifs
systemctl --user status waylestia-core
systemctl --user status waylestia-widgets


# Voir les logs
journalctl --user -u waylestia-core -f
journalctl --user -u waylestia-widgets -f
Structure du projet
waylestia/
├── core/                      # Daemon Rust principal
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs           # Point d’entrée
│   │   ├── lib.rs            # Export des modules
│   │   ├── hyprland.rs       # Intégration Wayland
│   │   ├── ipc.rs            # Serveur IPC
│   │   ├── perf.rs           # Surveillance des performances
│   │   ├── media.rs          # Contrôle audio
│   │   ├── security.rs       # Système de permissions
│   │   └── state.rs          # État global
│   └── build.rs              # Script de build protobuf
│
├── widgets/                   # Moteur et runtime des widgets
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs           # Daemon widgets
│   │   ├── lib.rs            # Export des modules
│   │   ├── manifest.rs       # Parsing des manifests TOML
│   │   ├── loader.rs         # Découverte des widgets
│   │   ├── renderer.rs       # Rendu HTML
│   │   ├── state.rs          # Gestion de l’état des widgets
│   │   └── ipc.rs            # Serveur IPC widgets
│   └── build.rs              # Script de build
│
├── apps/                      # Applications GJS
│   ├── browser/main.ts       # Navigateur web
│   ├── calendar/main.ts      # Calendrier
│   ├── editor/main.ts        # Éditeur de texte
│   ├── files/main.ts         # Gestionnaire de fichiers
│   ├── mail/main.ts          # Client mail
│   ├── media/main.ts         # Lecteur multimédia
│   ├── settings/main.ts      # Paramètres
│   └── terminal/main.ts      # Terminal
│
├── assets/                    # Ressources statiques
│   ├── icons/
│   ├── wallpapers/
│   └── widgets/              # Widgets utilisateur
│
├── protobuf/                  # Définitions des protocoles
│
├── webview/                   # Intégration webview
│
├── scripts/                   # Scripts de build et d’installation
│
├── Makefile
├── README.md
├── ARCHITECTURE.md
└── CONTRIBUTING.md
Compilation des composants
Tout compiler
make all
# ou
cargo build --release --workspace
Compiler le daemon core
make core
cd core && cargo build --release
cd core && cargo build        # debug
Compiler le moteur de widgets
make widgets
cd widgets && cargo build --release
Vérification sans build
make check
cargo check --workspace
Nettoyage
make clean
cargo clean
Exécution et tests
Lancer les tests
make test
cargo test --workspace
cd core && cargo test
cargo test -- --nocapture
Lancer les services
make start
make stop
make restart
Voir les logs
journalctl --user -u waylestia-core -f
journalctl --user -u waylestia-widgets -f
journalctl --user -u 'waylestia-*' -f
Lancer manuellement
cd core && cargo run --release
cd widgets && cargo run --release
gjs apps/browser/main.ts
Organisation du code
Modules Rust (core)
pub mod hyprland;
pub mod ipc;
pub mod perf;
pub mod media;
pub mod security;
pub mod state;
Modules Rust (widgets)
pub mod manifest;
pub mod loader;
pub mod renderer;
pub mod state;
pub mod ipc;
Ajouter un nouveau module

Créer src/new_module.rs

Ajouter dans lib.rs : pub mod new_module;

Implémenter le code

Exporter si nécessaire

Ajouter des commentaires ///

Débogage
Activer les logs debug
RUST_LOG=debug cargo run --release
Macro dbg!
dbg!(value);
Débogage avec GDB
cargo build
gdb ./target/debug/waylestia-core
Débogage VS Code

Installer CodeLLDB

Configurer launch.json

Appuyer sur F5

Profilage des performances
Flamegraph
cargo install flamegraph
cargo flamegraph --bin waylestia-core
Perf
perf record -g ./target/release/waylestia-core
perf report
Mémoire
valgrind --leak-check=full ./target/release/waylestia-core
Problèmes courants
Erreur protobuf
sudo apt-get install protobuf-compiler   # Ubuntu
sudo dnf install protobuf-compiler       # Fedora
Headers GTK manquants
sudo apt-get install libgtk-4-dev libadwaita-1-dev
Services qui ne démarrent pas
systemctl --user status waylestia-core
journalctl --user -u waylestia-core -n 20
Problème de permissions
INSTALL_PREFIX=$HOME/.local ./scripts/install.sh
Conseils de développement
Workflow efficace
cargo watch -x build --workspace
cargo watch -x test --workspace
Style de code
cargo fmt --all
cargo clippy -- -D warnings
Documentation
cargo doc --no-deps --open
Workflow Git
git checkout -b feature/ma-feature
git commit -m "Ajout de ma fonctionnalité"
git rebase origin/main
git push origin feature/ma-feature

Bon développement !
Pour toute question, ouvrez une issue ou rejoignez les discussions de la communauté.
