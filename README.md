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
| 🎨 Personnalisation extrême | Arch Linux |
| 🤝 Simplicité d’usage | Linux Mint |
| ⚡ Performances & optimisation | Gentoo / CachyOS |
| 🛡️ Stabilité & sécurité | Debian / Qubes OS |

👉 **Le meilleur de chaque monde, sans compromis.**

---

## 🧱 Architecture générale

```

Hyprland (WM)
<-> waylestia-core (Rust)
<-> waylestia-shell (JS / Deno / GTK)
<-> waylestia-engine (Servo, widgets web)
|-> waylestia-widgets (Flutter web)
|-> waylestia-assets
|-> waylestia-scripts
|-> waylestia-proto

````

### Composants

- **waylestia-core**  
  Daemon principal en Rust : état global, IPC Hyprland, performances, médias, sécurité, API locale

- **waylestia-shell**  
  UI système : barres, surfaces, logique desktop  
  (JS/Deno + GTK via FFI)

- **waylestia-engine**  
  Servo embarqué pour widgets web, Flutter web, HTML/CSS  
  + API native + support Wallpaper Engine

- **waylestia-widgets**  
  Widgets Flutter compilés en web

- **waylestia-assets**  
  Thèmes, wallpapers, icônes

- **waylestia-scripts**  
  Build, install, maintenance

- **waylestia-proto**  
  Schémas IPC (proto / JSON)

---

## 🛠️ Stack technique

- **Rust** — cœur du système
- **Hyprland** — composant Wayland (WM)
- **Wayland** — pas de X11
- **Servo** — moteur web embarqué
- **JavaScript / Deno / GJS**
- **GTK**
- **Flutter Web**

---

## ⚡ Build rapide

```bash
# Core
cd waylestia-core && cargo build

# Shell
cd waylestia-shell && deno task start

# Widgets Flutter
cd waylestia-widgets && flutter build web
````

> Servo nécessite un build custom (voir `waylestia-engine/servo`)

---

## 🧩 Fonctionnalités prévues

* Gestion avancée Hyprland (tiling, workspaces, input, IPC)
* Barres et UI système GTK/JS
* Widgets web animés (Servo / Flutter)
* Intégration Wallpaper Engine
* IPC robuste entre tous les modules
* Architecture modulaire, propre et scalable

---

## 🗺️ Roadmap

1. Core Rust (IPC, état global, sécurité)
2. Shell & barres UI
3. Widgets Servo / Flutter web
4. Wallpaper Engine
5. Polish UX, modules avancés, CI/CD

---

## 👥 Équipe

| Pseudo      | Rôle                  |
| ----------- | --------------------- |
| **A2ER7Y**  | Gestion de projet     |
| **aHIPPOz** | Développeur principal |
| **pyrrox**  | Développeur           |

---

## 🤝 Recrutement

Projet **100% passion** (non rémunéré).

* 🇫🇷 Français uniquement (vocaux réguliers)
* 🔧 Profils recherchés :

  * Dev **Rust**
  * Dev **Dart / Flutter**

👉 Intéressé ? Viens sur le Discord et présente-toi !

---

## 🔗 Liens

* 💬 Discord : [https://discord.gg/mP5JBWRFaY](https://discord.gg/mP5JBWRFaY)
* 🐙 GitHub : [https://github.com/aHIPPOz/waylestia](https://github.com/aHIPPOz/waylestia)

---

*Waylestia — construire le futur du desktop Linux, proprement.* 🐧