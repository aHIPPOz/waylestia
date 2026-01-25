// Terminal Waylestia — mock de base
export class WaylestiaTerminal {
  constructor() {
    this.initMockUI();
  }
  initMockUI() {
    console.log("[Terminal] UI initialisée (mock)");
  }
  runCommand(cmd: string) {
    console.log(`[Terminal] Exécution: ${cmd}`);
    // TODO: Intégrer un vrai terminal (pty, xterm.js, etc)
  }
}
