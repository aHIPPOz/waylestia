# Makefile multi-langages pour Ganymede Shell

.PHONY: all core hotbar runtime widgets proto clean

all: core hotbar runtime widgets

core:
	cd ganymede-core && cargo build

hotbar:
	cd ganymede-hotbar && cargo build

runtime:
	cd ganymede-runtime && deno task start

widgets:
	cd ganymede-widgets && flutter build linux || echo "Flutter non installé"

proto:
	@echo "Protos dans ./proto/ (voir fichiers .proto)"

clean:
	cd ganymede-core && cargo clean
	cd ganymede-hotbar && cargo clean
	cd ganymede-widgets && flutter clean || true
