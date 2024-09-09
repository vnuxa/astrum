
use std::{borrow::{Borrow, BorrowMut}, cell::RefCell, collections::HashMap, env::{self}, fmt::format, fs::File, io::{self, Read, Write}, ops::Deref, path::{Path, PathBuf}};

use mlua::{Lua, OwnedFunction, OwnedTable, Value};
use simple_home_dir::home_dir;
use std::fs;
use std::os::unix::net::UnixStream;

use crate::{config::lua::make_lua_context, services::{hyprland::{self, HyprlandListener}}, widgets::process_lua_element};
use clap::Parser;
use std::process::Command;

mod config;
mod app;
mod utils;
mod widgets;
mod services;
mod style;
mod animations;



// TODO: maybe make a argument for creating .neoconf.json file
// though i dont know if its needed, depends on if i can make one path work with nixos
// maybe make it so that it always updates the .neoconf.json file every time you run astrum


#[derive(Parser)]
struct Cli {
    /// Path to the config folder. Default is ~/.config/astrum/
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
    dir.push("lua/");
    dir.push("astrum/");
    dir.push("types");
    // dir.push("lib")/* ; */
    Ok(dir)
}


// IMPORTANT: tell the users that this project uses neoconf.nvim in order to make types work
// and in documentation have it so  that its documented that types work like this
// `---@type Astrum`
// or `---@cast custom_astrum_name Astrum`
// maybe even direct them to the lua_ls documentation

// TODO: use clap in order ot mkae a proper cli thingy
fn main() {
    let cli = Cli::parse();
    let config_path: &str = &cli.config
        .unwrap_or("~/.config/astrum/".to_string())
        .replace("~", home_dir().unwrap().to_str().unwrap());
    // println!("config path {:?}", config_path);

    make_cache(vec![
        "sockets"
    ]);
    if cli.call.is_some() {
        let socket = format!("{}/.cache/astrum/sockets/calls", home_dir().unwrap().display());
        let mut unix_stream = UnixStream::connect(socket).expect("Could not create stream");

        unix_stream
            .write_all(cli.call.clone().unwrap().as_bytes())
            .expect("failed at writing calls to the listener");

        // for socket in fs::read_dir(format!("{}/.cache/astrum/sockets/", home_dir().unwrap().display())).unwrap().flatten()
        //
        //     // Path::new("./")).unwrap().flatten()
        // {
        //     println!("Name: {}", socket.path().display());
        //
        //     let mut unix_stream =
        //         UnixStream::connect(socket.path()).expect("Could not create stream");
        //
        //     unix_stream
        //         .write_all(cli.call.clone().unwrap().as_bytes())
        //         .expect("failed at writing calls to listeners");
        //
        // }
        return
    }

    let neoconf: String = format!("
        {{
            \"lspconfig\": {{
                \"lua_ls\": {{
                    \"Lua.workspace.library\": [
                        \"{types_path}\"
                    ]
                }}
            }}
        }}",
        types_path = type_path().unwrap().display()
    );
    // TODO: make this update every single run
    // let mut file = File::create(config_path.to_owned() + ".neoconf.json").expect("Failed to create .neoconf.json");
    // file.write_fmt(format_args!(
    //     "
    //         {{
    //             \"lspconfig\": {{
    //                 \"lua_ls\": {{
    //                     \"Lua.workspace.library\": [
    //                         \"{types_path}\"
    //                     ]
    //                 }}
    //             }}
    //         }}
    //     ",
    //     types_path = type_path().unwrap().display()
    // ));

    // luarc for the lua-language-server
    let mut file = File::create(config_path.to_owned() + ".luarc.json").expect("Failed to create .luarc.json");
    file.write_fmt(format_args!(
        "
            {{
                \"$schema\": \"https://raw.githubusercontent.com/LuaLS/vscode-lua/master/setting/schema.json\",
                \"workspace.library\": [
                    \"{types_path}\"
                ],

                \"runtime.version\": \"Lua 5.1\",
            }}
        ",
        types_path = type_path().unwrap().display()
    ));
    if !Path::new(config_path).exists() {
        println!("config path doesnt exist");
        run_command(&format!("mkdir {}", config_path));
        let lua_file = File::create(config_path.to_owned() + "config.lua").expect("Failed to create config.lua");

        println!("types path: {}", type_path().unwrap().display())

    }

    app::create_app(Path::new(config_path).join("config.lua"));

}
