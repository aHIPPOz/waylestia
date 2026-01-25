// Gestionnaire de fichiers Waylestia — mock de base
export class WaylestiaFiles {
  constructor() {
    this.initMockUI();
  }
  initMockUI() {
    console.log("[Files] UI initialisée (mock)");
  }
  openPath(path: string) {
    console.log(`[Files] Ouvre: ${path}`);
    // TODO: Intégrer navigation réelle, drag&drop, etc
  }
}
