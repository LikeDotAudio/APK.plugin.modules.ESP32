# 🔍 Architecture audit — DATA.ESP32 (2026-10-30)

Audited against the APK.audio multi-repo architecture standard: coupling, boundaries, duplication, contracts.
← [Audits](INDEX.md) · Improvements: [🛣Roadmap/IMPROVEMENTS.md](../%F0%9F%9B%A3Roadmap/IMPROVEMENTS.md)

## Bounded context
The ESP32 node description: what an ESP32 node can carry, by subsystem — 124 AES70-shaped `OcaBin` manifests (13 domains, 110 modules) plus a thin reader crate (`apkaudio-plugin-modules-esp32`) over the shared OcaBin reader `apkaudio-ocabin` (`SRC/ocabin`, hosted here for all four DATA trees). Data, not a container. Sampled: `SRC/Cargo.toml`, `SRC/build.rs`, `SRC/src/lib.rs`, `SRC/src/main.rs`, `SRC/ESP32.json`, a module manifest (`SRC/AUDIO/I2S Codec In/`), and topic usage across `PODS/`.

## Context map
| Talks to | How (topic / endpoint / file / import) | Relationship |
|---|---|---|
| DISCOVERY.Engine workspace | reached through the engine's `SRC/members/DATA.ESP32` link (no workspace key in this repo) | member by the engine's own choice; this crate builds alone |
| APK.plugin.SDK contracts | `SRC/ocabin` → `apkaudio-contracts` by SDK tag (`sdk-v0.7` at HEAD) (engine `[patch]` → local) | Conformist, by tag |
| DISCOVERY.Engine / NODE.BareMetal / NMOS facade images | `COPY PODS/POD.THICK/DATA/DATA.ESP32/SRC` in their Dockerfiles | Supplier of files baked into others' images (by design: engine member) |
| DATA.NETWORK / DATA.NTP / DATA.PI | depend on `apkaudio-ocabin` (`SRC/ocabin`) | Supplier — the one reader |
| ESP32 firmware / live devices | manifest topic root `APK.audio/System/BareMetal/ESP32/{node}/…` | declared Published Language; every manifest `phase: planned` (no publisher yet) |

## Scorecard
| Category | Score (0–3) | Finding |
|---|---|---|
| Coupling & Cohesion | 3 | Standalone (plugin_standalone.py shape): no workspace key, no `../` path; contracts by SDK tag through `ocabin`; the engine reaches it through `SRC/members/` and a `[patch]`. |
| Boundary Integrity | 3 | Declarative published shape (OcaBin, `schema_version`, `phase`), every topic checked against the contract's grammar; the manifests say they are not implemented yet. |
| Duplication Control | 3 | Hosts the one reader (`apkaudio-ocabin`) all four DATA trees use. |
| Testing & Contracts | 3 | Reader rules tested once in `ocabin`; the tree's test binds a node topic to its manifest and refuses an unclaimed `live`. |
| Change management | 3 | Released as `esp32-v0.2.0` (crate 0.2.0); manifests carry `schema_version: 1`; no hand-kept counts. |
Score key: 3 meets the standard · 2 minor gaps · 1 significant gaps · 0 absent / violated.

## Findings
1. **Good — reader-first, compiled-in data.** `build.rs` (→ `apkaudio_ocabin::build::compile_manifests`) compiles every `SRC/**/*.json` with `include_str!`; `Tree::read_one` refuses a manifest that is not one `OcaBin` named after its file, lacks `schema_version`/`phase`, has no `Identity.topic` under the contract-built root or in the contract's grammar, or a widget topic outside its module; `read()` refuses shared topics and a bad layout.
