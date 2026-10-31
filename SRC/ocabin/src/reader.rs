//! The read half: [`Tree`] holds a compiled-in manifest tree to the shape the
//! crate docs list, and binds concrete bus topics to the manifest that
//! declares them.
use apkaudio_contracts::topics::{baremetal, parse, ParsedTopic};
use serde_json::Value;

/// The manifest shape this reader understands. A manifest carries it as
/// `"schema_version"`; a different number is refused, so a shape change is a
/// visible, versioned event and not a silent drift.
pub const SCHEMA_VERSION: u64 = 1;

/// Is anything on the bus serving this manifest yet?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Declared ("describe before you build"); nothing publishes it.
    Planned,
    /// Published today, by the `implemented_by` the manifest names.
    Live,
}

/// One manifest, as read.
#[derive(Debug, Clone)]
pub struct Module {
    /// Path under `SRC/`, e.g. `AUDIO/I2S Codec In/I2S Codec In.json`.
    pub path: &'static str,
    /// The OcaBin key, e.g. `I2S_Codec_In`.
    pub key: String,
    /// `blocks.Identity.fields.interface.lbl`, when the manifest names one.
    pub interface: Option<String>,
    /// `blocks.Identity.fields.topic.lbl` — with the tree's placeholder in it.
    pub topic: String,
    /// Block names in file order.
    pub blocks: Vec<String>,
    /// Widget topics declared in the blocks.
    pub widget_topics: usize,
    /// `status`.
    pub status: Status,
    /// `implemented_by` — who publishes it (required when live).
    pub implemented_by: Option<String>,
}

/// One compiled-in manifest tree: its BareMetal target, the placeholder its
/// manifests write where the node's name goes, and the three statics
/// [`crate::build::compile_manifests`] generated.
#[derive(Debug, Clone, Copy)]
pub struct Tree {
    /// One of `apkaudio_contracts::topics::BAREMETAL_TARGETS`, e.g. `"ESP32"`.
    pub target: &'static str,
    /// What a manifest writes for the node segment, e.g. `"{node}"`, `"{host}"`.
    pub placeholder: &'static str,
    pub manifests: &'static [(&'static str, &'static str)],
    pub folders: &'static [&'static str],
    pub strays: &'static [&'static str],
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

fn under(topic: &str, root: &str) -> bool {
    topic == root || topic.strip_prefix(root).is_some_and(|rest| rest.starts_with('/'))
}

impl Tree {
    /// `APK.audio/System/BareMetal/<target>/<placeholder>`, built from the
    /// contract's `baremetal` builder (so a target the contract does not
    /// declare has no root at all).
    pub fn topic_root(&self) -> Result<String, String> {
        let head = baremetal(self.target, &[]).map_err(|e| format!("bareMetal target {}: {e:?}", self.target))?;
        Ok(format!("{head}/{}", self.placeholder))
    }

    /// Is `topic` (placeholder filled in) a bareMetal topic of this tree's
    /// target by the contract's own parse?
    pub fn in_contract(&self, topic: &str) -> Result<(), String> {
        let concrete = topic.replace(self.placeholder, "node");
        match parse(&concrete) {
            ParsedTopic::BareMetal { target, .. }
            | ParsedTopic::BareMetalCommand { target, .. }
            | ParsedTopic::BareMetalStream { target, .. }
                if target == self.target =>
            {
                Ok(())
            }
            other => Err(format!("{topic} is not a declared bareMetal/{} topic (parsed as {other:?})", self.target)),
        }
    }

    /// Parse and check one manifest.
    pub fn read_one(&self, path: &'static str, text: &str) -> Result<Module, String> {
        let root = self.topic_root()?;
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
        match bin.get("schema_version").and_then(Value::as_u64) {
            Some(SCHEMA_VERSION) => {}
            other => return Err(format!("{path}: schema_version {other:?}, this reader reads {SCHEMA_VERSION}")),
        }
        let implemented_by = bin.get("implemented_by").and_then(Value::as_str).map(str::to_owned);
        let status = match (bin.get("status").and_then(Value::as_str), &implemented_by) {
            (Some("planned"), _) => Status::Planned,
            (Some("live"), Some(by)) if !by.trim().is_empty() => Status::Live,
            (Some("live"), _) => return Err(format!("{path}: status live but no implemented_by")),
            (other, _) => return Err(format!("{path}: status {other:?}, want \"planned\" or \"live\"")),
        };
        let blocks = bin.get("blocks").and_then(Value::as_object).ok_or_else(|| format!("{path}: no blocks"))?;
        let identity = blocks.get("Identity").ok_or_else(|| format!("{path}: no Identity block"))?;
        let topic = lbl(identity, "topic").ok_or_else(|| format!("{path}: Identity has no topic"))?;
        if !under(topic, &root) {
            return Err(format!("{path}: topic {topic} is not under {root}"));
        }
        self.in_contract(topic).map_err(|e| format!("{path}: {e}"))?;
        let mut widget_topics = 0;
        for (bname, block) in blocks {
            let Some(fields) = block.get("fields").and_then(Value::as_object) else { continue };
            for (fname, field) in fields {
                if let Some(t) = field.get("topic").and_then(Value::as_str) {
                    if !under(t, topic) {
                        return Err(format!("{path}: {bname}.{fname} topic {t} is not under {topic}"));
                    }
                    self.in_contract(t).map_err(|e| format!("{path}: {bname}.{fname}: {e}"))?;
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
            status,
            implemented_by,
        })
    }

    /// The tree's layout: one manifest at the root, exactly one per folder
    /// named after it, nothing else inside folders.
    pub fn check_layout(&self) -> Result<(), String> {
        if let Some(stray) = self.strays.first() {
            return Err(format!("{stray}: not a manifest — a folder holds one JSON named after it and nothing else"));
        }
        let at_root = self.manifests.iter().filter(|(p, _)| !p.contains('/')).count();
        if at_root != 1 {
            return Err(format!("the root holds {at_root} manifests, want 1 (the tree's own)"));
        }
        for folder in self.folders {
            let inside: Vec<&str> = self
                .manifests
                .iter()
                .map(|(p, _)| *p)
                .filter(|p| p.rsplit_once('/').map(|(dir, _)| dir) == Some(*folder))
                .collect();
            let name = folder.rsplit('/').next().unwrap();
            let want = format!("{folder}/{name}.json");
            if inside != [want.as_str()] {
                return Err(format!("{folder}: holds {inside:?}, want exactly [{want:?}]"));
            }
        }
        Ok(())
    }

    /// Read every compiled-in manifest; fails on the layout, on the first
    /// broken manifest, or on a topic two manifests share.
    pub fn read(&self) -> Result<Vec<Module>, String> {
        self.check_layout()?;
        let mut out: Vec<Module> = Vec::with_capacity(self.manifests.len());
        for (path, text) in self.manifests {
            let m = self.read_one(path, text)?;
            if let Some(prev) = out.iter().find(|p| p.topic == m.topic) {
                return Err(format!("{}: topic {} already used by {}", m.path, m.topic, prev.path));
            }
            out.push(m);
        }
        Ok(out)
    }

    /// The manifest a concrete bus topic belongs to: the module whose topic
    /// (placeholder = any one segment) is the longest prefix of `topic`. This
    /// is what a device or panel binding to the catalogue does.
    pub fn bind<'a>(&self, modules: &'a [Module], topic: &str) -> Option<&'a Module> {
        let have: Vec<&str> = topic.split('/').collect();
        modules
            .iter()
            .filter(|m| {
                let want: Vec<&str> = m.topic.split('/').collect();
                want.len() <= have.len()
                    && want
                        .iter()
                        .zip(&have)
                        .all(|(w, h)| if *w == self.placeholder { !h.is_empty() } else { w == h })
            })
            .max_by_key(|m| m.topic.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROOT: &str = r#"{"X":{"type":"OcaBin","schema_version":1,"status":"planned","blocks":{"Identity":{"fields":{"topic":{"lbl":"APK.audio/System/BareMetal/ESP32/{node}"}}}}}}"#;
    const MOD: &str = r#"{"M":{"type":"OcaBin","schema_version":1,"status":"live","implemented_by":"a test","blocks":{"Identity":{"fields":{"topic":{"lbl":"APK.audio/System/BareMetal/ESP32/{node}/D/M"}}},"Telemetry":{"fields":{"v":{"topic":"APK.audio/System/BareMetal/ESP32/{node}/D/M/v"}}}}}}"#;

    fn tree(manifests: &'static [(&'static str, &'static str)], folders: &'static [&'static str]) -> Tree {
        Tree { target: "ESP32", placeholder: "{node}", manifests, folders, strays: &[] }
    }

    static GOOD: [(&str, &str); 2] = [("X.json", ROOT), ("D/M/M.json", MOD)];

    #[test]
    fn a_good_tree_reads_and_binds() {
        let t = tree(&GOOD, &["D/M"]);
        let all = t.read().unwrap();
        assert_eq!(all.len(), 2);
        assert_eq!(all[1].status, Status::Live);
        assert_eq!(all[1].widget_topics, 1);
        let m = t.bind(&all, "APK.audio/System/BareMetal/ESP32/beskar/D/M/v").unwrap();
        assert_eq!(m.key, "M");
        assert_eq!(t.bind(&all, "APK.audio/System/BareMetal/ESP32/beskar").unwrap().key, "X");
        assert!(t.bind(&all, "APK.audio/System/BareMetal/PI/beskar/D/M").is_none());
    }

    #[test]
    fn the_root_is_built_from_the_contract() {
        assert_eq!(tree(&GOOD, &[]).topic_root().unwrap(), "APK.audio/System/BareMetal/ESP32/{node}");
        let unknown = Tree { target: "Arduino", ..tree(&GOOD, &[]) };
        assert!(unknown.topic_root().is_err());
    }

    #[test]
    fn a_broken_manifest_is_refused() {
        let t = tree(&GOOD, &[]);
        let bad = [
            r#"{"Y":{"type":"OcaBin","schema_version":1,"status":"planned","blocks":{}}}"#, // key does not name the file
            r#"{"X":{"type":"OcaBlock","schema_version":1,"status":"planned","blocks":{}}}"#, // not an OcaBin
            r#"{"X":{"type":"OcaBin","status":"planned","blocks":{"Identity":{"fields":{"topic":{"lbl":"APK.audio/System/BareMetal/ESP32/{node}"}}}}}}"#, // no schema_version
            r#"{"X":{"type":"OcaBin","schema_version":2,"status":"planned","blocks":{"Identity":{"fields":{"topic":{"lbl":"APK.audio/System/BareMetal/ESP32/{node}"}}}}}}"#, // newer shape
            r#"{"X":{"type":"OcaBin","schema_version":1,"status":"live","blocks":{"Identity":{"fields":{"topic":{"lbl":"APK.audio/System/BareMetal/ESP32/{node}"}}}}}}"#, // live, no publisher
            r#"{"X":{"type":"OcaBin","schema_version":1,"blocks":{"Identity":{"fields":{"topic":{"lbl":"APK.audio/System/BareMetal/ESP32/{node}"}}}}}}"#, // no status
            r#"{"X":{"type":"OcaBin","schema_version":1,"status":"planned","blocks":{"Identity":{"fields":{"topic":{"lbl":"elsewhere/x"}}}}}}"#, // off the root
            r#"{"X":{"type":"OcaBin","schema_version":1,"status":"planned","blocks":{"Identity":{"fields":{"topic":{"lbl":"APK.audio/System/BareMetal/ESP32/{node}x"}}}}}}"#, // a prefix, not a parent
            r#"{"X":{"type":"OcaBin","schema_version":1,"status":"planned","blocks":{"Identity":{"fields":{"topic":{"lbl":"APK.audio/System/BareMetal/ESP32/{node}/a b"}}}}}}"#, // not a contract segment
        ];
        for text in bad {
            assert!(t.read_one("X.json", text).is_err(), "accepted {text}");
        }
    }

    #[test]
    fn the_layout_is_held() {
        static TWO_AT_ROOT: [(&str, &str); 2] = [("X.json", ROOT), ("M.json", MOD)];
        assert!(tree(&TWO_AT_ROOT, &[]).check_layout().is_err());
        static MISNAMED: [(&str, &str); 2] = [("X.json", ROOT), ("D/M/N.json", MOD)];
        assert!(tree(&MISNAMED, &["D/M"]).check_layout().is_err());
        // an empty folder
        assert!(tree(&GOOD, &["D/M", "D/Empty"]).check_layout().is_err());
        let stray = Tree { strays: &["D/M/sweep.py"], ..tree(&GOOD, &["D/M"]) };
        assert!(stray.check_layout().is_err());
        // two manifests sharing a topic
        static TWICE: [(&str, &str); 3] = [("X.json", ROOT), ("D/M/M.json", MOD), ("E/M/M.json", MOD)];
        assert!(tree(&TWICE, &["D/M", "E/M"]).read().is_err());
    }
}
