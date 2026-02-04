// Module TopBar — barre du haut (heure, notifs, dashboard, media)
import { getCurrentTime } from "../utils/time.ts";

export class TopBar {
  clock: string;
  notifications: string[];
  dashboardOpen: boolean;
  mediaStatus: string;
  dashboardCallback: (() => void) | null;
  clockInterval: number | null;

  constructor() {
    this.clock = getCurrentTime();
    this.notifications = [];
    this.dashboardOpen = false;
    this.mediaStatus = "";
    this.dashboardCallback = null;
    this.clockInterval = null;
    // Simule l'UI GTK (mock)
    this.initMockUI();
    this.startClock();
  }

  initMockUI() {
    console.log("[TopBar] UI initialisée (mock)");
    // Simule affichage barre, clock, bouton dashboard, notifs, media
  }

  show() {
    console.log("[TopBar] Affichée");
  }

  hide() {
    console.log("[TopBar] Masquée");
  }

  startClock() {
    this.clockInterval = setInterval(() => {
      this.updateClock();
    }, 1000);
  }

  updateClock() {
    this.clock = getCurrentTime();
    console.log(`[TopBar] Clock: ${this.clock}`);
    // TODO: Mettre à jour l’affichage GTK
  }

  showNotification(msg: string) {
    this.notifications.push(msg);
    console.log(`[TopBar] Notification: ${msg}`);
    // TODO: Afficher la notif dans la barre
  }

  setMediaStatus(status: string) {
    this.mediaStatus = status;
    console.log(`[TopBar] Media: ${status}`);
    // TODO: Mettre à jour l’affichage media
  }

  onDashboardClick(cb: () => void) {
    this.dashboardCallback = cb;
    // Simule le clic bouton dashboard
    console.log("[TopBar] Dashboard bouton hooké");
  }

  simulateDashboardClick() {
    if (this.dashboardCallback) this.dashboardCallback();
  }
}
