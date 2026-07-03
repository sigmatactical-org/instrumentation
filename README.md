# Sigma Instrumentation (`sigma-dash`)

Digital instrument cluster UI for the **Sigma** motorcycle — a Rust + [Slint](https://slint.dev)
application that renders the rider-facing dashboard: RPM tach, digital speed, and a
button-driven **windowing system** for vehicle, navigation, diagnostics and connectivity
screens.

It runs on the desktop for development and as the sole full-screen UI on the target
**NXP i.MX 8M Plus** cockpit (Wayland + Weston), built into the
[`co-pilot`](../co-pilot) Yocto image.

## Target

- **Display:** 800×480, matte-black theme, no touch (glove-friendly buttons / rotary encoder).
- **Vehicle:** Yamaha CP3 triple (XSR900 GP calibration) — 12 000 r/min sweep, 11 250 redline.
- **Renderer:** Slint FemtoVG + winit backend.

## Quick start

```bash
cargo run --bin sigma-dash
```

This opens the dashboard in a window and runs a self-contained ride **simulation**
(launch → accelerate through the gears → top-speed hold → downshift back to a stop → repeat).

### Controls

| Key | Action |
|---|---|
| `←` / `→` | Cycle windows (within the set allowed for the current motion state) |
| `1`–`9` | Jump directly to a window |
| `↑` / `Esc` | Return to the default ride screen |
| `+` | Restart the acceleration run |
| `-` | Force the deceleration phase |

## Windowing system

The left of the ride screen is a window manager, split into two tiers:

**Left-panel windows** — glanceable while riding:

1. **Systems** *(default)* — fuel, coolant + oil temp, odometer, trip A/B, 24 h clock, lean angle, G-force
2. **Navigation** — turn-by-turn (compact)
3. **Compass / GPS** — heading, coordinates, elevation
4. **Diagnostics** — live CAN readings, bus load, DTCs

**Full-screen windows** — only open when the bike is **stopped** (they auto-close if it starts moving):

5. **Connectivity** — Bluetooth, notifications, companion app, Wi-Fi
6. **Camera** — front / rear feeds
7. **Maintenance** — oil life, brake pads, service interval, tire pressures
8. **Fuel** — detailed consumption stats and range
9. **Security** — alarm, immobilizer, GPS tracking, tilt

## Display modes

Panel and dial chrome use three **day / dusk / night** presets (`src/theme.rs`), applied at
startup to the `SigmaTheme` and `SigmaTone` Slint globals.

| Mode | When | `SIGMA_DISPLAY_MODE` |
|------|------|----------------------|
| **night** | Dark riding (default) | `night` |
| **dusk** | Twilight / mixed light | `dusk` |
| **day** | Sunlight / high contrast | `day` |

```bash
# desktop
SIGMA_DISPLAY_MODE=dusk cargo run --bin sigma-dash

# aliases (legacy)
SIGMA_UI_TONE=stealth   # → night
SIGMA_UI_TONE=normal    # → dusk
SIGMA_UI_TONE=bright    # → day
```

On the target, set `/etc/co-pilot/ui.env` (installed by the `instrumentation` recipe) or call
`theme::apply_mode()` from Rust when an ambient-light sensor or user setting is available.

## Project layout

```
src/
  main.rs        Entry point: ride simulation, window-nav state, per-frame UI updates
  gauge.rs       RPM-dial geometry — arcs, ticks, numerals and the spear-point needle
  theme.rs       Day / dusk / night display presets → SigmaTheme + SigmaTone globals
ui/
  app.slint            Top-level window + keyboard bindings + property/callback surface
  road_dashboard.slint   Ride screen layout: dial + speed + panel host + full-screen host
  theme.slint            Runtime-configurable colour + opacity globals
  widgets/
    rpm_dial.slint       Dial renderer (computed paths; bright-red at redline)
    speed_readout.slint  Fixed-width digit speed (no jitter)
  windows/             The nine window components + shared chrome (common.slint)
build.rs         Compiles the Slint UI and declares the `co_pilot_embedded` cfg
```

## Data / the CAN seam

The dashboard is driven by properties on the top-level `SigmaDashboard` component
(`ui/app.slint`), pushed from `src/main.rs`. Today those values come from the built-in
ride **simulation**; on the bike they will come from the M7 safety core over **CAN-FD**.
`SimulationState::step()` in `main.rs` is that seam — replace it with the live feed
(e.g. via `slint::invoke_from_event_loop`) and the UI is unchanged. Windows with no live
source yet (nav route, tire pressures, security state, phone/BT details) use realistic
placeholder defaults in their Slint components.

## Embedded build (Yocto / Co-Pilot)

This crate is the sole UI on the **Co-Pilot** Yocto image. Full distribution docs live in
[`co-pilot/README.md`](../co-pilot/README.md). Summary:

### Prerequisites

Sibling checkouts under `embedded/` (Scarthgap / Yocto 5.0):

| Directory | Source |
|-----------|--------|
| `poky` | `git://git.yoctoproject.org/poky` (branch `scarthgap`) |
| `meta-openembedded` | [openembedded/meta-openembedded](https://github.com/openembedded/meta-openembedded) |
| `meta-freescale`, `meta-freescale-3rdparty` | Freescale meta layers |
| `meta-rust`, `meta-clang`, `meta-rauc` | Rust toolchain, clang, OTA |
| `meta-imx` | NXP i.MX BSP (download from NXP, Scarthgap-matched release) |
| `instrumentation` | this repo |
| `co-pilot` | [`co-pilot`](../co-pilot) Yocto layer + build config |

Ensure `co-pilot/instrumentation` points at this tree (symlink or clone). Override in
`co-pilot/build/conf/local.conf` if needed:

```bitbake
SIGMA_INSTRUMENTATION_SRC = "/path/to/instrumentation"
```

### Build the image

```bash
cd ~/Source/sigma/embedded/co-pilot
chmod +x setup-environment.sh
source setup-environment.sh co-pilot-imx8mp

# Edit build/conf/local.conf — accept NXP EULA if required:
#   ACCEPT_FSL_EULA = "1"

bitbake co-pilot-image
```

Artifact:

```text
build/tmp/deploy/images/co-pilot-imx8mp/co-pilot-image-co-pilot-imx8mp.wic.gz
```

Flash:

```bash
zcat build/tmp/deploy/images/co-pilot-imx8mp/co-pilot-image-co-pilot-imx8mp.wic.gz \
  | sudo dd of=/dev/sdX bs=4M status=progress conv=fsync
```

### Rebuild UI only

After changing this repo, rebuild just the cluster app (much faster than a full image):

```bash
bitbake instrumentation -c cleansstate   # only if dependencies changed
bitbake instrumentation
```

Deploy the new binary to a running target, or bake into a fresh image with `bitbake co-pilot-image`.

### Target runtime

| Item | Value |
|------|-------|
| Binary | `/usr/bin/sigma-dash` |
| systemd unit | `cluster-ui.service` (enabled on boot) |
| Environment | `/etc/co-pilot/ui.env` |
| Compositor | Weston (headless kiosk, no shell UI) |

Default `/etc/co-pilot/ui.env`:

```bash
WAYLAND_DISPLAY=wayland-0
XDG_RUNTIME_DIR=/run/user/cluster
SLINT_BACKEND=femtovg
SLINT_FULLSCREEN=1
SIGMA_DISPLAY_MODE=night    # night · dusk · day
```

The Yocto recipe passes `--cfg co_pilot_embedded`, which enables fullscreen even without
`SLINT_FULLSCREEN=1`.

### Desktop parity

```bash
# windowed dev build
cargo run --bin sigma-dash

# match target kiosk behaviour
SLINT_FULLSCREEN=1 SIGMA_DISPLAY_MODE=night cargo run --bin sigma-dash
```

## Requirements

- Rust (edition 2024 — toolchain 1.85+)
- Slint `1.16.1` (pulled by Cargo)
- A GPU/GL-capable display for the FemtoVG renderer

## License

Licensed **MIT OR Apache-2.0** (see `LICENSE-MIT` and `LICENSE-APACHE` in this directory).
