// AppRegistry — centralise l’accès à toutes les apps de base Waylestia
import { WaylestiaTerminal } from "../../waylestia-apps/terminal/main.ts";
import { WaylestiaFiles } from "../../waylestia-apps/files/main.ts";
import { WaylestiaBrowser } from "../../waylestia-apps/browser/main.ts";
import { WaylestiaEditor } from "../../waylestia-apps/editor/main.ts";
import { WaylestiaCalendar } from "../../waylestia-apps/calendar/main.ts";
import { WaylestiaMail } from "../../waylestia-apps/mail/main.ts";
import { WaylestiaMedia } from "../../waylestia-apps/media/main.ts";
import { WaylestiaSettings } from "../../waylestia-apps/settings/main.ts";

export const AppRegistry = {
  terminal: new WaylestiaTerminal(),
  files: new WaylestiaFiles(),
  browser: new WaylestiaBrowser(),
  editor: new WaylestiaEditor(),
  calendar: new WaylestiaCalendar(),
  mail: new WaylestiaMail(),
  media: new WaylestiaMedia(),
  settings: new WaylestiaSettings(),
};
