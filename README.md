# 🚀 Waylestia — Suite OS Moderne

**Un environnement de bureau Linux modulaire et extensible, concurrent de KDE/Plasma, basé sur Wayland & Hyprland.**

> Waylestia veut réunir **personnalisation**, **performance**, **simplicité** et **solidité** dans un seul écosystème moderne.

---

## 👋 C’est quoi Waylestia ?

Salut !  
Le projet est porté par la **Waylestia Team**, un collectif de passionnés qui veulent pousser plus loin l’expérience desktop Linux.

L’objectif n’est pas de “réinventer Linux”, mais de **construire une suite cohérente**, moderne et hackable, pensée pour Wayland dès le départ.

---

## 🎯 Les objectifs

| Objectif | Inspiration |
|--------|-------------|
| 🎨 Personnalisation extrême | kde |
| 🤝 Simplicité d’usage | gnome |
| ⚡ Performances & optimisation | arch linux |
| 🛡️ Stabilité & sécurité | nix |

👉 **Le meilleur de chaque monde, sans compromis.**

---

## 🧱 Architecture générale


Hyprland (WM)
   ↕
waylestia-core (Rust)
   ↕
waylestia-shell (JavaScript via Deno + GTK)
   ↕
waylestia-webview (Servo-based WebView + GJS)
      ├─ waylestia-widgets   (Widgets moteur basés sur la webview)
      ├─ waylestia-assets    (Thèmes, icônes, wallpapers)
      ├─ waylestia-scripts   (Scripts build, install, maintenance)
      └─ waylestia-proto     (Schémas IPC / protocoles)


### Composants

- **waylestia-core**  
  Daemon principal en Rust : état global, IPC Hyprland, performances, médias, sécurité, API locale
  Base du code : base du gestionnaire de fenêtres ( events, logique, etc )
  API : compatibilitée principale gtk

- **waylestia-shell**  
  UI système : barres, surfaces, logique desktop  
  (JS/Deno + GTK via FFI)

- **waylestia-webview**  
  Servo pour le html et css mais patché pour remplacer js par GJS

- **waylestia-widgets**  
  Moteur de widget utilisant waylestia-webview avec gestion de l'attachement au fond d'écran + support Wallpaper Engine

- **waylestia-assets**  
  Thèmes, wallpapers, icônes, contenu des widgets, etc

- **waylestia-scripts**  
  Build, install, maintenance, etc

- **waylestia-proto**  
  Schémas IPC (proto / JSON)
  Examples de protocoles et d'intégration api(s)

---

## 🛠️ Stack technique

- **Rust** — cœur du système
- **Hyprland** — composant Wayland (WM)
- **Wayland** — pas de X11
- **Servo** — moteur web embarqué
- **GJS** — moteur javascript natif embarqué pour les widgets et webview
- **Deno** — moteur javascript embarqué pour le shell ui (gestionnaire de fenêtre frontend)
- **GTK** — api utilisée pour rester compatible avec les applis linux existentes
- **Flutter Web** — peut être utilisé pour faire des widgets

---

## ⚡ Build rapide

```asci
COMING SOON
````

---

## 🧩 Fonctionnalités prévues

* Gestion avancée Hyprland (tiling, workspaces, input, IPC)
* Barres et UI système GTK/JS
* Widgets web animés (Servo / Flutter)
* Intégration Wallpaper Engine
* IPC robuste entre tous les modules
* Architecture modulaire, propre et scalable
* Fond d'écran celestia

---

## 🗺️ Roadmap

1. Core Rust (IPC, état global, sécurité)
2. Shell & barres UI
3. Widgets Servo / Flutter web
4. Wallpaper Engine & Celestia
5. Polish UX, modules avancés, CI/CD

---

## 👥 Équipe

| Pseudo      | Rôle                                             |
| ----------- | -------------------------------------------------|
| **A2ER7Y**  | Commanditaire du projet                          |
| **aHIPPOz** | Développeur principal, et propriétaire du projet |
| **pyrrox**  | Développeur inactif pour l'instent mais ...      |

---

## 🤝 Recrutement

Projet **100% passion** (non rémunéré).

* 🇫🇷 Français uniquement (vocaux réguliers)
* 🔧 Profils recherchés :

  * Dev **Rust**
  * Dev **Javascript (deno with gtk ffi)**

👉 Intéressé ? Viens sur le Discord et présente-toi !

---

## 🔗 Liens

* 💬 Discord : [https://discord.gg/mP5JBWRFaY](https://discord.gg/mP5JBWRFaY)
* 🐙 GitHub : [https://github.com/aHIPPOz/waylestia](https://github.com/aHIPPOz/waylestia)

---

*Waylestia — construire le futur du desktop Linux, ensemble.* 🐧