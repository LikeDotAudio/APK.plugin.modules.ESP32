# DATA.ESP32

The ESP32 node description — what an ESP32 node can carry, by subsystem, written as AES70-shaped frames.

| | |
|---|---|
| **Recipe** | none — data, not a container: other stacks read it or copy it in |
| **Source** | `SRC/` (this repository — https://github.com/LikeDotAudio/APK.plugin.modules.ESP32.git) |
| **Manifests** | `SRC/ESP32.json` (the node) · one `<DOMAIN>/<DOMAIN>.json` per domain · one `<DOMAIN>/<Module>/<Module>.json` per module — 13 domains (AUDIO, BUSES, DIAGNOSTICS, DISPLAY, GPIO, IDENTITY, NETWORK, OTA, POWER, PROTOCOLS, SECURITY, STORAGE, TIME), 110 modules, 124 files |
| **Crate** | `apkaudio-plugin-modules-esp32` (`SRC/Cargo.toml`, `SRC/src/main.rs`) — a stub binary that prints its name and nothing else |
| **Used by** | the crate is a member of the `DISCOVERY.Engine/SRC` workspace, so every image that builds that workspace copies `SRC/` in (`DISCOVERY.Engine`, `NODE.BareMetal`, `DATABUS.Broker.MQTT` Dockerfiles). No code in the tree opens the JSON manifests by path; the firmware itself lives in `NODE.BareMetal` |

Was `APK:BareMetal/PLUGINS/PLUGIN:modules:ESP32`.

## What a manifest holds

Every file is one `OcaBin` keyed by the module name, with `OcaBlock`s inside:

- **Identity** — label, one-line purpose, the physical `interface` (e.g. `I²S RX, ES8388 / PCM1808`) and the module's bus `topic`, always under `APK.audio/System/BareMetal/ESP32/{node}/<domain>/<module>`.
- **Control** / **Telemetry** — the widgets (`_SmartToggle`, `_ValueBox`, …) with the topic each one reads or writes, and the AES70 class it maps to (`"AES70": "OcaStringSensor"`, …).

## Build and check

```bash
cd ../../DISCOVERY.Engine/SRC
cargo check -p apkaudio-plugin-modules-esp32      # passes, 2026-10-30
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
