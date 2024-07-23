// processes lua tables into widgets

use button::lua_button_widget;
use container::lua_container_widget;
use iced::Element;
use row::lua_row_widget;
use text::lua_text_widget;

use crate::app::WindowMessages;


mod text;
mod button;
mod custom;
mod row;
mod container;

pub fn process_lua_element<'a>(
    element: mlua::Table,
    // messages: WindowMessages
) -> Option<Element<'a, WindowMessages>>


{

    let element_name: mlua::String = element.get("widget_name").unwrap();
    // println!("the element is {}", wow.to_str().unwrap());

    match element_name.to_str().unwrap() {
        "text" => Some(lua_text_widget(element).into()),
        "button" => Some(lua_button_widget(element).into()),
        "row" => Some(lua_row_widget(element).into()),
        "container" => Some(lua_container_widget(element).into()),
        _ => None,
    }
}
