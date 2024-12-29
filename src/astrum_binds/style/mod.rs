use cosmic::{config::COSMIC_TK,  iced::Vector};
use log::{debug, info};

use crate::astrum_binds::widgets::make_static_str;



pub fn from_colors(
    data: mlua::Table
) -> cosmic::iced_winit::graphics::core::Color {
    let red_color: mlua::Number = data.get::<_, mlua::Number>("red").unwrap();
    let green_color: mlua::Number = data.get::<_, mlua::Number>("green").unwrap();
    let blue_color: mlua::Number = data.get::<_, mlua::Number>("blue").unwrap();
    let alpha: mlua::Number = data.get::<_, mlua::Number>("alpha").unwrap_or(1 as mlua::Number);

    cosmic::iced_winit::graphics::core::Color::from_rgba8(red_color as u8, green_color as u8, blue_color as u8, alpha as f32)
}

pub fn get_vector(
    data: mlua::Table
) -> Vector {
    let x: f32 = data.get::<_, mlua::Number>("x").unwrap() as f32;
    let y: f32 = data.get::<_, mlua::Number>("y").unwrap() as f32;

    Vector::new(x, y)
}

pub fn set_icon_theme(data: mlua::String) {
    unsafe {
        debug!("setting icon theme! {}", data.to_str().unwrap());
        cosmic::icon_theme::set_default(make_static_str(data.to_str().unwrap()));
    }
    debug!("wow the icon theme is: {}",cosmic::config::icon_theme());
}
