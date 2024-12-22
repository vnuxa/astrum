use std::rc::Rc;

use mlua::{Lua, Table};
use mpris::{LoopStatus, Player};

use crate::astrum_core::services::mpris::get_player_by_name;


pub fn bind<'lua>(lua: &'lua mlua::Lua, mut astrum_utils: &'lua Table ) -> anyhow::Result<()> {
    astrum_utils.set(
        "mpris_get_player",
        lua.create_function(move |lua: &Lua, player_name: mlua::String| {
            // turn this into a table of functions
            let player_table = lua.create_table().unwrap();
            let player: Rc<Player> = Rc::new(get_player_by_name(player_name.to_str().unwrap().to_string()));

            let player_play = player.clone();
            let player_next = player.clone();
            let player_previous = player.clone();

            let player_volume = player.clone();
            let player_loop = player.clone();
            let player_shuffle = player.clone();

            let player_get_volume = player.clone();

            player_table.set(
                "player_play_pause",
                lua.create_function(move |_, ()| {
                    player_play.play_pause();
                    std::result::Result::Ok(())
                })?
            )?;

            player_table.set(
                "player_next",
                lua.create_function(move |_, ()| {
                    player_next.next();
                    std::result::Result::Ok(())
                })?
            )?;

            player_table.set(
                "player_previous",
                lua.create_function(move |_, ()| {
                    player_previous.previous();
                    std::result::Result::Ok(())
                })?
            )?;

            player_table.set(
                "player_volume",
                lua.create_function(move |_, volume: mlua::Number| {
                    player_volume.set_volume(volume);
                    std::result::Result::Ok(())
                })?
            )?;

            player_table.set(
                "player_get_volume",
                lua.create_function(move |_, ()| {
                    let volume = player_get_volume.get_volume().unwrap();
                    std::result::Result::Ok(volume)
                })?
            )?;

            player_table.set(
                "player_looping",
                lua.create_function(move |_, status: mlua::String| {
                    player_loop.set_loop_status(match status.to_str().unwrap() {
                        "None" => LoopStatus::None,
                        "Track" => LoopStatus::Track,
                        "Playlist" => LoopStatus::Playlist,
                        _ => unimplemented!()
                    });
                    std::result::Result::Ok(())
                })?
            )?;

            player_table.set(
                "player_shuffle",
                lua.create_function(move |_, shuffle: bool| {
                    player_shuffle.set_shuffle(shuffle);
                    std::result::Result::Ok(())
                })?
            )?;


            // might have to add more like set_position_in_microseconds
            // but for me right now its useless
            // let me know if i should add more

            std::result::Result::Ok(player_table)
        })?
    )?;

    Ok(())
}
