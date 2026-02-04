// Module Notifications (gestion file, affichage, actions)
export class Notifications {
  queue: string[];
  actionCallback: ((msg: string) => void) | null;

  constructor() {
    this.queue = [];
    this.actionCallback = null;
    this.initMockUI();
  }

  initMockUI() {
    console.log("[Notifications] UI initialisée (mock)");
  }

  push(msg: string) {
    this.queue.push(msg);
    console.log(`[Notifications] Nouvelle notif: ${msg}`);
  }

  clear() {
    this.queue = [];
    console.log("[Notifications] Toutes les notifs masquées");
  }

  onAction(cb: (msg: string) => void) {
    this.actionCallback = cb;
  }

  simulateAction(msg: string) {
    if (this.actionCallback) this.actionCallback(msg);
  }
}
