use button::make_button_widget;
use centerbox::make_centerbox_widget;
use column::make_column_widget;
use container::make_container_widget;
use cosmic::Element;
use icon::make_icon_widget;
use image::make_image_widget;
use mouse_area::make_mouse_area_widget;
use row::make_row_widget;
use scrollable::make_scrollable_widget;
use space::make_space_widget;
use text::make_text_widget;
use text_input::make_text_input_widget;

use crate::astrum_core::app::main::AstrumMessages;

mod text;
mod button;
mod custom;
mod centerbox;
mod row;
mod column;
mod image;
mod icon;
mod text_input;
mod container;
mod space;
mod scrollable;
mod mouse_area;


// makes lua bindings for elements into rust ones
pub fn process_lua_element<'a>(
    element: mlua::Table
) -> Option<Element<'a, AstrumMessages>>
{
    let element_name: mlua::String = element.get("widget_name").unwrap();

    match element_name.to_string_lossy().as_str() {
        "text" => Some(make_text_widget(element).into()),
        "button" => Some(make_button_widget(element).into()),
        "centerbox" => Some(make_centerbox_widget(element).into()),
        "container" => Some(make_container_widget(element).into()),
        "row" => Some(make_row_widget(element).into()),
        "column" => Some(make_column_widget(element).into()),
        "image" => Some(make_image_widget(element).into()),
        "icon" => Some(make_icon_widget(element).into()),
        "text_input" => Some(make_text_input_widget(element).into()),
        "space" => Some(make_space_widget(element).into()),
        "scrollable" => Some(make_scrollable_widget(element).into()),
        "mouse_area" => Some(make_mouse_area_widget(element).into()),
        &_ => None,
    }
}

// for dealing with static strings with dynamic lua
// dont think theres another way to do it, but if theres a solution that doesnt require this
// lmk
pub unsafe fn make_static_str<'a>(key: &'a str) -> &'static str {
    std::mem::transmute::<&'a str, &'static str>(key)
}
