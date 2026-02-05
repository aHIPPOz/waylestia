# Waylestia Quick Reference

Fast lookup guide for common tasks and commands.

## Installation

```bash
# Clone and build
git clone https://github.com/yourusername/waylestia.git
cd waylestia

# Build everything
make all

# Install to user directory (recommended)
INSTALL_PREFIX=$HOME/.local ./scripts/install.sh

# Or use system-wide (requires sudo)
sudo ./scripts/install.sh
```

## Running

```bash
# Start daemon services
systemctl --user start waylestia-core
systemctl --user start waylestia-widgets

# Enable auto-start
systemctl --user enable waylestia-*

# Stop services
systemctl --user stop waylestia-*

# Restart after changes
systemctl --user restart waylestia-core
```

## Development

```bash
# Build all components
make all

# Build specific component
make core      # Core daemon
make widgets   # Widget engine

# Check code (no output, faster)
make check

# Run tests
make test

# Format code
cargo fmt --all

# Lint with clippy
cargo clippy -- -D warnings

# Clean build artifacts
make clean
```

## Logging & Debugging

```bash
# View core daemon logs (follow)
journalctl --user -u waylestia-core -f

# View widget engine logs (follow)
journalctl --user -u waylestia-widgets -f

# View both services
journalctl --user -u 'waylestia-*' -f

# Show last N lines
journalctl --user -u waylestia-core -n 50

# Enable debug logging
RUST_LOG=debug systemctl --user restart waylestia-core
```

## Project Structure

| Directory | Purpose |
|-----------|---------|
| `core/` | Rust daemon - window management, performance, audio |
| `widgets/` | Widget engine - discovery, loading, rendering |
| `apps/` | GJS applications - browser, editor, files, etc. |
| `protobuf/` | Protocol Buffers - IPC message definitions |
| `assets/` | Static resources - icons, wallpapers, widgets |
| `scripts/` | Build and installation scripts |
| `webview/` | Webview integration - TypeScript API |

## Key Files

| File | Purpose |
|------|---------|
| `Makefile` | Build system commands |
| `core/Cargo.toml` | Core daemon dependencies |
| `widgets/Cargo.toml` | Widget engine dependencies |
| `protobuf/*.proto` | IPC message definitions |
| `ARCHITECTURE.md` | System design documentation |
| `CONTRIBUTING.md` | How to contribute |
| `FAQ.md` | Frequently asked questions |

## Common Commands

### Building

```bash
make all              # Build release binaries
make clean            # Remove build artifacts
cargo build --release # Raw cargo build
cargo check           # Type-check only (fast)
```

### Testing

```bash
make test             # Run all tests
cargo test --quiet    # Run tests silently
cargo test mytest -- --nocapture  # Show output for specific test
```

### Code Quality

```bash
cargo fmt             # Auto-format code
cargo clippy          # Run linter
cargo clippy -- -D warnings  # Strict linting
cargo doc --no-deps --open   # Generate + view docs
```

### Services

```bash
systemctl --user start waylestia-core       # Start core
systemctl --user start waylestia-widgets    # Start widgets
systemctl --user status waylestia-core      # Check status
systemctl --user stop waylestia-core        # Stop service
systemctl --user enable waylestia-core      # Auto-start
systemctl --user restart waylestia-core     # Restart
```

### Monitoring

```bash
# Current resource usage
ps aux | grep waylestia

# Watch memory/CPU
watch -n 1 'ps aux | grep waylestia | grep -v grep'

# Monitor logs in real-time
journalctl --user -f

# Check open files/sockets
lsof -p $(pgrep waylestia-core)
```

## Widget Creation

### Directory Structure

```
~/.local/share/waylestia/widgets/mywidget/
├── manifest.toml      # Widget metadata
├── index.html         # Main UI
├── style.css          # Styling (optional)
└── script.js          # Logic (optional)
```

### Minimal Widget

**manifest.toml**:
```toml
[widget]
name = "My Widget"
version = "0.1.0"
main = "index.html"
permissions = []
```

**index.html**:
```html
<!DOCTYPE html>
<html>
<head>
  <title>My Widget</title>
</head>
<body>
  <h1>Hello!</h1>
  <script>
    // Signal widget is ready
    window.waylestia.sendMessage('ready', {});
  </script>
</body>
</html>
```

## Testing a Widget

```bash
# Copy to widget directory
cp -r mywidget ~/.local/share/waylestia/widgets/

# Restart widget engine
systemctl --user restart waylestia-widgets

# Check if loaded
journalctl --user -u waylestia-widgets -f | grep mywidget
```

## Environment Variables

```bash
# Enable debug output
RUST_LOG=debug

# Set widget directory
WAYLESTIA_WIDGET_DIR=$HOME/.local/share/waylestia/widgets

# Set config directory
WAYLESTIA_CONFIG_DIR=$HOME/.config/waylestia

# Set data directory
WAYLESTIA_DATA_DIR=$HOME/.local/share/waylestia

# Disable Hyprland features
WAYLESTIA_NO_HYPRLAND=1

# Custom socket path
WAYLESTIA_SOCKET=/tmp/waylestia-custom.sock
```

## Useful Aliases

Add to `~/.bashrc` or `~/.zshrc`:

```bash
# View Waylestia logs
alias wllog='journalctl --user -u waylestia-core -f'
alias wllog-widgets='journalctl --user -u waylestia-widgets -f'
alias wllog-all='journalctl --user -u "waylestia-*" -f'

# Service control
alias wlstart='systemctl --user start waylestia-*'
alias wlstop='systemctl --user stop waylestia-*'
alias wlrestart='systemctl --user restart waylestia-core && systemctl --user restart waylestia-widgets'
alias wlstatus='systemctl --user status waylestia-*'

# Development
alias wlbuild='cd /workspaces/waylestia && make all'
alias wltest='cargo test --workspace'
alias wlfmt='cargo fmt --all'
alias wllint='cargo clippy -- -D warnings'
```

## File Locations

| Item | Location |
|------|----------|
| **Binaries** | `~/.local/bin/waylestia-*` |
| **Config** | `~/.config/waylestia/` |
| **Data** | `~/.local/share/waylestia/` |
| **Widgets** | `~/.local/share/waylestia/widgets/` |
| **Icons** | `~/.local/share/icons/` |
| **Services** | `~/.local/share/systemd/user/` |
| **Logs** | via `journalctl` |
| **Cache** | `~/.cache/waylestia/` |

## Troubleshooting Quick Fixes

### Service won't start

```bash
# Kill any hanging processes
pkill -9 waylestia-core
pkill -9 waylestia-widgets

# Remove stale sockets
rm -f /tmp/waylestia-*.sock

# Start fresh
systemctl --user daemon-reload
systemctl --user start waylestia-core
```

### High CPU/Memory

```bash
# Check process details
ps aux | grep waylestia

# View resource-hungry processes
top -p $(pgrep waylestia-core)

# Restart service to reset
systemctl --user restart waylestia-core
```

### Dependencies missing

```bash
# Ubuntu/Debian
sudo apt-get install build-essential libgtk-4-dev libadwaita-1-dev protobuf-compiler gjs

# Fedora
sudo dnf install gcc gtk4-devel libadwaita-devel protobuf-compiler gjs

# After installing, rebuild
cargo clean && cargo build --release
```

### Permissions issue

```bash
# Fix widget directory permissions
chmod -R u+rwx ~/.local/share/waylestia/

# Fix binary permissions
chmod +x ~/.local/bin/waylestia-*

# Fix config permissions
chmod -R u+rwx ~/.config/waylestia/
```

## Git Workflow

```bash
# Create feature branch
git checkout -b feature/my-feature

# Make changes and commit
git add .
git commit -m "Add my feature"

# Push to fork
git push origin feature/my-feature

# Create pull request on GitHub
# (use web interface)

# After review, update
git add .
git commit --amend
git push -f origin feature/my-feature

# After merge, clean up
git checkout main
git pull origin main
git branch -d feature/my-feature
git push origin --delete feature/my-feature
```

## Documentation Links

- [README.md](README.md) - Project overview
- [ARCHITECTURE.md](ARCHITECTURE.md) - System design
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guide
- [DEVELOPMENT.md](DEVELOPMENT.md) - Development setup
- [TROUBLESHOOTING.md](TROUBLESHOOTING.md) - Common issues
- [FAQ.md](FAQ.md) - Frequently asked questions
- [ROADMAP.md](ROADMAP.md) - Feature roadmap
- [SECURITY.md](SECURITY.md) - Security policy

## Getting Help

```bash
# Read documentation
cat ARCHITECTURE.md | less
grep "your-question" FAQ.md

# Check existing issues
# https://github.com/yourusername/waylestia/issues

# View recent changes
git log --oneline -n 10

# Search codebase
grep -r "search term" src/

# Check error details
journalctl --user -u waylestia-core -p err
```

## Version Info

```bash
# Check installed version
~/.local/bin/waylestia-core --version

# Check Rust version
rustc --version

# Check dependencies
cat core/Cargo.toml | grep "^[a-z]"

# Show build info
ls -la ~/.local/bin/waylestia-*
```

## Useful Tools

```bash
# Process monitoring
htop                    # Interactive process viewer
glances                 # System monitoring dashboard

# Log viewing
bat -l log kern.log     # Colorized log viewing
lnav kern.log           # Log analysis tool

# Code analysis
tokio-console           # Async task debugger
cargo tree              # Dependency tree

# Formatting
rustfmt                 # Code formatter
prettier                # JavaScript formatter
```

## Performance Tips

```bash
# Build optimized binaries
cargo build --release -j $(nproc)

# Reduce rebuild time with mold linker
sudo apt-get install mold  # Ubuntu
rustup component add rust-src

# Use cargo-nextest for faster tests
cargo install cargo-nextest
cargo nextest run --workspace

# Cache builds
export CARGO_BUILD_JOBS=$(nproc)

# Enable parallel code generation
export RUSTFLAGS="-C codegen-units=1 -C lto=fat"
```

## VSCode Extensions

Recommended for Waylestia development:

- **rust-analyzer** - Rust language support
- **CodeLLDB** - Debugging support
- **Better TOML** - TOML syntax highlighting
- **Protobuf 3** - Protocol Buffer support
- **Even Better TOML** - Enhanced TOML support
- **Code Spell Checker** - Spelling detection

## Quick Stats

| Metric | Value |
|--------|-------|
| **Total Files** | 56+ |
| **Total LOC** | 15,000+ |
| **Core LOC** | 2,000+ |
| **Widget Engine LOC** | 2,000+ |
| **Applications LOC** | 800+ |
| **Test Coverage** | TBD |
| **Version** | 0.1.0 |
| **License** | GPL-3.0 |
| **Rust Edition** | 2021 |
| **MSRV** | 1.70 |

---

**Last Updated**: 2024  
**Quick Reference Version**: 1.0

For detailed information, see the full documentation files listed above.
