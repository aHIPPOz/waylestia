# Makefile multi-langages pour Waylestia (suite OS complète)

.PHONY: all core widgets clean help install uninstall start stop logs

# Build targets
all: core widgets

core:
	@echo "Building waylestia-core..."
	cd core && cargo build --release && cd ..
	@echo "✓ Core built: core/target/release/waylestia-core"

widgets:
	@echo "Building waylestia-widgets..."
	cd widgets && cargo build --release && cd ..
	@echo "✓ Widgets built: widgets/target/release/waylestia-widgets"

# Installation
install: all
	@echo "Installing Waylestia..."
	chmod +x scripts/*.sh
	./scripts/install.sh

uninstall:
	@echo "Uninstalling Waylestia..."
	./scripts/uninstall.sh

# Service management
start:
	@echo "Starting Waylestia services..."
	./scripts/start.sh

stop:
	@echo "Stopping Waylestia services..."
	./scripts/stop.sh

restart:
	@echo "Restarting Waylestia services..."
	./scripts/restart.sh

# Logs
logs-core:
	journalctl --user -u waylestia-core -f

logs-widgets:
	journalctl --user -u waylestia-widgets -f

logs-all:
	journalctl --user -u "waylestia-*" -f

# Development
clean:
	@echo "Cleaning build artifacts..."
	cd core && cargo clean && cd ..
	cd widgets && cargo clean && cd ..
	@echo "✓ Cleaned"

test:
	@echo "Running tests..."
	cd core && cargo test && cd ..
	cd widgets && cargo test && cd ..

check:
	@echo "Checking code..."
	cd core && cargo check && cd ..
	cd widgets && cargo check && cd ..

fmt:
	@echo "Formatting code..."
	cd core && cargo fmt && cd ..
	cd widgets && cargo fmt && cd ..

lint:
	@echo "Linting code..."
	cd core && cargo clippy && cd ..
	cd widgets && cargo clippy && cd ..

# Help
help:
	@echo "Waylestia — Modern Linux Desktop Suite"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Build targets:"
	@echo "  make all        - Build all components"
	@echo "  make core       - Build waylestia-core"
	@echo "  make widgets    - Build waylestia-widgets"
	@echo ""
	@echo "Installation:"
	@echo "  make install    - Install to system"
	@echo "  make uninstall  - Remove from system"
	@echo ""
	@echo "Service management:"
	@echo "  make start      - Start services"
	@echo "  make stop       - Stop services"
	@echo "  make restart    - Restart services"
	@echo ""
	@echo "Monitoring:"
	@echo "  make logs-core      - View core logs"
	@echo "  make logs-widgets   - View widgets logs"
	@echo "  make logs-all       - View all logs"
	@echo ""
	@echo "Development:"
	@echo "  make test       - Run tests"
	@echo "  make check      - Check code"
	@echo "  make fmt        - Format code"
	@echo "  make lint       - Run clippy linter"
	@echo "  make clean      - Clean build artifacts"
	@echo ""
	@echo "Documentation:"
	@echo "  See README.md and ARCHITECTURE.md"

