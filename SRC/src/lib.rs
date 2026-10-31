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

    #[test]
    fn every_manifest_reads() {
        let all = read().unwrap();
        assert!(!all.is_empty());
        assert_eq!(topic_root(), "APK.audio/System/BareMetal/ESP32/{node}");
    }

    /// What a node publishing a module's telemetry binds to. The firmware in
    /// NODE.BareMetal speaks no MQTT yet, and the only ESP32 topic on the bus
    /// today is mqtt_agent.py's --demo `BareMetal/ESP32_Beskar2d2` — a separate
    /// single-device target, deliberately not this node tree.
    #[test]
    fn a_node_announcement_binds_to_its_manifest() {
        let all = read().unwrap();
        let heartbeat = all.iter().find(|m| m.path == "IDENTITY/Heartbeat/Heartbeat.json").unwrap();
        let sample = format!("{}/uptime", heartbeat.topic.replace(NODE_PLACEHOLDER, "beskar2d2"));
        assert_eq!(TREE.bind(&all, &sample).unwrap().path, heartbeat.path);
        assert!(TREE.bind(&all, "APK.audio/System/BareMetal/ESP32_Beskar2d2/telemetry").is_none());
        assert!(all.iter().all(|m| m.phase == Phase::Planned), "a module went live: name its implemented_by");
    }
}
