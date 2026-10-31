//! `apkaudio-ocabin` — the one reader for the DATA.* OcaBin manifest trees
//! (Bare Metal rule 1: write the reader first). DATA.ESP32, DATA.NETWORK,
//! DATA.NTP and DATA.PI each used to carry a byte-identical `build.rs` and a
//! `lib.rs` that differed in two constants; now each names its [`Tree`] and
//! calls this crate, so a rule is changed once.
//!
//! A DATA crate is three lines of glue:
//!
//! ```ignore
//! // build.rs
//! fn main() { apkaudio_ocabin::build::compile_manifests(); }
//! // src/lib.rs
//! include!(concat!(env!("OUT_DIR"), "/manifests.rs"));
//! pub const TREE: apkaudio_ocabin::Tree = apkaudio_ocabin::Tree {
//!     target: "ESP32", placeholder: "{node}",
//!     manifests: MANIFESTS, folders: FOLDERS, strays: STRAYS,
//! };
//! ```
//!
//! The rules a manifest must keep ([`Tree::read_one`]):
//! - one top-level key, an `OcaBin`, named after its file (spaces, dashes,
//!   dots and colons fold to `_`, case ignored);
//! - `schema_version` is [`SCHEMA_VERSION`];
//! - `status` is `"planned"` (declared, nothing serves it yet) or `"live"`
//!   (something publishes it), and a live one names its publisher in
//!   `implemented_by`;
//! - `blocks.Identity.fields.topic.lbl` is its bus topic, under the tree's
//!   root (`APK.audio/System/BareMetal/<target>/<placeholder>`, built from
//!   `apkaudio_contracts::topics::baremetal`), a declared bareMetal topic of
//!   that target by the contract's own `parse`, and no two manifests share one;
//! - every widget topic in a block's `fields` sits under that module topic and
//!   is in the contract too.
//!
//! And the tree's layout ([`Tree::check_layout`], which replaced the
//! hand-kept `EXPECTED_FILES` counts): the root holds exactly one manifest,
//! every folder holds exactly one, named after the folder, and nothing else
//! (no stray scripts).

pub mod build;

#[cfg(feature = "reader")]
mod reader;
#[cfg(feature = "reader")]
pub use reader::*;
