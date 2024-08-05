use cosmic::iced::{Alignment, Length};
use cosmic::Element;
use mlua::Number;
use core::panic;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryInto;
use std::rc::Rc;
use cosmic::iced::Renderer;

use crate::app::WindowMessages;

use super::{container::lua_container_widget, custom, process_lua_element};


pub fn lua_centerbox_widget(
    data: mlua::Table,
    // mut identifiers: Rc<HashMap<String, String>>
) -> custom::centerbox::Centerbox<WindowMessages, cosmic::Theme>
    // Renderer: cosmic::iced::advanced::Renderer,
{
    let left_child = data.get("left_child").unwrap();
    let middle_child = data.get("middle_child").unwrap();
    let right_child = data.get("right_child").unwrap();

    let mut centerbox_widget: custom::centerbox::Centerbox<WindowMessages, cosmic::Theme> = custom::centerbox::Centerbox::new(
        [
            process_lua_element(left_child).unwrap(),
            process_lua_element(middle_child).unwrap(),
            process_lua_element(right_child).unwrap()

        ]);

    if let Ok(width) = data.get::<_, mlua::String>("width") {
        centerbox_widget = centerbox_widget.width(match width.to_str().unwrap() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(width) = data.get::<_, mlua::Table>("width") {
        // covers FillPortion(i32) and Fixed(u32)
        let width_type: mlua::String = width.get(1).unwrap();
        let width_contents: mlua::Number = width.get(2).unwrap();

        centerbox_widget = centerbox_widget.width(match width_type.to_str().unwrap() {
            "fill_portion" => Length::FillPortion(width_contents as u16),
            "fixed" => Length::Fixed(width_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(height) = data.get::<_, mlua::String>("height") {
        centerbox_widget = centerbox_widget.height(match height.to_str().unwrap() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(height) = data.get::<_, mlua::Table>("height") {
        // covers FillPortion(i32) and Fixed(u32)
        let height_type: mlua::String = height.get(1).unwrap();
        let height_contents: mlua::Number = height.get(2).unwrap();

        centerbox_widget = centerbox_widget.height(match height_type.to_str().unwrap() {
            "fill_portion" => Length::FillPortion(height_contents as u16),
            "fixed" => Length::Fixed(height_contents as f32),
            _ => Length::Shrink
        });
    }


    if let Ok(spacing) = data.get::<_, mlua::Number>("spacing") {
        centerbox_widget = centerbox_widget.spacing(spacing as f32);
    }

    if let Ok(padding) = data.get::<_, mlua::Number>("padding") {
        centerbox_widget = centerbox_widget.padding(padding as f32);
    } else  if let Ok(padding) = data.get::<_, mlua::Table>("padding") {
        let mut padding_list: Vec<f32> = Vec::new();
        for pair in padding.pairs::<Number, f32>() {
            let (_key, value): (Number, f32) = pair.unwrap();
            padding_list.push(value);
        }
        match padding_list.len() {
            2 => {
                centerbox_widget = centerbox_widget.padding(TryInto::<[f32; 2]>::try_into(padding_list).unwrap());
            },
            4 => {
                centerbox_widget = centerbox_widget.padding(TryInto::<[f32; 4]>::try_into(padding_list).unwrap());
            },
            _ => {
                panic!("Centerbox has unsupported padding array size!");
            }
        };
    }

    if let Ok(alignment) = data.get::<_, mlua::String>("align_items") {
        centerbox_widget = centerbox_widget.align_items(match alignment.to_str().unwrap() {
            "start" => Alignment::Start,
            "center" => Alignment::Center,
            "end" => Alignment::End,
            _ => unimplemented!("Specified centerbox alignment is not supported")
        });
    }

    centerbox_widget
}
