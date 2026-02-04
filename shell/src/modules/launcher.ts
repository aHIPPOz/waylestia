// Module Launcher (app search, flatpak, etc.)
import { AppEntry } from "../types/app.ts";

export class Launcher {
  openState: boolean;
  results: AppEntry[];
  launchCallback: ((app: AppEntry) => void) | null;

  constructor() {
    this.openState = false;
    this.results = [];
    this.launchCallback = null;
    this.initMockUI();
  }

  initMockUI() {
    console.log("[Launcher] UI initialisée (mock)");
  }

  open() {
    this.openState = true;
    console.log("[Launcher] Ouvert");
  }

  close() {
    this.openState = false;
    console.log("[Launcher] Fermé");
  }

  search(query: string) {
    // Simule une recherche d'apps
    this.results = [
      { name: "Terminal", exec: "alacritty" },
      { name: "Web", exec: "firefox" },
      { name: "Files", exec: "nautilus" }
    ].filter(app => app.name.toLowerCase().includes(query.toLowerCase()));
    console.log(`[Launcher] Résultats pour '${query}':`, this.results);
  }

  onAppLaunch(cb: (app: AppEntry) => void) {
    this.launchCallback = cb;
  }

  simulateAppLaunch(app: AppEntry) {
    if (this.launchCallback) this.launchCallback(app);
    this.close();
  }
}
