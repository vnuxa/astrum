use iced::{Element, Length, Theme};
use mlua::Number;
use core::panic;
use std::convert::TryInto;
use iced::Renderer;

use crate::app::WindowMessages;

use super::{container::lua_container_widget, custom, process_lua_element};


pub fn lua_button_widget<'a>(
    data: mlua::Table,
) -> custom::button::Button<'a, WindowMessages, Theme, Renderer>
where
    Theme: 'a,
    Renderer: iced::advanced::Renderer,

{
    let widget_child: mlua::Table = data.get("child").unwrap();
    // let mut button_widget = iced::widget::button(process_lua_element(widget_child).unwrap());
    let mut button_widget = custom::button::Button::new(process_lua_element(widget_child).unwrap());


    // width and height


    if let Ok(width) = data.get::<_, mlua::String>("width") {
        button_widget = button_widget.width(match width.to_str().unwrap() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(width) = data.get::<_, mlua::Table>("width") {
        // covers FillPortion(i32) and Fixed(u32)
        let width_type: mlua::String = width.get(1).unwrap();
        let width_contents: mlua::Number = width.get(2).unwrap();

        button_widget = button_widget.width(match width_type.to_str().unwrap() {
            "fill_portion" => Length::FillPortion(width_contents as u16),
            "fixed" => Length::Fixed(width_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(height) = data.get::<_, mlua::String>("height") {
        button_widget = button_widget.height(match height.to_str().unwrap() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(height) = data.get::<_, mlua::Table>("height") {
        // covers FillPortion(i32) and Fixed(u32)
        let height_type: mlua::String = height.get(1).unwrap();
        let height_contents: mlua::Number = height.get(2).unwrap();

        button_widget = button_widget.height(match height_type.to_str().unwrap() {
            "fill_portion" => Length::FillPortion(height_contents as u16),
            "fixed" => Length::Fixed(height_contents as f32),
            _ => Length::Shrink
        });
    }



    if let Ok(padding) = data.get::<_, mlua::Number>("padding") {
        button_widget = button_widget.padding(padding as f32);
    } else  if let Ok(padding) = data.get::<_, mlua::Table>("padding") {
        let mut padding_list: Vec<f32> = Vec::new();
        for pair in padding.pairs::<Number, f32>() {
            let (_key, value): (Number, f32) = pair.unwrap();
            padding_list.push(value);
        }
        match padding_list.len() {
            2 => {
                button_widget = button_widget.padding(TryInto::<[f32; 2]>::try_into(padding_list).unwrap());
            },
            4 => {
                button_widget = button_widget.padding(TryInto::<[f32; 4]>::try_into(padding_list).unwrap());
            },
            _ => {
                panic!("Button has unsupported padding array size!");
            }
        };
    }


    if let Ok(on_press) = data.get::<_, mlua::String>("on_press") {
        button_widget = button_widget.on_press(WindowMessages::Msg((on_press.to_str().unwrap().to_string(), "{}".to_string())));
    } else if let Ok(on_press) = data.get::<_, mlua::Table>("on_press") {
        button_widget = button_widget.on_press(WindowMessages::Msg(
            (
                on_press.get::<_, mlua::String>("signal_name").unwrap().to_str().unwrap().to_string(),
                on_press.get::<_, mlua::String>("signal_data").unwrap().to_str().unwrap().to_string(),

            )
        ));
    }
    if let Ok(on_scroll_up) = data.get::<_, mlua::String>("on_scroll_up") {
        button_widget = button_widget.on_scroll_up(WindowMessages::Msg((on_scroll_up.to_str().unwrap().to_string(), "{}".to_string())));
    }
    if let Ok(on_scroll_down) = data.get::<_, mlua::String>("on_scroll_down") {
        button_widget = button_widget.on_scroll_down(WindowMessages::Msg((on_scroll_down.to_str().unwrap().to_string(), "{}".to_string())));
    }
    button_widget

}
