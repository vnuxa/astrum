use std::rc::Rc;

use cosmic::prelude::ColorExt;
use cosmic::widget::text_input::Appearance;

use cosmic::iced::{Background, Color, Radius};

use super::{from_colors, get_vector};


fn get_appearance(
    table: mlua::Table,
    pallete: &cosmic::cosmic_theme::Theme,
    container: &cosmic::cosmic_theme::Container,
) -> cosmic::widget::text_input::Appearance {
    Appearance {
        background:
            if let Ok(background) = table.get::<mlua::Table>("background") {
                // TODO: add gradient support
                Background::Color(from_colors(background))
            } else {
                Background::Color(Color::from_rgb( 0.0, 0.0, 0.0 ))
            },
        border_radius:
            if let Ok(border_radius) = table.get::<mlua::Number>("border_radius") {
                Radius::from(border_radius as f32)
            } else if let Ok(radius) = table.get::<mlua::Table>("border_radius") {
                Radius::from(
                    [
                        radius.get::<mlua::Number>(1).unwrap() as f32,
                        radius.get::<mlua::Number>(2).unwrap() as f32,
                        radius.get::<mlua::Number>(3).unwrap() as f32,
                        radius.get::<mlua::Number>(4).unwrap() as f32,
                    ]
                )
            } else {
                pallete.corner_radii.radius_s.into()
            },
        border_width:
            if let Ok(border_width) = table.get::<mlua::Number>("border_width") {
                border_width as f32
            } else {
                2.0
            },
        border_color:
            if let Ok(color) = table.get::<mlua::Table>("border_color")  {
                from_colors(color)
            } else {
                container.component.divider.into()
            },
        border_offset:
            if let Ok(offfset) = table.get::<mlua::Number>("border_offset") {
                Some(offfset as f32)
            } else {
                None
            },
        label_color:
            if let Ok(label_color) = table.get::<mlua::Table>("label_color") {
                from_colors(label_color)
            } else {
                pallete.palette.neutral_9.into()
            },
        icon_color:
            if let Ok(icon_color) = table.get::<mlua::Table>("icon_color") {
               Some(from_colors(icon_color))
            } else {
                None
            },
        text_color:
            if let Ok(text_color) = table.get::<mlua::Table>("text_color") {
                Some(from_colors(text_color))
            } else {
                None
            },
        placeholder_color:
            if let Ok(placeholder_color) = table.get::<mlua::Table>("placeholder_color") {
                from_colors(placeholder_color)
            } else {
                let mut background: Color = container.component.base.into();
                background.a = 0.25;
                let color: Color = container.on.into();
                color.blend_alpha(background, 0.7)
            },
        selected_text_color:
            if let Ok(selected_text_color) = table.get::<mlua::Table>("selected_text_color") {
                from_colors(selected_text_color)
            } else {
                pallete.on_accent_color().into()
            },
        selected_fill:
            if let Ok(selected_fill) = table.get::<mlua::Table>("selected_fill") {
                from_colors(selected_fill)
            } else {
                pallete.accent_color().into()
            }
    }
}

pub fn lua_text_input_style(table: mlua::Table) -> cosmic::theme::TextInput {

    let data = Rc::new(table);

    let active_clone = data.clone();
    let error_clone = data.clone();
    let hovered_clone = data.clone();
    let focused_clone = data.clone();
    let disabled_clone = data.clone();

    cosmic::theme::TextInput::Custom {
        active: Box::new(move |theme| {
            if let Ok(active) = active_clone.get::<mlua::Table>("active") {
                get_appearance(active, theme.cosmic(), theme.current_container())
            } else {
                let active: mlua::Table = active_clone.get::<mlua::Table>("default").expect("Failed to get default appearance table");
                get_appearance(active, theme.cosmic(), theme.current_container())
            }
        }),
        error: Box::new(move |theme| {
            if let Ok(error) = error_clone.get::<mlua::Table>("error") {
                get_appearance(error, theme.cosmic(), theme.current_container())
            } else {
                let error: mlua::Table = error_clone.get::<mlua::Table>("default").expect("Failed to get default appearance table");
                get_appearance(error, theme.cosmic(), theme.current_container())
            }
        }),
        hovered: Box::new(move |theme| {
            if let Ok(hovered) = hovered_clone.get::<mlua::Table>("hovered") {
                get_appearance(hovered, theme.cosmic(), theme.current_container())
            } else {
                let hovered: mlua::Table = hovered_clone.get::<mlua::Table>("default").expect("Failed to get default appearance table");
                get_appearance(hovered, theme.cosmic(), theme.current_container())
            }
        }),
        focused: Box::new(move |theme| {
            if let Ok(focused) = focused_clone.get::<mlua::Table>("focused") {
                get_appearance(focused, theme.cosmic(), theme.current_container())
            } else {
                let focused: mlua::Table = focused_clone.get::<mlua::Table>("default").expect("Failed to get default appearance table");
                get_appearance(focused, theme.cosmic(), theme.current_container())
            }
        }),
        disabled: Box::new(move |theme| {
            if let Ok(disabled) = disabled_clone.get::<mlua::Table>("disabled") {
                get_appearance(disabled, theme.cosmic(), theme.current_container())
            } else {
                let disabled: mlua::Table = disabled_clone.get::<mlua::Table>("default").expect("Failed to get default appearance table");
                get_appearance(disabled, theme.cosmic(), theme.current_container())
            }
        }),

    }
}
