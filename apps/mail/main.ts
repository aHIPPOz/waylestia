/**
 * Waylestia Mail — Email client
 * Supports IMAP/SMTP, integrated notifications and calendar
 */

export class WaylestiaMail {
    private accounts: Map<string, EmailAccount> = new Map();
    private currentAccount: EmailAccount | null = null;
    private inbox: EmailMessage[] = [];

    constructor() {
        console.log('[Mail] Initializing...');
        this.initUI();
        this.setupIPC();
    }

    private initUI(): void {
        console.log('[Mail] UI initialized (GJS + GTK)');
    }

    private setupIPC(): void {
        if (globalThis.waylestia?.onMessage) {
            globalThis.waylestia.onMessage((msg: any) => {
                if (msg.type === 'message_received') {
                    this.handleNewMessage(msg.data);
                }
            });
        }
    }

    public addAccount(account: EmailAccount): void {
        console.log(`[Mail] Adding account: ${account.email}`);
        this.accounts.set(account.email, account);
        this.currentAccount = account;
        
        if (globalThis.waylestia?.sendMessage) {
            globalThis.waylestia.sendMessage('connect_email', account);
        }
    }

    public sendEmail(to: string, subject: string, body: string): void {
        console.log(`[Mail] Sending email to: ${to}`);
        if (this.currentAccount) {
            if (globalThis.waylestia?.sendMessage) {
                globalThis.waylestia.sendMessage('send_email', {
                    from: this.currentAccount.email,
                    to, subject, body
                });
            }
        }
    }

    public deleteMessage(messageId: string): void {
        console.log(`[Mail] Deleting message: ${messageId}`);
        if (globalThis.waylestia?.sendMessage) {
            globalThis.waylestia.sendMessage('delete_email', {messageId});
        }
    }

    private handleNewMessage(message: EmailMessage): void {
        console.log('[Mail] New message received:', message);
        this.inbox.push(message);
        // Show notification
        if (globalThis.waylestia?.sendMessage) {
            globalThis.waylestia.sendMessage('show_notification', {
                title: message.from,
                body: message.subject
            });
        }
    }
}

interface EmailAccount {
    email: string;
    password: string;
    imapServer: string;
    smtpServer: string;
}

interface EmailMessage {
    id: string;
    from: string;
    to: string;
    subject: string;
    body: string;
    date: Date;
    read: boolean;
}

const mail = new WaylestiaMail();
export default mail;
