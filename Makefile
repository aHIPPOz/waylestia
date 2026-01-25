# Makefile multi-langages pour Waylestia (suite OS complète)

.PHONY: all core shell widgets engine proto clean

all: core shell widgets engine

core:
	cd waylestia-core && cargo build

shell:
	cd waylestia-shell && deno task start

widgets:
	cd waylestia-widgets && flutter build web

engine:
	cd waylestia-engine/servo && ./build-servo.sh

proto:
	@echo "Protos dans ./waylestia-proto/ (voir fichiers .proto)"

clean:
	cd waylestia-core && cargo clean
	cd waylestia-widgets && flutter clean || true
	cd waylestia-engine/servo && ./clean-servo.sh || true
