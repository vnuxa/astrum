
use cosmic::{config::COSMIC_TK, iced::{Color, Vector}, icon_theme::set_default};

pub mod application;
pub mod button;
pub mod text;
pub mod container;
pub mod text_input;


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

    Vector::new(x, y)
}

unsafe fn make_static_str<'a>(key: &'a str) -> &'static str {
    std::mem::transmute::<&'a str, &'static str>(key)
}

pub fn set_icon_theme(data: mlua::String) {
    COSMIC_TK.lock().unwrap().set_icon_theme(&cosmic::cosmic_config::Config::libcosmic().unwrap(), data.to_str().unwrap().to_string());
    unsafe {
        println!("setting icon theme! {}", data.to_str().unwrap());
        set_default(make_static_str(data.to_str().unwrap()));
    }
    println!("wow the icon theme is: {}",cosmic::config::icon_theme());
}
