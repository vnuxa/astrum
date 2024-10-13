use std::rc::Rc;

use cosmic::{iced::{Background, Border, Color, Radius}, iced_core::Shadow, widget::{button::Appearance, container}};
use super::{from_colors, get_vector};

pub fn lua_container_style(
    data: mlua::OwnedTable
) -> cosmic::theme::style::Container {
    cosmic::theme::style::Container::Custom(Box::new(move |theme| {
        let table = data.to_ref();
        let mut result = container::Appearance {
            border: {

                if let Ok(border) = table.get::<_, mlua::Table>("border") {

                    let mut obj_color: Color = Default::default();
                    let mut obj_radius: Radius = Default::default();
                    let mut obj_width: f32 = Default::default();

                    if let Ok(color) = border.get::<_, mlua::Table>("color") {
                        obj_color = from_colors(color);
                    }
                    if let Ok(width) = border.get::<_, mlua::Number>("width") {
                        obj_width = width as f32
                    }
                    if let Ok(radius) = border.get::<_, mlua::Number>("radius") {
                        obj_radius = Radius::from(radius as f32)
                    } else if let Ok(radius) = border.get::<_, mlua::Table>("radius") {
                        obj_radius = Radius::from(
                            [
                                radius.get::<_, mlua::Number>(1).unwrap() as f32,
                                radius.get::<_, mlua::Number>(2).unwrap() as f32,
                                radius.get::<_, mlua::Number>(3).unwrap() as f32,
                                radius.get::<_, mlua::Number>(4).unwrap() as f32,
                            ]
                        )
                    }
                    Border {
                        color: obj_color,
                        width: obj_width,
                        radius: obj_radius
                    }
                } else {

                    Border {
                        color: cosmic::iced::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
                        width: 0.0,
                        radius: Radius::from(5),
                    }
                }
            },
            // border: {
            //     let border: mlua::Table = table.get::<_, mlua::Table>("border");
            //
            //     Border {
            //         color: from_colors(border.get::<_, mlua::Table>("color")),
            //         width: border.get::<_, mlua::Number>("width") as f32,
            //         radius: {
            //             if let Ok(radius) = border.get::<_, mlua::Number>("radius") {
            //                 Radius::from(radius as f32)
            //             } else {
            //                 let radius = table.get::<_, mlua::Table>("radius").expect("Border radius is expected to be either a table or number");
            //                 Radius::from(
            //                     [
            //                         radius.get::<_, mlua::Number>(1).unwrap() as f32,
            //                         radius.get::<_, mlua::Number>(2).unwrap() as f32,
            //                         radius.get::<_, mlua::Number>(3).unwrap() as f32,
            //                         radius.get::<_, mlua::Number>(4).unwrap() as f32,
            //                     ]
            //                 )
            //             }
            //
            //         },
            //     }
            // },
            shadow: Shadow {
                color: cosmic::iced::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
                offset: cosmic::iced::Vector { x: 0.0, y: 0.0 },
                blur_radius: 20.0,
            },
            // shadow: {
            //     let shadow: mlua::Table = table.get::<_, mlua::Table>("shadow");
            //
            //     Shadow {
            //         color: from_colors(shadow.get::<_, mlua::Table>("color")),
            //         blur_radius: shadow.get::<_, mlua::Number>("blur_radius") as f32,
            //         offset: get_vector(shadow.get::<_, mlua::Table>("offset"))
            //     }
            // },
            icon_color: None,
            text_color: None,
            background: None,
        };
        if let Ok(shadow) = table.get::<_, mlua::Table>("shadow") {
            result.shadow = Shadow {
                color: from_colors(shadow.get::<_, mlua::Table>("color").expect("Expected color within the shadow settings")),
                blur_radius: shadow.get::<_, mlua::Number>("blur_radius").expect("Expected blur radius within the shadow settings") as f32,
                offset: get_vector(shadow.get::<_, mlua::Table>("offset").expect("Expected offset within the shadow settings"))
            };
        }

        if let Ok(text_color) = table.get::<_, mlua::Table>("text_color") {
            result.text_color = Some(from_colors(text_color));
        }

        if let Ok(icon_color) = table.get::<_, mlua::Table>("icon_color") {
            result.icon_color = Some(from_colors(icon_color));
        }

        if let Ok(background) = table.get::<_, mlua::Table>("background") {
            // TODO: add gradient support
            result.background = Some(Background::Color(from_colors(background)));
        }

        result
    }))
}
