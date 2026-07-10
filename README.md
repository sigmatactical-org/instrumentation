# Sigma Racer Instrumentation

Rust workspace for the **Sigma Racer** motorcycle instrument cluster — reusable UI
library and desktop testbed.

| Crate | Binary | Role |
|-------|--------|------|
| [`sigma-instrumentation/`](sigma-instrumentation/) | *(library)* | Slint dashboard, gauge geometry, themes, `ClusterTelemetry` binding |
| [`testbed/`](testbed/) | `testbed` | Cluster UI + harness — candump replay, rate, day/dusk/night |

Production cluster binary **`sigma-racer-cluster`** lives in the sibling [`sigma-racer-cluster`](../sigma-racer-cluster/) repo.

## Architecture

```
CAN / IPC / candump  →  decode (sigma-racer-telemetry)  →  ClusterTelemetry  →  apply_telemetry()  →  Slint
```

The UI crate never sees raw CAN. Producers map into [`ClusterTelemetry`](sigma-instrumentation/src/telemetry/message.rs) and call [`apply_telemetry`](sigma-instrumentation/src/telemetry/apply.rs) (or the [`TelemetryPresenter`](sigma-instrumentation/src/telemetry/presenter.rs) trait).

## Quick start

```bash
cargo run -p testbed
```

(`cargo virt` is the same alias.)

### Testbed harness

| Control | Action |
|---------|--------|
| **Browse…** | Pick a `candump -L` log |
| **rate** slider | Replay speed 0.25×–4× |
| **display mode** | Cycle day → dusk → night |
| `←` / `→` | Cycle windows |
| `1`–`9` | Jump to window |
| `+` | Restart current log |
| `-` | Halve replay rate |

Default feed is the baked sample from `sigma-racer-cluster/testdata/sample-ride.log`.

## Display modes

Set `SIGMA_DISPLAY_MODE` to `day` (default), `dusk`, or `night`. The testbed button cycles day → dusk → night.

## Typography

Cluster UI embeds **Operation Napalm** (army stencil) from the Sigma theme
(`it/theme/assets/static/fonts/operation-napalm-regular.woff2`, converted to TTF for Slint).
Window `default-font-family` is `"Operation Napalm"` (Regular only).

## Embedded build (Wingman)

The Yocto recipe builds **`sigma-racer-cluster`** from the [`sigma-racer-cluster`](../sigma-racer-cluster/) crate:

```bash
bitbake sigma-racer-cluster
```

Full distribution docs: [`sigma-racer-wingman`](../sigma-racer-wingman/README.md).

## Requirements

- Rust 1.86+ (Yocto meta-rust scarthgap) / 1.85+ for local dev
- Slint 1.13.1 (pinned for Yocto Rust 1.86)

## License

MIT OR Apache-2.0 — see `LICENSE-MIT` and `LICENSE-APACHE`.
