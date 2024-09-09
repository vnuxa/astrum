use cosmic::iced::{Length};
use cosmic::iced_core::text::LineHeight;
use cosmic::Element;
use mlua::Number;
use core::panic;
use std::any::Any;
use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::convert::TryInto;
use std::rc::Rc;
use cosmic::Renderer;

use crate::app::WindowMessages;
use crate::style::text_input::lua_text_input_style;

use super::{container::lua_container_widget, custom, process_lua_element};

thread_local!(static IDENTIFIERS: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new()));

pub fn lua_text_input_widget(
    data: mlua::Table,
    // mut identifiers: RefCell<HashMap<String, String>>
) -> cosmic::widget::TextInput<WindowMessages> {

    // IDENTIFIERS.with_borrow_mut(|identifiers|{


    let widget_placeholder: mlua::String = data.get::<&str, mlua::String>("placeholder").unwrap();
    let widget_value: mlua::String = data.get::<&str, mlua::String>("value").unwrap();
    let mut text_input_widget: cosmic::widget::TextInput<WindowMessages> = cosmic::widget::text_input(
        widget_placeholder.to_str().unwrap().to_string(),
        widget_value.to_str().unwrap().to_string()
        // identifiers.get(widget_id.to_str().unwrap()).unwrap()
    );


    if let Ok(width) = data.get::<_, mlua::String>("width") {
        text_input_widget = text_input_widget.width(match width.to_str().unwrap() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(width) = data.get::<_, mlua::Table>("width") {
        // covers FillPortion(i32) and Fixed(u32)
        let width_type: mlua::String = width.get(1).unwrap();
        let width_contents: mlua::Number = width.get(2).unwrap();

        text_input_widget = text_input_widget.width(match width_type.to_str().unwrap() {
            "fill_portion" => Length::FillPortion(width_contents as u16),
            "fixed" => Length::Fixed(width_contents as f32),
            _ => Length::Shrink
        });
    }




    if let Ok(active) = data.get::<_, bool>("always_active")  {
        if active {
            text_input_widget = text_input_widget.always_active();
        }
    }
    if let Ok(password) = data.get::<_, bool>("password")  {
        if password {
            text_input_widget = text_input_widget.password();
        }
    }


    if let Ok(line_height) = data.get::<_, mlua::Table>("line_height") {
        let first_param: mlua::String = line_height.get(1).unwrap();
        let second_param: mlua::Number = line_height.get(2).unwrap();
        text_input_widget = text_input_widget.line_height(match first_param.to_str().unwrap() {
            "relative" => {
                LineHeight::Relative(second_param as f32)
            },
            "absolute" => {
                LineHeight::Absolute(cosmic::iced::Pixels(second_param as f32))
            }
            &_ => unimplemented!("Text input line height not supported")
        });
    }

    if let Ok(style) = data.get::<_, mlua::Table>("style") {
        text_input_widget = text_input_widget.style(lua_text_input_style(style.into_owned()));
    }
    if let Ok(size) = data.get::<_, mlua::Number>("size") {
        text_input_widget = text_input_widget.size(size as f32);
    }

    if let Ok(on_input) = data.get::<_, mlua::String>("on_input") {
        text_input_widget = text_input_widget.on_input(move |text| {
            return WindowMessages::Msg((on_input.to_str().unwrap().to_string(), format!("{{ text = '{text}' }}", text = text.replace("'", r"\'"))));
            // if let Ok(signal_name) = on_input.call::<_, mlua::String>(text.clone()) {
            //     println!("lol on input is a strin");
            //     // WindowMessages::WindowInputChanged(())
            // } else if let Ok(signal_table) = on_input.call::<_, mlua::Table>(text.clone()) {
            //     println!("text thingy is a table");
            //     return WindowMessages::Msg(
            //         (
            //             signal_table.get::<_, mlua::String>("signal_name").unwrap().to_str().unwrap().to_string(),
            //             signal_table.get::<_, mlua::String>("signal_data").unwrap().to_str().unwrap().to_string(),
            //
            //         )
            //     );
            // } else {
            //     unimplemented!();
            //     return WindowMessages::Msg(("".to_string(), "{}".to_string()));
            // }
        });
    }



    text_input_widget


    // {
    //     let mut identifiers = identifiers.borrow_mut();
    //     if let None = identifiers.get(widget_id.to_str().unwrap()) {
    //         identifiers.insert(widget_id.to_str().unwrap().to_string(), String::new());
    //     }
    // }

    // {
    //     if let None = identifiers.get(widget_id.to_str().unwrap()) {
    //         identifiers.insert(widget_id.to_str().unwrap().to_string(), String::new());
    //     }
    // }

    // let value = "Wow";

    // })
}
