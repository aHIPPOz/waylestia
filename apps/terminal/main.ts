/**
 * Waylestia Terminal — Modern terminal emulator
 * Built with GJS, supports tabs and customization
 */

export class WaylestiaTerminal {
    private tabs: TerminalTab[] = [];
    private activeTabIndex: number = 0;
    private history: string[] = [];
    private historyIndex: number = -1;

    constructor() {
        console.log('[Terminal] Initializing...');
        this.initUI();
        this.setupIPC();
        this.createNewTab();
    }

    private initUI(): void {
        console.log('[Terminal] UI initialized (GJS + GTK)');
    }

    private setupIPC(): void {
        if (globalThis.waylestia?.onMessage) {
            globalThis.waylestia.onMessage((msg: any) => {
                if (msg.type === 'command_output') {
                    this.handleCommandOutput(msg.data);
                } else if (msg.type === 'command_exit') {
                    this.handleCommandExit(msg.data);
                }
            });
        }
    }

    public createNewTab(): void {
        this.tabs.push({
            id: `tab-${Date.now()}`,
            title: `Terminal ${this.tabs.length + 1}`,
            cwd: '/home',
            shell: '/bin/bash',
            content: '',
            output: ''
        });
        this.activeTabIndex = this.tabs.length - 1;
    }

    public closeTab(index: number): void {
        if (this.tabs.length > 1) {
            this.tabs.splice(index, 1);
            this.activeTabIndex = Math.min(this.activeTabIndex, this.tabs.length - 1);
        }
    }

    public switchTab(index: number): void {
        if (index >= 0 && index < this.tabs.length) {
            this.activeTabIndex = index;
        }
    }

    public getActiveTab(): TerminalTab | undefined {
        return this.tabs[this.activeTabIndex];
    }

    public runCommand(cmd: string): void {
        console.log(`[Terminal] Executing: ${cmd}`);
        const activeTab = this.getActiveTab();
        if (!activeTab) return;

        // Add to history
        this.history.push(cmd);
        this.historyIndex = -1;
        activeTab.content += `$ ${cmd}\n`;

        if (globalThis.waylestia?.sendMessage) {
            globalThis.waylestia.sendMessage('execute_command', {
                command: cmd,
                cwd: activeTab.cwd,
                tabId: activeTab.id
            });
        }
    }

    public getPreviousCommand(): string {
        if (this.history.length === 0) return '';
        this.historyIndex = Math.min(this.historyIndex + 1, this.history.length - 1);
        return this.history[this.history.length - 1 - this.historyIndex];
    }

    public getNextCommand(): string {
        if (this.historyIndex > 0) {
            this.historyIndex--;
            return this.history[this.history.length - 1 - this.historyIndex];
        }
        this.historyIndex = -1;
        return '';
    }

    private handleCommandOutput(data: any): void {
        const tab = this.tabs.find(t => t.id === data.tabId);
        if (tab) {
            tab.output += data.output;
        }
    }

    private handleCommandExit(data: any): void {
        const tab = this.tabs.find(t => t.id === data.tabId);
        if (tab) {
            tab.output += `\n[Process exited with code ${data.exitCode}]\n`;
        }
    }

    public clearTab(): void {
        const tab = this.getActiveTab();
        if (tab) {
            tab.content = '';
            tab.output = '';
        }
    }
}

interface TerminalTab {
    id: string;
    title: string;
    cwd: string;
    shell: string;
    content: string;
    output: string;
}

const terminal = new WaylestiaTerminal();
export default terminal;
