// Module Dashboard central (widgets, calendrier, perf, media)
import { Widget } from "../types/widget.ts";

export class Dashboard {
  widgets: Widget[];
  calendar: any;
  perf: any;
  media: any;
  openState: boolean;
  openCallback: (() => void) | null;
  closeCallback: (() => void) | null;

  constructor() {
    this.widgets = [];
    this.calendar = null;
    this.perf = null;
    this.media = null;
    this.openState = false;
    this.openCallback = null;
    this.closeCallback = null;
    this.initMockUI();
  }

  initMockUI() {
    console.log("[Dashboard] UI initialisée (mock)");
  }

  open() {
    this.openState = true;
    console.log("[Dashboard] Ouvert");
    if (this.openCallback) this.openCallback();
  }

  close() {
    this.openState = false;
    console.log("[Dashboard] Fermé");
    if (this.closeCallback) this.closeCallback();
  }

  setWidget(widget: Widget) {
    this.widgets.push(widget);
    console.log(`[Dashboard] Widget ajouté: ${widget.name}`);
  }

  setCalendar(calendar: any) {
    this.calendar = calendar;
    console.log("[Dashboard] Calendrier mis à jour");
  }

  setPerf(perf: any) {
    this.perf = perf;
    console.log("[Dashboard] Perf mis à jour");
  }

  setMedia(media: any) {
    this.media = media;
    console.log("[Dashboard] Media mis à jour");
  }

  onOpen(cb: () => void) {
    this.openCallback = cb;
  }

  onClose(cb: () => void) {
    this.closeCallback = cb;
  }
}
