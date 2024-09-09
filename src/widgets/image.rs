use std::path::{Path, PathBuf};
use std::rc::Rc;

use cosmic::iced::{ContentFit, Element, Length, Radius};
use cosmic::widget::icon::{from_name, from_path, icon, IconFallback};
use cosmic::widget::image::{self, FilterMethod};
use cosmic::Renderer;

use crate::app::WindowMessages;
use crate::config::HOME_DIR;


pub fn lua_image_widget(
    data: mlua::Table
) -> cosmic::widget::image::Image<image::Handle>
{
    let mut path = PathBuf::from("/");
    for text in data.get::<_, mlua::String>("path").unwrap().to_str().unwrap().split("/") {
        if text == "~" {
            path.push(HOME_DIR.to_str().unwrap());
        } else {
            path.push(text);
        }
    }
    let mut image_widget: cosmic::widget::Image<image::Handle> = image::Image::new::<image::Handle>(image::Handle::from_path(path));
        // image_widget = image::Handle::from_path(path);
        // image_widget = image::Image::new::<image::Handle>(path)


    // covers Fill and Shrink
    if let Ok(width) = data.get::<_, mlua::String>("width") {
        image_widget = image_widget.width(match width.to_str().unwrap() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(width) = data.get::<_, mlua::Table>("width") {
        // covers FillPortion(i32) and Fixed(u32)
        let width_type: mlua::String = width.get(1).unwrap();
        let width_contents: mlua::Number = width.get(2).unwrap();

        image_widget = image_widget.width(match width_type.to_str().unwrap() {
            "fill_portion" => Length::FillPortion(width_contents as u16),
            "fixed" => Length::Fixed(width_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(height) = data.get::<_, mlua::String>("height") {
        image_widget = image_widget.height(match height.to_str().unwrap() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(height) = data.get::<_, mlua::Table>("height") {
        // covers FillPortion(i32) and Fixed(u32)
        let height_type: mlua::String = height.get(1).unwrap();
        let height_contents: mlua::Number = height.get(2).unwrap();

        image_widget = image_widget.height(match height_type.to_str().unwrap() {
            "fill_portion" => Length::FillPortion(height_contents as u16),
            "fixed" => Length::Fixed(height_contents as f32),
            _ => Length::Shrink
        });
    }


    if let Ok(content_fit) = data.get::<_, mlua::String>("content_fit") {
        image_widget = image_widget.content_fit(match content_fit.to_str().unwrap() {
            "contain" => ContentFit::Contain,
            "cover" => ContentFit::Cover,
            "fill" => ContentFit::Fill,
            "none" => ContentFit::None,
            "scale_down" => ContentFit::ScaleDown,
            _ => ContentFit::Contain,
        });
    }

    if let Ok(border_radius) = data.get::<_, mlua::Number>("border_radius") {
        image_widget = image_widget.border_radius([border_radius as f32; 4])
    } else if let Ok(border_radius) = data.get::<_, mlua::Table>("border_radius") {
        image_widget = image_widget.border_radius([
            border_radius.get::<_, mlua::Number>(1).unwrap() as f32,
            border_radius.get::<_, mlua::Number>(2).unwrap() as f32,
            border_radius.get::<_, mlua::Number>(3).unwrap() as f32,
            border_radius.get::<_, mlua::Number>(4).unwrap() as f32,
        ])
    }

    if let Ok(filter_method) = data.get::<_, mlua::String>("filter_method"){
        image_widget = image_widget.filter_method(match filter_method.to_str().unwrap() {
            "linear" => FilterMethod::Linear,
            "nearest" => FilterMethod::Nearest,
            &_ => unimplemented!("Filter method not supported, are you sure its not a typo?")
        })
    }


    image_widget
}
