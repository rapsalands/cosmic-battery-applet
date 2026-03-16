# cosmic-battery-applet

![Charging](Charging.png) ![Not Charging](NotCharging.png)

# cosmic-battery-applet

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

## What is this?

By default, Pop OS (COSMIC) does not show battery percentage directly in the top bar — you have to click the battery icon to see it. This applet fixes that by showing your battery percentage at all times, color-coded so you know at a glance how much charge you have left.

---

## Requirements

- Pop OS 24.04 with the COSMIC desktop
- An internet connection (for downloading dependencies)
- A terminal (press `Super + T` to open one)

That's it. Everything else will be installed automatically.

---

## Installation (Step by Step)

### Step 1 — Open a Terminal

Press `Super + T` (the Windows/Meta key + T) to open a terminal window.

---

### Step 2 — Install Rust

Rust is the programming language this applet is written in. Run this command to install it:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

When it asks, press `1` and then `Enter` to proceed with the default installation.

When it finishes, run this to activate Rust in your current terminal:

```bash
source "$HOME/.cargo/env"
```

Verify it worked:

```bash
rustc --version
```

You should see something like `rustc 1.75.0`. If you do, Rust is ready.

---

### Step 3 — Install Git

Git is used to download the applet source code. It may already be installed:

```bash
sudo apt install -y git
```

Enter your password when prompted.

---

### Step 4 — Download the Applet

```bash
cd ~/Downloads
git clone https://github.com/rapsalands/cosmic-battery-applet
cd cosmic-battery-applet
```

---

### Step 5 — Run the Installer

```bash
chmod +x install.sh
./install.sh
```

This will:
1. Install required build libraries (cmake, fontconfig, dbus, etc.)
2. Compile the applet from source — **this takes 5–10 minutes the first time**, please be patient
3. Install the applet binary to `~/.local/bin/`
4. Install the applet entry so COSMIC can find it

You will be asked for your password once for the `sudo apt install` step.

---

### Step 6 — Add to PATH

Make sure your terminal can find the applet binary:

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

---

### Step 7 — Restart the Panel

```bash
pkill -f cosmic-panel
```

The panel will restart automatically within a second.

---

### Step 8 — Add the Applet to Your Panel

1. Open **Settings** → **Desktop** → **Panel**
2. Click **Add Applet**
3. Find **Battery Applet** in the list and click it
4. Drag it to your preferred position in the panel

You should now see your battery percentage in the top bar!

---

## Color Guide

The percentage changes color based on your battery level:

| Color | Meaning |
|---|---|
| 🔵 Cyan | Charging |
| 🟢 Green | 80–100% — plenty of charge |
| ⚪ White | 40–79% — normal |
| 🟠 Orange | 15–39% — getting low |
| 🔴 Red | 0–14% — plug in soon! |

---

## Updating

To update to the latest version:

```bash
cd ~/Downloads/cosmic-battery-applet
git pull
cargo build --release
cp target/release/cosmic-battery-applet ~/.local/bin/
pkill -f cosmic-battery-applet
```

---

## Uninstall

```bash
rm ~/.local/bin/cosmic-battery-applet
rm ~/.local/share/applications/cosmic-battery-applet.desktop
pkill -f cosmic-battery-applet
```

Then go to **Settings → Desktop → Panel** and remove the Battery Applet from your panel.

---

## Troubleshooting

**The applet doesn't appear in the Add Applet list**
Make sure the `.desktop` file was installed correctly:
```bash
ls ~/.local/share/applications/cosmic-battery-applet.desktop
```
If the file is missing, re-run `./install.sh`.

**The applet appears but shows `--`**
This means UPower couldn't find your battery. Check if UPower is running:
```bash
systemctl status upower
```

**Build failed**
Make sure all dependencies are installed:
```bash
sudo apt install -y cmake just libexpat1-dev libfontconfig-dev \
    libfreetype-dev libxkbcommon-dev libdbus-1-dev pkgconf
```
Then try building again:
```bash
cargo build --release
```

---

## How it works (for the curious)

The applet reads battery data from **UPower**, a system service that is always running on Linux. It communicates via **D-Bus** (an inter-process messaging system), so it never touches hardware directly. It checks for updates every 30 seconds, which has negligible impact on battery life or CPU.

---

## Contributing

PRs and issues welcome! Built with [libcosmic](https://github.com/pop-os/libcosmic) — the same UI toolkit used by the COSMIC desktop itself.

---

## License

MIT
