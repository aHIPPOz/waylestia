# Waylestia — Suite OS Moderne

**Wayland shell complet, modulaire, extensible, concurrent de KDE/Plasma, centré Hyprland, widgets Servo/Flutter, barres GTK/JS, support Wallpaper Engine.**

## Architecture

```
Hyprland (WM) <-> waylestia-core (Rust) <-> waylestia-shell (JS/Deno/GTK) <-> waylestia-engine (Servo, widgets web)
                                   |-> waylestia-widgets (Flutter web)
                                   |-> waylestia-assets, waylestia-scripts, waylestia-proto
```

- **waylestia-core** : Daemon principal (état global, IPC Hyprland, perf, media, sécurité, API locale)
- **waylestia-shell** : Barres, surfaces, UI système (JS/Deno, GTK via FFI, IPC, orchestration)
- **waylestia-engine** : Servo embarqué (widgets web, Flutter web, HTML/CSS, API native, Wallpaper Engine)
- **waylestia-widgets** : Widgets Flutter web (compilés, servis par Servo)
- **waylestia-assets** : Wallpapers, icônes, thèmes
- **waylestia-scripts** : Scripts build, install, maintenance
- **waylestia-proto** : Schémas IPC (proto, JSON)

## Build rapide

- Rust : `cd waylestia-core && cargo build`
- JS/GTK : `cd waylestia-shell && deno task start`
- Flutter web : `cd waylestia-widgets && flutter build web`
- Servo : build custom (voir waylestia-engine/servo)

## Fonctionnalités prévues

- Gestion Hyprland avancée (tiling, workspaces, input, IPC)
- Barres et surfaces UI (GTK via JS, multi-process)
- Widgets web (Flutter web, HTML/CSS, animations, opacité, night/neon)
- Moteur Servo embarqué (API native, IPC, extensibilité)
- Intégration Wallpaper Engine (wallpapers animés, hooks, API)
- IPC robuste (core <-> shell <-> engine <-> widgets)
- Sécurité, stabilité, extensibilité (modulaire, clean, scalable)

## Roadmap

1. Core Rust (IPC, state, perf, media, sécurité)
2. Shell/Barres GTK/JS (UI, IPC, orchestration)
3. Widgets Servo/Flutter web (API, build, intégration)
4. Wallpaper Engine (intégration, API, compatibilité)
5. Polish UX, modules avancés, CI/CD

---

**Contact :** aHIPPOz / Waylestia Team
