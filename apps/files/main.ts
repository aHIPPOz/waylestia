/**
 * Waylestia Files — File manager
 * Modern file explorer with drag&drop and Flatpak support
 */

export class WaylestiaFiles {
    private currentPath: string = '/home';
    private history: string[] = ['/home'];
    private historyIndex: number = 0;

    constructor() {
        console.log('[Files] Initializing...');
        this.initUI();
        this.setupIPC();
    }

    private initUI(): void {
        console.log('[Files] UI initialized (GJS + GTK)');
    }

    private setupIPC(): void {
        if (globalThis.waylestia?.onMessage) {
            globalThis.waylestia.onMessage((msg: any) => {
                if (msg.type === 'list_files') {
                    this.listDirectory(msg.data.path);
                }
            });
        }
    }

    public navigateTo(path: string): void {
        console.log(`[Files] Navigating to: ${path}`);
        this.currentPath = path;
        this.history.push(path);
        this.historyIndex = this.history.length - 1;
        
        this.listDirectory(path);
    }

    public goBack(): void {
        if (this.historyIndex > 0) {
            this.historyIndex--;
            this.currentPath = this.history[this.historyIndex];
            this.listDirectory(this.currentPath);
        }
    }

    public goForward(): void {
        if (this.historyIndex < this.history.length - 1) {
            this.historyIndex++;
            this.currentPath = this.history[this.historyIndex];
            this.listDirectory(this.currentPath);
        }
    }

    public goHome(): void {
        this.navigateTo('/home');
    }

    public listDirectory(path: string): void {
        if (globalThis.waylestia?.sendMessage) {
            globalThis.waylestia.sendMessage('list_directory', {path});
        }
    }

    public deleteFile(path: string): void {
        if (globalThis.waylestia?.sendMessage) {
            globalThis.waylestia.sendMessage('delete_file', {path});
        }
    }
}

const files = new WaylestiaFiles();
export default files;
