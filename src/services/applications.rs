// mostly comprises of functions
// cant really have many events
// i mean maybe when an application is installed/uninstalled??

use std::{path::PathBuf, process::Command};

use ini::ini;
use walkdir::WalkDir;

use std::collections::HashSet;


// maybe add custom AppModel thing
// that just implements functions on apps like
// app.launch()

#[derive(Debug, Clone, Default)]
pub struct AppModel {
    pub name: String,

    pub icon_path: PathBuf,
    pub executable: PathBuf,
    pub desktop_path: PathBuf,

    pub description: Option<String>
}

pub trait AppControls {
    /// Launches the exectuable
    fn launch(self);
}

impl AppControls for AppModel {
    fn launch(self) {
        Command::new(self.executable)
            .output()
            .expect("failed to execute app");
    }
}


// make this async too
pub fn parse_desktop_file(desktop_file_path: PathBuf) -> Option<AppModel> {
    let mut app = AppModel { ..Default::default() };
    app.desktop_path = desktop_file_path.clone();
    let desktop_file_path_str = desktop_file_path.to_str().unwrap();
    let map = ini!(desktop_file_path_str);
    let desktop_entry_exists = map.contains_key("desktop entry");
    if desktop_entry_exists {
        let desktop_entry = map["desktop entry"].clone();
        // if desktop_entry.contains_key("Terminal") {
        //     if desktop_entry["Terminal"] == true {
        //         return None; // it is not an application
        //     }
        // }
        if
            desktop_entry.contains_key("exec") &&
            desktop_entry.contains_key("icon") &&
            desktop_entry.contains_key("name")
        {
            let exec = desktop_entry["exec"].clone();
            app.executable = PathBuf::from(exec.unwrap());

            let icon = desktop_entry["icon"].clone();
            app.icon_path = PathBuf::from(icon.unwrap());

            let name = desktop_entry["name"].clone();
            app.name = name.unwrap();
        } else {
            return None
        }
        if desktop_entry.contains_key("comment") {
            let description = desktop_entry["comment"].clone();
            app.description = description;
        }
    }

    Some(app)
}

// maybe make it async
pub fn get_all_apps() -> Result<Vec<AppModel>, String> {
    // read XDG_DATA_DIRS env var
    let xdg_data_dirs = std::env::var("XDG_DATA_DIRS").unwrap_or("/usr/share".to_string());
    let xdg_data_dirs: Vec<&str> = xdg_data_dirs.split(':').collect();
    // make a string sett from xdg_data_dirs
    let mut search_dirs: HashSet<&str> = xdg_data_dirs.iter().cloned().collect();
    search_dirs.insert("/usr/share/applications");
    // get home dir of current user
    let home_dir = std::env::var("HOME").unwrap();
    let home_path = PathBuf::from(home_dir);
    let local_share_apps = home_path.join(".local/share/applications");
    search_dirs.insert(local_share_apps.to_str().unwrap());
    search_dirs.insert("/usr/share/xsessions");
    search_dirs.insert("/etc/xdg/autostart");
    search_dirs.insert("/var/lib/snapd/desktop/applications");
    // for each dir, search for .desktop files
    let mut apps: Vec<AppModel> = Vec::new();
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
                    apps.push(app);
                }
            }
        }
    }
    Ok(apps)
}

fn launch_app(app_path: PathBuf) {
}
// fn get_all_apps() -> Vec<AppModel> {
//     let mut context = AppInfoContext::new();
//     context.refresh_apps().unwrap(); // this is blocking though
//
//     let mut apps = context.get_all_apps();
// }
