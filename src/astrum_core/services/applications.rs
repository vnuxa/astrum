//

use std::{borrow::Borrow, collections::HashSet, path::PathBuf, process::Command};

use freedesktop_entry_parser::parse_entry;
use std::env::home_dir;
use walkdir::WalkDir;

// missing features: open terminal apps via a terminal
// get a single app from name
pub fn parse_desktop_file(desktop_file_path: PathBuf) -> Option<String> {
    let desktop_path = desktop_file_path.clone();
    let desktop_path_file_str = desktop_file_path.to_str().unwrap();
    // make it so that if this entry deosnt work, return a None
    let map = parse_entry(desktop_path_file_str).expect("Failed to parse desktop entry");

    let mut executable = "".to_string();
    let mut icon_path = "".to_string();
    let mut app_name = "".to_string();
    let mut description = "".to_string();

    if map.has_section("Desktop Entry") {
        let desktop_entry = map.section("Desktop Entry");

        if desktop_entry.has_attr("Exec")
            && desktop_entry.has_attr("Icon")
            && desktop_entry.has_attr("Name")
            && !desktop_entry.has_attr("NoDisplay")
        // kind of lazy, lmk if this breaks something
        {
            executable = desktop_entry.attr("Exec").unwrap().to_string();
            icon_path = desktop_entry.attr("Icon").unwrap().to_string();
            app_name = desktop_entry.attr("Name").unwrap().to_string();
        } else {
            return None;
        }
        if desktop_entry.has_attr("Comment") {
            description = desktop_entry.attr("Comment").unwrap().to_string();
        }
    } else {
        return None;
    }
    if app_name == "".to_string() || icon_path == "".to_string() {
        return None;
    }

    // println!("file path: {:?}", desktop_file_path);
    Some(
        format!(

            "{{ \n name = '{name}',\n executable = '{executable}',\n icon = '{icon}',\n desktop_path = '{desktop}',\n description = '{description}', id = '{id}' }},\n ",
            name = app_name.replace("'", r"\'"),
            executable = executable.replace("'", r"\'"),
            icon = icon_path.replace("'", r"\'"),
            desktop = desktop_path.display().to_string().replace("'", r"\'"),
            description = description.replace("'", r"\'"),
            id = (|| {
                if let Ok(link) = std::fs::read_link(desktop_file_path.clone()) {
                    return link.file_name().unwrap().to_str().unwrap().replace(".desktop", "");
                }
                desktop_file_path.file_name().unwrap().to_str().unwrap().replace(".desktop", "")
                // std::fs::read_link(desktop_file_path).unwrap().file_name().unwrap().to_str().unwrap().replace(".desktop","")
            })().replace("'", r"\'")
        )
    )
}

pub fn get_all_apps() -> Option<String> {
    let xdg_data_dirs = std::env::var("XDG_DATA_DIRS").unwrap_or("/usr/share".to_string());
    let xdg_data_dirs: Vec<String> = xdg_data_dirs
        .split(':')
        .map(|str| format!("{}{}", str, "/applications"))
        .collect();
    let mut search_dirs: HashSet<String> = xdg_data_dirs.iter().cloned().collect();
    // println!("searching ||| {:?}", search_dirs);
    search_dirs.insert(home_dir()?.to_str().unwrap().to_owned() + "/.local/share/applications");
    let mut apps: String = "{".to_string();
    for dir in search_dirs {
        let dir = PathBuf::from(dir);
        if !dir.exists() {
            continue;
        }
        for entry in WalkDir::new(dir.clone()) {
            if entry.is_err() {
                continue;
            }
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().is_none() {
                continue;
            }

            if path.extension().unwrap() == "desktop" {
                let app = parse_desktop_file(path.to_path_buf());
                if let Some(app) = app {
                    apps.push_str(&app);
                }
            }
        }
    }
    apps.push_str("}");
    Some(apps)
}

pub fn run_command(arguments: String) {
    Command::new("bash")
        .arg("-c")
        .arg(arguments)
        .spawn()
        .expect("failed to execute app");
}

pub fn launch_app(app_path: String) {
    // try making this into a PathBuf::from(app_path)
    // if it doesnt work
    Command::new("bash")
        .arg("-c")
        .arg(app_path)
        .spawn()
        .expect("failed to execute app");
}
