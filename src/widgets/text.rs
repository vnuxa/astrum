use std::borrow::{Borrow, BorrowMut, Cow};
use std::cell::RefCell;
use std::rc::Rc;

use cosmic::iced::font::{Family, Stretch, Style, Weight};
use cosmic::iced::{Element, Font, Length};
use cosmic::widget::Widget;
use cosmic::{font, Renderer};

use crate::app::WindowMessages;
use crate::style::text::lua_text_style;

unsafe fn make_static_str<'a>(key: &'a str) -> &'static str {
    std::mem::transmute::<&'a str, &'static str>(key)
}
pub fn lua_text_widget(
    data: mlua::Table
) -> cosmic::widget::Text<cosmic::Theme>
{
    let widget_content: mlua::String = data.get::<&str, mlua::String>("content").unwrap();

    let mut text_widget: cosmic::widget::Text<cosmic::Theme, Renderer> = cosmic::widget::text(widget_content.to_str().unwrap().to_string());


    // covers Fill and Shrink
    if let Ok(width) = data.get::<_, mlua::String>("width") {
        text_widget = text_widget.width(match width.to_str().unwrap() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(width) = data.get::<_, mlua::Table>("width") {
        // covers FillPortion(i32) and Fixed(u32)
        let width_type: mlua::String = width.get(1).unwrap();
        let width_contents: mlua::Number = width.get(2).unwrap();

        text_widget = text_widget.width(match width_type.to_str().unwrap() {
            "fill_portion" => Length::FillPortion(width_contents as u16),
            "fixed" => Length::Fixed(width_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(height) = data.get::<_, mlua::String>("height") {
        text_widget = text_widget.height(match height.to_str().unwrap() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(height) = data.get::<_, mlua::Table>("height") {
        // covers FillPortion(i32) and Fixed(u32)
        let height_type: mlua::String = height.get(1).unwrap();
        let height_contents: mlua::Number = height.get(2).unwrap();

        text_widget = text_widget.height(match height_type.to_str().unwrap() {
            "fill_portion" => Length::FillPortion(height_contents as u16),
            "fixed" => Length::Fixed(height_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(horizontal_alignment) = data.get::<_, mlua::String>("horizontal_alignment") {
        text_widget = text_widget.horizontal_alignment(match horizontal_alignment.to_str().unwrap() {
            "center" => cosmic::iced::alignment::Horizontal::Center,
            "right" => cosmic::iced::alignment::Horizontal::Right,
            _ => cosmic::iced::alignment::Horizontal::Left
        });
    }

    if let Ok(vertical_alignment) = data.get::<_, mlua::String>("vertical_alignment") {
        text_widget = text_widget.vertical_alignment(match vertical_alignment.to_str().unwrap() {
            "center" => cosmic::iced::alignment::Vertical::Center,
            "bottom" => cosmic::iced::alignment::Vertical::Bottom,
            _ => cosmic::iced::alignment::Vertical::Top,
        });
    }

    if let Ok(style) = data.get::<_, mlua::Table>("style") {
        text_widget = text_widget.style(lua_text_style(style.into_owned()))
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

        text_widget = text_widget.font(Font {
            family: font_family.unwrap_or_default(),
            weight: font_weight.unwrap_or_default(),
            style: font_style.unwrap_or_default(),
            ..Default::default()
        });
    }
    if let Ok(size) = data.get::<_, mlua::Number>("size") {
        text_widget = text_widget.size(size as f32);
    }

    text_widget
}
