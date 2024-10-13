use cosmic::iced::font::{Family, Style, Weight};
use cosmic::iced::{Font, Length};
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

    // use std::time::Instant;

thread_local!(static IDENTIFIERS: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new()));

unsafe fn make_static_str<'a>(key: &'a str) -> &'static str {
    std::mem::transmute::<&'a str, &'static str>(key)
}
pub fn lua_text_input_widget(
    data: mlua::Table,
    // mut identifiers: RefCell<HashMap<String, String>>
) -> cosmic::widget::TextInput<WindowMessages> {

    // IDENTIFIERS.with_borrow_mut(|identifiers|{
    //
    // let now = Instant::now();


    let widget_placeholder: mlua::String = data.get::<&str, mlua::String>("placeholder").unwrap();
    let widget_value: mlua::String = data.get::<&str, mlua::String>("value").unwrap();
    let mut text_input_widget: cosmic::widget::TextInput<WindowMessages> = cosmic::widget::text_input(
        widget_placeholder.to_str().unwrap().to_string(),
        widget_value.to_str().unwrap().to_string()
        // identifiers.get(widget_id.to_str().unwrap()).unwrap()
    );
    if let Ok(on_input) = data.get::<_, mlua::String>("on_input") {
        text_input_widget = text_input_widget.on_input(move |text| {
            println!("rust on input: {:?}", text);
            return WindowMessages::Msg((on_input.to_str().unwrap().to_string(), format!("{{ text = '{text}' }}", text = text.replace("'", r"\'"))));
        });
    }


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



    if let Ok(font_settings) = data.get::<_, mlua::Table>("font") {
        // let font_settings = Rc::new(font_settings);
        // let mut current_font = Font::default();
        let mut font_family: Option<Family> = None;
        let mut font_weight: Option<Weight> = None;
        let mut font_style: Option<Style> = None;

        if let Ok(font_name) = font_settings.get::<_, mlua::String>("name") {
            // let font_thing = RefCell::new(String::from(font_name.to_str().unwrap()));
            // let font_thing: Cow<'static, str> = font_name.to_string_lossy();

            // for some reason font name is static???
            // and its either i leak memory or use unsafe code
            unsafe {
                font_family = Some(Family::Name(make_static_str(font_name.to_str().unwrap())));
            }
        }
        if let Ok(weight) = font_settings.get::<_, mlua::String>("weight") {
            font_weight = Some(match weight.to_str().unwrap() {
                "thin" => Weight::Thin,
                "extra_light" => Weight::ExtraLight,
                "light" => Weight::Light,
                "normal" => Weight::Normal,
                "medium" => Weight::Medium,
                "semibold" => Weight::Semibold,
                "bold" => Weight::Bold,
                "extrabold" => Weight::ExtraBold,
                "black" => Weight::Black,
                _ => Weight::Normal
            });
        }
        if let Ok(style) = font_settings.get::<_, mlua::String>("style") {
            font_style = Some(match style.to_str().unwrap() {
                "normal" => Style::Normal,
                "italic" => Style::Italic,
                "oblique" => Style::Oblique,
                _ => Style::Normal,
            });
        }

        text_input_widget= text_input_widget.font(Font {
            family: font_family.unwrap_or_default(),
            weight: font_weight.unwrap_or_default(),
            style: font_style.unwrap_or_default(),
            ..Default::default()
        });
    }

    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);

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
