APP_ID := io.github.mrqs001.vboard
BIN := vboard-rs
PREFIX ?= /usr/local
DESTDIR ?=
BINDIR := $(DESTDIR)$(PREFIX)/bin
DESKTOPDIR := $(DESTDIR)$(PREFIX)/share/applications
ICONDIR := $(DESTDIR)$(PREFIX)/share/icons/hicolor/scalable/apps

.PHONY: build release run check fmt fmt-check clippy test install uninstall

build:
	cargo build

release:
	cargo build --release

run:
	cargo run

check:
	cargo check

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all --check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test --all-targets

install: release
	install -Dm755 target/release/$(BIN) "$(BINDIR)/$(BIN)"
	install -Dm644 assets/$(APP_ID).desktop "$(DESKTOPDIR)/$(APP_ID).desktop"
	install -Dm644 assets/$(APP_ID).svg "$(ICONDIR)/$(APP_ID).svg"

uninstall:
	rm -f "$(BINDIR)/$(BIN)"
	rm -f "$(DESKTOPDIR)/$(APP_ID).desktop"
	rm -f "$(ICONDIR)/$(APP_ID).svg"
