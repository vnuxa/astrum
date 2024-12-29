use std::fs::{self, File};
use std::io::{self, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::env::{self, home_dir};

use astrum_core::app::main::start_application;
use clap::Parser;
use ftail::Ftail;
use log::{info, Level, LevelFilter};
mod astrum_core;
mod astrum_binds;

#[derive(Parser)]
struct Cli {
    /// Path to config folder. Default is ~/.config/astrum/
    #[arg(short, long)]
    config: Option<String>,

    /// Sends a specified call to a running process of astrum
    #[arg(long)]
    call: Option<String>,

    /// Instead of logging to a file, will log into
    #[arg(short, long)]
    log_std: bool
}
fn run_command(command: &str) {

    Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("well the command failed, wow");
}

fn make_cache(cache_names: Vec<&str>) {
    if !Path::new(&(home_dir().unwrap().to_str().unwrap().to_string() + "/.cache/astrum/")).exists() {
        run_command("mkdir ~/.cache/astrum/");
    }
    for cache in cache_names {
        if !Path::new(&format!("{}/.cache/astrum/{}", home_dir().unwrap().to_str().unwrap(), cache)).exists() {
            run_command(format!("mkdir ~/.cache/astrum/{}", cache).as_str());
        }
    }
}
fn type_path() -> io::Result<PathBuf> {
    let mut dir = Path::new(&env::var("CARGO_MANIFEST_DIR").expect("Couldnt find CARGO_MANIFEST_DIR"))
        .join("src")
        .join("lua_library")
        .join("astrum")
        .join("types");

    // dir.push("lib")/* ; */
    Ok(dir)
}

fn main() {
    let cli = Cli::parse();
    if cli.call.is_some() {
        let socket = format!("{}/.cache/astrum/sockets/calls", home_dir().unwrap().display());
        let mut unix_stream = UnixStream::connect(socket).expect("Could not create stream");

        unix_stream
            .write_all(cli.call.clone().unwrap().as_bytes())
            .expect("failed at writing calls to listener");

        return
    }

    if cli.log_std {
        Ftail::new()
            .console(LevelFilter::Off)
            .init().unwrap();
    } else {
        let log_dir = env::var("XDG_RUNTIME_DIR").unwrap() + "/astrum/";
        println!("log dir {}", log_dir);
        if !Path::new(&log_dir).exists() {
            println!("log dir doesnt exist");
            run_command(&format!("mkdir {}", log_dir));
        }

        Ftail::new()
            .daily_file(&log_dir, LevelFilter::Off)
            .init().unwrap();
    }



    let config_path: &str = &cli.config
        .unwrap_or("~/.config/astrum/".to_string())
        .replace("~", home_dir().unwrap().to_str().unwrap());

    // check if config path exists, if not then make it
    if !Path::new(config_path).exists() {
        info!("config path does not exist, making one now");
        run_command(&format!("mkdir {}", config_path));

        let lua_file = File::create(config_path.to_owned() + "config.lua").expect("Failed to create config.lua");
    }

    // luarc file for the lua-language-server

    if let Ok(md) = fs::metadata(config_path.to_owned()) {
        let permissions = md.permissions();
        let readonly = permissions.readonly();
        if readonly {
            info!("No write permission for .luarc, skipping");
        } else {
            let mut file = File::create(config_path.to_owned() + ".luarc.json").expect("Failed to create .luarc.json");
            file.write_fmt(format_args!(
                "
                    {{
                    \"$schema\": \"https://raw.githubusercontent.com/LuaLS/vscode-lua/master/setting/schema.json\",
                    \"workspace.library\": [
                    \"{types_path}\"
                    ],

                    \"runtime.version\": \"LuaJIT\",
                    }}
                    ",
                types_path = type_path().unwrap().display()
            ));
        }
    }

    start_application(Path::new(config_path).join("config.lua"));
}
