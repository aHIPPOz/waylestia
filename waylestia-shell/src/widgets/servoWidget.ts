// Widget Servo — intégration d’un widget web (Flutter web, HTML/CSS) via Servo
export class ServoWidget {
  url: string;
  name: string;
  constructor(url: string, name: string) {
    this.url = url;
    this.name = name;
    // TODO: Lancer une instance Servo embarquée avec ce widget
    this.initMockServo();
  }
  initMockServo() {
    console.log(`[ServoWidget] Widget web lancé: ${this.name} (${this.url})`);
  }
  sendMessage(msg: any) {
    // TODO: Envoyer un message au widget (WebSocket/gRPC)
    console.log(`[ServoWidget] Message envoyé à ${this.name}:`, msg);
  }
}
