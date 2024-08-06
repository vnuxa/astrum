use std::rc::Rc;

use super::from_colors;



pub fn lua_text_style(
    data: mlua::OwnedTable
) -> cosmic::style::Text {

    let table = data.to_ref();
    cosmic::style::Text::Color(from_colors(table.get::<_, mlua::Table>("text_color").expect("Failed to get text color")))
        // Box::new(
        //     move |theme| {
        //         let table = data.to_ref();
        //         cosmic::iced_style::{
        //             background_color: from_colors(table.clone().get::<_, mlua::Table>("background_color").expect("failed to get background color")),
        //             icon_color: from_colors(table.clone().get::<_, mlua::Table>("icon_color").expect("failed to get icon color")),
        //             text_color: from_colors(table.clone().get::<_, mlua::Table>("text_color").expect("failed to get text color")),
        //         }
        //     }
        // )
    // )
}
