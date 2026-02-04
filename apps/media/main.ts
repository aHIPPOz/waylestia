// Lecteur multimédia Waylestia — mock de base
export class WaylestiaMedia {
  constructor() {
    this.initMockUI();
  }
  initMockUI() {
    console.log("[Media] UI initialisée (mock)");
  }
  play(media: string) {
    console.log(`[Media] Lecture: ${media}`);
    // TODO: Intégrer PipeWire, player audio/vidéo
  }
}
