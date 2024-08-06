use std::rc::Rc;

use cosmic::{iced::{Background, Radius}, widget::button::Appearance};

use super::{from_colors, get_vector};


fn get_appearance(
    active: mlua::Table
) -> Appearance {
    let mut result = Appearance {
        border_width: active.get::<_, mlua::Number>("border_width").expect("Expected border_width") as f32,
        border_radius: {
            if let Ok(radius) = active.get::<_, mlua::Number>("border_radius") {
                Radius::from(radius as f32)
            } else {
                let radius = active.get::<_, mlua::Table>("border_radius").expect("Border radius is expected to be either a table or number");
                Radius::from(
                    [
                        radius.get::<_, mlua::Number>(1).unwrap() as f32,
                        radius.get::<_, mlua::Number>(2).unwrap() as f32,
                        radius.get::<_, mlua::Number>(3).unwrap() as f32,
                        radius.get::<_, mlua::Number>(4).unwrap() as f32,
                    ]
                )
            }

        },
        border_color: from_colors(active.get::<_, mlua::Table>("border_color").expect("Expected border_color")),
        outline_width: active.get::<_, mlua::Number>("outline_width").expect("Expected outline_width") as f32,
        outline_color: from_colors(active.get::<_, mlua::Table>("outline_color").expect("Expected outline_color")),
        shadow_offset: get_vector(active.get::<_, mlua::Table>("shadow_offset").expect("Expected vector")),
        background: None,
        overlay: None,
        text_color: None,
        icon_color: None,
    };

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
                let active: mlua::Table = table.get::<_, mlua::Table>("active").expect("failed to get active appearance table");
                // let active: mlua::Table = table.get::<_, mlua::Function>("active").unwrap().call(is_active).expect("failed to get active function");

                get_appearance(active)
            }
        ),
        disabled: Box::new(
            move |theme| {
                let table = disabled_clone.to_ref();
                let disabled: mlua::Table = table.get::<_, mlua::Table>("disabled").expect("failed to get disabled appearance table");

                get_appearance(disabled)
            }
        ),

        hovered: Box::new(
            move |is_hovered: bool, theme| {
                let table = hovered_clone.to_ref();
                let hovered: mlua::Table = table.get::<_, mlua::Table>("hovered").expect("failed to get hovered appearance table");
                // let hovered: mlua::Table = table.get::<_, mlua::Function>("hovered").unwrap().call(is_hovered).expect("failed to get hovered function");

                get_appearance(hovered)
            }
        ),
        pressed: Box::new(
            move |is_pressed: bool, theme| {
                let table = pressed_clone.to_ref();
                let pressed: mlua::Table = table.get::<_, mlua::Table>("pressed").expect("failed to get pressed appearance table");
                // let pressed: mlua::Table = table.get::<_, mlua::Function>("pressed").unwrap().call(is_pressed).expect("failed to get pressed function");

                get_appearance(pressed)
            }
        ),


    }
}
