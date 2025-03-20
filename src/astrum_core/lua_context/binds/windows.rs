use mlua::Table;

use crate::astrum_core::services::calls::call_windows_socket;

pub fn bind<'lua>(lua: &'lua mlua::Lua, mut astrum_utils: &'lua Table ) -> anyhow::Result<()> {
    astrum_utils.set(
        "toggle_window_call",
        lua.create_function(|_, window_name: mlua::String| {
            call_windows_socket(window_name.to_string_lossy());
            std::result::Result::Ok(())
        }).unwrap()
    ).unwrap();

    Ok(())
}
