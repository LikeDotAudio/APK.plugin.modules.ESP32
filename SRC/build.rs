//! Compiles every manifest under SRC/ into the crate — the shared walk in
//! `apkaudio-ocabin` (see `ocabin/src/build.rs`).
fn main() {
    apkaudio_ocabin::build::compile_manifests();
}
