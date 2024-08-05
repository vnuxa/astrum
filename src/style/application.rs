use super::from_colors;



pub fn lua_application_style(
    data: mlua::Table
) -> cosmic::iced::wayland::Appearance {
    cosmic::iced::wayland::Appearance {
        background_color: from_colors(data.get::<_, mlua::Table>("background_color").expect("failed to get background color")),
        icon_color: from_colors(data.get::<_, mlua::Table>("icon_color").expect("failed to get icon color")),
        text_color: from_colors(data.get::<_, mlua::Table>("text_color").expect("failed to get text color")),
    }
}
