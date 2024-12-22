use mlua::Table;

pub fn bind<'lua>(lua: &'lua mlua::Lua, mut astrum_utils: &'lua Table ) -> anyhow::Result<()> {
    astrum_utils.set(
        "call_delayed",
        lua.create_function(|_, data: (mlua::Number, mlua::Value)| {
            write_delay_call(data.0, data.1);
            std::result::Result::Ok(())
        })?
    )?;

    Ok(())
}
