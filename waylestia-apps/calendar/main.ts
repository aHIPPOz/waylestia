// Calendrier Waylestia — mock de base
export class WaylestiaCalendar {
  constructor() {
    this.initMockUI();
  }
  initMockUI() {
    console.log("[Calendar] UI initialisée (mock)");
  }
  addEvent(event: any) {
    console.log(`[Calendar] Ajout événement:`, event);
    // TODO: Intégrer calendrier réel, sync cloud
  }
}
