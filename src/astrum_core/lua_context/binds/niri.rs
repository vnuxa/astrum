use mlua::Table;

pub fn bind<'lua>(lua: &'lua mlua::Lua, mut astrum_utils: &'lua Table ) -> anyhow::Result<()> {
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

    Ok(())
}
