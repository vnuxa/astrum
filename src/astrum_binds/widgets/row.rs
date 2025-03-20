use cosmic::{iced::{alignment::Vertical, color, font::{Family, Style, Weight}, Alignment, Font, Length}, widget::Row};

use crate::astrum_core::app::main::AstrumMessages;

use super::process_lua_element;


pub fn make_row_widget<'a>(
    data: mlua::Table
) -> Row<'a, AstrumMessages>{
    let mut widgets = Vec::new();
    let children_table: mlua::Table = data.get("children").unwrap();

    for pairs in children_table.pairs::<mlua::Integer, mlua::Table>()  {
        let (_, value) = pairs.unwrap();
        let widget = process_lua_element(value);

        if let Some(element) = widget {
            widgets.push(element);
        }
    }

    let mut row_widget = Row::with_children(widgets);


    if let Ok(width) = data.get::<mlua::String>("width") {
        row_widget = row_widget.width(match width.to_string_lossy().as_str() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(width) = data.get::<mlua::Table>("width") {
        // covers FillPortion(i32) and Fixed(u32)
        let width_type: mlua::String = width.get(1).unwrap();
        let width_contents: mlua::Number = width.get(2).unwrap();

        row_widget = row_widget.width(match width_type.to_string_lossy().as_str() {
            "fill_portion" => Length::FillPortion(width_contents as u16),
            "fixed" => Length::Fixed(width_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(height) = data.get::<mlua::String>("height") {
        row_widget = row_widget.height(match height.to_string_lossy().as_str() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(height) = data.get::<mlua::Table>("height") {
        // covers FillPortion(i32) and Fixed(u32)
        let height_type: mlua::String = height.get(1).unwrap();
        let height_contents: mlua::Number = height.get(2).unwrap();

        row_widget = row_widget.height(match height_type.to_string_lossy().as_str() {
            "fill_portion" => Length::FillPortion(height_contents as u16),
            "fixed" => Length::Fixed(height_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(padding) = data.get::<mlua::Number>("padding") {
        row_widget = row_widget.padding(padding as f32);
    } else  if let Ok(padding) = data.get::<mlua::Table>("padding") {
        let mut padding_list: Vec<f32> = Vec::new();
        for pair in padding.pairs::<mlua::Number, f32>() {
            let (_key, value): (mlua::Number, f32) = pair.unwrap();
            padding_list.push(value);
        }
        match padding_list.len() {
            2 => {
                row_widget = row_widget.padding(TryInto::<[f32; 2]>::try_into(padding_list).unwrap());
            },
            4 => {
                row_widget = row_widget.padding(TryInto::<[f32; 4]>::try_into(padding_list).unwrap());
            },
            _ => {
                panic!("Row has unsupported padding array size!");
            }
        };
    }

    if let Ok(spacing) = data.get::<mlua::Number>("spacing") {
        row_widget = row_widget.spacing(spacing as f32);
    }

    if let Ok(align) = data.get::<mlua::String>("align_y") {
        row_widget = row_widget.align_y(match align.to_string_lossy().as_str() {
            "top" => Vertical::Top,
            "center" => Vertical::Center,
            "bottom" => Vertical::Bottom,
            _ => unimplemented!("Specified row alignment is not supported")
        });
    }

    if let Ok(clip) = data.get::<bool>("clip") {
        row_widget = row_widget.clip(clip);
    }

    // might need extend and stuff??

    row_widget
}
