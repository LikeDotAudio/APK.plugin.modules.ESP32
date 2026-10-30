# Delivery — DATA.ESP32

← [Wiki home](../README.md)

No image or service of its own. Because `SRC/Cargo.toml` is a member of the `DISCOVERY.Engine/SRC` workspace (`DISCOVERY.Engine/SRC/Cargo.toml` line 155), every image that builds that workspace copies `SRC/` in:

| Image | COPY |
|---|---|
| Discovery Engine | `DISCOVERY.Engine/Docker/Dockerfile` line 63 |
| `node-baremetal:local` | `NODE.BareMetal/Docker/Dockerfile` line 75 |
| Broker | `DATABUS.Broker.MQTT/Docker/Dockerfile` line 63 |

The JSON rides along; nothing in those images reads it. The stub crate builds: `cd ../../DISCOVERY.Engine/SRC && cargo check -p apkaudio-plugin-modules-esp32`. `python3 .apk.scripts/smoke.py PODS/POD.THICK/DATA/DATA.ESP32` (root) checks the repo is pushed — [SMOKE.md](../%F0%9F%A7%A9Manual%20and%20Artifacts/SMOKE.md).

**Not automated:** no CI, no JSON parse check in any gate, no live URL. Removing the workspace membership would need the three Dockerfiles and `Cargo.toml` changed together.
