use cosmic::iced::{color, font::{Family, Style, Weight}, Font, Length};

use crate::astrum_core::app::main::AstrumMessages;

use super::process_lua_element;



pub fn make_button_widget<'a>(
    data: mlua::Table
) -> cosmic::widget::Button<'a, AstrumMessages>
{
    let widget_child: mlua::Table = data.get("child").unwrap();
    let mut button_widget = cosmic::widget::Button::new_image(process_lua_element(widget_child).unwrap(), None);

    // covers Fill and Shrink
    if let Ok(width) = data.get::<mlua::String>("width") {
        button_widget = button_widget.width(match width.to_string_lossy().as_str() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(width) = data.get::<mlua::Table>("width") {
        // covers FillPortion(i32) and Fixed(u32)
        let width_type: mlua::String = width.get(1).unwrap();
        let width_contents: mlua::Number = width.get(2).unwrap();

        button_widget = button_widget.width(match width_type.to_string_lossy().as_str() {
            "fill_portion" => Length::FillPortion(width_contents as u16),
            "fixed" => Length::Fixed(width_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(height) = data.get::<mlua::String>("height") {
        button_widget = button_widget.height(match height.to_string_lossy().as_str() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(height) = data.get::<mlua::Table>("height") {
        // covers FillPortion(i32) and Fixed(u32)
        let height_type: mlua::String = height.get(1).unwrap();
        let height_contents: mlua::Number = height.get(2).unwrap();

        button_widget = button_widget.height(match height_type.to_string_lossy().as_str() {
            "fill_portion" => Length::FillPortion(height_contents as u16),
            "fixed" => Length::Fixed(height_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(on_press) = data.get::<mlua::String>("on_press") {
        button_widget = button_widget.on_press(AstrumMessages::Msg((on_press.to_string_lossy(), "{}".to_string())));
    } else if let Ok(on_press) = data.get::<mlua::Table>("on_press") {
        button_widget = button_widget.on_press(AstrumMessages::Msg(
            (
                on_press.get::<mlua::String>("signal_name").unwrap().to_string_lossy(),
                on_press.get::<mlua::String>("signal_data").unwrap().to_string_lossy(),
            )
        ));
    }

    // ifl
    button_widget
}
