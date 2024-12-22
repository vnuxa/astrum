use mlua::{Lua, Table};

pub fn bind<'lua>(lua: &'lua mlua::Lua, mut astrum_utils: &'lua Table ) -> anyhow::Result<()> {
    astrum_utils.set(
        "set_icon_theme",
        lua.create_function(|_, icon_theme: mlua::String| {
            set_icon_theme(icon_theme);
            std::result::Result::Ok(())
        })?
    )?;

    Ok(())
}
