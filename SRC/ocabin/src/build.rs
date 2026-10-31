//! The build half: a DATA crate's `build.rs` is one call,
//!
//! ```ignore
//! fn main() { apkaudio_ocabin::build::compile_manifests(); }
//! ```
//!
//! It walks `CARGO_MANIFEST_DIR` and writes `$OUT_DIR/manifests.rs` with three
//! statics the crate `include!`s and hands to [`crate::Tree`] (Bare Metal
//! rule 6: identities are compiled in, never read from an unverified path at
//! run time):
//!
//! - `MANIFESTS: &[(relative path, json text)]` — every `*.json`;
//! - `FOLDERS: &[relative path]` — every folder walked (not the root);
//! - `STRAYS: &[relative path]` — every other file inside a folder (the root
//!   may hold the crate's own files: Cargo.toml, build.rs, …).
//!
//! Skipped: dot-entries, `target`, `src`, and any folder holding its own
//! `Cargo.toml` (another crate, e.g. this one inside DATA.ESP32).
use std::{env, fs, path::Path};

#[derive(Default)]
struct Walk {
    manifests: Vec<String>,
    folders: Vec<String>,
    strays: Vec<String>,
}

fn rel(p: &Path, root: &Path) -> String {
    p.strip_prefix(root).unwrap().to_string_lossy().replace('\\', "/")
}

fn walk(dir: &Path, root: &Path, out: &mut Walk) {
    println!("cargo:rerun-if-changed={}", dir.display());
    let mut entries: Vec<_> = fs::read_dir(dir).expect("read the manifest tree").flatten().collect();
    entries.sort_by_key(|e| e.file_name());
    for e in entries {
        let p = e.path();
        let name = e.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') || name == "target" || name == "src" {
            continue;
        }
        if p.is_dir() {
            if p.join("Cargo.toml").exists() {
                continue;
            }
            out.folders.push(rel(&p, root));
            walk(&p, root, out);
        } else if name.ends_with(".json") {
            println!("cargo:rerun-if-changed={}", p.display());
            out.manifests
                .push(format!("    ({:?}, include_str!({:?})),\n", rel(&p, root), p.display().to_string()));
        } else if dir != root {
            out.strays.push(rel(&p, root));
        }
    }
}

fn list(rows: &[String]) -> String {
    rows.iter().map(|r| format!("    {r:?},\n")).collect()
}

/// Walk the calling crate's manifest dir and write `$OUT_DIR/manifests.rs`.
pub fn compile_manifests() {
    let root = Path::new(&env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR")).to_path_buf();
    let mut w = Walk::default();
    walk(&root, &root, &mut w);
    let body = format!(
        "pub static MANIFESTS: &[(&str, &str)] = &[\n{}];\npub static FOLDERS: &[&str] = &[\n{}];\npub static STRAYS: &[&str] = &[\n{}];\n",
        w.manifests.concat(),
        list(&w.folders),
        list(&w.strays)
    );
    let out = Path::new(&env::var("OUT_DIR").expect("OUT_DIR")).join("manifests.rs");
    fs::write(out, body).expect("write manifests.rs");
}
