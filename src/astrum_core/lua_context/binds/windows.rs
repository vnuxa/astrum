use mlua::Table;

pub fn bind<'lua>(lua: &'lua mlua::Lua, mut astrum_utils: &'lua Table ) -> anyhow::Result<()> {
    astrum_utils.set(
        "toggle_window_call",
        lua.create_function(|_, window_name: mlua::String| {
            call_windows_socket(window_name.to_str().unwrap().to_string());
            std::result::Result::Ok(())
        })?
    )?;

    Ok(())
}
