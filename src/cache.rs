use crate::desktop::DesktopEntry;
use std::{fs, path::PathBuf};

fn cache_file() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("bazooka_apps.json")
}

pub fn save_cache(entries: &[DesktopEntry]) -> std::io::Result<()> {
    let json = serde_json::to_string(entries)?;
    fs::write(cache_file(), json)
}

pub fn load_cache() -> Option<Vec<DesktopEntry>> {
    let path = cache_file();
    if path.exists() {
        let data = fs::read_to_string(path).ok()?;
        serde_json::from_str(&data).ok()
    } else {
        None
    }
}
