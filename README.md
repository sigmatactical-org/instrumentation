# Sigma Racer Instrumentation

Rust workspace for the **Sigma Racer** motorcycle instrument cluster — reusable UI
library, product app, and desktop testbed.

| Crate | Binary | Role |
|-------|--------|------|
| [`sigmaracer-instrumentation/`](sigmaracer-instrumentation/) | *(library)* | Slint dashboard, gauge geometry, themes, display helpers |
| [`sigma-racer/`](sigma-racer/) | `sigma-dash` | **Sigma Racer** product app — ships on Wingman (CAN-FD seam) |
| [`testbed/`](testbed/) | `testbed` | Interactive demo — ride simulation, window nav, component testing |

## Quick start

```bash
# Interactive ride simulation (desktop window)
cargo run -p testbed

# 800×480 panel — matches sigmaracer-wingman-imx8mp / sigmaracer-wingman-qemu
cargo virt

# Production binary (idle telemetry — same as embedded target)
cargo run -p sigma-racer --bin sigma-dash
```

### Testbed controls

| Key | Action |
|-----|--------|
| `←` / `→` | Cycle windows |
| `1`–`9` | Jump to window |
| `↑` / `Esc` | Return to ride screen |
| `+` | Restart acceleration run |
| `-` | Force deceleration |

## Workspace layout

```
sigmaracer-instrumentation/   # lib — ui/, gauge, theme, display helpers
sigma-racer/                  # product — sigma-dash binary, vehicle profile
testbed/                      # dev — XSR900 ride simulation
```

## Display modes

Set `SIGMA_DISPLAY_MODE` to `night` (default), `dusk`, or `day`. See
`sigmaracer-instrumentation/src/theme.rs`.

## Embedded build (Wingman)

The Yocto recipe builds **`sigma-dash`** from the `sigma-racer` crate:

```bash
bitbake sigmaracer-instrumentation
```

| Item | Value |
|------|-------|
| Binary | `/usr/bin/sigma-dash` |
| systemd | `cluster-ui.service` |
| Environment | `/etc/sigmaracer-wingman/ui.env` |

Full distribution docs: [`sigmaracer-wingman`](../sigmaracer-wingman/README.md).

## Requirements

- Rust 1.86+ (Yocto meta-rust scarthgap) / 1.85+ for local dev
- Slint 1.13.1 (pinned for Yocto Rust 1.86)

## License

MIT OR Apache-2.0 — see `LICENSE-MIT` and `LICENSE-APACHE`.
