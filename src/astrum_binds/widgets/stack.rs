use cosmic::{iced::Length, iced_widget::{scrollable::{Anchor, Direction, Scrollbar}, Scrollable, Stack}, widget::scrollable, Element};

use crate::astrum_core::app::main::AstrumMessages;

use super::process_lua_element;

pub struct StackElement<'a, Message>(Stack<'a, Message, cosmic::Theme>);

pub fn make_stack_widget<'a>(
    data: mlua::Table
) -> StackElement<'a, AstrumMessages> {
    let mut widgets = Vec::new();
    let children_table: mlua::Table = data.get("children").unwrap();

    for pairs in children_table.pairs::<mlua::Integer, mlua::Table>()  {
        let (_, value) = pairs.unwrap();
        let widget = process_lua_element(value);

        if let Some(element) = widget {
            widgets.push(element);
        }
    }

    let mut stack_widget = Stack::with_children(widgets); // maybe use from_vec??

    if let Ok(width) = data.get::<mlua::String>("width") {
        stack_widget = stack_widget.width(match width.to_string_lossy().as_str() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(width) = data.get::<mlua::Table>("width") {
        // covers FillPortion(i32) and Fixed(u32)
        let width_type: mlua::String = width.get(1).unwrap();
        let width_contents: mlua::Number = width.get(2).unwrap();

        stack_widget = stack_widget.width(match width_type.to_string_lossy().as_str() {
            "fill_portion" => Length::FillPortion(width_contents as u16),
            "fixed" => Length::Fixed(width_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(height) = data.get::<mlua::String>("height") {
        stack_widget = stack_widget.height(match height.to_string_lossy().as_str() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(height) = data.get::<mlua::Table>("height") {
        // covers FillPortion(i32) and Fixed(u32)
        let height_type: mlua::String = height.get(1).unwrap();
        let height_contents: mlua::Number = height.get(2).unwrap();

        stack_widget = stack_widget.height(match height_type.to_string_lossy().as_str() {
            "fill_portion" => Length::FillPortion(height_contents as u16),
            "fixed" => Length::Fixed(height_contents as f32),
            _ => Length::Shrink
        });
    }

   StackElement(stack_widget)
}

impl<'a, Message> From<StackElement<'a, Message>>
    for Element<'a, Message>
where
    Message: 'static + Clone
{
    fn from(value: StackElement<'a, Message>) -> Self {
        Element::new(value.0)
    }
}
