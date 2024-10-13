use std::{borrow::Borrow, cell::RefCell, collections::HashSet, io::Read, path::{Path, PathBuf}, process::Command, rc::Rc, sync::{Arc, Mutex}};

use anyhow::{anyhow, Context, Ok, Result};
use mlua::{ErrorContext, Lua, Table, UserData, Value, Variadic};
use ::mpris::{LoopStatus, Player};

use crate::{animations::{animate_value, make_animation, set_anim, toggle_state_anim}, config::HOME_DIR, services::{applications::{self, launch_app}, calls::call_windows_socket, hyprland, mpris, niri, time::write_delay_call}, style::set_icon_theme};

pub fn get_or_create_module<'lua>
(
    lua: &'lua Lua,
    name: &str
) -> anyhow::Result<mlua::Table<'lua>> {
    let globals = lua.globals();

    let package: Table = globals.get("package")?;
    let loaded: Table = package.get("loaded")?;

    let module = loaded.get(name)?;
    match module {
        Value::Nil => { // no modulle found
            let module = lua.create_table()?;
            loaded.set(name, module.clone())?;
            Ok(module)
        },

        Value::Table(table) => Ok(table),

        other => anyhow::bail!(
            "cant register module {} as package.loaded {} is already set to a value of type {}",
            name,
            name,
            other.type_name(),
        ),
    }
}

// IMPORTANT: change the odcumentaiton from wezterm
// to custom!!!!!!!
//
/// Set up a lua context for executing some code.
/// The path to the directory containing the configuration is
/// passed in and is used to pre-set some global values in
/// the environment.
///
/// The `package.path` is configured to search the user's
/// wezterm specific config paths for lua modules, should
/// they choose to `require` additional code from their config.
///
/// A `wezterm` module is registered so that the script can
/// `require "wezterm"` and call into functions provided by
/// wezterm.  The wezterm module contains:
/// * `executable_dir` - the directory containing the wezterm
///   executable.  This is potentially useful for portable
///   installs on Windows.
/// * `config_dir` - the directory containing the wezterm
///   configuration.
/// * `log_error` - a function that logs to stderr (or the server
///   log file for daemonized wezterm).
/// * `target_triple` - the rust compilation target triple.
/// * `version` - the version of the running wezterm instance.
/// * `home_dir` - the path to the user's home directory
///
/// In addition to this, the lua standard library, except for
/// the `debug` module, is also available to the script.
pub fn make_lua_context(config_file: &Path) -> anyhow::Result<Lua> {
    let lua = Lua::new();

    let config_dir = config_file.parent().unwrap_or_else(|| Path::new("/")); // dont know waht do
    {
        let globals = lua.globals();
        // this table will be the `astrum` module in script
        // dont think i need t ocreate the module, the module needs to be linked though
        // let astrum_mod = get_or_create_module(&lua, "astrum")?;

        // used to get modules required within the lua config
        let astrum_utils = get_or_create_module(&lua, "astrum_utils")?;

        let package: Table = globals.get("package").unwrap();
        let package_path: String = package.get("path").unwrap();
        let mut path_array: Vec<String> = package_path.split(";").map(|s| s.to_owned()).collect();


        // i think this is for lua modules and stuff??
        // include it if something is broken
        //
        fn prefix_path(array: &mut Vec<String>, path: &Path) {
            array.insert(0, format!("{}/?.lua", path.display()));
            array.insert(1, format!("{}/?/init.lua", path.display()));
        }
        let current_dir = std::env::current_exe().unwrap()
            .parent().unwrap()
            .parent().unwrap()
            .parent().unwrap()
            .join("src").join("lua");
        // println!("executable dir: {}", current_dir.display());
        prefix_path(&mut path_array, &current_dir.as_path());
        prefix_path(&mut path_array, &current_dir.join("astrum").as_path());
        // prefix_path(&mut path_array, &current_dir.join("types").as_path());

        // prefix_path(&mut path_array, &HOME_DIR.join(".config").join("astrum"));
        prefix_path(&mut path_array, config_dir);

        // TODO: document external packages, say that they use luarocks --local
        // and that its onyl on luarocks_5.4 or something
        if HOME_DIR.join(".luarocks").exists() {
            prefix_path(&mut path_array, &HOME_DIR.join(".luarocks").join("share").join("lua").join("5.4"));
        }



        let config_file_str = config_file
            .to_str()
            .ok_or_else(|| anyhow!("config file is not UTF-8"));

        // hook into loader and arrange to watch all required files.
        // see lua 5.4 docs on globals > package.searchers

        // TODO: add "add_to_config_reload_watchlist(name)" thing
        // lua.load(
        //     r#"
        //         local orig = package.searchers[2]
        //         package.searchers[2] = function(module)
        //             local name, err = package.searchpath(module, package.path)
        //             --if name then
        //                 --package.loaded.add_to_config_reload_watchlist(name)
        //             --end
        //             return orig(module)
        //         end
        //     "#
        // )
        // .set_name("=searcher")
        // .eval()?;


        astrum_utils.set(
            "toggle_window_call",
            lua.create_function(|_, window_name: mlua::String| {
                call_windows_socket(window_name.to_str().unwrap().to_string());
                std::result::Result::Ok(())
            })?
        )?;
        astrum_utils.set(
            "hyprland_set_workspace",
            lua.create_function(|_, workspace: i32| {
                hyprland::change_workspace(workspace);
                std::result::Result::Ok(())
            })?
        )?;
        astrum_utils.set(
            "hyprland_get_active",
            lua.create_function(|_, ()| {
                std::result::Result::Ok(hyprland::get_active_workspace())
            })?
        )?;

        // IMPORTANT: this does not work!!!
        //
        astrum_utils.set(
            "niri_get_active_workspace",
            lua.create_function(|_, ()| {
                std::result::Result::Ok(niri::niri_get_active())
            })?
        )?;
        astrum_utils.set(
            "niri_set_workspace",
            lua.create_function(|_, workspace: u64| {
                niri::niri_switch_to_workspace(workspace);
                std::result::Result::Ok(())
            })?
        )?;
        astrum_utils.set(
            "niri_focus_workspace_up",
            lua.create_function(|_, ()| {
                niri::niri_focus_workspace_up();
                std::result::Result::Ok(())
            })?
        )?;
        astrum_utils.set(
            "niri_focus_workspace_down",
            lua.create_function(|_, ()| {
                niri::niri_focus_workspace_down();
                std::result::Result::Ok(())
            })?
        )?;

        astrum_utils.set(
            "niri_focus_window",
            lua.create_function(|_, window: u64| {
                niri::niri_focus_window(window);
                std::result::Result::Ok(())
            })?
        )?;

        astrum_utils.set(
            "call_delayed",
            lua.create_function(|_, data: (mlua::Number, mlua::Value)| {
                write_delay_call(data.0, data.1);
                std::result::Result::Ok(())
            })?
        )?;

        astrum_utils.set(
            "mpris_get_player",
            lua.create_function(move |lua: &Lua, player_name: mlua::String| {
                // turn this into a table of functions
                let player_table = lua.create_table().unwrap();
                let player: Rc<Player> = Rc::new(mpris::get_player_by_name(player_name.to_str().unwrap().to_string()));

                let player_play = player.clone();
                let player_next = player.clone();
                let player_previous = player.clone();

                let player_volume = player.clone();
                let player_loop = player.clone();
                let player_shuffle = player.clone();

                let player_get_volume = player.clone();

                player_table.set(
                    "player_play_pause",
                    lua.create_function(move |_, ()| {
                        player_play.play_pause();
                        std::result::Result::Ok(())
                    })?
                )?;

                player_table.set(
                    "player_next",
                    lua.create_function(move |_, ()| {
                        player_next.next();
                        std::result::Result::Ok(())
                    })?
                )?;

                player_table.set(
                    "player_previous",
                    lua.create_function(move |_, ()| {
                        player_previous.previous();
                        std::result::Result::Ok(())
                    })?
                )?;

                player_table.set(
                    "player_volume",
                    lua.create_function(move |_, volume: mlua::Number| {
                        player_volume.set_volume(volume);
                        std::result::Result::Ok(())
                    })?
                )?;

                player_table.set(
                    "player_get_volume",
                    lua.create_function(move |_, ()| {
                        let volume = player_get_volume.get_volume().unwrap();
                        std::result::Result::Ok(volume)
                    })?
                )?;

                player_table.set(
                    "player_looping",
                    lua.create_function(move |_, status: mlua::String| {
                        player_loop.set_loop_status(match status.to_str().unwrap() {
                            "None" => LoopStatus::None,
                            "Track" => LoopStatus::Track,
                            "Playlist" => LoopStatus::Playlist,
                            _ => unimplemented!()
                        });
                        std::result::Result::Ok(())
                    })?
                )?;

                player_table.set(
                    "player_shuffle",
                    lua.create_function(move |_, shuffle: bool| {
                        player_shuffle.set_shuffle(shuffle);
                        std::result::Result::Ok(())
                    })?
                )?;


                // might have to add more like set_position_in_microseconds
                // but for me right now its useless
                // let me know if i should add more

                std::result::Result::Ok(player_table)
            })?
        )?;

        astrum_utils.set(
            "get_all_applications",
            lua.create_function(move |lua: &Lua, ()| {
                let signal_data = match lua.load(applications::get_all_apps().unwrap()).eval().expect("failed to evaluate signal_data") {
                    Value::Table(table_data) => Some(table_data),
                    _ => None,
                };
                std::result::Result::Ok(signal_data.unwrap())
            })?
        )?;

        astrum_utils.set(
            "launch_application",
            lua.create_function(|_, executable: mlua::String| {
                launch_app(executable.to_str().unwrap().to_string());
                std::result::Result::Ok(())
            })?
        )?;

        astrum_utils.set(
            "execute_command",
            lua.create_function(|lua: &mlua::Lua, arguments: Variadic<mlua::String>, | {

                let args_vec = arguments.to_vec();
                let command = Command::new("bash")
                    .arg("-c")
                    .args(args_vec.iter().map(|val| val.to_str().unwrap()))
                    .output()
                    .expect("failed to execute command");

                std::result::Result::Ok(
                    lua.create_string(command.stdout)
                )
            })?
        )?;

        astrum_utils.set(
            "make_animation",
            lua.create_function(|_, animation_settings: mlua::Table| {
                make_animation(animation_settings);
                std::result::Result::Ok(())
            })?
        )?;

        astrum_utils.set(
            "animate_value",
            lua.create_function(|_, data: mlua::Table| {
                std::result::Result::Ok(
                    animate_value(data.get("animation_id")?, data.get("from_value")?, data.get("to_value")?)
                )
            })?
        )?;

        astrum_utils.set(
            "set_animation",
            lua.create_function(|_, data: mlua::Table| {
                std::result::Result::Ok(
                    set_anim(
                        data.get("animation_id")?,
                        {
                            // println!("does data contain key {:?}", data.get::<_, bool>("value"));
                            if data.contains_key("value")? {
                                Some(data.get("value")?)
                            } else {
                                None
                            }
                        }
                    )
                )
            })?
        )?;

        astrum_utils.set(
            "change_anim_state",
            lua.create_function(|_, data: mlua::Table| {
                std::result::Result::Ok(
                    toggle_state_anim(
                        data.get("animation_id")?,
                        {
                            // println!("does data contain key {:?}", data.get::<_, bool>("value"));
                            if data.contains_key("value")? {
                                Some(data.get("value")?)
                            } else {
                                None
                            }
                        }
                    )
                )
            })?
        )?;
        astrum_utils.set(
            "set_icon_theme",
            lua.create_function(|_, icon_theme: mlua::String| {
                set_icon_theme(icon_theme);
                std::result::Result::Ok(())
            })?
        )?;

        // println!("package array!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        // println!("{}", path_array.join(";"));
        package
            .set("path", path_array.join(";"))?;
            // .context("errored when assigning package.path")?;

    }

    Ok(lua)
}


// struct LoadedConfig<'lua> {
//     config: mlua::Table<'lua>,
//     lua: Rc<mlua::Lua>,
// }

// pub fn try_load<'lua>(path_item: &Path) -> anyhow::Result<LoadedConfig<'lua>>  {
//     let mut file = std::fs::File::open(path_item).unwrap();
//
//
//     return Ok(LoadedConfig {
//         config: match config {
//             Value::Table(table) => Ok(table)?,
//             _ => anyhow::bail!("failed to read config, its not a table")
//         },
//         lua: lua.into(),
//     })
//
//
// }

// pub struct LuaManager {
//     // lua_state: Arc<Mutex<Lua>>
//     lua_state: Lua
// }
//
//
// impl LuaManager {
//     pub fn eval<T>(&self, chunk: &str) -> anyhow::Result<T>
//     where
//         T: Clone + UserData
//     {
//         let value = self.lua_state.load(chunk).eval::<T>()?;
//         Ok(value)
//     }
// }
