# Principles — DATA.ESP32

What this description keeps, and where. Ecosystem rules are cited, not copied: [Data handoff](../../../../../../Documentation/%F0%9F%8F%9BPhilosophy/DATA%20HANDOFF%20PHILOSOPHY.md), [Bare Metal](../../../../../../Documentation/%F0%9F%8F%9BPhilosophy/Bare%20Metal/goldenrule.md).

← [Wiki home](../README.md)

| Principle | Kept by | Where it breaks |
|---|---|---|
| **Data, not a container.** Nothing runs; no `Docker/`. | the tree: `SRC/` holds JSON and its reader crate | The reader `apkaudio-plugin-modules-esp32` (`SRC/src/lib.rs`) is a `DISCOVERY.Engine/SRC` workspace member, so it is compiled into three images |
| **The tree is the taxonomy.** Domain folder → module folder → one JSON named after its folder; 13 domains, 110 modules (some nested, e.g. `GPIO/LED/NEOPIXELS/`). | `SRC/<DOMAIN>/<DOMAIN>.json`, `SRC/<DOMAIN>/<Module>/<Module>.json` | — |
| **Every module names its topic.** Its `Identity` block carries `interface` and `topic`, always under `APK.audio/System/BareMetal/ESP32/{node}/<domain>/<module>`, so a panel binds without a lookup table. | `blocks.Identity.fields.topic` in each module file (checked: every second-level module carries it) | — |
| **AES70 vocabulary.** Each field names its AES70 class (`"AES70": "OcaStringSensor"`, …), every file is one `OcaBin` of `OcaBlock`s. | every manifest | — |
| **Described is not implemented.** These frames say what a node *can* carry; the firmware is in `NODE.BareMetal`. | every manifest's `status` (`planned` until something publishes it; `live` must name `implemented_by`) | The firmware has no MQTT yet, so every module is `planned` |
| **One reader for every DATA tree.** The OcaBin rules live once, in `SRC/ocabin` (`apkaudio-ocabin`); DATA.NETWORK/NTP/PI depend on it rather than copy it. | `SRC/ocabin`; each sibling's `Cargo.toml` | — |
| **The layout is the count.** No hand-kept manifest count: the reader refuses a folder without exactly one JSON named after it, a second root manifest, or a stray file. | `Tree::check_layout` | — |
| **Engine member, by design for now.** The crate stays a DISCOVERY.Engine workspace member (the engine's `[patch]` + `SRC/members/` design, `.apk.scripts/plugin_standalone.py`); its standalone form is staged as a patch applied with the engine's half. | `SRC/Cargo.toml` `workspace =` | until the coordinator applies `fix/P06-data-standalone.patch` + `W02-members.sh --data` |
