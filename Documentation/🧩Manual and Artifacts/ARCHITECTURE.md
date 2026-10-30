# Architecture — DATA.ESP32

← [Wiki home](../README.md)

The ESP32 node description: AES70-shaped frames, one per subsystem and module. No running code.

## Layout

```
SRC/ESP32.json                         the node (one OcaBin)
SRC/<DOMAIN>/<DOMAIN>.json             13 domains: AUDIO BUSES DIAGNOSTICS DISPLAY GPIO IDENTITY
                                       NETWORK OTA POWER PROTOCOLS SECURITY STORAGE TIME
SRC/<DOMAIN>/<Module>/<Module>.json    110 modules (a few one level deeper, e.g. GPIO/LED/NEOPIXELS)
SRC/Cargo.toml, SRC/src/main.rs        stub crate apkaudio-plugin-modules-esp32
```

## A manifest

`{ "<Module_Name>": { "type": "OcaBin", "description": {"En": …}, "blocks": { … } } }` with blocks:

- **Identity** — `identity`, `purpose` (`_Label`, `OcaStringSensor`), `interface` (the physical part, e.g. `I²S RX, ES8388 / PCM1808`) and `topic` (`APK.audio/System/BareMetal/ESP32/{node}/<domain>/<module>`).
- **Control** / **Telemetry** — widgets (`_SmartToggle`, `_ValueBox`, …), each with the topic it reads or writes and its AES70 class.

## Use

Read the JSON directly; `{node}` is filled with the node's id by whatever binds it. Check every file parses:

```bash
python3 -c 'import json,glob; [json.load(open(f)) for f in glob.glob("SRC/**/*.json", recursive=True)]'   # 124 files
```
