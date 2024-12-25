
use mlua::Table;

use crate::astrum_core::services::hyprland;

pub fn bind<'lua>(lua: &'lua mlua::Lua, mut astrum_utils: &'lua Table ) -> anyhow::Result<()> {
    astrum_utils.set(
        "hyprland_set_workspace",
        lua.create_function(|_, workspace: i32| {
            hyprland::change_workspace(workspace);
            std::result::Result::Ok(())
        })?
    )?;
    astrum_utils.set(
        "hyprland_get_active_workspace",
        lua.create_function(|_, ()| {
            std::result::Result::Ok(hyprland::get_active_workspace())
        })?
    )?;
    astrum_utils.set(
        "hyprland_get_active_client",
        lua.create_function(|_, ()| {
            std::result::Result::Ok(hyprland::get_active_client())
        })?
    )?;

    Ok(())
}
