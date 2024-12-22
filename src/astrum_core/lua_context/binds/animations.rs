use mlua::{Lua, Table};

use crate::astrum_core::animations::{animate_value, make_animation, set_anim, toggle_state_anim};

pub fn bind<'lua>(lua: &'lua mlua::Lua, mut astrum_utils: &'lua Table ) -> anyhow::Result<()> {
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

    Ok(())
}
