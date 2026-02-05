# Waylestia Protocol Buffer Schemas

This directory contains all the protocol buffer definitions for Waylestia's IPC system. Unlike GNOME/DBus and KDE's D-Bus, Waylestia uses Protocol Buffers for all inter-process communication.

## Architecture

```
Hyprland (Wayland WM)
  ↕ IPC Events
waylestia-core (Rust Daemon)
  ↕ core_runtime.proto + core_shell.proto
Deno Runtime (Shell UI)
  ↕ shell_widgets.proto / runtime_widgets.proto
Applications (GJS) + Widgets (Servo)
  ↕ apps.proto
External Services (GTK Compat, Qt Compat, etc)
```

## Files Overview

### 1. `core_runtime.proto`
**Core ↔ Runtime Communication**
- Performance statistics (CPU, GPU, RAM, uptime)
- Media source and volume management
- Hyprland window/workspace events
- Security permissions
- Notifications
- Configuration updates (replaces GSettings)
- Shell state management

**Usage:**
- Core publishes performance metrics every 1s
- Runtime subscribes to window focus changes
- Bidirectional media control

### 2. `core_shell.proto`
**Core ↔ Shell Communication (RPC)**
- RPC service for shell to query core state
- `GetPerfStats()` - Get current performance
- `GetWindowList()` - List all Hyprland windows
- `FocusWindow()` - Focus a window
- `SendNotification()` - Show system notification
- Configuration sync
- Event streaming (subscribe pattern)

**Usage:**
- Shell calls `GetWindowList()` to update window list
- Shell subscribes to `perf_stats` events for status bar
- Apps use this to interact with core

### 3. `runtime_widgets.proto`
**Runtime ↔ Widgets (Flutter Web) Communication**
- Create/destroy widget instances
- Send messages to Flutter web widgets
- State management (get/set)
- Event subscriptions
- Designed for widgets built with Flutter Web + Servo

**Usage:**
- Runtime spawns Servo instances with Flutter widgets
- Bidirectional messaging for interactivity
- State synchronization

### 4. `shell_widgets.proto`
**Shell ↔ Widgets (Servo) Communication**
- Widget registration with shell
- Display management (position, size, visibility)
- Input events (mouse, keyboard, touch)
- Resource loading (HTML, CSS, JS)
- Error reporting

**Usage:**
- Widgets (HTML/CSS/GJS) register with shell
- Shell manages widget windows and focus
- Widget IPC bridge for GJS ↔ Shell messaging

### 5. `apps.proto`
**Applications (GJS) ↔ Core Communication**
- App registration/unregistration
- App launching
- Generic IPC requests (replaces D-Bus)
- Permission requests
- App lifecycle events
- **D-Bus compatibility layer** for legacy tools

**Usage:**
- Apps register with core and declare permissions
- Apps request IPC services from core
- Replaces D-Bus org.freedesktop.* services
- Allows GTK apps to work via compatibility layer

## Communication Patterns

### 1. Streaming (Subscribe Pattern)
```
client: SubscribeRequest { event_types: ["perf", "window"] }
server: (stream of Event messages)
```

### 2. Request-Reply (RPC)
```
client: GetPerfStats {}
server: PerfStats { cpu: 45.2, ... }
```

### 3. Async Messaging
```
client: SendWidgetMessage { instance_id: "...", data: {...} }
server: Response { success: true }
(server may emit WidgetEvent later)
```

## GTK/Glib Compatibility

Waylestia provides compatibility for existing GTK applications:

1. **GSettings Bridge**: Core translates `org.gnome.desktop.*` GSettings to `waylestia.proto.ConfigRequest/Response`
2. **D-Bus Compatibility Layer**: Apps using D-Bus are automatically bridged to protobuf
3. **Notifications**: GNOME Notification Daemon protocol is supported

## KDE/Qt Compatibility

For KDE applications:

1. **Settings** (`org.kde.kdeglobals`): Bridged to protobuf
2. **KNotifications**: Supported via compatibility layer
3. **Services**: KService files can register services accessible via protobuf

## Socket Locations

All communication happens via Unix domain sockets (not TCP):

- `waylestia-core`: `/tmp/waylestia-core.sock`
- `waylestia-shell`: `/tmp/waylestia-shell.sock`
- `waylestia-widgets`: `/tmp/waylestia-widgets.sock`
- `waylestia-runtime`: `/tmp/waylestia-runtime.sock`

## Wire Format

All messages are transmitted as:
1. 4-byte length prefix (big-endian)
2. Protobuf message bytes
3. Optional newline separator for streaming

JSON fallback is available for compatibility layers.

## Building Protobuf Code

### Rust
```bash
cd ../core
cargo build  # build.rs compiles protos
```

### JavaScript/Deno
```bash
# Generated JavaScript stubs from .proto files
# Manual compilation with protoc or deno_protobuf
```

### Python (scripts)
```bash
python -m grpc_tools.protoc -I. --python_out=. --grpc_python_out=. *.proto
```

## Service Examples

### Getting Performance Stats
```
Request: CoreRPC.GetPerfStats()
Response: PerfStats {
  cpu_usage: 28.5,
  gpu_usage: 45.2,
  ram_usage: 62.0,
  ram_used_mb: 8192,
  ram_total_mb: 16384,
  uptime_seconds: 3600
}
```

### Creating a Widget
```
Request: ShellWidgetService.RegisterWidget({
  widget_id: "dashboard",
  instance_id: "dash-1",
  html_url: "file:///assets/widgets/dashboard/index.html",
  width: 600,
  height: 400
})
Response: RegisterWidgetResponse {
  success: true,
  instance_id: "dash-1",
  window_id: 42
}
```

## Future Extensions

- Mesh networking for multi-display systems
- GPU acceleration for widget rendering
- Hot reload for widget development
- Network transparency (wayland protocol forwarding)
