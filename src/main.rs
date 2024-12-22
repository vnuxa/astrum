use std::fs::File;
use std::io::{self, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::env::{self, home_dir};

use astrum_core::app::main::start_application;
use clap::Parser;
mod astrum_core;
mod astrum_binds;

#[derive(Parser)]
struct Cli {
    /// Path to config folder. Default is ~/.config/astrum/
    #[arg(short, long)]
    config: Option<String>,

    /// Sends a specified call to a running process of astrum
    #[arg(long)]
    call: Option<String>
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
    let mut dir = env::current_exe()?;
    dir.pop();
    dir.pop();
    dir.pop();
    dir.push("src/");
    dir.push("lua_library/");
    dir.push("astrum/");
    dir.push("types");
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

    let config_path: &str = &cli.config
        .unwrap_or("~/.config/astrum/".to_string())
        .replace("~", home_dir().unwrap().to_str().unwrap());

    // check if config path exists, if not then make it
    if !Path::new(config_path).exists() {
        println!("config path does not exist, making one now");
        run_command(&format!("mkdir {}", config_path));

        let lua_file = File::create(config_path.to_owned() + "config.lua").expect("Failed to create config.lua");
    }

    // luarc file for the lua-language-server
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

    start_application(Path::new(config_path).join("config.lua"));
}
