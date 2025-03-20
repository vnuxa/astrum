use cosmic::iced::{alignment::{Horizontal, Vertical}, color, font::{Family, Style, Weight}, Alignment, Font, Length};

use cosmic::widget::Container;
use mlua::Number;
use crate::{astrum_binds::style::container::lua_container_style, astrum_core::app::main::AstrumMessages};

use super::process_lua_element;

pub fn make_container_widget<'a>(
    data: mlua::Table
) -> Container<'a, AstrumMessages, cosmic::Theme> {
    let widget_child = data.get("child").unwrap();
    let mut container_widget = Container::new(process_lua_element(widget_child).expect("Container child not specified"));

        // width and height


    if let Ok(width) = data.get::<mlua::String>("width") {
        container_widget = container_widget.width(match width.to_string_lossy().as_str() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(width) = data.get::<mlua::Table>("width") {
        // covers FillPortion(i32) and Fixed(u32)
        let width_type: mlua::String = width.get(1).unwrap();
        let width_contents: mlua::Number = width.get(2).unwrap();

        container_widget = container_widget.width(match width_type.to_string_lossy().as_str() {
            "fill_portion" => Length::FillPortion(width_contents as u16),
            "fixed" => Length::Fixed(width_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(height) = data.get::<mlua::String>("height") {
        container_widget = container_widget.height(match height.to_string_lossy().as_str() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(height) = data.get::<mlua::Table>("height") {
        // covers FillPortion(i32) and Fixed(u32)
        let height_type: mlua::String = height.get(1).unwrap();
        let height_contents: mlua::Number = height.get(2).unwrap();

        container_widget = container_widget.height(match height_type.to_string_lossy().as_str() {
            "fill_portion" => Length::FillPortion(height_contents as u16),
            "fixed" => Length::Fixed(height_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(width) = data.get::<mlua::String>("center_x") {
        container_widget = container_widget.center_x(match width.to_string_lossy().as_str() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(width) = data.get::<mlua::Table>("center_x") {
        // covers FillPortion(i32) and Fixed(u32)
        let width_type: mlua::String = width.get(1).unwrap();
        let width_contents: mlua::Number = width.get(2).unwrap();

        container_widget = container_widget.center_x(match width_type.to_string_lossy().as_str() {
            "fill_portion" => Length::FillPortion(width_contents as u16),
            "fixed" => Length::Fixed(width_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(height) = data.get::<mlua::String>("center_y") {
        container_widget = container_widget.center_y(match height.to_string_lossy().as_str() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(height) = data.get::<mlua::Table>("center_y") {
        // covers FillPortion(i32) and Fixed(u32)
        let height_type: mlua::String = height.get(1).unwrap();
        let height_contents: mlua::Number = height.get(2).unwrap();

        container_widget = container_widget.center_y(match height_type.to_string_lossy().as_str() {
            "fill_portion" => Length::FillPortion(height_contents as u16),
            "fixed" => Length::Fixed(height_contents as f32),
            _ => Length::Shrink
        });
    }



    if let Ok(padding) = data.get::<mlua::Number>("padding") {
        container_widget = container_widget.padding(padding as f32);
    } else  if let Ok(padding) = data.get::<mlua::Table>("padding") {
        let mut padding_list: Vec<f32> = Vec::new();
        for pair in padding.pairs::<Number, f32>() {
            let (_key, value): (Number, f32) = pair.unwrap();
            padding_list.push(value);
        }
        match padding_list.len() {
            2 => {
                container_widget = container_widget.padding(TryInto::<[f32; 2]>::try_into(padding_list).unwrap());
            },
            4 => {
                container_widget = container_widget.padding(TryInto::<[f32; 4]>::try_into(padding_list).unwrap());
            },
            _ => {
                panic!("Container has unsupported padding array size!");
            }
        };
    }

    if let Ok(max_width) = data.get::<mlua::Number>("max_width")  {
        container_widget = container_widget.max_width(max_width as f32);
    }
    if let Ok(max_height) = data.get::<mlua::Number>("max_height")  {
        container_widget = container_widget.max_height(max_height as f32);
    }

    if let Ok(horizontal_alignment) = data.get::<mlua::String>("align_x") {
        container_widget = container_widget.align_x(match horizontal_alignment.to_string_lossy().as_str() {
            "left" => Horizontal::Left,
            "center" => Horizontal::Center,
            "right" => Horizontal::Right,
            _ => unimplemented!("Container horizontal alignment value not supported")
        });
    }
    if let Ok(vertical_alignment) = data.get::<mlua::String>("align_y") {
        container_widget = container_widget.align_y(match vertical_alignment.to_string_lossy().as_str() {
            "top" => Vertical::Top,
            "center" => Vertical::Center,
            "bottom" => Vertical::Bottom,
            _ => unimplemented!("Container vertical alignment value not supported")
        });
    }
    if let Ok(style) = data.get::<mlua::Table>("style") {
        container_widget = container_widget.class(lua_container_style(style));
    }

    container_widget
}
