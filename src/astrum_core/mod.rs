use std::path::PathBuf;

use std::env::home_dir;

pub mod app;
pub mod lua_context;
pub mod services;
pub mod animations;


lazy_static::lazy_static!{
    pub static ref HOME_DIR: PathBuf = home_dir().expect("getting home_dir failed");
}
