use std::path::{Path, PathBuf};
use std::rc::Rc;

use cosmic::iced::{ContentFit, Element, Length, Radius};
use cosmic::widget::icon::{from_name, from_path, icon, IconFallback};
use cosmic::widget::image::{self, FilterMethod};
use cosmic::widget::Space;
use cosmic::Renderer;

pub fn make_space_widget(
    data: mlua::Table
) -> cosmic::widget::Space
{
    cosmic::widget::Space::new(
        // width
        {

            if let Ok(width) = data.get::<mlua::String>("width") {
                match width.to_string_lossy().as_str() {
                    "fill" => Length::Fill,
                    _ => Length::Shrink, // since shrink is default
                }

            } else if let Ok(width) = data.get::<mlua::Table>("width") {
                let width_type: mlua::String = width.get(1).unwrap();
                let width_contents: mlua::Number = width.get(2).unwrap();

                match width_type.to_string_lossy().as_str() {
                    "fill_portion" => Length::FillPortion(width_contents as u16),
                    "fixed" => Length::Fixed(width_contents as f32),
                    _ => Length::Shrink
                }
            } else {
                Length::Shrink
            }
        },
        // height
        {
            if let Ok(height) = data.get::<mlua::String>("height") {
                match height.to_string_lossy().as_str() {
                    "fill" => Length::Fill,
                    _ => Length::Shrink, // since shrink is default
                }

            } else if let Ok(height) = data.get::<mlua::Table>("height") {
                // covers FillPortion(i32) and Fixed(u32)
                let height_type: mlua::String = height.get(1).unwrap();
                let height_contents: mlua::Number = height.get(2).unwrap();

                match height_type.to_string_lossy().as_str() {
                    "fill_portion" => Length::FillPortion(height_contents as u16),
                    "fixed" => Length::Fixed(height_contents as f32),
                    _ => Length::Shrink
                }
            } else {
                Length::Shrink
            }

            // Length::Shrink
        }
    )
}
