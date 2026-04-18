# VBoard

VBoard is a movable on-screen keyboard for Linux, written in Rust with GTK3. It sends key input through the XDG desktop portal and currently ships with `US` and `PT` layouts.

## Features

- Floating keyboard window with drag and resize support
- Runtime controls for opacity, font size, bold text, colors, and gradients
- Persistent window size and optional position memory
- Input emission through `eitype` and the desktop portal instead of direct device injection
- Built-in `US` and `PT` layouts with layout switching in the UI

## Compatibility

VBoard is a Linux desktop application. The current implementation is tested as a source build on Linux with GTK 3.24.

- X11: the default and recommended backend for the windowing behavior used here
- Wayland: input can work through the portal, but popup placement and saved window positioning may be less predictable
- Desktop portal: requires a working `xdg-desktop-portal` setup because key emission is delegated through `eitype`

If you want to override the default backend choice, set either `GDK_BACKEND` directly or `VBOARD_GDK_BACKEND` before launching the app.

## Dependencies

Runtime and build requirements:

- Rust toolchain with `cargo`
- `pkg-config`
- GTK 3 development files
- `xdg-desktop-portal`

Examples:

```bash
# Debian / Ubuntu
sudo apt install build-essential pkg-config libgtk-3-dev xdg-desktop-portal

# Arch Linux
sudo pacman -S base-devel pkgconf gtk3 xdg-desktop-portal
```

## Build

```bash
make release
```

Useful development targets:

```bash
make check
make fmt-check
make clippy
make test
```

## Install

User-local install:

```bash
make install PREFIX="$HOME/.local"
```

System-wide install:

```bash
sudo make install PREFIX=/usr
```

The install target places:

- the binary in `bin`
- a desktop entry in `share/applications`
- a scalable icon in `share/icons/hicolor/scalable/apps`

`install.sh` is kept as a small wrapper around `make install` for convenience.

## Run

```bash
cargo run
```

Or, after installation:

```bash
vboard-rs
```

## Configuration

Settings are stored at:

```text
$XDG_CONFIG_HOME/vboard/settings.conf
```

If `XDG_CONFIG_HOME` is not set, VBoard falls back to the standard config location reported by the platform.

## Project Status

This is currently a source-first project. There are no release tarballs or distro packages yet, but the repository includes CI, tests for core config/layout behavior, and a standard install target for packaging.

## License

MIT. See [LICENSE](LICENSE).
