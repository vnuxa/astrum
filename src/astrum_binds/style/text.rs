use super::from_colors;



pub fn lua_text_style(data: mlua::Table) -> cosmic::style::Text {
    cosmic::style::Text::Color(from_colors(data.get::<mlua::Table>("text_color").unwrap()))
}
