// Navigateur Web Waylestia — mock de base
export class WaylestiaBrowser {
  constructor() {
    this.initMockUI();
  }
  initMockUI() {
    console.log("[Browser] UI initialisée (mock)");
  }
  openUrl(url: string) {
    console.log(`[Browser] Ouvre: ${url}`);
    // TODO: Intégrer Servo natif, navigation réelle
  }
}
