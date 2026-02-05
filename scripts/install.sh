#!/bin/bash

# Waylestia Installation Script
# This script installs waylestia and all its components

set -e

VERSION="0.1.0"
INSTALL_PREFIX="${INSTALL_PREFIX:-/usr/local}"
INSTALL_BIN="$INSTALL_PREFIX/bin"
INSTALL_LIB="$INSTALL_PREFIX/lib"
INSTALL_SHARE="$INSTALL_PREFIX/share/waylestia"

echo "======================================"
echo "Waylestia Suite OS - Installation $VERSION"
echo "======================================"
echo "Target prefix: $INSTALL_PREFIX"
echo ""

# Check dependencies
check_dependencies() {
    echo "[1/5] Checking dependencies..."
    
    local deps=("rustc" "cargo" "git" "pkg-config" "gcc")
    local missing=()
    
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            missing+=("$dep")
        fi
    done
    
    if [ ${#missing[@]} -gt 0 ]; then
        echo "ERROR: Missing dependencies: ${missing[*]}"
        echo "Please install them and try again."
        exit 1
    fi
    
    echo "✓ All dependencies found"
}

# Build core
build_core() {
    echo "[2/5] Building waylestia-core..."
    
    cd core
    cargo build --release
    cd ..
    
    echo "✓ Core built successfully"
}

# Build widgets engine
build_widgets() {
    echo "[3/5] Building waylestia-widgets..."
    
    cd widgets
    cargo build --release
    cd ..
    
    echo "✓ Widgets engine built successfully"
}

# Install binaries
install_binaries() {
    echo "[4/5] Installing binaries..."
    
    mkdir -p "$INSTALL_BIN"
    mkdir -p "$INSTALL_SHARE"
    
    # Core daemon
    cp target/release/waylestia-core "$INSTALL_BIN/"
    chmod +x "$INSTALL_BIN/waylestia-core"
    
    # Widgets daemon
    cp target/release/waylestia-widgets "$INSTALL_BIN/"
    chmod +x "$INSTALL_BIN/waylestia-widgets"
    
    # Shell (will be built/copied later)
    # cp shell/dist/waylestia-shell "$INSTALL_BIN/"
    
    echo "✓ Binaries installed to $INSTALL_BIN"
}

# Install resources
install_resources() {
    echo "[5/5] Installing resources..."
    
    mkdir -p "$INSTALL_SHARE/assets"
    mkdir -p "$INSTALL_SHARE/apps"
    mkdir -p "$INSTALL_SHARE/protocols"
    
    # Copy assets
    cp -r assets/* "$INSTALL_SHARE/assets/"
    
    # Copy apps
    cp -r apps/* "$INSTALL_SHARE/apps/"
    
    # Copy proto specs
    cp -r protobuf/*.proto "$INSTALL_SHARE/protocols/"
    
    echo "✓ Resources installed to $INSTALL_SHARE"
}

# Create systemd user service
create_systemd_service() {
    echo "Creating systemd user service..."
    
    local service_dir="$HOME/.config/systemd/user"
    mkdir -p "$service_dir"
    
    cat > "$service_dir/waylestia-core.service" << EOF
[Unit]
Description=Waylestia Core Daemon
After=display-manager.service
PartOf=graphical-session.target
Wants=graphical-session-pre.target

[Service]
Type=simple
ExecStart=$INSTALL_BIN/waylestia-core
Restart=on-failure
RestartSec=5

[Install]
WantedBy=graphical-session.target
EOF

    cat > "$service_dir/waylestia-widgets.service" << EOF
[Unit]
Description=Waylestia Widgets Engine
After=waylestia-core.service
PartOf=graphical-session.target

[Service]
Type=simple
ExecStart=$INSTALL_BIN/waylestia-widgets
Restart=on-failure
RestartSec=5
Requires=waylestia-core.service

[Install]
WantedBy=graphical-session.target
EOF

    # Reload systemd
    systemctl --user daemon-reload
    
    echo "✓ Systemd services created"
    echo ""
    echo "To start services, run:"
    echo "  systemctl --user start waylestia-core"
    echo "  systemctl --user start waylestia-widgets"
    echo ""
    echo "To enable auto-start:"
    echo "  systemctl --user enable waylestia-core"
    echo "  systemctl --user enable waylestia-widgets"
}

# Main installation flow
main() {
    check_dependencies
    build_core
    build_widgets
    install_binaries
    install_resources
    create_systemd_service
    
    echo ""
    echo "======================================"
    echo "Installation completed successfully!"
    echo "======================================"
    echo ""
    echo "Waylestia is now installed at $INSTALL_PREFIX"
    echo "Next steps:"
    echo "  1. Start the daemon: systemctl --user start waylestia-core"
    echo "  2. Start widgets: systemctl --user start waylestia-widgets"
    echo "  3. Launch applications from /usr/local/share/waylestia/apps/"
    echo ""
}

main "$@"
