use crate::{
    cache::save_cache,
    desktop::{app_dirs, parse_desktop_entry},
};
use tokio::task;
use walkdir::WalkDir;

pub async fn run_service() {
    println!("Running background indexing service...");

    let entries = task::spawn_blocking(|| {
        let mut entries = Vec::new();

        for dir in app_dirs() {
            if !dir.exists() {
                continue;
            }

            for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("desktop")
                    && let Ok(content) = std::fs::read_to_string(path)
                {
                    let parsed_entries = parse_desktop_entry(&content);
                    if !parsed_entries.is_empty() {
                        entries.extend(parsed_entries);
                    }
                }
            }
        }

        entries
    })
    .await
    .unwrap();

    let _ = save_cache(&entries);
    println!("Indexing complete: {} apps cached.", entries.len());
}
