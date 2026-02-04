// Paramètres Waylestia — mock de base
export class WaylestiaSettings {
  constructor() {
    this.initMockUI();
  }
  initMockUI() {
    console.log("[Settings] UI initialisée (mock)");
  }
  setConfig(cfg: any) {
    console.log(`[Settings] Config appliquée:`, cfg);
    // TODO: Intégrer centre de configuration réel
  }
}
