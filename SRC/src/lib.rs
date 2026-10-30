//! The reader for this repository's OcaBin manifests (Bare Metal rule 1:
//! write the reader first). Every `*.json` under `SRC/` is compiled in by
//! `build.rs`; [`read`] parses each one and holds it to the shape the
//! README promises, and the binary prints the catalogue it read.
//!
//! The rules a manifest must keep:
//! - one top-level key, an `OcaBin`, named after its file (spaces, dashes,
//!   dots and colons fold to `_`, case ignored);
//! - `blocks.Identity.fields.topic.lbl` is its bus topic, under [`TOPIC_ROOT`],
//!   and no two manifests share one;
//! - every widget topic in a block's `fields` sits under that module topic.

/// Every manifest topic sits under this root.
pub const TOPIC_ROOT: &str = "APK.audio/System/BareMetal/ESP32/{node}";
/// How many manifests SRC/ holds (the README counts the same files).
pub const EXPECTED_FILES: usize = 124;
use serde_json::Value;

include!(concat!(env!("OUT_DIR"), "/manifests.rs"));

/// One manifest, as read.
#[derive(Debug, Clone)]
pub struct Module {
    /// Path under `SRC/`, e.g. `AUDIO/I2S Codec In/I2S Codec In.json`.
    pub path: &'static str,
    /// The OcaBin key, e.g. `I2S_Codec_In`.
    pub key: String,
    /// `blocks.Identity.fields.interface.lbl`, when the manifest names one.
    pub interface: Option<String>,
    /// `blocks.Identity.fields.topic.lbl`.
    pub topic: String,
    /// Block names in file order.
    pub blocks: Vec<String>,
    /// Widget topics declared in the blocks.
    pub widget_topics: usize,
}

fn fold(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            ' ' | '-' | '.' | ':' => '_',
            c => c.to_ascii_lowercase(),
        })
        .collect()
}

fn lbl<'a>(identity: &'a Value, field: &str) -> Option<&'a str> {
    identity.get("fields")?.get(field)?.get("lbl")?.as_str()
}

/// Parse and check one manifest.
pub fn read_one(path: &'static str, text: &str) -> Result<Module, String> {
    let doc: Value = serde_json::from_str(text).map_err(|e| format!("{path}: not JSON: {e}"))?;
    let obj = doc.as_object().ok_or_else(|| format!("{path}: not an object"))?;
    if obj.len() != 1 {
        return Err(format!("{path}: {} top-level keys, want 1", obj.len()));
    }
    let (key, bin) = obj.iter().next().unwrap();
    let stem = path.rsplit('/').next().unwrap().trim_end_matches(".json");
    if fold(key) != fold(stem) {
        return Err(format!("{path}: key {key:?} does not name the file"));
    }
    if bin.get("type").and_then(Value::as_str) != Some("OcaBin") {
        return Err(format!("{path}: {key} is not an OcaBin"));
    }
    let blocks = bin.get("blocks").and_then(Value::as_object).ok_or_else(|| format!("{path}: no blocks"))?;
    let identity = blocks.get("Identity").ok_or_else(|| format!("{path}: no Identity block"))?;
    let topic = lbl(identity, "topic").ok_or_else(|| format!("{path}: Identity has no topic"))?;
    if !topic.starts_with(TOPIC_ROOT) {
        return Err(format!("{path}: topic {topic} is not under {TOPIC_ROOT}"));
    }
    let mut widget_topics = 0;
    for (bname, block) in blocks {
        let Some(fields) = block.get("fields").and_then(Value::as_object) else { continue };
        for (fname, field) in fields {
            if let Some(t) = field.get("topic").and_then(Value::as_str) {
                if !t.starts_with(topic) {
                    return Err(format!("{path}: {bname}.{fname} topic {t} is not under {topic}"));
                }
                widget_topics += 1;
            }
        }
    }
    Ok(Module {
        path,
        key: key.clone(),
        interface: lbl(identity, "interface").map(str::to_owned),
        topic: topic.to_owned(),
        blocks: blocks.keys().cloned().collect(),
        widget_topics,
    })
}

/// Read every compiled-in manifest; fails on the first broken one or on a
/// topic two manifests share.
pub fn read() -> Result<Vec<Module>, String> {
    let mut out: Vec<Module> = Vec::with_capacity(MANIFESTS.len());
    for (path, text) in MANIFESTS {
        let m = read_one(path, text)?;
        if let Some(prev) = out.iter().find(|p| p.topic == m.topic) {
            return Err(format!("{}: topic {} already used by {}", m.path, m.topic, prev.path));
        }
        out.push(m);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_manifest_reads() {
        let all = read().unwrap();
        assert_eq!(all.len(), EXPECTED_FILES, "manifest count changed; update EXPECTED_FILES and the README");
    }

    #[test]
    fn a_broken_manifest_is_refused() {
        assert!(read_one("X/X.json", r#"{"Y":{"type":"OcaBin","blocks":{}}}"#).is_err());
        assert!(read_one("X/X.json", r#"{"X":{"type":"OcaBlock","blocks":{}}}"#).is_err());
        let off_root = r#"{"X":{"type":"OcaBin","blocks":{"Identity":{"fields":{"topic":{"lbl":"elsewhere/x"}}}}}}"#;
        assert!(read_one("X/X.json", off_root).is_err());
    }
}
