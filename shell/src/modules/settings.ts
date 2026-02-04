// Module Settings (UI, préférences, thèmes, etc.)
export class Settings {
  theme: string;
  openState: boolean;
  themeChangeCallback: ((theme: string) => void) | null;

  constructor() {
    this.theme = "night";
    this.openState = false;
    this.themeChangeCallback = null;
    this.initMockUI();
  }

  initMockUI() {
    console.log("[Settings] UI initialisée (mock)");
  }

  open() {
    this.openState = true;
    console.log("[Settings] Ouvert");
  }

  close() {
    this.openState = false;
    console.log("[Settings] Fermé");
  }

  setTheme(theme: string) {
    this.theme = theme;
    console.log(`[Settings] Thème appliqué: ${theme}`);
    if (this.themeChangeCallback) this.themeChangeCallback(theme);
  }

  onThemeChange(cb: (theme: string) => void) {
    this.themeChangeCallback = cb;
  }
}
