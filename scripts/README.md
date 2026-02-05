# Waylestia Scripts

Build, installation, service management, and deployment scripts for the Waylestia suite.

## Installation Scripts

### `install.sh`
Main installation script. Builds all components and installs to the system.

```bash
./scripts/install.sh
```

**What it does:**
1. Checks dependencies (rustc, cargo, git, pkg-config, gcc)
2. Builds `waylestia-core` (Rust daemon)
3. Builds `waylestia-widgets` (Widget engine)
4. Installs binaries to `/usr/local/bin/`
5. Installs resources to `/usr/local/share/waylestia/`
6. Creates systemd user services

You can customize the installation prefix:
```bash
INSTALL_PREFIX=$HOME/.local ./scripts/install.sh
```

### `uninstall.sh`
Removes Waylestia from the system.

```bash
./scripts/uninstall.sh
```

## Service Management Scripts

### `start.sh`
Start all Waylestia services.

```bash
./scripts/start.sh
```

Starts:
- `waylestia-core` (main daemon)
- `waylestia-widgets` (widget engine)

### `stop.sh`
Stop all Waylestia services.

```bash
./scripts/stop.sh
```

### `restart.sh`
Restart all Waylestia services.

```bash
./scripts/restart.sh
```

## Service Status

Check service status with systemctl:

```bash
# Check core daemon
systemctl --user status waylestia-core

# Check widgets engine
systemctl --user status waylestia-widgets

# View logs
journalctl --user -u waylestia-core -f
journalctl --user -u waylestia-widgets -f
```

## Enable Auto-Start

Enable services to auto-start on login:

```bash
systemctl --user enable waylestia-core
systemctl --user enable waylestia-widgets
```

## Directory Structure

```
scripts/
├── README.md              # This file
├── install.sh            # Main installation script
├── uninstall.sh          # Uninstallation script
├── start.sh              # Start services
├── stop.sh               # Stop services
├── restart.sh            # Restart services
└── installers/
    └── (Distribution-specific installers)
```

## Build from Source

To build Waylestia manually:

```bash
# Build core
cd core && cargo build --release && cd ..

# Build widgets  
cd widgets && cargo build --release && cd ..
```

Binaries will be at:
- `core/target/release/waylestia-core`
- `widgets/target/release/waylestia-widgets`

## Environment Variables

- `INSTALL_PREFIX` - Installation directory (default: `/usr/local`)
- `CARGO_RELEASE_ARGS` - Additional cargo build arguments

## Systemd Services

Services are installed to `~/.config/systemd/user/`:

- `waylestia-core.service` - Main daemon
- `waylestia-widgets.service` - Widget engine (depends on core)

Both are part of the `graphical-session.target`, so they will start automatically if enabled.

## Troubleshooting

### Services fail to start
Check logs:
```bash
journalctl --user -u waylestia-core -n 50
journalctl --user -u waylestia-widgets -n 50
```

### Permission denied errors
Make sure scripts are executable:
```bash
chmod +x scripts/*.sh
```

### Build fails
- Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Update Rust: `rustup update`
- Install dependencies: `sudo apt install build-essential pkg-config`

## Contributing

Scripts should follow these patterns:
- Use `set -e` for error handling
- Check dependencies before building
- Provide clear progress indicators
- Support custom `INSTALL_PREFIX`
- Log all operations
- Include cleanup on failure (if applicable)

