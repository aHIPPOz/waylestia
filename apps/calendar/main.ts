/**
 * Waylestia Calendar — Calendar and event management application
 * Integrates with system notifications and settings via IPC
 */

export class WaylestiaCalendar {
    private events: Map<string, CalendarEvent> = new Map();
    private selectedDate: Date = new Date();

    constructor() {
        console.log('[Calendar] Initializing...');
        this.initUI();
        this.setupIPC();
    }

    private initUI(): void {
        console.log('[Calendar] UI initialized (GJS + GTK)');
        // UI code would render calendar widget
    }

    private setupIPC(): void {
        if (globalThis.waylestia?.onMessage) {
            globalThis.waylestia.onMessage((msg: any) => {
                if (msg.type === 'notification_click') {
                    this.handleNotificationClick(msg.data);
                }
            });
        }
    }

    public addEvent(event: CalendarEvent): void {
        console.log(`[Calendar] Adding event: ${event.title}`);
        this.events.set(event.id, event);
        
        // Notify core about new event
        if (globalThis.waylestia?.sendMessage) {
            globalThis.waylestia.sendMessage('event_added', event);
        }
    }

    public getEventsForDate(date: Date): CalendarEvent[] {
        return Array.from(this.events.values()).filter(e => 
            this.isSameDay(e.startDate, date)
        );
    }

    private isSameDay(d1: Date, d2: Date): boolean {
        return d1.getFullYear() === d2.getFullYear() &&
               d1.getMonth() === d2.getMonth() &&
               d1.getDate() === d2.getDate();
    }

    private handleNotificationClick(data: any): void {
        console.log('[Calendar] Notification clicked:', data);
    }
}

interface CalendarEvent {
    id: string;
    title: string;
    description?: string;
    startDate: Date;
    endDate: Date;
    allDay?: boolean;
    recurring?: string; // 'daily', 'weekly', 'monthly', etc.
}

const calendar = new WaylestiaCalendar();
export default calendar;
