# Architecture — DATA.ESP32

← [Wiki home](../README.md)

The ESP32 node description: AES70-shaped frames, one per subsystem and module. No running code.

## Layout

```
SRC/ESP32.json                         the node (one OcaBin)
SRC/<DOMAIN>/<DOMAIN>.json             13 domains: AUDIO BUSES DIAGNOSTICS DISPLAY GPIO IDENTITY
                                       NETWORK OTA POWER PROTOCOLS SECURITY STORAGE TIME
SRC/<DOMAIN>/<Module>/<Module>.json    110 modules (a few one level deeper, e.g. GPIO/LED/NEOPIXELS)
SRC/Cargo.toml, build.rs, src/         apkaudio-plugin-modules-esp32 — names the tree, calls the shared reader
SRC/ocabin/                            apkaudio-ocabin — THE OcaBin reader for DATA.ESP32/NETWORK/NTP/PI
```

## The shared reader (`SRC/ocabin`, crate `apkaudio-ocabin`)

The four DATA trees used to carry a byte-identical `build.rs` and a `lib.rs`
two constants apart. Now there is one reader, hosted here:

- `apkaudio_ocabin::build::compile_manifests()` — the whole of each crate's
  `build.rs`: compiles every `*.json` in (`MANIFESTS`), lists every folder
  (`FOLDERS`) and every non-JSON file inside a folder (`STRAYS`). Skips dot
  entries, `target`, `src` and any folder with its own `Cargo.toml` (so
  `ocabin/` is not read as part of the ESP32 tree). Std only — a build script
  names the crate with `default-features = false`.
- `apkaudio_ocabin::Tree { target, placeholder, manifests, folders, strays }`
  — `read()` (layout + every manifest + unique topics), `read_one()`,
  `check_layout()`, `topic_root()` (built from `apkaudio_contracts::topics::
  baremetal(target, [])`), `in_contract()` (the contract's `parse`), and
  `bind(modules, topic)` — the manifest a concrete bus topic belongs to.
- Manifest shape v1 (`SCHEMA_VERSION`): `schema_version: 1`, `phase:
  "planned" | "live"`, `implemented_by` when live.

**Why here, and how the siblings take it.** DATA.ESP32 is already copied into
every image that builds the DISCOVERY.Engine workspace, so hosting the reader
here adds no `COPY` line anywhere. No new repository (the owner's rule).

| Crate | Takes `apkaudio-ocabin` by |
|---|---|
| DATA.ESP32 | `path = "ocabin"` — inside its own repository |
| DATA.NETWORK | today `path = "../../DATA.ESP32/SRC/ocabin"` (an engine member, like ESP32); standalone form — `git = APK.plugin.modules.ESP32, rev = …` + the engine `[patch]` — in `fix/P06-data-standalone.patch`, applied with the engine's half |
| DATA.NTP, DATA.PI | `git = "https://github.com/LikeDotAudio/APK.plugin.modules.ESP32", rev = …` (own workspaces); a locally generated, git-excluded `SRC/.cargo/config.toml` patches it (and the SDK) to the local checkout, per `.apk.scripts/plugin_standalone.py` |

A rule change: edit `SRC/ocabin`, commit, then bump the `rev` in DATA.NTP and
DATA.PI (NETWORK follows the engine `[patch]`). The contracts come from the
SDK by `tag = "sdk-v0.5"`; the NETWORK and NTP targets are in contracts 0.2.0,
so a standalone build of those two trees' tests needs the next SDK tag.

## A manifest

`{ "<Module_Name>": { "type": "OcaBin", "schema_version": 1, "phase": "planned", "description": {"En": …}, "blocks": { … } } }` with blocks:

- **Identity** — `identity`, `purpose` (`_Label`, `OcaStringSensor`), `interface` (the physical part, e.g. `I²S RX, ES8388 / PCM1808`) and `topic` (`APK.audio/System/BareMetal/ESP32/{node}/<domain>/<module>`).
- **Control** / **Telemetry** — widgets (`_SmartToggle`, `_ValueBox`, …), each with the topic it reads or writes and its AES70 class.

## Topics, and who publishes them

Every module topic sits under `APK.audio/System/BareMetal/ESP32/{node}` (the
contract's `ESP32` bareMetal target). **Nothing publishes it yet**: the
firmware in `NODE.BareMetal/FIRMWARE/ESP32_Beskar2d2` has no MQTT client, and
the only ESP32-looking topic on the bus is `DATABUS.agent.MQTT/mqtt_agent.py
--demo`'s synthetic `BareMetal/ESP32_Beskar2d2` — a separate single-device
target in the contract (no node segment), deliberately not this tree. So every
manifest says `"phase": "planned"`; the test
`a_node_announcement_binds_to_its_manifest` pins how a node's topic binds
(`TREE.bind`) and fails the day a module is marked live without naming its
`implemented_by`. When the firmware gains MQTT it publishes under
`ESP32/{node}/<domain>/<module>`.

## Use

`{node}` is filled with the node's id by whatever binds it. Check:

```bash
cd ../../DISCOVERY.Engine/SRC && cargo test -p apkaudio-plugin-modules-esp32 -p apkaudio-ocabin
```
