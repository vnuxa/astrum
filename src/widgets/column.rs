use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use cosmic::widget::{column, Column};
use cosmic::iced::{Alignment, Length};
use cosmic::Element;
use cosmic::Renderer;
use mlua::Number;
use mlua::Integer;

use crate::app::WindowMessages;

use super::process_lua_element;


pub fn lua_column_widget(
    data: mlua::Table,
    // mut identifiers: Rc<HashMap<String, String>>
) -> Column<WindowMessages> {
    let mut widgets = Vec::new();
    let children_table: mlua::Table = data.get("children").unwrap();
    for pairs in children_table.pairs::<mlua::Integer, mlua::Table>() {
        let (key, value) = pairs.unwrap();
        let widget = process_lua_element(value);
        if let Some(element) = widget {
            widgets.push(element);
        }
    }
    let mut column_widget = Column::with_children(widgets);

    if let Ok(width) = data.get::<_, mlua::String>("width") {
        column_widget = column_widget.width(match width.to_str().unwrap() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(width) = data.get::<_, mlua::Table>("width") {
        // covers FillPortion(i32) and Fixed(u32)
        let width_type: mlua::String = width.get(1).unwrap();
        let width_contents: mlua::Number = width.get(2).unwrap();

        column_widget = column_widget.width(match width_type.to_str().unwrap() {
            "fill_portion" => Length::FillPortion(width_contents as u16),
            "fixed" => Length::Fixed(width_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(height) = data.get::<_, mlua::String>("height") {
        column_widget = column_widget.height(match height.to_str().unwrap() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(height) = data.get::<_, mlua::Table>("height") {
        // covers FillPortion(i32) and Fixed(u32)
        let height_type: mlua::String = height.get(1).unwrap();
        let height_contents: mlua::Number = height.get(2).unwrap();

        column_widget = column_widget.height(match height_type.to_str().unwrap() {
            "fill_portion" => Length::FillPortion(height_contents as u16),
            "fixed" => Length::Fixed(height_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(padding) = data.get::<_, mlua::Number>("padding") {
        column_widget = column_widget.padding(padding as f32);
    } else  if let Ok(padding) = data.get::<_, mlua::Table>("padding") {
        let mut padding_list: Vec<f32> = Vec::new();
        for pair in padding.pairs::<Number, f32>() {
            let (_key, value): (Number, f32) = pair.unwrap();
            padding_list.push(value);
        }
        match padding_list.len() {
            2 => {
                column_widget = column_widget.padding(TryInto::<[f32; 2]>::try_into(padding_list).unwrap());
            },
            4 => {
                column_widget = column_widget.padding(TryInto::<[f32; 4]>::try_into(padding_list).unwrap());
            },
            _ => {
                panic!("Row has unsupported padding array size!");
            }
        };
    }

    if let Ok(spacing) = data.get::<_, mlua::Number>("spacing") {
        column_widget = column_widget.spacing(spacing as f32);
    }

    if let Ok(alignment) = data.get::<_, mlua::String>("align_items") {
        column_widget = column_widget.align_items(match alignment.to_str().unwrap() {
            "start" => Alignment::Start,
            "center" => Alignment::Center,
            "end" => Alignment::End,
            _ => unimplemented!("Specified row alignment is not supported")
        });
    }

    if let Ok(max_width) = data.get::<_, mlua::Number>("max_width")  {
        column_widget = column_widget.max_width(max_width as f32)
    }

    column_widget
}
