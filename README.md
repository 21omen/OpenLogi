> [!WARNING]
> **OpenLogi is under active development** and not yet stable — features and config may still change. Give the repo a **Star** ⭐ and **Watch** 👀 it to get notified when a new release lands.

<p align="center">
    <img src="https://assets.openlogi.org/brand/openlogi-icon.png" width="138" alt="OpenLogi"/>
</p>

<h1 align="center">OpenLogi</h1>
<p align="center"><strong>⚡️ A native, local-first alternative to Logitech Options+, written in Rust 🦀<br/>Unlock the full capabilities of Logitech mice, keyboards, and webcams over HID++ and UVC</strong></p>

<div align="center">
    <a href="https://github.com/AprilNEA/OpenLogi/releases/latest" target="_blank">
    <img alt="GitHub release" src="https://img.shields.io/github/v/release/AprilNEA/OpenLogi?style=flat"></a>
    <a href="https://github.com/AprilNEA/OpenLogi/releases" target="_blank">
    <img alt="GitHub downloads" src="https://img.shields.io/github/downloads/AprilNEA/OpenLogi/total.svg?style=flat"></a>
    <a href="https://github.com/AprilNEA/OpenLogi/commits" target="_blank">
    <img alt="GitHub commit" src="https://img.shields.io/github/commit-activity/m/AprilNEA/OpenLogi?style=flat"></a>
    <a href="#license">
    <img alt="License" src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue?style=flat"></a>
    <img alt="Hits" src="https://hits.aprilnea.com/hits?url=https://github.com/aprilnea/openlogi">
</div>

<div align="center">
    <a href="https://x.com/AprilNEA" target="_blank">
    <img alt="X" src="https://img.shields.io/badge/follow-AprilNEA-black?style=flat&logo=x"></a>
    <a href="https://t.me/+VDtkR5OSAT04NzVh" target="_blank">
    <img alt="telegram" src="https://img.shields.io/badge/chat-telegram-blueviolet?style=flat&logo=Telegram"></a>
</div>

<div align="center">
    <a href="README.md">
    <img alt="English" src="https://img.shields.io/badge/English-blue?style=flat"></a>
    <a href="docs/README.zh-CN.md">
    <img alt="简体中文" src="https://img.shields.io/badge/%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-lightgrey?style=flat"></a>
    <a href="docs/README.ja.md">
    <img alt="日本語" src="https://img.shields.io/badge/%E6%97%A5%E6%9C%AC%E8%AA%9E-lightgrey?style=flat"></a>
    <a href="docs/README.de.md">
    <img alt="Deutsch" src="https://img.shields.io/badge/Deutsch-lightgrey?style=flat"></a>
    <a href="docs/README.fr.md">
    <img alt="Français" src="https://img.shields.io/badge/Fran%C3%A7ais-lightgrey?style=flat"></a>
    <a href="docs/README.ko.md">
    <img alt="한국어" src="docs/assets/badge-lang-ko.svg"></a>
    <a href="docs/README.ru.md">
    <img alt="Русский" src="https://img.shields.io/badge/%D0%A0%D1%83%D1%81%D1%81%D0%BA%D0%B8%D0%B9-lightgrey?style=flat"></a>
    <a href="docs/README.es.md">
    <img alt="Español" src="https://img.shields.io/badge/Espa%C3%B1ol-lightgrey?style=flat"></a>
    <a href="docs/README.pt-BR.md">
    <img alt="Português" src="https://img.shields.io/badge/Portugu%C3%AAs-lightgrey?style=flat"></a>
    <a href="docs/README.tr.md">
    <img alt="Türkçe" src="https://img.shields.io/badge/T%C3%BCrk%C3%A7e-lightgrey?style=flat"></a>
</div>

<p align="center">
    <a href="https://trendshift.io/repositories/42303" target="_blank">
    <img src="https://trendshift.io/api/badge/repositories/42303" alt="AprilNEA%2FOpenLogi | Trendshift" width="250" height="55"/></a>
    <a href="https://www.producthunt.com/products/openlogi?embed=true&amp;utm_source=badge-featured&amp;utm_medium=badge&amp;utm_campaign=badge-openlogi" target="_blank" rel="noopener noreferrer">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://api.producthunt.com/widgets/embed-image/v1/top-post-badge.svg?post_id=openlogi&amp;theme=dark&amp;period=daily">
        <source media="(prefers-color-scheme: light)" srcset="https://api.producthunt.com/widgets/embed-image/v1/top-post-badge.svg?post_id=openlogi&amp;theme=light&amp;period=daily">
        <img alt="OpenLogi - A local-first alternative to Logitech Options+ | Product Hunt" width="250" height="54" src="https://api.producthunt.com/widgets/embed-image/v1/top-post-badge.svg?post_id=openlogi&amp;theme=light&amp;period=daily">
    </picture></a>
</p>

> **Fed up with Options+? Try OpenLogi.**

Runs on macOS, Linux, and Windows.

---

## Beyond Options+

Things OpenLogi does that Options+ won't:

- **Stay light.** Native Rust + GPUI.
- **Run on Linux.** Linux is a first-class platform in OpenLogi.
- **Gestures on supported buttons.** Assign gesture actions to supported controls — or turn gestures off entirely.
- **Plain-text config.** Everything is one TOML file you can sync between machines however you like.
- **Script it.** A real CLI alongside the GUI: a JSON [automation API](docs/USAGE.md#automation-api) for DPI, SmartShift, Fn-lock, and report rate, plus hardware diagnostics.

## Features

<table>
    <tr>
        <td align="center" width="50%"><img src="https://assets.openlogi.org/screenshots/macos/light/devices-grid.png" width="420" alt="Device list with four Logitech mice and their connections"></td>
        <td align="center" width="50%"><img src="https://assets.openlogi.org/screenshots/macos/light/buttons-gestures.png" width="420" alt="Button mapping for an MX Master 4, with the gesture button inspector open"></td>
    </tr>
    <tr>
        <td align="center"><b>Devices</b><br><sub>Every receiver and connection, with battery level</sub></td>
        <td align="center"><b>Buttons and gestures</b><br><sub>Actions, shortcuts, and per-direction gestures</sub></td>
    </tr>
    <tr>
        <td align="center"><img src="https://assets.openlogi.org/screenshots/macos/light/actions-ring.png" width="420" alt="Actions Ring editor with eight slots around the cursor"></td>
        <td align="center"><img src="https://assets.openlogi.org/screenshots/macos/light/pointer-scrolling.png" width="420" alt="Pointer tab with DPI, SmartShift, and scrolling settings"></td>
    </tr>
    <tr>
        <td align="center"><b>Actions Ring</b><br><sub>Eight actions around the cursor, per application</sub></td>
        <td align="center"><b>Pointer and scrolling</b><br><sub>DPI, SmartShift, and wheel behavior</sub></td>
    </tr>
</table>

### Devices

- Logi Bolt and Unifying receivers, Bluetooth, and wired connections, with battery level and charge state
- Low-battery notices at the bottom of the screen, once per discharge, with no OS notification permission
- An on-screen indicator when a keyboard turns Caps Lock on or off, plus Num Lock and Scroll Lock on Windows and Linux
- Easy-Switch: see which computer is paired on each channel, with its OS and connection, and rename it on devices that allow it

### Buttons and actions

- Remap mouse buttons and keyboard F-keys to a built-in action catalog, custom shortcuts, typed text, or multi-step workflows
- Separate tap and hold actions, and hold-to-talk chords that release with the button
- Gestures on supported buttons: Middle Click, Back/Forward, DPI/ModeShift, the gesture button, and the haptic panel
- Actions Ring: an eight-slot overlay around the cursor, with its own layout per application
- Per-application profiles that switch with the focused app; an `[apps]` alias lets one profile follow an application across platforms

### Pointer and scrolling

- DPI presets with cycle and set-preset actions, pointer speed, and report rate
- Precision Mode: one button switches the mouse to its slowest speed and back
- SmartShift: mode, sensitivity, and scrolling force on SmartShift Enhanced mice
- Scroll inversion per device, natively or in software, plus smooth wheel scrolling
- Hold Back or Forward and roll the wheel to scroll sideways on mice without a thumb wheel

### Keyboard and lighting

- Multi-platform keyboards stay on the host OS layout; restore one key to its device default, or reset the whole row
- Keyboard backlight: level, automatic brightness, and battery saving, with an on-screen level indicator
- Static RGB on supported keyboards
- Litra lights: power, brightness, and color temperature; follows camera activity if you want, and keeps changes made on the light itself

### Webcams

- Any Logitech UVC webcam (Brio, StreamCam, the C920 series, …), plug and play
- Live preview that holds the camera only while you watch; leave it and the LED goes off
- Zoom, pan, tilt, focus, exposure, white balance, and every other UVC control, written to the hardware so Meet, Zoom, and OBS see the change
- Built-in Default / Streaming / Video call profiles plus your own snapshots, kept per camera

Feature coverage depends on what each device reports; the app shows only the controls a device supports.

### Platform support

| Feature | macOS | Windows | Linux |
| --- | --- | --- | --- |
| Mouse button remapping and gestures | ✓ | ✓ | ✓ |
| Keyboard F-key remapping | ✓ | ✓ | — |
| Per-application profiles | ✓ | ✓ | X11, wlroots (sway, Hyprland), GNOME with the [companion extension](crates/openlogi-hook/gnome-shell-extension/README.md) |
| Mouse profile follows the window under the pointer | ✓ | ✓ | X11 only |
| Media keys | Native | Native | D-Bus MPRIS |
| Window-manager actions (desktops, Mission Control, …) | Native | Native equivalents | GNOME, KDE (KWin), Hyprland; some macOS-only actions do nothing |
| Menu-bar / tray icon | ✓ | ✓ | — |
| In-app updates | Sparkle | Built-in | Built-in |

## Install

> [!IMPORTANT]
> Quit **Logi Options+** first: the two applications fight over HID++ access, and only one can own a given receiver at a time.

### macOS

Requires macOS 13 or later.

Download the signed, notarized `.dmg` from the [latest release](https://github.com/AprilNEA/OpenLogi/releases/latest) and drag `OpenLogi.app` to `/Applications`. The app updates itself through Sparkle when update checks are on in Settings → Updates.

Or install via [Homebrew](https://brew.sh):

```sh
brew install --cask openlogi
```

The official Homebrew cask is the default installation path. To explicitly
track the latest GitHub release from `aprilnea/tap` instead:

```sh
brew tap aprilnea/tap
brew install --cask aprilnea/tap/openlogi@latest
```

`openlogi@latest` is maintained by OpenLogi's release workflow and may update
before the official cask autobump lands. Install either `openlogi` or
`openlogi@latest`, not both.

### Linux

Download the installer over HTTPS, inspect it, then run it. Do not pipe it into
a shell:

```sh
curl --proto '=https' --proto-redir '=https' --tlsv1.2 \
  --fail --location --silent --show-error \
  --retry 3 --retry-connrefused \
  --output openlogi-install.sh \
  https://raw.githubusercontent.com/AprilNEA/OpenLogi/master/packaging/linux/install.sh
less openlogi-install.sh
sh openlogi-install.sh
rm openlogi-install.sh
```

The script picks the right `.deb`, `.rpm`, or `.pkg.tar.zst` for your package manager and architecture, verifies its minisign signature and checksum, installs udev rules, and starts the agent as a user service. Install `minisign` first and run the script as your normal user, not with `sudo`. Packages exist for `x86_64` and `arm64` and need GLIBC 2.35 or newer.

Each release also carries a Flatpak bundle, and the repository provides a NixOS module. See [docs/INSTALL-linux.md](docs/INSTALL-linux.md) for both, fixed-version and source installs, distros without systemd, and Hyprland/Omarchy notes.

### Windows

Signed portable `.zip` archives and per-user `.msi` installers (x86_64 and
arm64) are attached to each release. Both ship the GUI (`OpenLogi.exe`)
together with the background agent (`openlogi-agent.exe`), which owns all
device I/O. Keep the two files side by side when using the portable zip, or
the GUI has nothing to connect to.

Windows support has been validated end-to-end on Windows 11 with real
hardware (a wired keyboard and a Unifying-receiver mouse), including
install, in-place upgrade, and uninstall of the MSI. It is newer than the
macOS build, so if you hit a rough edge please
[report it](https://github.com/AprilNEA/OpenLogi/issues). The agent shows a
system-tray icon (Show Main Window / Quit) so the app stays reachable after
the main window is closed; turn it off in Settings → General.

To build from source, see [DEVELOPMENT.md](docs/DEVELOPMENT.md).


## Usage (CLI)

See [USAGE.md](docs/USAGE.md)

## Configuration

Settings use plain TOML; saves preserve symlinked config files. See
[CONFIGURATION.md](docs/CONFIGURATION.md).

## Developing

See [DEVELOPMENT.md](docs/DEVELOPMENT.md), including [macOS input-hook safety](docs/DEVELOPMENT.md#macos-input-hook-safety).

## Acknowledgments

- **Windows, cameras, and i18n** by [@davidbudnick](https://github.com/davidbudnick) — keyboard RGB, Windows support, Logitech webcam support
- **Linux port** by [@cserby](https://github.com/cserby) — Linux support
- [Solaar](https://github.com/pwr-Solaar/Solaar) by [@pwr](https://github.com/pwr) — open-source HID++ implementation
- [Mouser](https://github.com/TomBadash/Mouser) by [@TomBadash](https://github.com/TomBadash) — a local, account-free Options+ replacement

## License

The code in this repository is dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

### Third-party code

`crates/openlogi-hidpp` is a vendored fork of [`hidpp`](https://crates.io/crates/hidpp)
by [@lus](https://github.com/lus), licensed 0BSD.

### Logo & brand assets

Thanks to [@kubai087](https://github.com/kubai087) for designing the OpenLogi
logo. The OpenLogi logo and app icon (the brand assets under
[`design/`](design/)) are © 2026 AprilNEA, all rights reserved, and are not covered by the MIT/Apache
licenses above; see [`design/LICENSE`](design/LICENSE). Forking the code grants
no right to the OpenLogi name, logo, or icon; please don't use them to represent
your own projects, forks, or distributions without prior written permission.

---

**Not affiliated with Logitech.** "Logitech", "MX Master", and "Options+" are trademarks of Logitech International S.A.

## Repo activity

![Repobeats analytics image](https://repobeats.com/AprilNEA/OpenLogi "Repobeats analytics image")
