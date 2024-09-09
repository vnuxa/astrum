use std::rc::Rc;

use cosmic::{cosmic_theme::palette::cast::ComponentsInto, iced::{Background, Color, Radius}, prelude::ColorExt, widget::{style::StyleSheet, text_input::Appearance}};

use super::from_colors;


fn get_appearance(
    table: mlua::Table,
    pallete: &cosmic::cosmic_theme::Theme,
    container: &cosmic::cosmic_theme::Container,
) -> Appearance {

    let mut background: Color = container.component.base.into();
    background.a = 0.25;

    let mut result = Appearance {
        background: Background::Color(Color::from_rgb( 0.0, 0.0, 0.0 )),
        border_radius: pallete.corner_radii.radius_s.into(),
        border_width: 2.0,
        border_color: container.component.divider.into(),
        border_offset: None,
        label_color: pallete.palette.neutral_9.into(),
        placeholder_color: {
            let color: Color = container.on.into();
            color.blend_alpha(background, 0.7)
        },
        selected_text_color: pallete.on_accent_color().into(),
        selected_fill: pallete.accent_color().into(),
        icon_color: None,
        text_color: None,
    };

    if let Ok(border_width) = table.get::<_, mlua::Number>("border_width") {
        result.border_width = border_width as f32;
    }

    if let Ok(border_radius) = table.get::<_, mlua::Number>("border_radius") {
        result.border_radius = Radius::from(border_radius as f32);
    } else if let Ok(radius) = table.get::<_, mlua::Table>("border_radius") {
        result.border_radius = Radius::from(
            [
                radius.get::<_, mlua::Number>(1).unwrap() as f32,
                radius.get::<_, mlua::Number>(2).unwrap() as f32,
                radius.get::<_, mlua::Number>(3).unwrap() as f32,
                radius.get::<_, mlua::Number>(4).unwrap() as f32,
            ]
        );
    }
    if let Ok(color) = table.get::<_, mlua::Table>("border_color")  {
        result.border_color = from_colors(color);
    }
    if let Ok(offfset) = table.get::<_, mlua::Number>("border_offset") {
        result.border_offset = Some(offfset as f32);
    }

    if let Ok(text_color) = table.get::<_, mlua::Table>("text_color") {
        result.text_color = Some(from_colors(text_color));
    }
    if let Ok(icon_color) = table.get::<_, mlua::Table>("icon_color") {
        result.icon_color = Some(from_colors(icon_color));
    }
    if let Ok(label_color) = table.get::<_, mlua::Table>("label_color") {
        result.label_color = from_colors(label_color);
    }
    if let Ok(placeholder_color) = table.get::<_, mlua::Table>("placeholder_color") {
        result.placeholder_color = from_colors(placeholder_color);
    }
    if let Ok(selected_text_color) = table.get::<_, mlua::Table>("selected_text_color") {
        result.selected_text_color = from_colors(selected_text_color);
    }
    if let Ok(selected_fill) = table.get::<_, mlua::Table>("selected_fill") {
        result.selected_fill = from_colors(selected_fill);
    }

    if let Ok(background) = table.get::<_, mlua::Table>("background") {
        // TODO: add gradient support
        result.background = Background::Color(from_colors(background));
    }


    result
}
pub fn lua_text_input_style(
    data: mlua::OwnedTable
) -> cosmic::style::TextInput {
    let data = Rc::new(data);

    let active_clone = data.clone();
    let error_clone = data.clone();
    let hovered_clone = data.clone();
    let focused_clone = data.clone();
    let disabled_clone = data.clone();

    cosmic::style::TextInput::Custom {
        active: Box::new(move |theme: &cosmic::Theme| {
            let table = active_clone.to_ref();
            if let Ok(active) = table.get::<_, mlua::Table>("active") {
                get_appearance(active, theme.cosmic(), theme.current_container())
            } else {
                let active: mlua::Table = table.get::<_, mlua::Table>("default").expect("Failed to get default appearance table");
                get_appearance(active, theme.cosmic(), theme.current_container())
            }

        }),
        error: Box::new(move |theme| {
            let table = error_clone.to_ref();
            if let Ok(error) = table.get::<_, mlua::Table>("error") {
                get_appearance(error, theme.cosmic(), theme.current_container())
            } else {
                let error: mlua::Table = table.get::<_, mlua::Table>("default").expect("Failed to get default appearance table");
                get_appearance(error, theme.cosmic(), theme.current_container())
            }

        }),

        hovered: Box::new(move |theme| {
            let table = hovered_clone.to_ref();
            if let Ok(hovered) = table.get::<_, mlua::Table>("hovered") {
                get_appearance(hovered, theme.cosmic(), theme.current_container())
                // get_appearance(hovered, self.cosmic(), self.current_container)
            } else {
                let hovered: mlua::Table = table.get::<_, mlua::Table>("default").expect("Failed to get default appearance table");
                get_appearance(hovered, theme.cosmic(), theme.current_container())
            }

        }),
        focused: Box::new(move |theme| {
            let table = focused_clone.to_ref();
            if let Ok(focused) = table.get::<_, mlua::Table>("focused") {
                get_appearance(focused, theme.cosmic(), theme.current_container())
            } else {
                let focused: mlua::Table = table.get::<_, mlua::Table>("default").expect("Failed to get default appearance table");
                get_appearance(focused, theme.cosmic(), theme.current_container())
            }

        }),
        disabled: Box::new(move |theme| {
            let table = disabled_clone.to_ref();
            if let Ok(disabled) = table.get::<_, mlua::Table>("disabled") {
                get_appearance(disabled, theme.cosmic(), theme.current_container())
            } else {
                let disabled: mlua::Table = table.get::<_, mlua::Table>("default").expect("Failed to get default appearance table");
                get_appearance(disabled, theme.cosmic(), theme.current_container())
            }

        })

    }
}
