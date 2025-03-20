use cosmic::{iced::{Background, Border, Color, Radius, Shadow}, widget::container};

use super::{from_colors, get_vector};



pub fn lua_container_style<'a>(table: mlua::Table) -> cosmic::theme::Container<'a> {
    cosmic::theme::style::Container::Custom(Box::new(move |theme| {
        let mut result = container::Style {
            border: {

                if let Ok(border) = table.get::<mlua::Table>("border") {

                    let mut obj_color: Color = Default::default();
                    let mut obj_radius: Radius = Default::default();
                    let mut obj_width: f32 = Default::default();

                    if let Ok(color) = border.get::<mlua::Table>("color") {
                        obj_color = from_colors(color);
                    }
                    if let Ok(width) = border.get::<mlua::Number>("width") {
                        obj_width = width as f32
                    }
                    if let Ok(radius) = border.get::<mlua::Number>("radius") {
                        obj_radius = Radius::from(radius as f32)
                    } else if let Ok(radius) = border.get::<mlua::Table>("radius") {
                        obj_radius = Radius::from(
                            [
                                radius.get::<mlua::Number>(1).unwrap() as f32,
                                radius.get::<mlua::Number>(2).unwrap() as f32,
                                radius.get::<mlua::Number>(3).unwrap() as f32,
                                radius.get::<mlua::Number>(4).unwrap() as f32,
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
            //     let border: mlua::Table = table.get::<mlua::Table>("border");
            //
            //     Border {
            //         color: from_colors(border.get::<mlua::Table>("color")),
            //         width: border.get::<mlua::Number>("width") as f32,
            //         radius: {
            //             if let Ok(radius) = border.get::<mlua::Number>("radius") {
            //                 Radius::from(radius as f32)
            //             } else {
            //                 let radius = table.get::<mlua::Table>("radius").expect("Border radius is expected to be either a table or number");
            //                 Radius::from(
            //                     [
            //                         radius.get::<mlua::Number>(1).unwrap() as f32,
            //                         radius.get::<mlua::Number>(2).unwrap() as f32,
            //                         radius.get::<mlua::Number>(3).unwrap() as f32,
            //                         radius.get::<mlua::Number>(4).unwrap() as f32,
            //                     ]
            //                 )
            //             }
            //
            //         },
            //     }
            // },
            // shadow: Shadow {
            //     color: cosmic::iced::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
            //     offset: cosmic::iced::Vector { x: 0.0, y: 0.0 },
            //     blur_radius: 20.0,
            // },
            shadow: if let Ok(shadow) = table.get::<mlua::Table>("shadow") {
                Shadow {
                    color: from_colors(shadow.get::<mlua::Table>("color").unwrap()),
                    blur_radius: shadow.get::<mlua::Number>("blur_radius").unwrap() as f32,
                    offset: get_vector(shadow.get::<mlua::Table>("offset").unwrap())
                }
            } else {
                Shadow::default()

            },
            icon_color: None,
            text_color: None,
            background: None,
        };
        if let Ok(shadow) = table.get::<mlua::Table>("shadow") {
            result.shadow = Shadow {
                color: from_colors(shadow.get::<mlua::Table>("color").expect("Expected color within the shadow settings")),
                blur_radius: shadow.get::<mlua::Number>("blur_radius").expect("Expected blur radius within the shadow settings") as f32,
                offset: get_vector(shadow.get::<mlua::Table>("offset").expect("Expected offset within the shadow settings"))
            };
        }

        if let Ok(text_color) = table.get::<mlua::Table>("text_color") {
            result.text_color = Some(from_colors(text_color));
        }

        if let Ok(icon_color) = table.get::<mlua::Table>("icon_color") {
            result.icon_color = Some(from_colors(icon_color));
        }

        if let Ok(background) = table.get::<mlua::Table>("background") {
            result.background = Some(Background::Color(from_colors(background)));
        }

        result
    }))

}
