use cosmic::{config::COSMIC_TK,  iced::Vector};
use log::{debug, info};

use crate::astrum_binds::widgets::make_static_str;

pub mod text;
pub mod container;
pub mod button;
pub mod text_input;


pub fn from_colors(
    data: mlua::Table
) -> cosmic::iced_winit::graphics::core::Color {
    let red_color: mlua::Number = data.get::<mlua::Number>("red").unwrap();
    let green_color: mlua::Number = data.get::<mlua::Number>("green").unwrap();
    let blue_color: mlua::Number = data.get::<mlua::Number>("blue").unwrap();
    let alpha: mlua::Number = data.get::<mlua::Number>("alpha").unwrap_or(1 as mlua::Number);

    cosmic::iced_winit::graphics::core::Color::from_rgba8(red_color as u8, green_color as u8, blue_color as u8, alpha as f32)
}

pub fn get_vector(
    data: mlua::Table
) -> Vector {
    let x: f32 = data.get::<mlua::Number>("x").unwrap() as f32;
    let y: f32 = data.get::<mlua::Number>("y").unwrap() as f32;

    Vector::new(x, y)
}

pub fn set_icon_theme(data: mlua::String) {
    unsafe {
        debug!("setting icon theme! {}", data.to_str().unwrap());
        cosmic::icon_theme::set_default(make_static_str(&data.to_string_lossy()));
    }
    debug!("wow the icon theme is: {}",cosmic::config::icon_theme());
}
