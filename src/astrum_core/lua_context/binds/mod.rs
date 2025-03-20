use std::process::Command;

use mlua::{Table, Variadic};

mod windows;
mod hyprland;
// mod niri;
// mod calls;
mod mpris;
mod animations;
mod applications;
mod greetd;
mod calls;
// mod animations;
// mod styling;

pub fn add_util_binds<'lua>(lua: &'lua mlua::Lua, mut astrum_utils: &'lua Table) -> anyhow::Result<()> {
    // add all of the different types of bindings
    // most of these relate to services
    windows::bind(lua, astrum_utils);
    hyprland::bind(lua, astrum_utils);
    // niri::bind(lua, astrum_utils);
    // calls::bind(lua, astrum_utils);
    mpris::bind(lua, astrum_utils);
    applications::bind(lua, astrum_utils);
    animations::bind(lua, astrum_utils);
    greetd::bind(lua, astrum_utils);
    calls::bind(lua, astrum_utils);
    // styling::bind(lua, astrum_utils);

    // other misc utils

    astrum_utils.set(
        "execute_command",
        lua.create_function(|lua: &mlua::Lua, arguments: Variadic<mlua::String> | {

            let args_vec = arguments.to_vec();
            let command = Command::new("bash")
                .arg("-c")
                .args(args_vec.iter().map(|val| val.to_string_lossy()))
                .output()
                .expect("failed to execute command");

            std::result::Result::Ok(
                lua.create_string(command.stdout)
            )
        }).unwrap()
    ).unwrap();

    Ok(())
}
