#!/bin/bash

# Waylestia Uninstallation Script

set -e

INSTALL_PREFIX="${INSTALL_PREFIX:-/usr/local}"
INSTALL_BIN="$INSTALL_PREFIX/bin"
INSTALL_SHARE="$INSTALL_PREFIX/share/waylestia"

echo "======================================"
echo "Waylestia Suite OS - Uninstallation"
echo "======================================"

# Stop services
stop_services() {
    echo "Stopping services..."
    
    systemctl --user stop waylestia-widgets 2>/dev/null || true
    systemctl --user stop waylestia-core 2>/dev/null || true
    
    echo "✓ Services stopped"
}

# Disable services
disable_services() {
    echo "Disabling systemd services..."
    
    systemctl --user disable waylestia-widgets 2>/dev/null || true
    systemctl --user disable waylestia-core 2>/dev/null || true
    
    rm -f "$HOME/.config/systemd/user/waylestia-core.service"
    rm -f "$HOME/.config/systemd/user/waylestia-widgets.service"
    
    systemctl --user daemon-reload
    
    echo "✓ Services disabled"
}

# Remove binaries
remove_binaries() {
    echo "Removing binaries..."
    
    rm -f "$INSTALL_BIN/waylestia-core"
    rm -f "$INSTALL_BIN/waylestia-widgets"
    rm -f "$INSTALL_BIN/waylestia-shell"
    
    echo "✓ Binaries removed"
}

# Remove resources
remove_resources() {
    echo "Removing resources..."
    
    rm -rf "$INSTALL_SHARE"
    
    echo "✓ Resources removed"
}

# Main uninstallation flow
main() {
    read -p "Are you sure you want to uninstall Waylestia? (yes/no) " answer
    
    if [ "$answer" != "yes" ]; then
        echo "Uninstallation cancelled."
        exit 0
    fi
    
    stop_services
    disable_services
    remove_binaries
    remove_resources
    
    echo ""
    echo "======================================"
    echo "Uninstallation completed!"
    echo "======================================"
    echo ""
    echo "Waylestia has been removed from $INSTALL_PREFIX"
}

main "$@"
