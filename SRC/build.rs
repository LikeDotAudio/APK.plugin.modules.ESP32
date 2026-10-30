//! Compiles every manifest under SRC/ into the crate (Bare Metal rule 6:
//! identities are compiled in, never read from an unverified path at run time).
//! Writes `$OUT_DIR/manifests.rs`: `MANIFESTS: &[(relative path, json text)]`.
use std::{env, fs, path::Path};

fn walk(dir: &Path, root: &Path, out: &mut Vec<String>) {
    println!("cargo:rerun-if-changed={}", dir.display());
    let mut entries: Vec<_> = fs::read_dir(dir).expect("read SRC").flatten().collect();
    entries.sort_by_key(|e| e.file_name());
    for e in entries {
        let p = e.path();
        let name = e.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') || name == "target" || name == "src" {
            continue;
        }
        if p.is_dir() {
            walk(&p, root, out);
        } else if name.ends_with(".json") {
            println!("cargo:rerun-if-changed={}", p.display());
            let rel = p.strip_prefix(root).unwrap().to_string_lossy().replace('\\', "/");
            out.push(format!("    ({:?}, include_str!({:?})),\n", rel, p.display().to_string()));
        }
    }
}

fn main() {
    let root = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).to_path_buf();
    let mut rows = Vec::new();
    walk(&root, &root, &mut rows);
    let body = format!("pub static MANIFESTS: &[(&str, &str)] = &[\n{}];\n", rows.concat());
    let out = Path::new(&env::var("OUT_DIR").unwrap()).join("manifests.rs");
    fs::write(out, body).expect("write manifests.rs");
}
