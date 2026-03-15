# cosmic-battery-applet

![Charging](Charging.png) ![Not Charging](NotCharging.png)

A lightweight battery percentage applet for the [COSMIC desktop](https://system76.com/cosmic) (Pop OS 24.04+).

Displays live battery status directly in the top panel with color-coded indicators.

```
+54%   ← charging        (cyan)
 89%   ← high battery    (green)
 55%   ← mid battery     (white)
 28%   ← low battery     (orange)
  8%   ← critical        (red)
```

---

## Requirements

| Requirement | Notes |
|---|---|
| Pop OS 24.04 (COSMIC) | Other COSMIC distros may work |
| Rust 1.75+ | [Install via rustup](https://rustup.rs) |
| libdbus | Usually pre-installed |

---

## Install

```bash
git clone https://github.com/rapsalands/cosmic-battery-applet
cd cosmic-battery-applet
chmod +x install.sh
./install.sh
```

The script will:
1. Install build dependencies via `apt`
2. Compile the applet (takes ~5 mins first time, cached after)
3. Install the binary to `~/.local/bin/`
4. Install the `.desktop` entry to `~/.local/share/applications/`

Then restart the panel:
```bash
pkill -f cosmic-panel
```

---

## Add to Panel

1. Open **Settings → Desktop → Panel**
2. Click **Add Applet**
3. Select **Battery Applet**
4. Drag it to your preferred position

---

## How it works

- Reads battery data from **UPower** over **D-Bus** — no direct hardware polling
- Refreshes every **30 seconds** (change `REFRESH_SECS` in `src/main.rs`)
- Color thresholds:

| Level | Color |
|---|---|
| Charging | 🔵 Cyan |
| 80–100% | 🟢 Green |
| 40–79% | ⚪ White |
| 15–39% | 🟠 Orange |
| 0–14% | 🔴 Red |

---

## Uninstall

```bash
rm ~/.local/bin/cosmic-battery-applet
rm ~/.local/share/applications/cosmic-battery-applet.desktop
pkill -f cosmic-battery-applet
```

Then remove it from your panel via **Settings → Desktop → Panel**.

---

## Contributing

PRs welcome! Built with [libcosmic](https://github.com/pop-os/libcosmic).

---

## License

MIT
