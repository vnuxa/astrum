use std::rc::Rc;

use cosmic::iced::{Element, Length};
use cosmic::Renderer;

use crate::app::WindowMessages;
use crate::style::text::lua_text_style;


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

    text_widget
}
