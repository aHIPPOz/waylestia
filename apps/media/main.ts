/**
 * Waylestia Media — Audio/Video player
 * Uses PipeWire for audio, integrated media controls
 */

export class WaylestiaMedia {
    private currentMedia: MediaFile | null = null;
    private isPlaying: boolean = false;
    private currentTime: number = 0;
    private duration: number = 0;
    private volume: number = 0.7;
    private playlist: MediaFile[] = [];

    constructor() {
        console.log('[Media] Initializing...');
        this.initUI();
        this.setupIPC();
    }

    private initUI(): void {
        console.log('[Media] UI initialized (GJS + GTK)');
    }

    private setupIPC(): void {
        if (globalThis.waylestia?.onMessage) {
            globalThis.waylestia.onMessage((msg: any) => {
                if (msg.type === 'mediakey_play') {
                    this.togglePlay();
                } else if (msg.type === 'mediakey_next') {
                    this.nextTrack();
                } else if (msg.type === 'mediakey_prev') {
                    this.previousTrack();
                }
            });
        }
    }

    public play(mediaPath: string): void {
        console.log(`[Media] Playing: ${mediaPath}`);
        this.currentMedia = { path: mediaPath, title: mediaPath };
        this.isPlaying = true;
        
        if (globalThis.waylestia?.sendMessage) {
            globalThis.waylestia.sendMessage('play_media', {path: mediaPath});
        }
    }

    public pause(): void {
        console.log('[Media] Paused');
        this.isPlaying = false;
        if (globalThis.waylestia?.sendMessage) {
            globalThis.waylestia.sendMessage('pause_media', {});
        }
    }

    public resume(): void {
        console.log('[Media] Resumed');
        this.isPlaying = true;
        if (globalThis.waylestia?.sendMessage) {
            globalThis.waylestia.sendMessage('resume_media', {});
        }
    }

    public togglePlay(): void {
        if (this.isPlaying) this.pause();
        else this.resume();
    }

    public setVolume(level: number): void {
        this.volume = Math.max(0, Math.min(1, level));
        if (globalThis.waylestia?.sendMessage) {
            globalThis.waylestia.sendMessage('set_volume', {volume: this.volume});
        }
    }

    public nextTrack(): void {
        console.log('[Media] Next track');
        if (globalThis.waylestia?.sendMessage) {
            globalThis.waylestia.sendMessage('next_track', {});
        }
    }

    public previousTrack(): void {
        console.log('[Media] Previous track');
        if (globalThis.waylestia?.sendMessage) {
            globalThis.waylestia.sendMessage('previous_track', {});
        }
    }

    public addToPlaylist(media: MediaFile): void {
        this.playlist.push(media);
    }
}

interface MediaFile {
    path: string;
    title: string;
    artist?: string;
    album?: string;
    duration?: number;
    thumbnail?: string;
}

const media = new WaylestiaMedia();
export default media;
