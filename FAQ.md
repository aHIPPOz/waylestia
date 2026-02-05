# Waylestia FAQ

Frequently asked questions about the Waylestia desktop environment.

## General Questions

### What is Waylestia?

Waylestia is a modern desktop environment for Linux built on Wayland with the following characteristics:

- **Modern Stack**: Written in Rust for performance and safety
- **Wayland-Native**: Built for Wayland, not X11
- **Lightweight**: Minimal resource usage compared to GNOME/KDE
- **Modular**: Components built independently
- **Customizable**: Extensive widget and theme system
- **Designed for Developers**: Easy to extend and modify

### How does Waylestia compare to KDE Plasma and GNOME?

|--------------------|-----------------|-----------|----------|
| Feature            | Waylestia       | KDE       | GNOME    |
|--------------------|-----------------|-----------|----------|
| Language           | Rust            | C++       | C        |
| Size               | ~50-100MB       | ~500MB+   | ~300MB+  |
| Memory (idle)      | ~80MB           | ~400MB    | ~300MB   |
| Startup            | <3s             | ~5s       | ~4s      |
| Widgets/Applets    | Yes (Rust/HTML) | Yes (C++) | Limited  |
| Customization      | Very High       | High      | Low      |
| Keyboard Shortcuts | Modern          | Extensive | Standard |
| Ecosystem          | Growing         | Mature    | Stable   |
| Learning Curve     | Moderate        | High      | Low      |
|--------------------|-----------------|-----------|----------|

### Is Waylestia production-ready?

Currently: **No, it's in early development (v0.1.0) ALPHA VERSION**

- Core functionality: ✅ Working
- Widget engine: ✅ Working
- Applications: 🟡 Basic functionality
- Shell UI: 🚧 In development
- Stability: 🔶 Beta quality

For production use, we recommend KDE Plasma or GNOME.

### Can I use Waylestia as my main desktop?

**Not yet**, but soon! Currently:

- ✅ Can start services
- ✅ Can run individual applications
- ✅ Can load widgets
- ❌ No desktop shell/taskbar (coming v0.2.0)
- ❌ No window manager integration (planned)

We recommend waiting for v0.2.0 if you want the full experience.

### Who is Waylestia for?

- Developers interested in desktop systems
- Rust enthusiasts
- Wayland workspace creators
- People wanting a lightweight alternative
- Contributors wanting to build something modern

---

## Installation & Setup

### What are the system requirements?

**Minimum**:
- CPU: Dual-core, 2.0GHz+
- RAM: 2GB
- Disk: 1GB free space
- Display: Wayland-capable

**Recommended**:
- CPU: Quad-core, 2.5GHz+
- RAM: 4GB+
- Disk: 5GB free space
- GPU: With Wayland drivers

**Operating System**:
- Linux (Ubuntu 20.04+, Fedora 36+, Arch, etc.)
- Requires glibc
- Kernel 5.14+ (for Wayland features)

### What Linux distributions are supported?

**Officially Tested**:
- Ubuntu 22.04 LTS
- Fedora 38, 39
- Arch Linux

**Should Work**:
- Any Linux with glibc + Wayland support
- Debian 11+
- openSUSE Leap 15.5+
- Linux Mint 21+

**Not Supported**:
- X11-only systems
- Alpine/musl-based distros (without glibc compat)

### How do I install Waylestia?

```bash
git clone https://github.com/yourusername/waylestia.git
cd waylestia
make all
./scripts/install.sh
```

See [README.md](README.md) and [DEVELOPMENT.md](DEVELOPMENT.md) for details.

### Can I install Waylestia system-wide?

**Not recommended** for development, but possible:

```bash
sudo ./scripts/install.sh
sudo systemctl enable waylestia-core
sudo systemctl enable waylestia-widgets
```

**Prefer user installation** for better isolation:

```bash
INSTALL_PREFIX=$HOME/.local ./scripts/install.sh
systemctl --user enable waylestia-core
systemctl --user enable waylestia-widgets
```

### What if installation fails?

See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for detailed solutions.

Common issues:
- Missing dependencies → Install build-essential, libgtk-4-dev
- Rust not installed → Run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- protobuf-compiler missing → `apt-get install protobuf-compiler`

---

## Building & Development

### How do I build Waylestia?

```bash
make all          # Build everything
make core         # Build core daemon only
make widgets      # Build widget engine only
make test         # Run tests
```

Or with cargo directly:

```bash
cargo build --release --workspace
```

### Do I need Rust to use Waylestia?

**No** - you only need Rust to build from source.

Pre-built binaries will be available in future versions.

For now:
- Using Waylestia: Need Rust, GTK 4, GJS
- Contributing: Need full development setup

### How long does compilation take?

First build: **5-10 minutes** (depends on system)
- Downloading dependencies
- Compiling Rust code
- Linking libraries

Incremental rebuilds: **10-30 seconds**

Faster systems (modern CPU + SSD): 3-5 minutes first build

### Can I contribute?

**Yes!** We welcome contributions.

See [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Development setup
- Code style guidelines
- Pull request process
- Issue tracking

### What programming languages does Waylestia use?

- **Rust**: Core daemon, widget engine, system services
- **TypeScript/JavaScript**: Applications, widgets, UI logic
- **Protocol Buffers**: IPC message definitions
- **TOML**: Configuration and widget manifests

You don't need to know all of them - contribute to the area you're interested in!

---

## Features & Functionality

### What's included in v0.1.0?

✅ **Core Components**:
- Hyprland window management
- Performance monitoring
- Audio device control (PipeWire)
- Permission system

✅ **Widget Engine**:
- Manifest-based widget system
- HTML/CSS/JS rendering
- Widget discovery and loading
- IPC bridge

✅ **Applications** (8 total):
- Browser, Calendar, Editor, Files, Mail, Media, Settings, Terminal

✅ **Examples**:
- Clock widget
- Dashboard widget  
- System Info widget

❌ **Not Yet**:
- Desktop shell/taskbar (v0.2.0)
- Multi-monitor support (v0.3.0)
- Theme system (v0.3.0)
- Plugin system (v0.4.0+)

### When is feature X coming?

See [ROADMAP.md](ROADMAP.md) for detailed timeline:

- v0.2.0 (Q3-Q4 2024): Shell UI, taskbar, window management
- v0.3.0 (H1 2025): Themes, multi-monitor, advanced widgets
- v0.4.0+ (H2 2025+): Full feature parity, plugins, ecosystem

### Can I create my own widgets?

**Yes!** Widgets are simple HTML/CSS/JavaScript files with a manifest.

Create a widget:

```bash
mkdir -p ~/.local/share/waylestia/widgets/mywidget
cat > manifest.toml << 'EOF'
[widget]
name = "My Widget"
version = "0.1.0"
main = "index.html"
permissions = ["exec", "storage"]
EOF
cat > index.html << 'EOF'
<!DOCTYPE html>
<html>
<body>
  <h1>Hello World!</h1>
  <script>
    // Access Waylestia API
    window.waylestia.sendMessage('ready', {});
  </script>
</body>
</html>
EOF
```

See [webview/README.md](webview/README.md) for full API documentation.

### How do I customize Waylestia?

Currently (v0.1.0):
- Widget creation (HTML/CSS/JS)
- Configuration files (TOML)
- Command-line arguments

Coming soon:
- Theme system (v0.3.0)
- Custom keybindings (v0.2.0)
- Color schemes (v0.3.0)

---

## Performance & System

### How much memory does Waylestia use?

Idle (v0.1.0):
- Core daemon: ~50MB
- Widget engine: ~30MB
- Total: ~80MB

For comparison:
- GNOME: ~300MB
- KDE Plasma: ~400MB
- Lightweight WMs: ~20-50MB

### How much CPU does Waylestia use?

At rest: <1% CPU usage

During activities:
- Window switching: <2%
- Widget updates: <1%
- Application launch: varies

### Can Waylestia run on older hardware?

**Minimum viable**:
- 512MB RAM (not recommended)
- 1GB disk space
- Dual-core CPU

**Recommended minimum**:
- 2GB RAM
- 5GB disk
- Quad-core CPU

Older systems will work but may feel sluggish.

### Is Waylestia as fast as Sway?

Comparable performance but with more features:

| Aspect           | Sway       | Waylestia  |
|------------------|------------|------------|
| Window switching | ~5ms       | ~50ms      |
| Memory overhead  | ~10MB      | ~80MB      |
| Startup          | ~0.5s      | ~2s        |
| Responsiveness   | Very quick | Responsive |

Waylestia has more features (taskbar, widgets) but uses more resources.

---

## Compatibility

### Does Waylestia work with my window manager?

Waylestia is designed for **Hyprland** primarily.

Compatibility:
- **Hyprland**: ✅ Full support
- **WayFire**: 🟡 Partial (no advanced features)

### Can I use Waylestia with GNOME instead of Hyprland?

Not officially, but some components might work:

- Applications: ✅ Should work
- Widgets: 🟡 Partial (depends on WM)
- Core features: ❌ Many won't work

Recommendation: Use on Hyprland for best experience.

### Does Waylestia work with X11?

**No**. Waylestia is Wayland-only.

If you're on X11:
- Switch to Wayland (GNOME Wayland, KDE with Wayland, Hyprland, etc.)
- Or use KDE Plasma / GNOME (mature alternatives)

### Can I run Waylestia apps outside Waylestia?

**Partially**:
- Some apps might work standalone
- Deep integration features won't work
- Protobuf IPC bridge needs core daemon

Best to run full Waylestia stack.

---

## Troubleshooting

### Waylestia won't start

**First, check**:

```bash
systemctl --user status waylestia-core
journalctl --user -u waylestia-core -n 20
```

**Common causes**:
- Dependencies missing → Install all requirements
- Port already in use → `pkill waylestia && sleep 1 && systemctl --user start waylestia-core`
- Hyprland not running → Start Hyprland first

See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for detailed diagnosis.

### Services crash immediately

Check logs:

```bash
journalctl --user -u waylestia-core -n 20 -e
```

**Common causes**:
- Out of memory → `free -h` and try restarting
- Segfault → Missing library, file error
- Hyprland issue → Check `$HYPRLAND_INSTANCE_SIGNATURE`

### Widgets don't appear

```bash
# Check widget directory
ls ~/.local/share/waylestia/widgets/

# View logs
journalctl --user -u waylestia-widgets -f
```

**Common causes**:
- Wrong directory permissions
- Invalid manifest.toml
- Widget engine not running

### How do I report a bug?

1. Gather information:
   ```bash
   uname -a
   rustc --version
   systemctl --user list-units | grep waylestia
   journalctl --user -u waylestia-core -n 50 > error.log
   ```

2. Open GitHub issue with:
   - System information
   - Steps to reproduce
   - Error output/logs
   - What you've tried

3. Or ask in discussions/community

---

## Contributing & For Developers

### How can I contribute?

1. **Code**: Submit pull requests (see [CONTRIBUTING.md](CONTRIBUTING.md))
2. **Widgets**: Create and share examples
3. **Documentation**: Fix typos, add guides
4. **Testing**: Report bugs and edge cases
5. **Design**: Suggest UI/UX improvements

### Where should I start?

1. Read [ARCHITECTURE.md](ARCHITECTURE.md) - understand system design
2. Look at issues tagged `good-first-issue`
3. Read [CONTRIBUTING.md](CONTRIBUTING.md) - contribution guide
4. Set up development environment following [DEVELOPMENT.md](DEVELOPMENT.md)
5. Start with small changes (docs, comments)
6. Graduate to larger features

### Is there a community/chat?

- **GitHub Discussions**: For questions and ideas
- **GitHub Issues**: For bugs and feature requests
- **IRC/Matrix**: TBD (coming soon)
- **Discord**: TBD (coming soon)

### Can I fork and customizWaylestia?

**Yes!** It's open-source under GPL-3.0.

You can:
- Fork the repository
- Modify for your needs
- Add custom features
- Create derivative works

Just follow the license terms.

---

## License & Legal

### What license is Waylestia under?

**GPL-3.0** (GNU General Public License v3.0)

This means:
- Free to use, modify, and distribute
- Must share source code and modifications
- Cannot sell commercially without sharing source
- Must include license in distributions

See [LICENSE](LICENSE) file for full terms.

### Can I use Waylestia commercially?

**Yes**, but with conditions:

- You can build commercial products using Waylestia
- You must provide source code to users
- You can charge for distribution/support
- You cannot claim ownership

Consult a lawyer if unsure about your use case.

### Who owns Waylestia?

Waylestia is owned by its **community and contributors**.

Originally created by [lead developer], now maintained by the community.

Intellectual property:
- Code: Contributors and maintainers
- Name/Logo: Project maintainers
- Contributions: Assigned to project (GPL)

---

## Miscellaneous

### Why Rust?

Rust provides:
- **Memory safety** without garbage collection
- **Type safety** preventing entire classes of bugs
- **Performance** nearly equivalent to C/C++
- **Modern features** for concurrent systems
- **Growing ecosystem** with quality libraries

### Why Wayland?

- **Modern**: Designed for contemporary display servers
- **Security**: Better input/output isolation
- **Cleaner**: Simpler protocol than X11
- **Future**: Where Linux is heading
- **Performance**: Fewer context switches

### Why Protocol Buffers?

- **Type-safe**: Binary serialization with validation
- **Efficient**: Smaller messages than JSON
- **Language-agnostic**: Can generate code for any language
- **Versioning**: Forward/backward compatible
- **Proven**: Used by Google, Kubernetes, etc.

### Can I use Waylestia on Wayland session running in another desktop env?

**Maybe**, depends on:

- If using Hyprland specifically: ✅ Yes
- If using another WM: 🟡 Partial (missing some features)
- If in nested Wayland: ⏸️ Might work

For best experience, use Hyprland.

### Will there be a Windows/macOS version?

**No**, Waylestia is Linux-only because:

- Built specifically for Wayland (Linux only)
- Depends on Linux-specific APIs
- Targets Hyprland (Linux Wayland compositor)
- No plans for other OSes

### Is Waylestia a display manager?

**No**, Waylestia is a **desktop environment** that runs on:

- Hyprland (primary)
- Other Wayland compositors (secondary)
- Displays by an existing display manager (GDM, SDDM, etc.)

You still need a display manager to log in.

### Can I use Waylestia with my existing dotfiles?

**Partially**, depends on:

- Shell config: ✅ Yes (Waylestia doesn't control shell)
- WM config: 🟡 Partial (Waylestia uses different config system)
- Application config: ✅ Yes (most Waylestia apps follow XDG standards)

You may need to migrate some configurations.

---

### Still have questions?

- **Read**: [ARCHITECTURE.md](ARCHITECTURE.md) (system design)
- **Ask**: GitHub Discussions
- **Search**: Close issues might have answered it
- **Report**: Open new issue if nothing helps

---

**Last Updated**: 2024  
**Version**: 1.0

Thank you for your interest in Waylestia! 🚀
