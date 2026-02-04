// Module Zones latérales (widgets, chat IA, agrégateur, etc.)
import { Widget } from "../types/widget.ts";

export class Zones {
  leftWidgets: Widget[];
  rightWidgets: Widget[];
  widgetActionCallback: ((zone: 'left'|'right', widget: Widget) => void) | null;

  constructor() {
    this.leftWidgets = [];
    this.rightWidgets = [];
    this.widgetActionCallback = null;
    this.initMockUI();
  }

  initMockUI() {
    console.log("[Zones] UI initialisée (mock)");
  }

  showLeft() {
    console.log("[Zones] Zone gauche affichée");
  }

  showRight() {
    console.log("[Zones] Zone droite affichée");
  }

  setWidget(zone: 'left'|'right', widget: Widget) {
    if (zone === 'left') this.leftWidgets.push(widget);
    else this.rightWidgets.push(widget);
    console.log(`[Zones] Widget ajouté à ${zone}: ${widget.name}`);
  }

  onWidgetAction(cb: (zone: 'left'|'right', widget: Widget) => void) {
    this.widgetActionCallback = cb;
  }

  simulateWidgetAction(zone: 'left'|'right', widget: Widget) {
    if (this.widgetActionCallback) this.widgetActionCallback(zone, widget);
  }
}
