use std::path::{Path, PathBuf};
use std::rc::Rc;

use cosmic::iced::{ContentFit, Element, Length};
use cosmic::widget::icon::{from_name, from_path, icon};
use cosmic::Renderer;

use crate::app::WindowMessages;
use crate::config::HOME_DIR;


pub fn lua_icon_widget(
    data: mlua::Table
) -> cosmic::widget::Icon
{
    let mut icon_widget: cosmic::widget::Icon;

    if let Ok(icon_path) = data.get::<_, mlua::String>("icon_path") {
        let mut path = PathBuf::from("/");
        for text in icon_path.to_str().unwrap().split("/") {
            if text == "~" {
                path.push(HOME_DIR.to_str().unwrap());
            } else {
                path.push(text);
            }
        }
        icon_widget = icon(from_path(path));

    } else {
        let icon_name = data.get::<_, mlua::String>("icon_name").unwrap();
        icon_widget = icon(from_name(icon_name.to_str().unwrap()).into());
    }

    // covers Fill and Shrink
    if let Ok(width) = data.get::<_, mlua::String>("width") {
        icon_widget = icon_widget.width(match width.to_str().unwrap() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(width) = data.get::<_, mlua::Table>("width") {
        // covers FillPortion(i32) and Fixed(u32)
        let width_type: mlua::String = width.get(1).unwrap();
        let width_contents: mlua::Number = width.get(2).unwrap();

        icon_widget = icon_widget.width(match width_type.to_str().unwrap() {
            "fill_portion" => Length::FillPortion(width_contents as u16),
            "fixed" => Length::Fixed(width_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(height) = data.get::<_, mlua::String>("height") {
        icon_widget = icon_widget.height(match height.to_str().unwrap() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(height) = data.get::<_, mlua::Table>("height") {
        // covers FillPortion(i32) and Fixed(u32)
        let height_type: mlua::String = height.get(1).unwrap();
        let height_contents: mlua::Number = height.get(2).unwrap();

        icon_widget = icon_widget.height(match height_type.to_str().unwrap() {
            "fill_portion" => Length::FillPortion(height_contents as u16),
            "fixed" => Length::Fixed(height_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(size) = data.get::<_, mlua::Integer>("size") {
        icon_widget = icon_widget.size(size as u16);
    }

    if let Ok(content_fit) = data.get::<_, mlua::String>("content_fit") {
        icon_widget = icon_widget.content_fit(match content_fit.to_str().unwrap() {
            "contain" => ContentFit::Contain,
            "cover" => ContentFit::Cover,
            "fill" => ContentFit::Fill,
            "none" => ContentFit::None,
            "scale_down" => ContentFit::ScaleDown,
            _ => ContentFit::Contain,
        });
    }


    icon_widget
}
