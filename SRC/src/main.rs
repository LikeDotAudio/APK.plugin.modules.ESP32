//! Prints the catalogue of this repository's manifests, one line per file:
//! path, phase, topic, interface. Exits 1 if any manifest breaks the shape
//! `apkaudio-ocabin` checks.
fn main() {
    match apkaudio_plugin_modules_esp32::read() {
        Ok(all) => {
            for m in &all {
                println!("{}\t{:?}\t{}\t{}", m.path, m.phase, m.topic, m.interface.as_deref().unwrap_or("-"));
            }
            let widgets: usize = all.iter().map(|m| m.widget_topics).sum();
            eprintln!("{} manifests read, {} widget topics, all under {}", all.len(), widgets, apkaudio_plugin_modules_esp32::topic_root());
        }
        Err(e) => {
            eprintln!("manifest error: {e}");
            std::process::exit(1);
        }
    }
}
