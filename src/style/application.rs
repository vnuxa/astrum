use std::rc::Rc;

use super::from_colors;



pub fn lua_application_style(
    data: mlua::OwnedTable
) -> <cosmic::Theme as cosmic::iced_style::application::StyleSheet>::Style {

    cosmic::style::Application::Custom(
        Box::new(
            move |theme| {
                let table = data.to_ref();
                cosmic::iced::wayland::Appearance {
                    background_color: from_colors(table.clone().get::<_, mlua::Table>("background_color").expect("failed to get background color")),
                    icon_color: from_colors(table.clone().get::<_, mlua::Table>("icon_color").expect("failed to get icon color")),
                    text_color: from_colors(table.clone().get::<_, mlua::Table>("text_color").expect("failed to get text color")),
                }
            }
        )
    )
}
