# Waylestia Troubleshooting Guide

## Common Issues & Solutions

This guide helps you resolve common problems when building, installing, or running Waylestia.

## Installation Issues

### Issue: `command not found: cargo`

**Symptoms**: Build command fails, cargo not recognized

**Causes**:
- Rust not installed
- Rust not in PATH
- Shell environment not updated

**Solutions**:

1. Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Source the environment:
```bash
source $HOME/.cargo/env
```

3. Verify installation:
```bash
cargo --version
rustc --version
```

4. Restart terminal if still not found

---

### Issue: Missing development libraries

**Symptoms**: 
```
error: pkg-config cannot find...
error: cannot find -lgtk-4
error: cannot find -lglib-2.0
```

**Causes**:
- GTK development headers not installed
- Wayland development packages missing
- System libraries not in pkg-config path

**Solutions**:

**Ubuntu/Debian**:
```bash
sudo apt-get update
sudo apt-get install -y \
  build-essential \
  pkg-config \
  libgtk-4-dev \
  libadwaita-1-dev \
  libglib2.0-dev \
  libssl-dev \
  libsystemd-dev
```

**Fedora/RHEL**:
```bash
sudo dnf install -y \
  gcc \
  make \
  pkg-config \
  gtk4-devel \
  libadwaita-devel \
  glib2-devel \
  openssl-devel \
  systemd-devel
```

**Arch**:
```bash
sudo pacman -S base-devel gtk4 libadwaita glib2 openssl systemd
```

---

### Issue: Protobuf compiler not found

**Symptoms**:
```
error: protoc is not found, cannot generate protobuf files
```

**Causes**:
- Protocol Buffers compiler not installed
- Not in system PATH

**Solutions**:

**Ubuntu/Debian**:
```bash
sudo apt-get install protobuf-compiler
```

**Fedora**:
```bash
sudo dnf install protobuf-compiler
```

**Verify**:
```bash
protoc --version  # Should show version >= 3.12
```

---

### Issue: GJS not available

**Symptoms**:
```
error: Cannot find GJS in pkg-config path
applications fail to run
```

**Causes**:
- GJS runtime not installed
- Development headers missing
- Version mismatch

**Solutions**:

**Ubuntu/Debian**:
```bash
sudo apt-get install gjs gir1.2-gtk-4.0 gir1.2-adwaita-1
```

**Fedora**:
```bash
sudo dnf install gjs gjs-devel gobject-introspection-devel
```

**Verify**:
```bash
gjs --version
gir1.2-gtk-4.0 --libs  # Should output library info
```

---

## Build Issues

### Issue: `error[E0308]: mismatched types`

**Symptoms**:
```
error[E0308]: expected i32, found u32
```

**Causes**:
- Type mismatch in Rust code
- Incompatible types being used
- Rust version incompatibility

**Solutions**:

1. Check Rust version:
```bash
rustc --version
rustup update stable
```

2. Clean and rebuild:
```bash
cargo clean
cargo build --release
```

3. Check error details:
```bash
cargo build --verbose 2>&1 | grep -A5 "error\["
```

4. If in custom code, verify types match function signatures

---

### Issue: `error: failed to resolve: use of undeclared type`

**Symptoms**:
```
error: failed to resolve: use of undeclared type 'MyStruct'
```

**Causes**:
- Missing module import
- Type not exported from module
- Typo in type name

**Solutions**:

1. Check module is exported in `lib.rs`:
```rust
// src/lib.rs
pub mod mymodule;
pub use mymodule::MyStruct;  // <-- Must export
```

2. Import at module level:
```rust
use crate::mymodule::MyStruct;
```

3. Check spelling matches exactly

---

### Issue: `error: could not compile 'waylestia-core'`

**Symptoms**:
```
error: could not compile `waylestia-core`
```

**Causes**:
- Multiple compilation errors
- Dependency version conflicts
- Feature flag issues

**Solutions**:

1. See full error output:
```bash
cargo build 2>&1 | head -50
```

2. Check for duplicate errors:
```bash
cargo build 2>&1 | grep "^error"
```

3. Clean and retry:
```bash
cargo clean
cargo build --release
```

4. Check feature flags:
```bash
cargo build --release --features "feature1,feature2"
```

---

## Runtime Issues

### Issue: Services won't start (systemd)

**Symptoms**:
```
Failed to start waylestia-core.service
Failed to start waylestia-widgets.service
```

**Solutions**:

1. Check service status:
```bash
systemctl --user status waylestia-core
```

2. View detailed error:
```bash
journalctl --user -u waylestia-core -n 20 -e
```

3. Check if service files exist:
```bash
ls -la ~/.local/share/systemd/user/waylestia*
```

4. Reinstall services:
```bash
./scripts/uninstall.sh
./scripts/install.sh
```

5. Reload systemd daemon:
```bash
systemctl --user daemon-reload
systemctl --user restart waylestia-core
```

---

### Issue: Service crashes immediately

**Symptoms**:
```
Process exited with code SIGKILL
Active: failed (Result: signal)
```

**Causes**:
- Out of memory
- Segmentation fault
- Missing runtime dependencies
- Hyprland not available

**Solutions**:

1. Check memory:
```bash
free -h
ps aux | grep waylestia
```

2. Check for segfault:
```bash
journalctl --user -u waylestia-core -n 5
# Look for: "segmentation fault"
```

3. Run service in foreground to see error:
```bash
systemctl --user stop waylestia-core
~/.local/bin/waylestia-core  # Run directly
```

4. Check Hyprland socket:
```bash
ls -la $HYPRLAND_INSTANCE_SIGNATURE
# Should show socket connection available
```

---

### Issue: `error: IPC server failed to bind`

**Symptoms**:
```
Error: Failed to bind IPC socket: Address already in use
```

**Causes**:
- Service already running
- Previous instance didn't clean up
- Socket file exists

**Solutions**:

1. Check if service running:
```bash
systemctl --user status waylestia-core
```

2. Kill existing process:
```bash
pkill waylestia-core
pkill waylestia-widgets
```

3. Remove stale socket files:
```bash
rm -f /tmp/waylestia-*.sock
```

4. Start services:
```bash
systemctl --user start waylestia-core
systemctl --user start waylestia-widgets
```

---

### Issue: Widgets don't load

**Symptoms**:
```
No widgets appear
Logs show: "No widgets found"
```

**Causes**:
- Widget directory not found
- Manifest files invalid
- Loader configuration wrong

**Solutions**:

1. Check widget directory:
```bash
ls -la ~/.local/share/waylestia/widgets/
# Or check installed location
find / -path "*/assets/widgets" 2>/dev/null
```

2. Verify manifest files:
```bash
cat ~/.local/share/waylestia/widgets/dashboard/manifest.toml
# Should have: [widget], name, version, main
```

3. Check loader output:
```bash
RUST_LOG=debug systemctl --user start waylestia-widgets
journalctl --user -u waylestia-widgets -f
# Look for: "Loading widgets from..."
```

4. Test loader directly:
```bash
~/.local/bin/waylestia-widgets --list-widgets
```

---

### Issue: High CPU usage

**Symptoms**:
```
waylestia-core uses 80%+ CPU
System sluggish
```

**Causes**:
- Busy-wait loop in code
- Inefficient event polling
- Memory leak causing GC pressure

**Solutions**:

1. Profile with top:
```bash
top -p $(pgrep -f waylestia-core)
# Press 'H' for threads, 'u' for user filter
```

2. Use perf for detailed analysis:
```bash
perf record -p $(pgrep -f waylestia-core) -g -- sleep 10
perf report
```

3. Check logs for repeated errors:
```bash
journalctl --user -u waylestia-core -f | head -100
```

4. Temporarily disable features to isolate:
```bash
# Edit and rebuild with fewer features
cargo build --release --no-default-features
```

---

### Issue: High memory usage

**Symptoms**:
```
waylestia-core uses 1GB+ RAM
System becomes unresponsive
```

**Causes**:
- Widget state not cleaning up
- Memory leak in Rust code
- Large data structures held in memory

**Solutions**:

1. Monitor memory:
```bash
watch -n 1 'ps aux | grep waylestia | grep -v grep'
```

2. Check with valgrind:
```bash
valgrind --leak-check=full --show-leak-kinds=all \
  ~/.local/bin/waylestia-core 2>&1 | tee valgrind.log
```

3. Take memory snapshot:
```bash
pmap -x $(pgrep -f waylestia-core) | tail -1
```

4. Restart service to reset:
```bash
systemctl --user restart waylestia-core
watch -n 1 'ps aux | grep waylestia-core'
```

---

## Network & IPC Issues

### Issue: `error: Failed to connect to IPC socket`

**Symptoms**:
```
Connection refused
Socket not found
```

**Causes**:
- Service not running
- Socket path incorrect
- Permissions wrong

**Solutions**:

1. Verify service running:
```bash
systemctl --user status waylestia-core
```

2. Check socket exists:
```bash
ls -la /tmp/waylestia-*.sock
# Or check configured path
grep -r "SOCKET_PATH" ~/.local/share/waylestia/
```

3. Check permissions:
```bash
ls -la /tmp/waylestia-*.sock
# Should be readable by your user
chmod 600 /tmp/waylestia-*.sock
```

4. Restart service:
```bash
systemctl --user restart waylestia-core
```

---

### Issue: Protobuf message deserialization fails

**Symptoms**:
```
Error: failed to parse protobuf message
Error: invalid wire type
```

**Causes**:
- Message format incompatible
- Code generation out of sync
- Wrong message version

**Solutions**:

1. Regenerate protobuf code:
```bash
cargo build --release  # Triggers build.rs
```

2. Check proto file syntax:
```bash
protoc --cpp_out=/tmp protobuf/core_runtime.proto
# If errors, fix syntax
```

3. Reset and rebuild:
```bash
cargo clean
cargo build --release
```

---

## Wayland / Display Issues

### Issue: `error: HYPRLAND_INSTANCE_SIGNATURE not set`

**Symptoms**:
```
Hyprland manager initialization failed
Cannot connect to Hyprland
```

**Causes**:
- Not running on Hyprland
- Session environment not set
- Display not configured

**Solutions**:

1. Check if Hyprland running:
```bash
echo $HYPRLAND_INSTANCE_SIGNATURE
# Should output something like: 0:0
```

2. If empty, you're not on Hyprland:
```bash
# Check current WM
echo $XDG_SESSION_TYPE  # Should be "wayland"
ps aux | grep -E "hyprland|sway|gnome"
```

3. Start on Hyprland first, or skip Hyprland features:
```bash
# Set fallback mode
export WAYLESTIA_NO_HYPRLAND=1
```

---

### Issue: Display/screen blank

**Symptoms**:
```
Screen goes blank when Waylestia starts
Cannot see anything
```

**Causes**:
- Shell UI crash
- Display server issue
- Rendering problem

**Solutions**:

1. Switch to terminal (Ctrl+Alt+F2):
```bash
# Login to TTY2
```

2. Kill Waylestia processes:
```bash
pkill -9 waylestia
pkill -9 gjs
```

3. Check logs:
```bash
journalctl --user -u 'waylestia-*' --since "10 minutes ago"
```

4. Start on different display:
```bash
DISPLAY=:1 ~/.local/bin/waylestia-core
```

---

## Permission & Security Issues

### Issue: Permission denied errors

**Symptoms**:
```
error: Permission denied
error: Access denied
```

**Causes**:
- Files owned by wrong user
- Wrong file permissions
- Running with insufficient privileges

**Solutions**:

1. Check ownership:
```bash
ls -la ~/.local/share/waylestia/
ls -la ~/.local/bin/waylestia-*
```

2. Fix permissions:
```bash
chown -R $USER:$USER ~/.local/share/waylestia/
chmod -R u+wxr ~/.local/share/waylestia/
```

3. Check running user:
```bash
ps aux | grep waylestia
# Should show YOUR_USER, not root
```

---

### Issue: SELinux blocking Waylestia

**Symptoms** (Fedora/RHEL):
```
SELinux is preventing... denied
```

**Solutions**:

1. Check SELinux status:
```bash
getenforce  # Shows Enforcing, Permissive, or Disabled
```

2. Temporarily disable:
```bash
# For testing only!
sudo setenforce 0
```

3. Check denials:
```bash
sudo ausearch -m AVC | grep waylestia
```

4. Create policy:
```bash
sudo audit2allow -M waylestia
sudo semodule -i waylestia.pp
```

---

## File & Configuration Issues

### Issue: Config file not found

**Symptoms**:
```
Error: Configuration not found
```

**Solutions**:

1. Find config:
```bash
find ~/.config -name "*waylestia*" 2>/dev/null
find ~/.local/share -path "*waylestia*" 2>/dev/null
```

2. Create default config:
```bash
mkdir -p ~/.config/waylestia
touch ~/.config/waylestia/config.toml
```

3. Check expected location:
```bash
ls -la ~/.config/waylestia/
```

---

### Issue: Widget manifest invalid

**Symptoms**:
```
Error parsing manifest: invalid TOML
```

**Solutions**:

1. Validate TOML:
```bash
cat ~/.local/share/waylestia/widgets/mywidget/manifest.toml
# Check syntax manually
```

2. Validate with online tool:
```bash
# Copy contents to: https://www.toml-lint.com/
```

3. Common errors:
```toml
# ❌ Wrong - missing quotes
version = 1.0

# ✅ Right
version = "1.0"

# ❌ Wrong - invalid section
[widget info]

# ✅ Right  
[widget]
```

---

## Getting Help

If you're still stuck:

1. **Check logs first**:
```bash
journalctl --user -u waylestia-core -n 100 > logs.txt
journalctl --user -u waylestia-widgets -n 100 >> logs.txt
```

2. **Gather system info**:
```bash
uname -a
rustc --version
cargo --version
protoc --version
gjs --version
```

3. **Open an issue** with:
   - Command that failed
   - Complete error output
   - System information
   - Relevant logs
   - What you've already tried

4. **Join community**: Ask in discussions or IRC

---

**Last Updated**: 2024
**Version**: 1.0
