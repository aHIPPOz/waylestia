// Mail Waylestia — mock de base
export class WaylestiaMail {
  constructor() {
    this.initMockUI();
  }
  initMockUI() {
    console.log("[Mail] UI initialisée (mock)");
  }
  sendMail(mail: any) {
    console.log(`[Mail] Envoi:`, mail);
    // TODO: Intégrer client mail réel (IMAP/SMTP)
  }
}
