
use iced::advanced::graphics::text::cosmic_text::ttf_parser::vorg::VerticalOriginMetrics;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{container, row, Container, Row};
use iced::{Alignment, Element, Length, Theme};
use iced::Renderer;
use mlua::Number;
use mlua::Integer;

use crate::app::WindowMessages;

use super::process_lua_element;


pub fn lua_container_widget<'a>(
    data: mlua::Table
) -> Container<'a, WindowMessages> {
    let widget_child: mlua::Table = data.get("child").unwrap();
    let mut container_widget = Container::new(process_lua_element(widget_child).unwrap());


    // width and height


    if let Ok(width) = data.get::<_, mlua::String>("width") {
        container_widget = container_widget.width(match width.to_str().unwrap() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(width) = data.get::<_, mlua::Table>("width") {
        // covers FillPortion(i32) and Fixed(u32)
        let width_type: mlua::String = width.get(1).unwrap();
        let width_contents: mlua::Number = width.get(2).unwrap();

        container_widget = container_widget.width(match width_type.to_str().unwrap() {
            "fill_portion" => Length::FillPortion(width_contents as u16),
            "fixed" => Length::Fixed(width_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(height) = data.get::<_, mlua::String>("height") {
        container_widget = container_widget.height(match height.to_str().unwrap() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(height) = data.get::<_, mlua::Table>("height") {
        // covers FillPortion(i32) and Fixed(u32)
        let height_type: mlua::String = height.get(1).unwrap();
        let height_contents: mlua::Number = height.get(2).unwrap();

        container_widget = container_widget.height(match height_type.to_str().unwrap() {
            "fill_portion" => Length::FillPortion(height_contents as u16),
            "fixed" => Length::Fixed(height_contents as f32),
            _ => Length::Shrink
        });
    }



    if let Ok(padding) = data.get::<_, mlua::Number>("padding") {
        container_widget = container_widget.padding(padding as f32);
    } else  if let Ok(padding) = data.get::<_, mlua::Table>("padding") {
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


    if let Ok(max_width) = data.get::<_, mlua::Number>("max_width")  {
        container_widget = container_widget.max_width(max_width as f32)
    }
    if let Ok(max_height) = data.get::<_, mlua::Number>("max_height")  {
        container_widget = container_widget.max_height(max_height as f32)
    }

    if let Ok(_center) = data.get::<_, bool>("center_x")  {
        container_widget = container_widget.center_x();
    }
    if let Ok(_center) = data.get::<_, bool>("center_y")  {
        container_widget = container_widget.center_y();
    }

    if let Ok(horizontal_alignment) = data.get::<_, mlua::String>("horizontal_alignment") {
        container_widget = container_widget.align_x(match horizontal_alignment.to_str().unwrap() {
            "left" => Horizontal::Left,
            "center" => Horizontal::Center,
            "right" => Horizontal::Right,
            _ => unimplemented!("Container horizontal alignment value not supported")
        });
    }
    if let Ok(vertical_alignment) = data.get::<_, mlua::String>("vertical_alignment") {
        container_widget = container_widget.align_y(match vertical_alignment.to_str().unwrap() {
            "top" => Vertical::Top,
            "center" => Vertical::Center,
            "bottom" => Vertical::Bottom,
            _ => unimplemented!("Container vertical alignment value not supported")
        });
    }

    container_widget
}
