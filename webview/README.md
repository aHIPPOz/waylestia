# Waylestia Webview

Custom Servo webview engine patched for Waylestia with GJS JavaScript engine.

## Architecture

```
Servo (HTML/CSS rendering)
  ↓
GJS Bridge (JavaScript execution)
  ↓
Waylestia IPC (Protobuf-based communication)
  ↓
Core/Shell/Widgets
```

## Components

### Servo Fork
- Base: Servo layout and rendering engine
- Custom patches:
  - Replace SpiderMonkey JS engine with GJS
  - Add Wayland/Hyprland window integration
  - Implement Waylestia IPC bridge
  - Add system color scheme support

### GJS Integration
- JavaScript runtime: GObject Introspection
- GTK bindings through GJS
- Native system access (files, network, etc.)
- Compatible with existing GObject libraries

### Waylestia Webview Library
Provides the webview host for rendering widgets and web content:

```typescript
interface WaylestiaBrowser {
  loadURL(url: string): void;
  loadHTML(html: string): void;
  executeScript(js: string): void;
  on(event: string, callback: Function): void;
  setSize(width: number, height: number): void;
  show(): void;
  hide(): void;
}
```

## Building Servo

Requirements:
- Python 3
- Rust nightly
- LLVM
- XCB development libraries (Linux)

```bash
cd servo
./build-servo.sh
```

## GJS Bindings

The webview exposes a global `gjs` object:

```typescript
// Shell scripting
const result = gjs.exec('ls -la');

// IPC messaging
gjs.sendMessage('core', {type: 'get_perf'});

// File system
const content = gjs.readFile('/tmp/test.txt');
gjs.writeFile('/tmp/test.txt', 'content');

// Notifications
gjs.notify('Title', 'Message');
```

## Waylestia IPC Bridge

JavaScript in Servo can communicate with core/shell via the bridge:

```typescript
window.waylestia = {
  sendMessage(type, data): void,
  onMessage(callback): void,
  instanceId: string,
  widgetId: string
};

// Example usage
window.waylestia.sendMessage('open_settings', {});
window.waylestia.onMessage(msg => {
  if (msg.type === 'perf_update') {
    updateStatsDisplay(msg.data);
  }
});
```

## Widget Rendering Flow

1. Widget manifest is loaded (TOML)
2. HTML/CSS/JS files are served via file:// URL
3. Servo renders the content with available GJS APIs
4. Widget communicates with core via IPC bridge
5. Updates are pushed back through Waylestia IPC protocol

## Security Model

- Each widget runs in an isolated Servo context
- No direct file system access (uses IPC)
- Network access is mediated through core
- Permissions are checked via security module
- Sandboxing via standard Linux seccomp

## Performance

- Servo's  rendering is hardware-accelerated
- GJS provides efficient scripting
- IPC uses Unix sockets in local mode
- Widgets are cached in memory
- Selective re-rendering of changed areas

## Future Enhancements

- GPU-accelerated widget composition
- Live widget hot-reloading
- Network transparency (remote Hyprland)
- WebAssembly module support
- Custom shader support for rendering
