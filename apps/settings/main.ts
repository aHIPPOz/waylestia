/**
 * Waylestia Settings — System settings and configuration center  
 * Manages themes, accounts, security, and system preferences
 */

export class WaylestiaSettings {
    private settings: Map<string, unknown> = new Map();
    private schemas: Map<string, SettingsSchema> = new Map();

    constructor() {
        console.log('[Settings] Initializing...');
        this.initUI();
        this.setupIPC();
        this.loadSettings();
    }

    private initUI(): void {
        console.log('[Settings] UI initialized (GJS + GTK + Webview)');
    }

    private setupIPC(): void {
        if (globalThis.waylestia?.onMessage) {
            globalThis.waylestia.onMessage((msg: any) => {
                if (msg.type === 'setting_updated') {
                    this.applyRemoteSetting(msg.data);
                }
            });
        }
    }

    private loadSettings(): void {
        // Register common schemas
        this.schemas.set('org.waylestia.desktop', {
            name: 'Desktop',
            keys: ['theme', 'color-scheme', 'font-size']
        });
        this.schemas.set('org.waylestia.appearance', {
            name: 'Appearance',
            keys: ['wallpaper', 'icon-theme', 'cursor-theme']
        });
    }

    public getSetting(schema: string, key: string): unknown {
        const fullKey = `${schema}:${key}`;
        return this.settings.get(fullKey);
    }

    public setSetting(schema: string, key: string, value: unknown): void {
        console.log(`[Settings] Setting ${schema}.${key} = ${value}`);
        const fullKey = `${schema}:${key}`;
        this.settings.set(fullKey, value);
        
        if (globalThis.waylestia?.sendMessage) {
            globalThis.waylestia.sendMessage('update_setting', {
                schema, key, value
            });
        }
    }

    public getSchema(schema: string): SettingsSchema | undefined {
        return this.schemas.get(schema);
    }

    public listSchemas(): SettingsSchema[] {
        return Array.from(this.schemas.values());
    }

    public applyRemoteSetting(data: any): void {
        const {schema, key, value} = data;
        const fullKey = `${schema}:${key}`;
        this.settings.set(fullKey, value);
        console.log('[Settings] Applied remote setting:', fullKey, value);
    }

    public exportSettings(): Record<string, unknown> {
        const exported: Record<string, unknown> = {};
        for (const [key, value] of this.settings) {
            exported[key] = value;
        }
        return exported;
    }

    public importSettings(data: Record<string, unknown>): void {
        for (const [key, value] of Object.entries(data)) {
            this.settings.set(key, value);
        }
        console.log('[Settings] Imported', Object.keys(data).length, 'settings');
    }
}

interface SettingsSchema {
    name: string;
    keys: string[];
}

const settings = new WaylestiaSettings();
export default settings;
