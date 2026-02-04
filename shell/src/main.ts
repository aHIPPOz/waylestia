// Waylestia Shell (JS/Deno + GTK via FFI)
import { TopBar } from "./modules/topbar.ts";
import { Dashboard } from "./modules/dashboard.ts";
import { Launcher } from "./modules/launcher.ts";
import { Notifications } from "./modules/notifications.ts";
import { Zones } from "./modules/zones.ts";
import { Settings } from "./modules/settings.ts";

// TODO: Import FFI GTK, initialiser la lib GTK
// import { dlopen, FFIType, UnsafePointer } from "https://deno.land/x/ffi/mod.ts";
// const libgtk = dlopen("libgtk-4.so", { ... });

console.log("[waylestia-shell] Démarrage complet du shell...");

export class WaylestiaShell {
  topbar: TopBar;
  dashboard: Dashboard;
  launcher: Launcher;
  notifications: Notifications;
  zones: Zones;
  settings: Settings;

  constructor() {
    this.topbar = new TopBar();
    this.dashboard = new Dashboard();
    this.launcher = new Launcher();
    this.notifications = new Notifications();
    this.zones = new Zones();
    this.settings = new Settings();
    // TODO: Initialiser GTK, surfaces, IPC avec core
  }

  start() {
    // TODO: Lancer la boucle principale, afficher la topbar, etc.
    this.topbar.show();
    // ...
  }
}

const shell = new WaylestiaShell();
shell.start();
