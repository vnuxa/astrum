// processes lua tables into widgets

use std::cell::Ref;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use column::lua_column_widget;
use cosmic::Element;
use button::lua_button_widget;
use centerbox::lua_centerbox_widget;
use container::lua_container_widget;
use icon::lua_icon_widget;
use image::lua_image_widget;
use row::lua_row_widget;
use scrollable::lua_scrollable_widget;
use space::lua_space_widget;
use text::lua_text_widget;
use text_input::lua_text_input_widget;

use crate::app::WindowMessages;


mod text;
mod button;
mod custom;
mod row;
mod column;
mod container;
mod centerbox;
mod scrollable;
mod text_input;
mod icon;
mod image;
mod space;

pub fn process_lua_element(
    element: mlua::Table,
    // mut identifiers: Rc<HashMap<String, String>>
    // messages: WindowMessages
) -> Option<(Element<WindowMessages>)>
{
    let element_name: mlua::String = element.get("widget_name").unwrap();
    // println!("the element is {}", wow.to_str().unwrap());

    match element_name.to_str().unwrap() {
        "text" => Some(lua_text_widget(element).into()),
        "button" => Some(lua_button_widget(element).into()),
        "row" => Some(lua_row_widget(element).into()),
        "container" => Some(lua_container_widget(element).into()),
        "centerbox" => Some(lua_centerbox_widget(element).into()),
        "scrollable" => Some(lua_scrollable_widget(element).into()),
        "column" => Some(lua_column_widget(element).into()),
        "text_input" => Some(lua_text_input_widget(element).into()),
        "icon" => Some(lua_icon_widget(element).into()),
        "image" => Some(lua_image_widget(element).into()),
        "space" => Some(lua_space_widget(element).into()),
        _ => None,
    }
}
