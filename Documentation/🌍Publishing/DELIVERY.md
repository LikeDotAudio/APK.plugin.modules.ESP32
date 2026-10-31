# Delivery — DATA.ESP32

← [Wiki home](../README.md)

No image or service of its own. Because `SRC/Cargo.toml` is a member of the `DISCOVERY.Engine/SRC` workspace (`DISCOVERY.Engine/SRC/Cargo.toml` line 155), every image that builds that workspace copies `SRC/` in:

| Image | COPY |
|---|---|
| Discovery Engine | `DISCOVERY.Engine/Docker/Dockerfile` line 63 |
| `node-baremetal:local` | `NODE.BareMetal/Docker/Dockerfile` line 75 |
| Broker | `DATABUS.Broker.MQTT/Docker/Dockerfile` line 63 |

The crate compiles the JSON in and reads it (`build.rs`, `src/lib.rs`); no service in those images consumes it. The reader's check: `cd ../../DISCOVERY.Engine/SRC && cargo test -p apkaudio-plugin-modules-esp32` (every manifest + layout; `-p apkaudio-ocabin` for the shared reader's rules). `python3 .apk.scripts/smoke.py PODS/POD.THICK/DATA/DATA.ESP32` (root) checks the repo is pushed — [SMOKE.md](../%F0%9F%A7%A9Manual%20and%20Artifacts/SMOKE.md).

**Not automated:** no CI runs the reader, no gate runs `cargo test` on it, no live URL. Removing the workspace membership would need the three Dockerfiles and `Cargo.toml` changed together.

## Delivered from the 2026-10-30 audit

- **One OcaBin reader for the four DATA trees** — new crate `apkaudio-ocabin` in `SRC/ocabin` (`build::compile_manifests` for build.rs; `Tree` with `read`/`read_one`/`check_layout`/`topic_root`/`in_contract`/`bind`); this crate's `build.rs` is one call and `src/lib.rs` names only its `TREE`. DATA.NETWORK, DATA.NTP and DATA.PI depend on it (NETWORK by path while it is an engine member; NTP/PI by `rev` of this repository). Hosted here because this tree is already copied into every image that builds the engine workspace, so no Dockerfile changed. Verified: `cd PODS/POD.THICK/DISCOVERY.Engine/SRC && cargo test -p apkaudio-ocabin -p apkaudio-plugin-modules-esp32` (4 + 2 pass). Part of the DISCOVERY.Engine workspace (and so of the engine, node-baremetal and NMOS facade images).
- **Topic namespace reconciled with what publishes** — measured: the firmware (`NODE.BareMetal/FIRMWARE/ESP32_Beskar2d2`) has no MQTT client and the only ESP32 topic on the bus is `mqtt_agent.py --demo`'s `BareMetal/ESP32_Beskar2d2`, a separate single-device target in the contract. So the manifests keep `ESP32/{node}` (the contract's ESP32 node tree) and every one says `"phase": "planned"`; test `a_node_announcement_binds_to_its_manifest` binds a node's heartbeat topic to `IDENTITY/Heartbeat`, refuses the demo topic, and fails if a module goes live without `implemented_by`. Documented in ARCHITECTURE.md "Topics, and who publishes them".
- **Unused dependencies dropped** — `tokio` (full) and the direct `apkaudio-contracts` path dependency are gone; the contracts are used, by SDK tag, only inside `ocabin` where every topic is checked with `topics::parse` and the root is built with `topics::baremetal`.
- **No hand-kept count** — `EXPECTED_FILES = 124` replaced by `Tree::check_layout` (root holds one manifest; every folder exactly one JSON named after it; no stray files). The README no longer pins a number. Manifests carry `schema_version: 1` (refused otherwise); crate bumped to 0.2.0.
- **Standalone crate (relative-path coupling)** — no `workspace =`, `edition = "2024"` (8095c65, applied by W02 from this batch's patch); the engine reaches it through `SRC/members/DATA.ESP32` and patches `apkaudio-ocabin` to `members/DATA.ESP32/ocabin` (engine 464b00d). Verified: `cd PODS/POD.THICK/DISCOVERY.Engine/SRC && cargo test --locked -p apkaudio-ocabin -p apkaudio-plugin-modules-esp32 -p apkaudio-plugin-modules-network` (4 + 2 + 3 pass). Kept by design: the image COPYs of DATA.ESP32/SRC (engine membership).
- **CC-02 manifest phase is not a liveness word** (from `check_state_words.py`) — the declared/served marker is `"phase": "planned" | "live"`, not `status` (that key is the bus liveness vocabulary of `contracts/tokens/state.json`); reader field `Module::phase` / enum `Phase`. Verified: `python3 .apk.scripts/check_state_words.py` → 0 findings.
