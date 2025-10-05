use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopEntry {
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
    pub comment: Option<String>,
}

pub fn app_dirs() -> Vec<PathBuf> {
    let mut dirs = vec![];
    // User applications dir
    if let Ok(xdg_home) = std::env::var("XDG_DATA_HOME") {
        dirs.push(PathBuf::from(xdg_home).join("applications"));
    } else {
        dirs.push(dirs::home_dir().unwrap().join(".local/share/applications"));
    }

    // System dirs
    if let Ok(xdg_data_dirs) = std::env::var("XDG_DATA_DIRS") {
        for dir in xdg_data_dirs.split(':') {
            dirs.push(PathBuf::from(dir).join("applications"));
        }
    } else {
        dirs.push(PathBuf::from("/usr/local/share/applications"));
        dirs.push(PathBuf::from("/usr/share/applications"));
    }

    // Adding Flatpak and Snap manually
    dirs.push(PathBuf::from("/var/lib/flatpak/exports/share/applications"));
    dirs.push(PathBuf::from("/var/lib/snapd/desktop/applications"));
    dirs
}

pub fn parse_desktop_entry(content: &str) -> Vec<DesktopEntry> {
    let mut sections: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            current_section = line.to_string();
            sections
                .entry(current_section.clone())
                .or_insert_with(HashMap::new);
        } else if !current_section.is_empty()
            && let Some((key, value)) = line.split_once('=')
            && let Some(section_map) = sections.get_mut(&current_section)
        {
            section_map.insert(key.to_string(), value.to_string());
        }
    }

    // --- Pass 2: Build DesktopEntry objects from the parsed data ---
    let mut results = Vec::new();

    // First, try to build the main entry
    if let Some(main_section) = sections.get("[Desktop Entry]") {
        // skip all KDE only entries
        // TODO: check xdgdesktop and clear according to that
        if let Some(value) = main_section.get("OnlyShowIn")
            && value.split(';').any(|de| de.trim() == "KDE")
        {
            return Vec::new();
        }

        if let Some(value) = main_section.get("NoDisplay")
            && value.split(';').any(|de| de.trim() == "true")
        {
            return Vec::new();
        }

        let name = main_section.get("Name").cloned();
        let exec = main_section.get("Exec").cloned();
        let icon = main_section.get("Icon").cloned();
        let comment = main_section.get("Comment").cloned();
        let actions = main_section.get("Actions").cloned();

        if let (Some(name), Some(exec)) = (name, exec) {
            let main_entry = DesktopEntry {
                name: name.clone(),
                exec: clean_exec(&exec),
                icon: icon.clone(),
                comment: comment.clone(),
            };

            results.push(main_entry);

            // building each action
            if let Some(action_list) = actions {
                for action_id in action_list.split(';').filter(|s| !s.is_empty()) {
                    let action_section_name = format!("[Desktop Action {}]", action_id);
                    if let Some(action_section) = sections.get(&action_section_name)
                        && let (Some(action_name), Some(action_exec)) =
                            (action_section.get("Name"), action_section.get("Exec"))
                    {
                        let action_entry = DesktopEntry {
                            // Combine names for clarity
                            name: format!("{} - {}", name, action_name),
                            exec: clean_exec(action_exec),
                            // Use action's icon, or fall back to main icon
                            icon: action_section.get("Icon").cloned().or_else(|| icon.clone()),
                            // Inherit the main comment
                            comment: Some(action_name.clone()),
                        };
                        results.push(action_entry);
                    }
                }
            }
        }
    }

    results
}

fn clean_exec(exec_str: &str) -> String {
    exec_str
        .split_whitespace()
        .filter(|tok| !tok.starts_with('%'))
        .collect::<Vec<_>>()
        .join(" ")
}
