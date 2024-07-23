use std::{path::PathBuf};

use lazy_static::lazy_static;
use simple_home_dir::home_dir;

pub mod lua;





lazy_static::lazy_static!{
    pub static ref HOME_DIR: PathBuf = home_dir().expect("getting home_dir failed");
    // pub static ref CONFIG_DIRS: Vec<PathBuf> = config_dirs();
    // pub static ref DATA_DIR: PathBuf = compute_data_dir().unwrap();

}


// fn config_dirs() -> Vec<PathBuf> {
//     let mut dirs = Vec::new();
//     dirs.push(xdg_config_home());
//
//     #[cfg(unix)]
//     if let Some(d) = std::env::var_os("XDG_CONFIG_DIRS") {
//         dirs.extend(std::env::split_paths(&d).map(|s| PathBuf::from(s).join("wezterm")));
//     }
//
//     dirs
// }

// fn compute_data_dir() -> anyhow::Result<PathBuf> {
//     if let Some(runtime) = dirs_next::data_dir() {
//         return Ok(runtime.join("astrum"));
//     }
//
//     Ok(HOME_DIR.join(".local/share/astrum"))
// }
