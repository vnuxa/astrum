use std::rc::Rc;

use cosmic::{iced::{Background, Radius, Vector}, widget::{button::Appearance, style::StyleSheet}, Theme};

use super::{from_colors, get_vector};


fn get_appearance(
    active: mlua::Table,
) -> Appearance {
    let mut result = Appearance {
        shadow_offset: Vector::default(),
        border_radius: Radius::from(5),
        border_width: 0.0,
        border_color: cosmic::iced::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
        outline_width: 0.0,
        outline_color: cosmic::iced::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
        // border_width: active.get::<_, mlua::Number>("border_width").expect("Expected border_width") as f32,
        // border_radius: {
        //     if let Ok(radius) = active.get::<_, mlua::Number>("border_radius") {
        //         Radius::from(radius as f32)
        //     } else {
        //         let radius = active.get::<_, mlua::Table>("border_radius").expect("Border radius is expected to be either a table or number");
        //         Radius::from(
        //             [
        //                 radius.get::<_, mlua::Number>(1).unwrap() as f32,
        //                 radius.get::<_, mlua::Number>(2).unwrap() as f32,
        //                 radius.get::<_, mlua::Number>(3).unwrap() as f32,
        //                 radius.get::<_, mlua::Number>(4).unwrap() as f32,
        //             ]
        //         )
        //     }
        //
        // },
        // border_color: from_colors(active.get::<_, mlua::Table>("border_color").expect("Expected border_color")),
        // outline_width: active.get::<_, mlua::Number>("outline_width").expect("Expected outline_width") as f32,
        // outline_color: from_colors(active.get::<_, mlua::Table>("outline_color").expect("Expected outline_color")),
        // shadow_offset: get_vector(active.get::<_, mlua::Table>("shadow_offset").expect("Expected vector")),
        background: None,
        overlay: None,
        text_color: None,
        icon_color: None,
    };
    if let Ok(border_width) = active.get::<_, mlua::Number>("border_width") {
        result.border_width = border_width as f32;
    }
    if let Ok(border_radius) = active.get::<_, mlua::Number>("border_radius") {
        result.border_radius = Radius::from(border_radius as f32);
    } else if let Ok(radius) = active.get::<_, mlua::Table>("border_radius") {
        result.border_radius = Radius::from(
            [
                radius.get::<_, mlua::Number>(1).unwrap() as f32,
                radius.get::<_, mlua::Number>(2).unwrap() as f32,
                radius.get::<_, mlua::Number>(3).unwrap() as f32,
                radius.get::<_, mlua::Number>(4).unwrap() as f32,
            ]
        );
    }
    if let Ok(color) = active.get::<_, mlua::Table>("border_color")  {
        result.border_color = from_colors(color);
    }

    if let Ok(width) = active.get::<_, mlua::Number>("outline_width") {
        result.outline_width = width as f32;
    }
    if let Ok(color) = active.get::<_, mlua::Table>("outline_color") {
        result.outline_color= from_colors(color);
    }

    if let Ok(offset) = active.get::<_, mlua::Table>("shadow_offset")  {
        result.shadow_offset = get_vector(offset);
    }

    if let Ok(text_color) = active.get::<_, mlua::Table>("text_color") {
        result.text_color = Some(from_colors(text_color));
    }
    if let Ok(icon_color) = active.get::<_, mlua::Table>("icon_color") {
        result.icon_color = Some(from_colors(icon_color));
    }

    if let Ok(background) = active.get::<_, mlua::Table>("background") {
        // TODO: add gradient support
        result.background = Some(Background::Color(from_colors(background)));
    }
    if let Ok(overlay) = active.get::<_, mlua::Table>("overlay") {
        // TODO: add gradient support
        result.overlay = Some(Background::Color(from_colors(overlay)));
    }

    return result;
}

pub fn lua_button_style(
    data: mlua::OwnedTable
) -> cosmic::theme::style::Button {

    let data = Rc::new(data);

    let active_clone = data.clone();
    let disabled_clone = data.clone();
    let hovered_clone = data.clone();
    let pressed_clone = data.clone();
    cosmic::theme::style::Button::Custom {
        active: Box::new(
            move |is_active: bool, theme| {
                let table = active_clone.to_ref();
                if let Ok(active) = table.get::<_, mlua::Table>("active") {
                    get_appearance(active)
                } else {
                    let active: mlua::Table = table.get::<_, mlua::Table>("default").expect("Failed to get default appearance table");
                    get_appearance(active)
                }

            }
        ),
        disabled: Box::new(
            move |theme| {
                let table = disabled_clone.to_ref();
                if let Ok(disabled) = table.get::<_, mlua::Table>("disabled") {
                    get_appearance(disabled)
                } else {
                    let disabled: mlua::Table = table.get::<_, mlua::Table>("default").expect("Failed to get default appearance table");
                    get_appearance(disabled)
                }

            }
        ),

        hovered: Box::new(
            move |is_hovered: bool, theme| {
                let table = hovered_clone.to_ref();
                if let Ok(hovered) = table.get::<_, mlua::Table>("hovered") {
                    get_appearance(hovered)
                } else {
                    let hovered: mlua::Table = table.get::<_, mlua::Table>("default").expect("Failed to get default appearance table");
                    get_appearance(hovered)
                }
            }
        ),
        pressed: Box::new(
            move |is_pressed: bool, theme| {
                let table = pressed_clone.to_ref();
                if let Ok(pressed) = table.get::<_, mlua::Table>("pressed") {
                    get_appearance(pressed)
                } else {
                    let pressed: mlua::Table = table.get::<_, mlua::Table>("default").expect("Failed to get default appearance table");
                    get_appearance(pressed)
                }
            }
        ),


    }
}
