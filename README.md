# Ganymede Shell

**Shell Wayland custom (Hyprland-centric) — UX 100% souris, design bleu/violet nuit + rose néon, modulaire, clean.**

## Architecture

```
Hyprland (WM) <-> ganymede-core (Rust) <-> ganymede-runtime (Deno/TS) <-> ganymede-widgets (Flutter)
                                   |-> ganymede-hotbar (GTK)
                                   |-> proto/ (IPC)
                                   |-> scripts/, assets/
```

- **ganymede-core** : Daemon principal (état global, IPC Hyprland, perf, media, API locale)
- **ganymede-runtime** : Orchestration UI, logique métier, glue Rust <-> Widgets
- **ganymede-widgets** : Dashboard, media, perf, calendrier (Flutter, surfaces Wayland)
- **ganymede-hotbar** : Barre du haut (GTK, non-tiling, opacité 50/80%)
- **proto/** : Schémas IPC (JSON, protobuf)
- **scripts/** : Scripts utilitaires
- **assets/** : Logos, icônes, thèmes

## Build rapide

- Rust : `cd ganymede-core && cargo build`
- GTK : `cd ganymede-hotbar && cargo build`
- Deno : `cd ganymede-runtime && deno task start`
- Flutter : `cd ganymede-widgets && flutter run`

## MVP inclus

- Daemon Rust (dummy perf/uptime via Unix socket)
- Hotbar GTK (mock heure, bouton dashboard)
- Dashboard Flutter (surface indépendante, 80% opacité)
- Schémas IPC (JSON, proto)

## Roadmap

1. Core Rust (IPC, state, perf, media)
2. Hotbar GTK (UI, IPC)
3. Widgets Flutter (dashboard, media, perf, calendrier)
4. Intégrations (chat IA, agrégateur, polish UX)

---

**Contact :** aHIPPOz / Waylestia Team
