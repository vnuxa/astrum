use mlua::{Lua, Table, Value};

use crate::astrum_core::services::greetd::greetd_log_in;


pub fn bind<'lua>(lua: &'lua mlua::Lua, mut astrum_utils: &'lua Table ) -> anyhow::Result<()> {
    astrum_utils.set(
        "greetd_login",
        lua.create_function(move |_, (username, attempt, command): (mlua::String, mlua::String, mlua::String)| {
            let output = greetd_log_in(username.to_string_lossy(), attempt.to_string_lossy(), command.to_string_lossy());
            std::result::Result::Ok(output)
        }).unwrap()
    ).unwrap();

    Ok(())
}
