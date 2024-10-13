// mostly comprises of functions
// cant really have many events
// i mean maybe when an application is installed/uninstalled??

use std::{fs, path::PathBuf, process::Command};

use ini::ini;
use walkdir::WalkDir;

use std::collections::HashSet;


// maybe add custom AppModel thing
// that just implements functions on apps like
// app.launch()



// make this async too
pub fn parse_desktop_file(desktop_file_path: PathBuf) -> Option<String> {
    let desktop_path = desktop_file_path.clone();
    let desktop_file_path_str = desktop_file_path.to_str().unwrap();
    let map = ini!(desktop_file_path_str);
    let desktop_entry_exists = map.contains_key("desktop entry");
    let mut executable = "".to_string();
    let mut icon_path = "".to_string();
    let mut app_name = "".to_string();
    let mut description = "".to_string();

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
            executable = exec.unwrap();

            let icon = desktop_entry["icon"].clone();
            icon_path = icon.unwrap();

            let name = desktop_entry["name"].clone();
            app_name = name.unwrap();
        } else {
            return None;
        }
        if desktop_entry.contains_key("comment") {
            let app_description = desktop_entry["comment"].clone();
            description = app_description.unwrap();
        }
    }  else {
        return None;
    }
    if app_name == "".to_string() || icon_path == "".to_string()  {
        return None;
    }

    Some(
        format!(

            "{{ \n name = '{name}',\n executable = '{executable}',\n icon = '{icon}',\n desktop_path = '{desktop}',\n description = '{description}' }},\n ",
            name = app_name,
            executable = executable,
            icon = icon_path,
            desktop = desktop_path.display(),
            description = description
        )
    )
}

// maybe make it async
pub fn get_all_apps() -> Option<String> {
    // read XDG_DATA_DIRS env var
    let xdg_data_dirs = std::env::var("XDG_DATA_DIRS").unwrap_or("/usr/share".to_string());
    let xdg_data_dirs: Vec<String> = xdg_data_dirs.split(':').map(|str| format!("{}{}",str, "/applications")).collect();
    // make a string sett from xdg_data_dirs
    let mut search_dirs: HashSet<String> = xdg_data_dirs.iter().cloned().collect();
    println!("searching ||| {:?}", search_dirs);
    // search_dirs.insert("/usr/share/applications");
    // get home dir of current user
    // let home_dir = std::env::var("HOME").unwrap();
    // let home_path = PathBuf::from(home_dir);
    // let local_share_apps = home_path.join(".local/share/applications");
    // search_dirs.insert(local_share_apps.to_str().unwrap());
    // search_dirs.insert("/usr/share/xsessions");
    // search_dirs.insert("/etc/xdg/autostart");
    // search_dirs.insert("/var/lib/snapd/desktop/applications");
    // for each dir, search for .desktop files
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


pub fn launch_app(app_path: String) {
    // try making this into a PathBuf::from(app_path)
    // if it doesnt work
    Command::new("bash")
        .arg("-c")
        .arg(app_path)
        .spawn()
        .expect("failed to execute app");
}
// fn get_all_apps() -> Vec<AppModel> {
//     let mut context = AppInfoContext::new();
//     context.refresh_apps().unwrap(); // this is blocking though
//
//     let mut apps = context.get_all_apps();
// }
