/**
 * Waylestia Webview Host API
 * Browser bindings for Servo webview with GJS integration
 */

interface WaylestiaBrowser {
    // Window management
    loadURL(url: string): void;
    loadHTML(html: string, baseURL?: string): void;
    executeScript(code: string): Promise<any>;
    
    // Window properties
    setTitle(title: string): void;
    setSize(width: number, height: number): void;
    setPosition(x: number, y: number): void;
    show(): void;
    hide(): void;
    close(): void;
    
    // Window state
    setFullscreen(fullscreen: boolean): void;
    setAlwaysOnTop(alwaysOnTop: boolean): void;
    minimize(): void;
    maximize(): void;
    restore(): void;
    
    // Dev tools
    openDevTools(): void;
    closeDevTools(): void;
    reload(): void;
    reloadIgnoringCache(): void;
    
    // Event handling
    on(event: string, callback: Function): void;
    once(event: string, callback: Function): void;
    removeListener(event: string, callback: Function): void;
}

interface WaylestiaBrowserOptions {
    width?: number;
    height?: number;
    x?: number;
    y?: number;
    minWidth?: number;
    minHeight?: number;
    maxWidth?: number;
    maxHeight?: number;
    show?: boolean;
    fullscreen?: boolean;
    alwaysOnTop?: boolean;
    transparent?: boolean;
    icon?: string;
    context?: BrowserContext;
}

interface BrowserContext {
    debug?: boolean;
    preload?: string;
    nodeIntegration?: boolean;
    enableRemoteModule?: boolean;
}

// Global Waylestia widget API
declare global {
    interface Window {
        waylestia: {
            instanceId: string;
            widgetId: string;
            sendMessage<T = void>(type: string, data?: any): Promise<T>;
            onMessage(callback: (msg: WidgetMessage) => void): void;
            requestPermission(permission: string): Promise<boolean>;
            getSystemInfo(): Promise<SystemInfo>;
            notify(title: string, options?: NotifyOptions): void;
        };
        
        gjs: {
            // Shell execution
            exec(command: string): string;
            execAsync(command: string): Promise<string>;
            
            // File system
            readFile(path: string): string;
            writeFile(path: string, content: string): void;
            deleteFile(path: string): void;
            listDirectory(path: string): string[];
            
            // IPC
            sendMessage(service: string, method: string, args: any[]): any;
            
            // Notifications
            notify(title: string, body?: string): string;
            
            // System
            openURL(url: string): void;
            getEnv(name: string): string;
            
            // Logging
            log(message: string): void;
            warn(message: string): void;
            error(message: string): void;
        };
    }
}

interface WidgetMessage {
    instanceId: string;
    widgetId: string;
    type: string;
    data: any;
    timestamp: number;
}

interface NotifyOptions {
    timeout?: number;
    urgency?: 'low' | 'normal' | 'high';
    icon?: string;
    actions?: Array<{id: string; label: string}>;
}

interface SystemInfo {
    hostname: string;
    platform: string;
    arch: string;
    cpus: number;
    totalMemory: number;
    freeMemory: number;
    uptime: number;
    userInfo: {
        username: string;
        homedir: string;
        shell: string;
    };
}

// Initialization helper
export async function initializeWidget() {
    console.log('[Widget] Initializing Waylestia bridge...');
    
    // Wait for window.waylestia to be available
    while (!window.waylestia) {
        await new Promise(resolve => setTimeout(resolve, 100));
    }
    
    console.log(`[Widget] Instance: ${window.waylestia.instanceId}`);
    console.log(`[Widget] Widget: ${window.waylestia.widgetId}`);
    
    // Request required permissions
    try {
        const hasIPC = await window.waylestia.requestPermission('ipc');
        console.log(`[Widget] IPC permission: ${hasIPC}`);
    } catch (e) {
        console.error('[Widget] Failed to request permissions:', e);
    }
    
    return window.waylestia;
}

// Helper function to send RPC calls to core
export async function callCoreService(service: string, method: string, args: any = {}) {
    const result = await window.waylestia.sendMessage('rpc_call', {
        service,
        method,
        args
    });
    return result;
}

// Helper for subscribing to core events
export function subscribeToEvents(eventType: string, callback: (data: any) => void) {
    window.waylestia.onMessage(msg => {
        if (msg.type === eventType) {
            callback(msg.data);
        }
    });
}

export default {
    initializeWidget,
    callCoreService,
    subscribeToEvents
};
