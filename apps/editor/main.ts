// Éditeur de texte/code Waylestia — mock de base
export class WaylestiaEditor {
  constructor() {
    this.initMockUI();
  }
  initMockUI() {
    console.log("[Editor] UI initialisée (mock)");
  }
  openFile(path: string) {
    console.log(`[Editor] Ouvre: ${path}`);
    // TODO: Intégrer éditeur réel (Monaco, CodeMirror, etc)
  }
}
