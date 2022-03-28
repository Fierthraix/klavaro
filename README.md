# klavaro
Save the current keyboard layout (`xkb_active_layout`) to a file on [Sway](https://swaywm.org/). Useful with `i3status`.

```bash
$ klavaro --help
Print the current xkb_layout in sway.
The default output file is `/tmp/.xkb_lingvo'
USAGE:
    klavaro [OUTPUT_FILE]
```

## Instalation
### Local
Install to `$HOME/.cargo/bin/klavaro`
```
cargo install klavaro
```
### Global
Install to `/usr/local/bin/klavaro`
```bash
sudo -E cargo install --root /usr/local klavaro
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
