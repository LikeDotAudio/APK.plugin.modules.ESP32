//! The reader for this repository's OcaBin manifests (Bare Metal rule 1:
//! write the reader first). Every `*.json` under `SRC/` is compiled in by
//! `build.rs`; the rules every manifest keeps are the shared
//! `apkaudio-ocabin` crate's (`ocabin/`, in this repository), and this crate
//! names only its tree.

include!(concat!(env!("OUT_DIR"), "/manifests.rs"));

pub use apkaudio_ocabin::{Module, Phase, Tree, SCHEMA_VERSION};

/// The contract's BareMetal target this tree publishes under.
pub const TARGET: &str = "ESP32";
/// The placeholder a manifest writes where the node's name goes.
pub const NODE_PLACEHOLDER: &str = "{node}";

/// The ESP32 tree, compiled in.
pub const TREE: Tree = Tree { target: TARGET, placeholder: NODE_PLACEHOLDER, manifests: MANIFESTS, folders: FOLDERS, strays: STRAYS };

/// `APK.audio/System/BareMetal/ESP32/{node}`, built from the contract.
pub fn topic_root() -> String {
    TREE.topic_root().expect("ESP32 is a declared bareMetal target")
}

/// Read every compiled-in manifest; fails on the first broken one.
pub fn read() -> Result<Vec<Module>, String> {
    TREE.read()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The modules firmware 0.3.0 publishes (its `BUS_MODULE_*`); the firmware's
    /// `tools/check_bus_topics.py` holds the other side.
    const FIRMWARE_LIVE: &[&str] = &[
        "IDENTITY/Heartbeat/Heartbeat.json",
        "GPIO/Sensors/Proximity/Proximity.json",
        "GPIO/Sensors/Accelerometer/Accelerometer.json",
        "GPIO/Sensors/Hall/Hall.json",
        "GPIO/Sensors/Voltage Current/Voltage Current.json",
        "GPIO/MOTORS/BLDC/BLDC.json",
        "TIME/Uptime/Uptime.json",
        "NETWORK/WiFi Station/WiFi Station.json",
        "DIAGNOSTICS/Heap and Task Monitor/Heap and Task Monitor.json",
    ];

    #[test]
    fn every_manifest_reads() {
        let all = read().unwrap();
        assert!(!all.is_empty());
        assert_eq!(topic_root(), "APK.audio/System/BareMetal/ESP32/{node}");
    }

    /// What a node publishing a module's telemetry binds to. Firmware 0.3.0
    /// (LikeDotAudio/APK.firmware.ESP32_Beskar2d2, `bus_topics.h`) publishes
    /// nine modules; `mqtt_agent.py --demo`'s `BareMetal/ESP32_Beskar2d2` is a
    /// separate single-device target, deliberately not this node tree.
    #[test]
    fn a_node_announcement_binds_to_its_manifest() {
        let all = read().unwrap();
        let heartbeat = all.iter().find(|m| m.path == "IDENTITY/Heartbeat/Heartbeat.json").unwrap();
        let sample = format!("{}/uptime", heartbeat.topic.replace(NODE_PLACEHOLDER, "beskar2d2"));
        assert_eq!(TREE.bind(&all, &sample).unwrap().path, heartbeat.path);
        assert!(TREE.bind(&all, "APK.audio/System/BareMetal/ESP32_Beskar2d2/telemetry").is_none());
        let live: Vec<&str> = all.iter().filter(|m| m.phase == Phase::Live).map(|m| m.path).collect();
        assert_eq!(live.len(), FIRMWARE_LIVE.len(), "live modules: {live:?}");
        for path in FIRMWARE_LIVE {
            let m = all.iter().find(|m| m.path == *path).unwrap();
            assert_eq!(m.phase, Phase::Live, "{path}");
            assert!(m.implemented_by.as_deref().unwrap().starts_with("LikeDotAudio/APK.firmware.ESP32_Beskar2d2"), "{path}");
        }
    }
}
