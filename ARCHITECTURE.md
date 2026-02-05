Documentation de l'Architecture de Waylestia
Suite bureautique Linux moderne construite sur Wayland/Hyprland et wayfire prochainement avec un cœur en Rust, une interface GTK, et une communication IPC via Protocol Buffers.
Architecture Générale
┌─────────────────────────────────────────────────────────────────┐
│                     Suite Bureautique Waylestia                 │
└─────────────────────────────────────────────────────────────────┘

Applications (GJS + GTK)
├── Navigateur (basé sur Servo)
├── Calendrier (GTK)
├── Éditeur (Servo + CodeMirror)
├── Fichiers (GTK)
├── Mail (GTK + IMAP/SMTP)
├── Média (GTK + PipeWire)
├── Paramètres (interface Servo)
└── Terminal (GTK + Pty)

        ↓ IPC Protobuf (sockets Unix)

┌─────────────────────────────────────────────────────────────────┐
│              Shell Waylestia (JavaScript/Deno)                  │
│  - Barre des tâches, panneau, gestion du bureau                 │
│  - Opérations de fenêtres via protobuf                          │
│  - Gestion des widgets                                          │
└─────────────────────────────────────────────────────────────────┘

        ↓ IPC Protobuf

┌─────────────────────────────────────────────────────────────────┐
│                   Cœur Waylestia (Rust)                         │
│  ├─ IPC Hyprland & gestion des fenêtres                         │
│  ├─ Surveillance des performances (CPU/GPU/RAM)                 │
│  ├─ Contrôle média (PipeWire)                                   │
│  ├─ Sécurité & permissions                                      │
│  ├─ Gestion des paramètres (remplace GSettings)                 │
│  └─ Notifications système                                       │
└─────────────────────────────────────────────────────────────────┘

        ↓ IPC Hyprland

┌─────────────────────────────────────────────────────────────────┐
│              Compositeur Wayland Hyprland                       │
└─────────────────────────────────────────────────────────────────┘
Détails des Composants
1. Daemon Principal (/core)
Langage : Rust (asynchrone avec Tokio)
Responsabilités :

Gérer la communication avec Hyprland (liste des fenêtres, changements d'espace de travail)
Surveiller les performances système (CPU, GPU, RAM, temps de fonctionnement)
Contrôler les périphériques audio via PipeWire
Gérer les permissions et politiques de sécurité
Fournir une interface IPC unifiée via protobuf

Modules Principaux :
core/src/
├── lib.rs          # Exports de la bibliothèque
├── main.rs         # Point d'entrée du daemon
├── hyprland.rs     # Intégration avec Hyprland
├── ipc.rs          # Serveur IPC (socket Unix)
├── state.rs        # Gestion de l'état global
├── perf.rs         # Surveillance des performances
├── media.rs        # Gestion des périphériques audio
└── security.rs     # Permissions & journalisation
Protocoles IPC :

core_runtime.proto - Cœur ↔ Runtime Deno
core_shell.proto - Définition du service RPC du Cœur

2. Moteur de Widgets (/widgets)
Langage : Rust (asynchrone avec Tokio)
Responsabilités :

Découvrir et charger les manifestes de widgets depuis assets/widgets/
Gérer le cycle de vie des widgets (créer, afficher, masquer, détruire)
Communiquer avec la webview Servo
Transmettre les messages des widgets au shell/cœur

Modules Principaux :
widgets/src/
├── lib.rs       # Exports de la bibliothèque
├── main.rs      # Point d'entrée du daemon
├── manifest.rs  # Lecture des manifestes (TOML)
├── loader.rs    # Découverte & chargement des widgets
├── state.rs     # État des instances de widgets
├── renderer.rs  # Préparation des widgets pour l'affichage
└── ipc.rs       # Serveur IPC des widgets
Protocoles IPC :

runtime_widgets.proto - Runtime ↔ Widgets Flutter
shell_widgets.proto - Shell ↔ Widgets Servo

3. Applications (/apps)
Langage : TypeScript/JavaScript (GJS)
Framework : GTK 4 + Adwaita (via GObject Introspection)
Les applications communiquent avec le cœur via IPC protobuf :

Navigateur - Navigation web avec Servo
Calendrier - Gestion du calendrier et des événements
Éditeur - Éditeur de texte avec coloration syntaxique
Fichiers - Gestionnaire de fichiers
Mail - Client email (IMAP/SMTP)
Média - Lecteur audio/vidéo
Paramètres - Configuration système
Terminal - Émulateur de terminal

Protocole IPC :

apps.proto - Applications ↔ Cœur

4. Widgets (/assets/widgets)
Format : HTML/CSS/JavaScript + manifeste TOML
Rendu : Moteur webview Servo
Chaque widget contient :
widget-id/
├── manifest.toml  # Métadonnées & permissions du widget
├── index.html     # Fichier HTML principal
├── style.css      # Styles (optionnel)
└── script.js      # Logique (optionnel)
Widgets Inclus :

dashboard - Accès rapide et statistiques système
clock - Horloge analogique/numérique
sysinfo - Informations système en temps réel

5. Webview (/webview)
Moteur : Servo (modifié)
Runtime JavaScript : GJS (GObject JavaScript)
Bindings : Pont IPC Waylestia
Fonctionnalités :

Rendu HTML/CSS/JS personnalisé
Accès système via les APIs GJS
Pont IPC vers le cœur/shell
Isolation de sécurité (sandboxing)

6. Protocol Buffers (/protobuf)
Spécification centralisée des messages pour tous les composants :

core_runtime.proto - Performances, média, événements fenêtres
core_shell.proto - Définitions des services RPC
runtime_widgets.proto - Cycle de vie des widgets
shell_widgets.proto - Gestion de l'affichage des widgets
apps.proto - Intégration des applications

Avantages par rapport à D-Bus :

Typage fort (validation du schéma)
Meilleures performances (encodage binaire)
Pas de dépendance à un daemon (client-serveur direct)
Compatibilité entre versions
Définitions d'API propres

7. Scripts (/scripts)
Automatisation et gestion des services :

install.sh - Compiler et tout installer
uninstall.sh - Supprimer l'installation
start.sh - Démarrer les services
stop.sh - Arrêter les services
restart.sh - Redémarrer les services

Utilise les services utilisateur systemd pour la gestion automatique.
Exemples de Flux de Données
Exemple 1 : Obtenir les Statistiques de Performance
Interface Shell
  └─> sendMessage('GetPerfStats')
      └─> Cœur (via IPC protobuf)
          └─> bibliothèque sysinfo (perf.rs)
          └─> returnPerfStats()
      └─> Le Shell reçoit les stats
      └─> Mise à jour de la barre d'état
Exemple 2 : Lancer une Application
L'utilisateur clique sur une app dans le menu
  └─> Le Shell envoie : requête LaunchApp au Cœur
  └─> Cœur (apps.proto)
      └─> Vérifier les permissions (security.rs)
      └─> Véork du processus + surveillance
      └─> Envoie l'événement app_started au shell
  └─> Le Shell attend l'événement fenêtre
  └─> La fenêtre Hyprland apparaît
  └─> Le Shell met à jour la liste des apps
Exemple 3 : Rendu d'un Widget
L'utilisateur fait un clic droit sur le bureau
  └─> Shell : RegisterWidget('sysinfo')
  └─> Cœur (daemon widgets)
      └─> Charger manifest.toml
      └─> Lancer une instance Servo
      └─> Initialisation du pont IPC
  └─> Servo charge index.html
  └─> JavaScript initialise window.waylestia
  └─> Le widget envoie des requêtes IPC au cœur pour les données
  └─> Le cœur répond via protobuf
  └─> Le widget s'affiche en temps réel
Couches de Compatibilité
Intégration GNOME

GSettings → Protobuf ConfigRequest
D-Bus → Protobuf IPCRequest (message DBusCompat)
Notifications → Message Protobuf Notification

Intégration KDE

Services KDE Plasma → Appels de service Protobuf
KConfig → Protobuf ConfigRequest
KNotifications → Message Protobuf Notification

Modèle de Sécurité

Isolation (Sandboxing) : Les apps demandent des permissions (système de fichiers, réseau, etc.)
Journalisation : Toutes les demandes de permissions sont enregistrées
AppArmor/Seccomp : Isolation supplémentaire via Linux
Validation IPC : Tous les messages sont vérifiés via protobuf
Approbation Utilisateur : Autorisations interactives pour les opérations sensibles

Caractéristiques de Performance
Composant Latence Mémoire Notes Démarrage du cœur< 1s~20 Mo Minimal, asynchrone Rendu widget< 500ms~50-150 Mo Par instance Message IPC< 10msN/ASocket Unix Mise à jour perf~100msN/A Toutes les secondes Lancement app1-3sVariable Dépend de l'app
Feuille de Route

Transparence Réseau - Hyprland distant via protocole Wayland
Accélération GPU - Vulkan/OpenGL pour les widgets
Multi-Écrans - Réseau mesh pour les écrans
WebAssembly - Modules WASM dans les widgets
Rechargement à Chaud - Hot reload pour le développement de widgets
Flutter Desktop - Intégration native des apps Flutter

Workflow de Développement
bash# Tout compiler
./scripts/install.sh

# Lancer en développement
systemctl --user start waylestia-core
systemctl --user start waylestia-widgets

# Voir les logs
journalctl --user -u waylestia-core -f

# Modifier le code et recompiler
cd core && cargo build --release && cd ..

# Redémarrer le service
systemctl --user restart waylestia-core
Emplacements des Fichiers (Après Installation)
/usr/local/bin/
├── waylestia-core     # Daemon principal
├── waylestia-widgets  # Moteur de widgets
└── waylestia-shell    # Shell (Deno)

/usr/local/share/waylestia/
├── assets/
│   ├── icons/
│   ├── wallpapers/
│   └── widgets/
│       ├── dashboard/
│       ├── clock/
│       └── sysinfo/
├── apps/
│   ├── browser/
│   ├── calendar/
│   └── ...
└── protocols/
    ├── core_runtime.proto
    ├── core_shell.proto
    └── ...

~/.config/systemd/user/
├── waylestia-core.service
└── waylestia-widgets.service
Conclusion
Waylestia offre un environnement de bureau moderne et modulaire qui :
✅ Remplace GNOME/KDE avec une architecture plus simple
✅ Utilise Rust pour la performance et la sécurité
✅ Implémente une IPC propre basée sur protobuf
✅ Maintient la compatibilité GTK
✅ Supporte des widgets personnalisés via Servo + GJS
✅ Priorise la sécurité grâce à l'isolation (sandboxing)
✅ Permet l'accélération matérielle
Une suite bureautique Linux complète et cohérente pour l'ère Wayland.
