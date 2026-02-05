# Waylestia Architecture Documentation

Complete modern Linux desktop suite built on Wayland/Hyprland with Rust core, GTK UI, and Protocol Buffer IPC.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     Waylestia Desktop Suite                     │
└─────────────────────────────────────────────────────────────────┘

Applications (GJS + Gtk)
├── Browser (Servo-based)
├── Calendar (GTK)
├── Editor (Servo + CodeMirror)
├── Files (GTK)
├── Mail (GTK + IMAP/SMTP)
├── Media (GTK + PipeWire)
├── Settings (Servo UI)
└── Terminal (GTK + Pty)

        ↓ Protobuf IPC (Unix sockets)

┌─────────────────────────────────────────────────────────────────┐
│              Waylestia Shell (JavaScript/Deno)                  │
│  - Taskbar, panel, desktop management                          │
│  - Window operations via protobuf                              │
│  - Widget management                                            │
└─────────────────────────────────────────────────────────────────┘

        ↓ Protobuf IPC

┌─────────────────────────────────────────────────────────────────┐
│                   Waylestia Core (Rust)                         │
│  ├─ Hyprland IPC & window management                           │
│  ├─ Performance monitoring (CPU/GPU/RAM)                       │
│  ├─ Media control (PipeWire)                                   │
│  ├─ Security & permissions                                     │
│  ├─ Settings management (replaces GSettings)                   │
│  └─ System notifications                                        │
└─────────────────────────────────────────────────────────────────┘

        ↓ Hyprland IPC

┌─────────────────────────────────────────────────────────────────┐
│              Hyprland Wayland Compositor                        │
└─────────────────────────────────────────────────────────────────┘
```

## Component Details

### 1. Core Daemon (`/core`)

**Language:** Rust (async with Tokio)  
**Responsibilities:**
- Manage Hyprland communication (window list, workspace changes)
- Monitor system performance (CPU, GPU, RAM, uptime)
- Control audio devices via PipeWire
- Manage permissions and security policies
- Provide unified IPC interface via protobuf

**Key Modules:**
```
core/src/
├── lib.rs          # Library exports
├── main.rs         # Daemon entry point
├── hyprland.rs     # Hyprland WM integration
├── ipc.rs          # Unix socket IPC server
├── state.rs        # Global state management
├── perf.rs         # Performance monitoring
├── media.rs        # Audio device management
└── security.rs     # Permission & audit logging
```

**IPC Protocols:**
- `core_runtime.proto` - Core ↔ Deno Runtime
- `core_shell.proto` - Core RPC service definition

### 2. Widgets Engine (`/widgets`)

**Language:** Rust (async with Tokio)  
**Responsibilities:**
- Discover and load widget manifests from `assets/widgets/`
- Manage widget lifecycle (create, show, hide, destroy instances)
- Communicate with Servo webview
- Forward widget messages to shell/core

**Key Modules:**
```
widgets/src/
├── lib.rs       # Library exports
├── main.rs      # Daemon entry point
├── manifest.rs  # Widget manifest (TOML) parsing
├── loader.rs    # Widget discovery & loading
├── state.rs     # Widget instance state
├── renderer.rs  # Prepare widgets for rendering
└── ipc.rs       # Widget IPC server
```

**IPC Protocols:**
- `runtime_widgets.proto` - Runtime ↔ Flutter widgets
- `shell_widgets.proto` - Shell ↔ Servo widgets

### 3. Applications (`/apps`)

**Language:** TypeScript/JavaScript (GJS)  
**Framework:** GTK 4 + Adwaita (via GObject Introspection)

Applications communicate with core via protobuf-based IPC:

- **Browser** - Web navigation using Servo
- **Calendar** - Calendar and event management
- **Editor** - Text editor with syntax highlighting
- **Files** - File manager
- **Mail** - Email client (IMAP/SMTP)
- **Media** - Audio/video player
- **Settings** - System configuration
- **Terminal** - Terminal emulator

**IPC Protocol:**
- `apps.proto` - Applications ↔ Core

### 4. Widgets (`/assets/widgets`)

**Format:** HTML/CSS/JavaScript + TOML manifest  
**Rendering:** Servo webview engine

Each widget has:
```
widget-id/
├── manifest.toml  # Widget metadata & permissions
├── index.html     # Main HTML file
├── style.css      # Styles (optional)
└── script.js      # Logic (optional)
```

**Included Widgets:**
- **dashboard** - System quick access and stats
- **clock** - Analog/digital clock
- **sysinfo** - Real-time system information

### 5. Webview (`/webview`)

**Engine:** Servo (patched)  
**JavaScript Runtime:** GJS (GObject JavaScript)
**Bindings:** Waylestia IPC bridge

Features:
- Custom HTML/CSS/JS rendering
- System access through GJS APIs
- IPC bridge to core/shell
- Security sandboxing

### 6. Protocol Buffers (`/protobuf`)

Central messaging specification for all components:

- **core_runtime.proto** - Performance, media, window events
- **core_shell.proto** - RPC service definitions
- **runtime_widgets.proto** - Widget lifecycle
- **shell_widgets.proto** - Widget display management
- **apps.proto** - Application integration

**Benefits over D-Bus:**
- Type safety (schema validation)
- Better performance (binary encoding)
- No daemon dependency (direct client-server)
- Version compatibility
- Clean API definitions

### 7. Scripts (`/scripts`)

Automation and service management:

- **install.sh** - Build and install everything
- **uninstall.sh** - Remove installation
- **start.sh** - Start services
- **stop.sh** - Stop services
- **restart.sh** - Restart services

Uses systemd user services for auto-management.

## Data Flow Examples

### Example 1: Getting Performance Stats

```
Shell UI
  └─> sendMessage('GetPerfStats')
      └─> Core (via protobuf IPC)
          └─> sysinfo library (perf.rs)
          └─> returnPerfStats()
      └─> Shell receives stats
      └─> Update status bar
```

### Example 2: Launching an Application

```
User clicks app in menu
  └─> Shell sends: LaunchApp request to Core
  └─> Core (apps.proto)
      └─> Check permissions (security.rs)
      └─> Verify not already running
      └─> Fork process + monitor
      └─> Send app_started event to shell
  └─> Shell waits for window event
  └─> Hyprland window appears
  └─> Shell updates app list
```

### Example 3: Widget Rendering

```
User right-clicks desktop
  └─> Shell: RegisterWidget('sysinfo')
  └─> Core (widgets daemon)
      └─> Load manifest.toml
      └─> Spawn Servo instance
      └─> IPC bridge initialization
  └─> Servo loads index.html
  └─> JavaScript initializes window.waylestia
  └─> Widget sends IPC requests to core for data
  └─> Core responds via protobuf
  └─> Widget renders in real-time
```

## Compatibility Layers

### GNOME Integration

- **GSettings** → Protobuf ConfigRequest
- **D-Bus** → Protobuf IPCRequest (DBusCompat message)
- **Notifications** → Protobuf Notification message

### KDE Integration

- **KDE Plasma** services → Protobuf service calls
- **KConfig** → Protobuf ConfigRequest
- **KNotifications** → Protobuf Notification message

## Security Model

1. **Sandboxing**: Apps request permissions (filesystem, network, etc.)
2. **Audit Logging**: All permission requests logged
3. **AppArmor/Seccomp**: Additional sandboxing via Linux
4. **IPC Validation**: All messages type-checked via protobuf
5. **User Approval**: Interactive permission grants for sensitive operations

## Performance Characteristics

| Component | Latency | Memory | Notes |
|-----------|---------|--------|-------|
| Core startup | < 1s | ~20 MB | Minimal, async |
| Widget render | < 500ms | ~50-150 MB | Per instance |
| IPC message | < 10ms | N/A | Unix socket |
| Perf update | ~100ms | N/A | Every 1 second |
| App launch | 1-3s | Variable | Depends on app |

## Future Roadmap

1. **Network Transparency** - Remote Hyprland over Wayland protocol
2. **GPU Acceleration** - Vulkan/OpenGL for widgets
3. **Multi-Display** - Mesh networking for displays
4. **WebAssembly** - WASM modules in widgets
5. **Live UI Reload** - Hot reload for widget development
6. **Flutter Desktop** - Native Flutter app integration

## Development Workflow

```bash
# Build everything
./scripts/install.sh

# Run in development
systemctl --user start waylestia-core
systemctl --user start waylestia-widgets

# Check logs
journalctl --user -u waylestia-core -f

# Modify code and rebuild
cd core && cargo build --release && cd ..

# Restart service
systemctl --user restart waylestia-core
```

## File Locations (After Installation)

```
/usr/local/bin/
├── waylestia-core     # Core daemon
├── waylestia-widgets  # Widgets engine
└── waylestia-shell    # Shell (Deno)

/usr/local/share/waylestia/
├── assets/
│   ├── icons/
│   ├── wallpapers/
│   └── widgets/
│       ├── dashboard/
│       ├── clock/
│       └── sysinfo/
├── apps/
│   ├── browser/
│   ├── calendar/
│   └── ...
└── protocols/
    ├── core_runtime.proto
    ├── core_shell.proto
    └── ...

~/.config/systemd/user/
├── waylestia-core.service
└── waylestia-widgets.service
```

## Conclusion

Waylestia provides a modern, modular desktop environment that:

✅ Replaces GNOME/KDE with simpler architecture  
✅ Uses Rust for performance and safety  
✅ Implements clean protobuf-based IPC  
✅ Maintains GTK compatibility  
✅ Supports custom widgets via Servo + GJS  
✅ Prioritizes security through sandboxing  
✅ Enables hardware acceleration  

A complete, cohesive, Linux desktop suite for the Wayland era.
