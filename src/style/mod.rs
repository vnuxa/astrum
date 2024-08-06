
use cosmic::iced::{Color, Vector};

pub mod application;
pub mod button;
pub mod text;


pub fn from_colors(
    data: mlua::Table
) -> Color {
    let red_color: mlua::Number = data.get::<&str, mlua::Number>("red").unwrap();
    let green_color: mlua::Number = data.get::<&str, mlua::Number>("green").unwrap();
    let blue_color: mlua::Number = data.get::<&str, mlua::Number>("blue").unwrap();
    let alpha: mlua::Number = data.get::<&str, mlua::Number>("alpha").unwrap();

    cosmic::iced::Color::from_rgba8(red_color as u8, green_color as u8, blue_color as u8, alpha as f32)
}

pub fn get_vector(
    data: mlua::Table
) -> Vector {
    let x: f32 = data.get::<_, mlua::Number>("x").unwrap() as f32;
    let y: f32 = data.get::<_, mlua::Number>("y").unwrap() as f32;

    Vector { x, y }
}
