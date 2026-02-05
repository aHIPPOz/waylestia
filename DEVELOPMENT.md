# Waylestia Development Guide

Welcome to the Waylestia development guide! This document covers everything you need to know about building, testing, and contributing to Waylestia.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Quick Start](#quick-start)
3. [Project Structure](#project-structure)
4. [Building Components](#building-components)
5. [Running & Testing](#running--testing)
6. [Code Organization](#code-organization)
7. [Debugging](#debugging)
8. [Performance Profiling](#performance-profiling)
9. [Common Issues](#common-issues)
10. [Development Tips](#development-tips)

## Prerequisites

### System Requirements

- **OS**: Linux (Ubuntu 20.04+, Fedora 36+, or similar)
- **Kernel**: 5.14+ (for Wayland support)
- **RAM**: 4GB minimum, 8GB recommended
- **Disk**: 10GB free space

### Runtime Dependencies

#### Ubuntu/Debian

```bash
sudo apt-get install -y \
  build-essential \
  pkg-config \
  git \
  curl \
  libgtk-4-dev \
  libadwaita-1-dev \
  libglib2.0-dev \
  libssl-dev \
  pkg-config \
  libsystemd-dev \
  gjs \
  gir1.2-gtk-4.0 \
  gir1.2-adwaita-1
```

#### Fedora/RHEL

```bash
sudo dnf install -y \
  gcc \
  make \
  pkg-config \
  git \
  curl \
  gtk4-devel \
  libadwaita-devel \
  glib2-devel \
  openssl-devel \
  systemd-devel \
  gjs \
  gjs-devel \
  gobject-introspection-devel
```

### Rust Setup

Install Rust using rustup (if not already installed):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Update Rust:

```bash
rustup update stable
```

Verify installation:

```bash
rustc --version
cargo --version
```

## Quick Start

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/yourusername/waylestia.git
cd waylestia

# Build all components
make all

# Or use cargo directly
cargo build --release --workspace
```

### Install Locally

```bash
# Install to ~/.local (recommended for development)
INSTALL_PREFIX=$HOME/.local make install

# Or use the install script
./scripts/install.sh --user

# Start services
systemctl --user start waylestia-core
systemctl --user start waylestia-widgets
```

### Verify Installation

```bash
# Check systemd services are running
systemctl --user status waylestia-core
systemctl --user status waylestia-widgets

# View logs
journalctl --user -u waylestia-core -f
journalctl --user -u waylestia-widgets -f
```

## Project Structure

```
waylestia/
├── core/                      # Main Rust daemon
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs           # Entry point
│   │   ├── lib.rs            # Module exports
│   │   ├── hyprland.rs       # Wayland integration
│   │   ├── ipc.rs            # IPC server
│   │   ├── perf.rs           # Performance monitoring
│   │   ├── media.rs          # Audio device control
│   │   ├── security.rs       # Permission system
│   │   └── state.rs          # Global state
│   └── build.rs              # Build script for protobuf
│
├── widgets/                   # Widget engine & runtime
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs           # Widget daemon
│   │   ├── lib.rs            # Module exports
│   │   ├── manifest.rs       # TOML manifest parsing
│   │   ├── loader.rs         # Widget discovery
│   │   ├── renderer.rs       # HTML rendering
│   │   ├── state.rs          # Widget state management
│   │   └── ipc.rs            # Widget IPC server
│   └── build.rs              # Build script
│
├── apps/                      # GJS applications
│   ├── browser/main.ts       # Web browser
│   ├── calendar/main.ts      # Calendar app
│   ├── editor/main.ts        # Text editor
│   ├── files/main.ts         # File manager
│   ├── mail/main.ts          # Email client
│   ├── media/main.ts         # Media player
│   ├── settings/main.ts      # Settings panel
│   └── terminal/main.ts      # Terminal emulator
│
├── assets/                    # Static resources
│   ├── icons/
│   ├── wallpapers/
│   └── widgets/              # User widgets
│       ├── clock/
│       ├── dashboard/
│       └── sysinfo/
│
├── protobuf/                  # Protocol definitions
│   ├── core_runtime.proto
│   ├── core_shell.proto
│   ├── runtime_widgets.proto
│   ├── shell_widgets.proto
│   ├── apps.proto
│   └── README.md
│
├── webview/                   # Webview integration
│   ├── README.md
│   └── waylestia-webview-api.ts
│
├── scripts/                   # Build & install scripts
│   ├── install.sh
│   ├── uninstall.sh
│   ├── start.sh
│   ├── stop.sh
│   └── restart.sh
│
├── Makefile                   # Build system
├── README.md
├── ARCHITECTURE.md
└── CONTRIBUTING.md
```

## Building Components

### Build All

```bash
# Release build (optimized)
make all

# Or using cargo
cargo build --release --workspace
```

### Build Core Daemon

```bash
# Full build
make core

# Using cargo directly
cd core && cargo build --release

# Debug build
cd core && cargo build

# With verbose output
cd core && cargo build --release --verbose
```

### Build Widget Engine

```bash
# Full build
make widgets

# Using cargo directly
cd widgets && cargo build --release
```

### Build Check (no output)

```bash
# Type check all components
make check

# Or using cargo
cargo check --workspace
```

### Clean Build Artifacts

```bash
# Clean everything
make clean

# Or using cargo
cargo clean
```

## Running & Testing

### Run Tests

```bash
# Run all tests
make test

# Or using cargo
cargo test --workspace

# Run tests for specific crate
cd core && cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name -- --nocapture
```

### Run Services

```bash
# Start all services
make start

# Or manually
systemctl --user start waylestia-core
systemctl --user start waylestia-widgets

# Stop services
make stop

# Restart
make restart
```

### View Logs

```bash
# Core daemon logs (follow)
journalctl --user -u waylestia-core -f

# Widget engine logs (follow)
journalctl --user -u waylestia-widgets -f

# Both services
journalctl --user -u 'waylestia-*' -f

# Show last N lines
journalctl --user -u waylestia-core -n 50

# Show since timestamp
journalctl --user -u waylestia-core --since "2024-01-01 12:00:00"

# Export to file
journalctl --user -u waylestia-core > waylestia-core.log
```

### Run Individual Components

```bash
# Core daemon (manual)
cd core && cargo run --release

# Widget engine (manual)
cd widgets && cargo run --release

# Application (requires GJS)
gjs --version  # Verify GJS is installed
gjs apps/browser/main.ts
```

## Code Organization

### Rust Modules (core)

```rust
// core/src/lib.rs
pub mod hyprland;    // Wayland window management
pub mod ipc;         // Protocol buffer IPC
pub mod perf;        // Performance monitoring
pub mod media;       // Audio device control
pub mod security;    // Permission system
pub mod state;       // Global state types
```

### Rust Modules (widgets)

```rust
// widgets/src/lib.rs
pub mod manifest;    // TOML manifest parsing
pub mod loader;      // Widget discovery
pub mod renderer;    // HTML generation
pub mod state;       // Widget state management
pub mod ipc;         // Widget IPC server
```

### Adding New Modules

1. Create file: `src/new_module.rs`
2. Add to `lib.rs`: `pub mod new_module;`
3. Implement: `pub struct MyStruct { ... }`
4. Export: `pub use new_module::*;` (in lib.rs)
5. Document: Add `///` comments

Example:

```rust
// src/myfeature.rs
/// Manages my awesome feature
pub struct MyFeature {
    name: String,
}

impl MyFeature {
    /// Creates a new instance
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
```

## Debugging

### Enable Debug Logging

Set environment variable before running:

```bash
# Enable Rust debug logging
RUST_LOG=debug cargo run --release

# Or with service
export RUST_LOG=debug
systemctl --user restart waylestia-core

# Check logs
journalctl --user -u waylestia-core -f
```

### Use `dbg!` Macro

```rust
let value = some_function();
dbg!(value);  // Prints: [src/main.rs:42] value = 123
```

### GDB Debugging (Rust)

```bash
# Build with debug symbols
cargo build  # Debug build includes symbols

# Run under debugger
gdb ./target/debug/waylestia-core
(gdb) run
(gdb) backtrace
(gdb) print variable_name
(gdb) step
(gdb) continue
```

### VSCode Debugging

Install CodeLLDB extension:

```bash
code --install-extension vadimcn.vscode-lldb
```

Create `.vscode/launch.json`:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug Waylestia Core",
      "cargo": {
        "args": [
          "build",
          "--bin=waylestia-core",
          "--package=waylestia-core"
        ],
        "filter": {
          "name": "waylestia-core",
          "kind": "bin"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

Then press F5 to start debugging.

## Performance Profiling

### Using Flamegraph

Install flamegraph:

```bash
cargo install flamegraph
```

Generate profile:

```bash
cd core
cargo flamegraph --bin waylestia-core -o flamegraph.svg
# Opens in browser automatically
```

### Using Perf

```bash
# Build with symbols
cargo build --release

# Run under perf
perf record -g ./target/release/waylestia-core

# View results
perf report

# Generate flamegraph from perf
perf script > out.perf-folded
# Use online flamegraph tool
```

### Memory Profiling

```bash
# Check memory usage
valgrind --leak-check=full ./target/release/waylestia-core

# Monitor runtime memory
watch -n 1 'ps aux | grep waylestia'
```

## Common Issues

### Issue: `cargo build` fails with protobuf error

**Cause**: Missing protobuf compiler

**Solution**:
```bash
# Ubuntu
sudo apt-get install protobuf-compiler

# Fedora
sudo dnf install protobuf-compiler

# Then rebuild
cargo clean
cargo build --release
```

### Issue: Can't find GTK development headers

**Cause**: Missing development packages

**Solution**:
```bash
# Ubuntu
sudo apt-get install libgtk-4-dev libadwaita-1-dev

# Fedora
sudo dnf install gtk4-devel libadwaita-devel

# Then rebuild
cargo clean
cargo build --release
```

### Issue: Services fail to start

**Cause**: Installation issue or dependency missing

**Solution**:
```bash
# Check service status
systemctl --user status waylestia-core

# View error logs
journalctl --user -u waylestia-core -n 20

# Reinstall
./scripts/uninstall.sh
./scripts/install.sh
```

### Issue: Permission denied when installing

**Cause**: Wrong installation prefix or permissions

**Solution**:
```bash
# Use user-local installation
INSTALL_PREFIX=$HOME/.local ./scripts/install.sh

# Or use sudo for system-wide (not recommended for development)
sudo ./scripts/install.sh
```

### Issue: `error: unable to infer generic parameters`

**Cause**: Rust compilation issue, usually type mismatch

**Solution**:
```bash
# Clear build cache
cargo clean

# Rebuild with verbose output
cargo build --verbose

# Check Rust version
rustup update

# Check for type errors in IDE
cargo check
```

## Development Tips

### Efficient Development Workflow

1. **Watch mode**: Auto-rebuild on changes
   ```bash
   cargo watch -x build --workspace
   ```

2. **Hot reload services**:
   ```bash
   cargo build && systemctl --user restart waylestia-core
   ```

3. **Run tests on change**:
   ```bash
   cargo watch -x test --workspace
   ```

4. **Format on save** (VSCode):
   - Install Rust Analyzer
   - Set `rust-analyzer.checkOnSave.command` to `clippy`

### Code Style

Automatic formatting:

```bash
# Format all code
cargo fmt --all

# Check format without changing
cargo fmt --all -- --check

# Format specific file
cargo fmt -- src/myfile.rs
```

Linting:

```bash
# Run clippy (smart linting)
cargo clippy -- -D warnings

# Or with all features
cargo clippy --all-features -- -D warnings
```

### Testing Strategy

```bash
# Unit tests (in src/)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        assert_eq!(1 + 1, 2);
    }
}

# Integration tests (in tests/)
#[test]
fn test_e2e_workflow() {
    // Full system test
}
```

### Documentation

Generate docs:

```bash
# Build and open docs
cargo doc --no-deps --open

# Or just build
cargo doc --no-deps
```

Add documentation comments:

```rust
/// Brief description (one line)
///
/// Longer description with details.
///
/// # Examples
/// ```
/// let result = function();
/// assert_eq!(result, expected);
/// ```
///
/// # Errors
/// Returns `Err` if...
pub fn my_function() -> Result<T> {
    // ...
}
```

### Git Workflow

```bash
# Create feature branch
git checkout -b feature/my-feature

# Make changes and commit
git add .
git commit -m "Add my feature"

# Keep branch updated
git fetch origin
git rebase origin/main

# Push to your fork
git push origin feature/my-feature

# Create pull request on GitHub
```

### Pre-commit Checks

Run before committing:

```bash
#!/bin/bash
# .git/hooks/pre-commit

cargo fmt --all
cargo clippy -- -D warnings
cargo test --all

if [ $? -ne 0 ]; then
  echo "Pre-commit checks failed!"
  exit 1
fi
```

Make executable:

```bash
chmod +x .git/hooks/pre-commit
```

---

**Happy coding!** If you have questions, open an issue or join our community discussions.
