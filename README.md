# APK:data:ESP32

The ESP32 module description — what an ESP32 node can carry, by subsystem.

| | |
|---|---|
| **Recipe** | none — data, not a container: other stacks read it or copy it in |
| **Source** | `SRC/` (this repository — https://github.com/LikeDotAudio/APK.plugin.modules.ESP32.git) — `SRC/ESP32.json` and one folder per subsystem (AUDIO, BUSES, DISPLAY, GPIO, NETWORK, OTA, POWER, …), plus the `apkaudio-plugin-modules-esp32` crate |
| **Used by** | a member of the plugin workspace, so the plugin image compiles it; the firmware itself is `APK:BareMetal/FIRMWARE` |

Was `APK:BareMetal/PLUGINS/PLUGIN:modules:ESP32`.

---

*© 2026 — Written & Maintained by Anthony P. Kuzub. MIT Licence.*
