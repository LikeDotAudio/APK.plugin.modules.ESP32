//! Prints the catalogue of this repository's manifests, one line per file:
//! path, topic, interface. Exits 1 if any manifest breaks the shape `lib.rs` checks.
fn main() {
    match apkaudio_plugin_modules_esp32::read() {
        Ok(all) => {
            for m in &all {
                println!("{}\t{}\t{}", m.path, m.topic, m.interface.as_deref().unwrap_or("-"));
            }
            let widgets: usize = all.iter().map(|m| m.widget_topics).sum();
            eprintln!("{} manifests read, {} widget topics, all under {}", all.len(), widgets, apkaudio_plugin_modules_esp32::TOPIC_ROOT);
        }
        Err(e) => {
            eprintln!("manifest error: {e}");
            std::process::exit(1);
        }
    }
}
