/**
 * Waylestia Editor — Text and code editor
 * Built with GJS, supports Markdown, syntax highlighting via Servo webview
 */

export class WaylestiaEditor {
    private openFiles: Map<string, FileBuffer> = new Map();
    private currentFile: FileBuffer | null = null;
    private unsavedChanges: Set<string> = new Set();

    constructor() {
        console.log('[Editor] Initializing...');
        this.initUI();
        this.setupIPC();
    }

    private initUI(): void {
        console.log('[Editor] UI initialized (GJS + GTK + Webview)');
    }

    private setupIPC(): void {
        if (globalThis.waylestia?.onMessage) {
            globalThis.waylestia.onMessage((msg: any) => {
                if (msg.type === 'file_open') {
                    this.openFile(msg.data.path);
                } else if (msg.type === 'file_save') {
                    this.saveFile(msg.data.path);
                }
            });
        }
    }

    public openFile(path: string): void {
        console.log(`[Editor] Opening: ${path}`);
        // Send to core to read file
        if (globalThis.waylestia?.sendMessage) {
            globalThis.waylestia.sendMessage('read_file', {path});
        }
    }

    public saveFile(path: string): void {
        console.log(`[Editor] Saving: ${path}`);
        if (this.currentFile) {
            if (globalThis.waylestia?.sendMessage) {
                globalThis.waylestia.sendMessage('write_file', {
                    path,
                    content: this.currentFile.content
                });
            }
            this.unsavedChanges.delete(path);
        }
    }

    public updateContent(path: string, content: string): void {
        if (this.openFiles.has(path)) {
            const file = this.openFiles.get(path)!;
            file.content = content;
            this.unsavedChanges.add(path);
        }
    }

    public getUnsavedFiles(): string[] {
        return Array.from(this.unsavedChanges);
    }
}

interface FileBuffer {
    path: string;
    content: string;
    language: string;
}

const editor = new WaylestiaEditor();
export default editor;
