# klavaro

[![CI](https://github.com/Fierthraix/klavaro/actions/workflows/ci.yml/badge.svg)](https://github.com/Fierthraix/klavaro/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Fierthraix/klavaro?display_name=tag)](https://github.com/Fierthraix/klavaro/releases)
[![Crates.io](https://img.shields.io/crates/v/klavaro.svg)](https://crates.io/crates/klavaro)
[![Downloads](https://img.shields.io/crates/d/klavaro.svg)](https://crates.io/crates/klavaro)
[![Docs.rs](https://docs.rs/klavaro/badge.svg)](https://docs.rs/klavaro)
[![License](https://img.shields.io/crates/l/klavaro.svg)](LICENSE)
[![AUR bin](https://img.shields.io/aur/version/klavaro-bin)](https://aur.archlinux.org/packages/klavaro-bin)
[![AUR git](https://img.shields.io/aur/version/klavaro-git)](https://aur.archlinux.org/packages/klavaro-git)

Save the current keyboard layout (`xkb_active_layout`) to a file on [Sway](https://swaywm.org/). Useful with `i3status`.

```bash
$ klavaro --help
Print the current xkb_layout in sway.
The default output file is `/tmp/.xkb_lingvo'
USAGE:
    klavaro [OUTPUT_FILE]
```

## Installation

### Cargo

```bash
cargo install klavaro
```

### Arch Linux / AUR

```bash
yay -S klavaro-bin
yay -S klavaro-git
```

### macOS / Homebrew

```zsh
brew install --cask Fierthraix/tap/klavaro
```

### Nix

```bash
nix profile install github:Fierthraix/nur-packages#klavaro
```

### Release Assets

```text
https://github.com/Fierthraix/klavaro/releases/latest
```

## i3status
Your current Sway keyboard layout can be printed in `i3status` thusly:

```
~/.i3status.conf
```
```
order += "read_file keyboard"

read_file keyboard {
        path = "/tmp/.xkb_lingvo"
        color_good = "#FFFFFF"
}
```

However, the `klavaro` program must already be running, which can be accomplished via `systemd` user service as below.

## SystemD User Service
Since `sway` is a _user_ process, a systemd _user_ service must be used in order to get the `SWAYSOCK` successfully.

This is the service file needed:

```
/etc/systemd/user/klavaro.service
```
```
[Unit]
Description=klavaro

[Service]
Type=simple
ExecStart=/usr/local/bin/klavaro
Restart=always
RestartSec=1s

[Install]
WantedBy=multi-user.target
```

Then the service can be started:
```bash
systemctl --user enable klavaro # Schedule klavaro on startup.
systemctl --user start klavaro  # Start klavaro immediately.
```

## swaymsg
This is basically equivalent to (but _muuch_ more efficient than)
```bash
swaymsg -r -t subscribe -m '["input"]' \
   | jq '.input.xkb_active_layout_name'
```
and saving the result to a file.
