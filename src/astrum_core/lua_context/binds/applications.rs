use mlua::{Lua, Table, Value};

use crate::astrum_core::services::applications::{get_all_apps, launch_app};

pub fn bind<'lua>(lua: &'lua mlua::Lua, mut astrum_utils: &'lua Table ) -> anyhow::Result<()> {
    astrum_utils.set(
        "get_all_applications",
        lua.create_function(move |lua: &Lua, ()| {
            let signal_data = match lua.load(get_all_apps().unwrap()).eval().expect("failed to evaluate signal_data") {
                Value::Table(table_data) => Some(table_data),
                _ => None,
            };
            std::result::Result::Ok(signal_data.unwrap())
        }).unwrap()
    ).unwrap();

    astrum_utils.set(
        "launch_application",
        lua.create_function(|_, executable: mlua::String| {
            launch_app(executable.to_string_lossy());
            std::result::Result::Ok(())
        }).unwrap()
    ).unwrap();

    Ok(())
}
