use std::{borrow::Cow, path::PathBuf};

use cosmic::{cosmic_theme::palette::{Alpha, Srgba}, iced::{color, font::{Family, Style, Weight}, ContentFit, Font, Length}, widget::icon::{from_name, from_path, from_raster_pixels}};
use system_tray::item::IconPixmap;

use crate::astrum_core::{services::system_tray::ICON_PIXMAPS, HOME_DIR};

pub fn make_icon_widget(
    data: mlua::Table
) -> cosmic::widget::Icon
{
    // images will have a "content" proprety, where content can be either
    // a icon name (somehow figuere that one out, primarily will be used with system tray)
    // or a path (paths are identified by having a "~" or "/" at the start)
    // i need to make a pixbuf variant, though dont know how ill convert pixbufs from lua to rust
    // and from rust to lua

    // TODO: make it so that it recognizes that content is either
    // a icon name (something like gtk somehow?) or path name
    // or maybe an id somehow for system tray
    // wehre system tray makes id's for icons and those get transferred to a compatible format

    let mut icon_widget: cosmic::widget::Icon;
    if let Ok(icon_path) = data.get::<mlua::String>("icon_path") {
        let mut path = PathBuf::from("/");

        for text in icon_path.to_string_lossy().split("/") {
            if text == "~" {
                path.push(HOME_DIR.to_str().unwrap());
            } else {
                path.push(text);
            }
        }

        icon_widget = cosmic::widget::icon(from_path(path));
    } else if let Ok(icon_pixmap) = data.get::<mlua::String>("icon_pixmap") {
        let pixmaps = ICON_PIXMAPS.lock().unwrap();
        let pixmap: &IconPixmap = pixmaps.get(&icon_pixmap.to_string_lossy()).unwrap();

        icon_widget = cosmic::widget::icon(from_raster_pixels(pixmap.width as u32, pixmap.height as u32, Cow::Owned(pixmap.pixels.clone())));
    } else {
        let icon_name = data.get::<mlua::String>("icon_name").unwrap();

        icon_widget = cosmic::widget::icon(from_name(icon_name.to_string_lossy()).into());
    }

    // covers Fill and Shrink
    if let Ok(width) = data.get::<mlua::String>("width") {
        icon_widget = icon_widget.width(match width.to_string_lossy().as_str() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(width) = data.get::<mlua::Table>("width") {
        // covers FillPortion(i32) and Fixed(u32)
        let width_type: mlua::String = width.get(1).unwrap();
        let width_contents: mlua::Number = width.get(2).unwrap();

        icon_widget = icon_widget.width(match width_type.to_string_lossy().as_str() {
            "fill_portion" => Length::FillPortion(width_contents as u16),
            "fixed" => Length::Fixed(width_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(height) = data.get::<mlua::String>("height") {
        icon_widget = icon_widget.height(match height.to_string_lossy().as_str() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(height) = data.get::<mlua::Table>("height") {
        // covers FillPortion(i32) and Fixed(u32)
        let height_type: mlua::String = height.get(1).unwrap();
        let height_contents: mlua::Number = height.get(2).unwrap();

        icon_widget = icon_widget.height(match height_type.to_string_lossy().as_str() {
            "fill_portion" => Length::FillPortion(height_contents as u16),
            "fixed" => Length::Fixed(height_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(content_fit) = data.get::<mlua::String>("content_fit") {
        icon_widget = icon_widget.content_fit(match content_fit.to_string_lossy().as_str() {
            "contain" => ContentFit::Contain,
            "cover" => ContentFit::Cover,
            "fill" => ContentFit::Fill,
            "none" => ContentFit::None,
            "scale_down" => ContentFit::ScaleDown,
            _ => ContentFit::Contain,
        });
    }

    if let Ok(size) = data.get::<mlua::Number>("size") {
        icon_widget = icon_widget.size(size as u16);
    }

    icon_widget

}
