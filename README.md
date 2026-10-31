# DATA.ESP32

**Wiki:** [Documentation/README.md](Documentation/README.md) — principles, architecture, delivery.

The ESP32 node description — what an ESP32 node can carry, by subsystem, written as AES70-shaped frames.

| | |
|---|---|
| **Recipe** | none — data, not a container: other stacks read it or copy it in |
| **Source** | `SRC/` (this repository — https://github.com/LikeDotAudio/APK.plugin.modules.ESP32.git) |
| **Manifests** | `SRC/ESP32.json` (the node) · one `<DOMAIN>/<DOMAIN>.json` per domain · one `<DOMAIN>/<Module>/<Module>.json` per module — 13 domains (AUDIO, BUSES, DIAGNOSTICS, DISPLAY, GPIO, IDENTITY, NETWORK, OTA, POWER, PROTOCOLS, SECURITY, STORAGE, TIME), 110 modules. Every manifest is `"status": "planned"` — nothing publishes the ESP32 node tree yet (the firmware has no MQTT) |
| **Crate** | `apkaudio-plugin-modules-esp32` (`SRC/Cargo.toml`, `SRC/build.rs`, `SRC/src/`) names the tree (`TREE`: target `ESP32`, placeholder `{node}`) and calls **`apkaudio-ocabin`** (`SRC/ocabin/`) — the one OcaBin reader shared by DATA.ESP32/NETWORK/NTP/PI: `build.rs` compiles every `SRC/**/*.json` in; `read()` refuses a manifest that is not a single `OcaBin` named after its file, lacks `schema_version: 1` or a `status` (`planned`/`live` + `implemented_by`), has no `Identity.topic` under the node root (built from `apkaudio_contracts::topics::baremetal`) or outside the contract's grammar, reuses another's topic, or declares a widget topic outside its own; and a folder that does not hold exactly one JSON named after it. The binary prints `path  status  topic  interface` per manifest |
| **Used by** | the crate is a member of the `DISCOVERY.Engine/SRC` workspace, so every image that builds that workspace copies `SRC/` in (`DISCOVERY.Engine`, `NODE.BareMetal`, `DATABUS.Broker.MQTT` Dockerfiles). The crate is the manifests' only reader (compiled in, not opened by path); the firmware itself lives in `NODE.BareMetal` |

Was `APK:BareMetal/PLUGINS/PLUGIN:modules:ESP32`.

## What a manifest holds

Every file is one `OcaBin` keyed by the module name, with `OcaBlock`s inside:

- **`schema_version`** (1) and **`status`** (`planned` until something publishes it; `live` names its `implemented_by`).
- **Identity** — label, one-line purpose, the physical `interface` (e.g. `I²S RX, ES8388 / PCM1808`) and the module's bus `topic`, always under `APK.audio/System/BareMetal/ESP32/{node}/<domain>/<module>`.
- **Control** / **Telemetry** — the widgets (`_SmartToggle`, `_ValueBox`, …) with the topic each one reads or writes, and the AES70 class it maps to (`"AES70": "OcaStringSensor"`, …).

## Build and check

```bash
cd ../../DISCOVERY.Engine/SRC
cargo test -p apkaudio-plugin-modules-esp32 -p apkaudio-ocabin   # every manifest + the layout; the shared reader's own rules
cargo run -q -p apkaudio-plugin-modules-esp32     # the catalogue: path, topic, interface
python3 -c 'import json,glob; [json.load(open(f)) for f in glob.glob("SRC/**/*.json", recursive=True)]'   # from this folder: every manifest parses
```

## Philosophy

- **Data, not a container.** There is no `Docker/` here and nothing runs; the description is copied in by whoever needs it (`Docker` COPY lines in the `DISCOVERY.Engine` and `NODE.BareMetal` Dockerfiles).
- **One folder per thing.** The tree is the taxonomy: domain folder → module folder → one JSON named after its folder (`SRC/AUDIO/I2S Codec In/I2S Codec In.json`).
- **Every module names its topic.** A module is declared with its interface and its place on the bus, so a panel can bind to it without a lookup table (`identity.topic` in every module file).
- **AES70 vocabulary.** Fields carry their AES70 class, so the same frame can be rendered by the OS or published as an AES70 object (`"AES70": …` throughout).

Plans for this repo live in the house roadmap, `Documentation/🛣Roadmap/roadmap.htm` in APK.audio.

---

*© 2026 — Written & Maintained by Anthony P. Kuzub. MIT Licence.*

<!-- wiki:smoke -->
**Smoke test** — ✅ PASS 1/1 since 2026-10-30 12:51 EDT — [results](Documentation/%F0%9F%A7%A9Manual%20and%20Artifacts/SMOKE.md)
<!-- /wiki:smoke -->
