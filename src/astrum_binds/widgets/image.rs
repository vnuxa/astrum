use std::path::PathBuf;

use cosmic::{cosmic_theme::palette::{Alpha, Srgba}, iced::{color, font::{Family, Style, Weight}, ContentFit, Font, Length, Radians, Rotation}, widget::image::FilterMethod};
use cosmic::widget::image;
use log::debug;

use crate::astrum_core::HOME_DIR;

pub fn make_image_widget<'a>(
    data: mlua::Table
) -> cosmic::widget::Image<'a, image::Handle>
{
    // INFO: just decided, icons will handle systray stuff
    // images are only to display image paths

    // images will have a "content" proprety, where content can be either
    // a icon name (somehow figuere that one out, primarily will be used with system tray)
    // or a path (paths are identified by having a "~" or "/" at the start)
    // i need to make a pixbuf variant, though dont know how ill convert pixbufs from lua to rust
    // and from rust to lua

    // TODO: make it so that it recognizes that content is either
    // a icon name (something like gtk somehow?) or path name
    // or maybe an id somehow for system tray
    // wehre system tray makes id's for icons and those get transferred to a compatible format
    let mut path = PathBuf::from("/");
    for text in data.get::<mlua::String>("content").unwrap().to_string_lossy().split("/") {
        if text == "~" {
            path.push(HOME_DIR.to_str().unwrap());
        } else {
            path.push(text);
        }
    }

    let mut image_widget: cosmic::widget::Image<image::Handle> = image::Image::new(image::Handle::from_path(path));


    // covers Fill and Shrink
    if let Ok(width) = data.get::<mlua::String>("width") {
        image_widget = image_widget.width(match width.to_string_lossy().as_str() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(width) = data.get::<mlua::Table>("width") {
        // covers FillPortion(i32) and Fixed(u32)
        let width_type: mlua::String = width.get(1).unwrap();
        let width_contents: mlua::Number = width.get(2).unwrap();

        image_widget = image_widget.width(match width_type.to_string_lossy().as_str() {
            "fill_portion" => Length::FillPortion(width_contents as u16),
            "fixed" => Length::Fixed(width_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(height) = data.get::<mlua::String>("height") {
        image_widget = image_widget.height(match height.to_string_lossy().as_str() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(height) = data.get::<mlua::Table>("height") {
        // covers FillPortion(i32) and Fixed(u32)
        let height_type: mlua::String = height.get(1).unwrap();
        let height_contents: mlua::Number = height.get(2).unwrap();

        image_widget = image_widget.height(match height_type.to_string_lossy().as_str() {
            "fill_portion" => Length::FillPortion(height_contents as u16),
            "fixed" => Length::Fixed(height_contents as f32),
            _ => Length::Shrink
        });
    }


    if let Ok(content_fit) = data.get::<mlua::String>("content_fit") {
        image_widget = image_widget.content_fit(match content_fit.to_string_lossy().as_str() {
            "contain" => ContentFit::Contain,
            "cover" => ContentFit::Cover,
            "fill" => ContentFit::Fill,
            "none" => ContentFit::None,
            "scale_down" => ContentFit::ScaleDown,
            _ => ContentFit::Contain,
        });
    }

    if let Ok(border_radius) = data.get::<mlua::Number>("border_radius") {
        image_widget = image_widget.border_radius([border_radius as f32; 4]);
        debug!("set border raidus to {:?}", [border_radius as f32; 4]);
    } else if let Ok(border_radius) = data.get::<mlua::Table>("border_radius") {
        image_widget = image_widget.border_radius([
            border_radius.get::<mlua::Number>(1).unwrap() as f32,
            border_radius.get::<mlua::Number>(2).unwrap() as f32,
            border_radius.get::<mlua::Number>(3).unwrap() as f32,
            border_radius.get::<mlua::Number>(4).unwrap() as f32,
        ]);
    }

    if let Ok(filter_method) = data.get::<mlua::String>("filter_method"){
        image_widget = image_widget.filter_method(match filter_method.to_string_lossy().as_str() {
            "linear" => FilterMethod::Linear,
            "nearest" => FilterMethod::Nearest,
            &_ => unimplemented!("Filter method not supported, are you sure its not a typo?")
        })
    }

    if let Ok(rotation) = data.get::<mlua::Table>("rotation") {
        let rotation_type: mlua::String = rotation.get(1).unwrap();
        let radians: mlua::Number = rotation.get(2).unwrap();

        image_widget = image_widget.rotation(match rotation_type.to_string_lossy().as_str() {
            "floating" => Rotation::Floating(Radians(radians as f32)),
            "solid" => Rotation::Solid(Radians(radians as f32)),
            &_ => unimplemented!("Rotation type not supported")
        });
    }

    if let Ok(opacity) = data.get::<mlua::Number>("opacity") {
        image_widget = image_widget.opacity(opacity as f32);
    }

    image_widget

}
