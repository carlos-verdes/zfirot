.PHONY: dev run build bundle dmg install css css-watch fmt lint test check icon

PRESENTATION := crates/presentation

# Where `dx bundle` writes the macOS artifacts.
BUNDLE_DIR := target/dx/zfirot/bundle/macos/bundle
APP := $(BUNDLE_DIR)/macos/Zfirot.app

# Compile the Tailwind + daisyUI + Iconify stylesheet into the bundled asset.
css:
	cd $(PRESENTATION) && npm install && npm run build:css

# Recompile the stylesheet on change (run alongside `make dev`).
css-watch:
	cd $(PRESENTATION) && npm run watch:css

# Rasterise the ZF monogram (assets/logo.svg) into the bundled window icon
# (assets/icon.png) plus the macOS app icon (assets/icon.icns). icon.png is the
# runtime window icon (Windows/Linux); icon.icns is the macOS .app dock icon.
# Uses macOS QuickLook + sips + iconutil, so it needs no extra tooling; rerun
# after editing logo.svg.
icon:
	cd $(PRESENTATION)/assets && \
	qlmanage -t -s 512 -o . logo.svg >/dev/null && \
	sips -s format png logo.svg.png --out icon.png >/dev/null && \
	rm -f logo.svg.png && \
	rm -rf icon.iconset && mkdir icon.iconset && \
	for s in 16 32 64 128 256 512; do \
		sips -z $$s $$s icon.png --out icon.iconset/icon_$${s}x$${s}.png >/dev/null; \
		d=$$((s*2)); \
		sips -z $$d $$d icon.png --out icon.iconset/icon_$${s}x$${s}@2x.png >/dev/null; \
	done && \
	iconutil -c icns icon.iconset -o icon.icns && \
	rm -rf icon.iconset

# Start the desktop app in dev mode (hot-reload) via the Dioxus CLI.
# Compiles the stylesheet first so styling is up to date. Loads .env (if present)
# so ZFIROT_GITHUB_TOKEN reaches the dev-only env secure store, avoiding repeated
# OS keychain prompts across rebuilds.
dev: css
	set -a; [ -f .env ] && . ./.env; set +a; dx serve --package zfirot --platform desktop

# Run the app once without the Dioxus CLI.
run: css
	set -a; [ -f .env ] && . ./.env; set +a; cargo run --package zfirot

# Build the whole workspace.
build:
	cargo build

# Build a distributable macOS .app bundle (the only way to get the ZF dock icon
# on macOS — a bare `cargo run` always uses the OS default dock icon). Compiles
# the stylesheet first so the bundled UI is styled. The .app lands at $(APP).
# Pass PACKAGE_TYPES=dmg to build a .dmg instead (or use the `dmg` target).
PACKAGE_TYPES ?= macos
bundle: css
	dx bundle --package zfirot --platform desktop --package-types $(PACKAGE_TYPES)

# Build the macOS .dmg installer and print its path so you can open/distribute it.
dmg:
	$(MAKE) bundle PACKAGE_TYPES=dmg
	@echo "DMG ready at:" && ls -1 $(BUNDLE_DIR)/dmg/*.dmg

# Build the .app and install it into /Applications, clearing the Gatekeeper
# quarantine flag so the unsigned app opens without a right-click → Open dance.
install: bundle
	rm -rf /Applications/Zfirot.app
	cp -R $(APP) /Applications/Zfirot.app
	xattr -dr com.apple.quarantine /Applications/Zfirot.app
	@echo "Installed /Applications/Zfirot.app — launch it from Spotlight or the dock."

fmt:
	cargo fmt --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test

# Full quality gate: format check, lints, and tests.
check:
	cargo fmt --all --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test
