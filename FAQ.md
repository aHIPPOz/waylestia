FAQ Waylestia

Foire aux questions concernant l’environnement de bureau Waylestia.

Questions générales
Qu’est-ce que Waylestia ?

Waylestia est un environnement de bureau moderne pour Linux, conçu nativement pour Wayland, avec les caractéristiques suivantes :

Pile moderne : écrit principalement en Rust (sécurité mémoire, performances)

Natif Wayland : aucune dépendance à X11

Léger : objectif de consommation de ressources réduite

Modulaire : composants développés et exécutés séparément

Personnalisable : système de widgets et de configuration extensible

Orienté développeurs : architecture conçue pour être comprise et modifiée

Comment Waylestia se compare-t-il à KDE Plasma et GNOME ?

⚠️ Les valeurs ci-dessous sont indicatives et dépendent fortement de la configuration système, des versions et des extensions utilisées.
Elles ne constituent pas des benchmarks officiels.

Fonctionnalité	Waylestia (objectif)	KDE Plasma	GNOME
Langage principal	Rust	C++	C
Taille installée	~50–100 Mo	~500 Mo+	~300 Mo+
Mémoire au repos	~80 Mo	~400 Mo	~300 Mo
Démarrage	< 3 s	~5 s	~4 s
Widgets/Applets	Oui (Rust / HTML)	Oui	Limité
Personnalisation	Très élevée	Élevée	Faible
Courbe d’apprentissage	Modérée	Élevée	Faible
Écosystème	En construction	Mature	Stable
Waylestia est-il prêt pour un usage en production ?

Non.
Waylestia est actuellement en développement précoce (version alpha).

État des composants :

Cœur du système : ✅ fonctionnel

Moteur de widgets : ✅ fonctionnel

Applications : 🟡 fonctionnalités de base

Interface Shell : 🚧 en développement

Stabilité globale : 🔶 non garantie

👉 Pour un usage quotidien fiable, KDE Plasma ou GNOME sont recommandés.

Puis-je utiliser Waylestia comme environnement principal ?

Pas encore.

Fonctionnalités actuelles :

✅ Démarrage des services

✅ Exécution d’applications individuelles

✅ Chargement de widgets

❌ Shell de bureau complet (prévu)

❌ Intégration complète gestionnaire de fenêtres (prévue)

L’expérience complète est prévue dans une version ultérieure, lorsque le Shell sera stabilisé.

À qui s’adresse Waylestia ?

Développeurs intéressés par les environnements de bureau

Utilisateurs de Rust

Créateurs d’environnements Wayland

Personnes recherchant une alternative légère

Contributeurs open-source

Installation et configuration
Configuration système requise

Minimum (indicatif) :

CPU : double cœur ≥ 2 GHz

RAM : 2 Go

Disque : 1 Go libre

Affichage : session Wayland fonctionnelle

Recommandé :

CPU : quad-cœur ≥ 2,5 GHz

RAM : 4 Go+

Disque : 5 Go

GPU avec pilotes Wayland stables

Systèmes supportés :

Linux avec glibc

Noyau Linux ≥ 5.14

Wayland requis

Distributions Linux compatibles

Testées :

Ubuntu 22.04 LTS

Fedora 38 / 39

Arch Linux

Probablement compatibles :

Debian 11+

openSUSE Leap 15.5+

Linux Mint 21+

Non supportées :

Environnements X11 uniquement

Distributions basées sur musl (ex. Alpine, sans compatibilité glibc)

Installation
git clone https://github.com/<organisation>/waylestia.git
cd waylestia
make all
./scripts/install.sh

📘 Voir également :

README.md

DEVELOPMENT.md

Installation système ou utilisateur ?

✔️ Installation utilisateur recommandée :

INSTALL_PREFIX=$HOME/.local ./scripts/install.sh
systemctl --user enable waylestia-core
systemctl --user enable waylestia-widgets

⚠️ Installation système possible, mais déconseillée en développement.

Compilation et développement
Compilation
make all
make core
make widgets
make test

Ou directement avec Cargo :

cargo build --release --workspace
Rust est-il obligatoire ?

Oui pour compiler depuis les sources

Non pour l’utilisation future, lorsque des binaires seront distribués

Actuellement, le projet suppose un environnement de développement complet.

Langages utilisés

Rust : services principaux

TypeScript / JavaScript : applications et widgets

Protocol Buffers : IPC

TOML : configuration

Fonctionnalités
Fonctionnalités actuelles (version alpha)

✔️ Cœur système
✔️ Surveillance des performances
✔️ Gestion audio (PipeWire)
✔️ Moteur de widgets basé sur manifest
✔️ IPC structuré via protobuf

❌ Shell complet
❌ Thèmes globaux
❌ Plugins tiers stables

Création de widgets

Les widgets sont basés sur HTML / CSS / JavaScript avec un manifeste TOML.

📘 Documentation complète : webview/README.md

Compatibilité
Gestionnaires de fenêtres

Hyprland : support principal
prochainement : wayfire 
Autres compositeurs Wayland : support partiel

X11 : ❌ non supporté

Waylestia est-il un display manager ?

Non.
Waylestia est un environnement de bureau, exécuté au-dessus d’un compositeur Wayland.
Un gestionnaire de connexion (GDM, SDDM…) reste nécessaire.

Licence et aspects légaux
Licence

Waylestia est distribué sous GPL-3.0.

Cela implique notamment :

liberté d’utilisation, modification et redistribution

obligation de fournir le code source

conservation de la licence dans les redistributions

📄 Voir le fichier LICENSE.

Usage commercial

Autorisé dans le respect de la GPL-3.0.
En cas de doute juridique, consulte un professionnel du droit.

Divers
Pourquoi Rust ?

Sécurité mémoire, performances, concurrence sûre, écosystème moderne.

Pourquoi Wayland ?

Architecture moderne, sécurité, avenir de l’écosystème Linux graphique.

Autres OS ?

Non prévu. Waylestia cible exclusivement Linux + Wayland.

Support et communauté

GitHub Issues : bugs et demandes

GitHub Discussions : questions générales

Autres canaux : à définir

Dernière mise à jour : 2026
Statut : documentation en évolution
