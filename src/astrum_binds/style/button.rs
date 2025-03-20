use std::rc::Rc;

use cosmic::iced::{Background, Radius, Vector};

use super::{from_colors, get_vector};


fn get_appearance(data: mlua::Table) -> cosmic::widget::button::Style {
    let mut result = cosmic::widget::button::Style::new();
    // let mut result = cosmic::widget::button::Style {
    //     shadow_offset: Vector::default(),
    //     border_radius: Radius::from(5),
    //     border_width: 0.0,
    //     border_color: cosmic::iced::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
    //     outline_width: 0.0,
    //     outline_color: cosmic::iced::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
    //     background: None,
    //     overlay: None,
    //     text_color: None,
    //     icon_color: None,
    // };


    if let Ok(border_width) = data.get::<mlua::Number>("border_width") {
        result.border_width = border_width as f32;
    }
    if let Ok(border_radius) = data.get::<mlua::Number>("border_radius") {
        result.border_radius = Radius::from(border_radius as f32);
    } else if let Ok(radius) = data.get::<mlua::Table>("border_radius") {
        result.border_radius = Radius::from(
            [
                radius.get::<mlua::Number>(1).unwrap() as f32,
                radius.get::<mlua::Number>(2).unwrap() as f32,
                radius.get::<mlua::Number>(3).unwrap() as f32,
                radius.get::<mlua::Number>(4).unwrap() as f32,
            ]
        );
    }
    if let Ok(color) = data.get::<mlua::Table>("border_color")  {
        result.border_color = from_colors(color);
    }

    if let Ok(width) = data.get::<mlua::Number>("outline_width") {
        result.outline_width = width as f32;
    }
    if let Ok(color) = data.get::<mlua::Table>("outline_color") {
        result.outline_color= from_colors(color);
    }

    if let Ok(offset) = data.get::<mlua::Table>("shadow_offset")  {
        result.shadow_offset = get_vector(offset);
    }

    if let Ok(text_color) = data.get::<mlua::Table>("text_color") {
        result.text_color = Some(from_colors(text_color));
    }
    if let Ok(icon_color) = data.get::<mlua::Table>("icon_color") {
        result.icon_color = Some(from_colors(icon_color));
    }

    if let Ok(background) = data.get::<mlua::Table>("background") {
        // TODO: add gradient support
        result.background = Some(Background::Color(from_colors(background)));
    }
    if let Ok(overlay) = data.get::<mlua::Table>("overlay") {
        // TODO: add gradient support
        result.overlay = Some(Background::Color(from_colors(overlay)));
    }


    result
}

pub fn lua_button_style(data: mlua::Table) -> cosmic::theme::Button {

    let data = Rc::new(data);

    let active_clone = data.clone();
    let disabled_clone = data.clone();
    let hovered_clone = data.clone();
    let pressed_clone = data.clone();


    cosmic::theme::Button::Custom {
        active: Box::new(move |is_active: bool, theme| {
            if let Ok(active) = active_clone.get::<mlua::Table>("active") {
                get_appearance(active)
            } else {
                let active: mlua::Table = active_clone.get::<mlua::Table>("default").expect("Failed to get default appearance table");
                get_appearance(active)
            }
        }),
        disabled: Box::new(move |theme| {
            if let Ok(disabled) = disabled_clone.get::<mlua::Table>("disabled") {
                get_appearance(disabled)
            } else {
                let disabled: mlua::Table = disabled_clone.get::<mlua::Table>("default").expect("Failed to get default appearance table");
                get_appearance(disabled)
            }
        }),
        hovered: Box::new(move |is_hovered: bool, theme| {
            if let Ok(hovered) = hovered_clone.get::<mlua::Table>("hovered") {
                get_appearance(hovered)
            } else {
                let hovered: mlua::Table = hovered_clone.get::<mlua::Table>("default").expect("Failed to get default appearance table");
                get_appearance(hovered)
            }
        }),
        pressed: Box::new(move |is_pressed: bool, theme| {
            if let Ok(pressed) = pressed_clone.get::<mlua::Table>("pressed") {
                get_appearance(pressed)
            } else {
                let pressed: mlua::Table = pressed_clone.get::<mlua::Table>("default").expect("Failed to get default appearance table");
                get_appearance(pressed)
            }
        })

    }
}
