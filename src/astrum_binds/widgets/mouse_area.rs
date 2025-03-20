use cosmic::{iced::{alignment::Vertical, color, font::{Family, Style, Weight}, mouse::ScrollDelta, Alignment, Font, Length}, widget::{MouseArea, Row}, Theme};

use crate::astrum_core::app::main::AstrumMessages;

use super::process_lua_element;

pub fn make_mouse_area_widget<'a>(
    data: mlua::Table
) -> MouseArea<'a, AstrumMessages, Theme>{
    let child: mlua::Table = data.get("child").unwrap();
    let mut mouse_area_widget = MouseArea::new(process_lua_element(child).unwrap());


    macro_rules! add_signal {
        ($name:ident) => {
            if let Ok($name) = data.get::<mlua::String>(stringify!($name)) {
                mouse_area_widget = mouse_area_widget.$name(AstrumMessages::Msg(($name.to_string_lossy(), "{}".to_string())));
            } else if let Ok($name) = data.get::<mlua::Table>(stringify!($name)) {
                mouse_area_widget = mouse_area_widget.$name(AstrumMessages::Msg(
                    (
                        $name.get::<mlua::String>("signal_name").unwrap().to_string_lossy(),
                        $name.get::<mlua::String>("signal_data").unwrap().to_string_lossy(),
                    )
                ));
            }
        };
    }

    add_signal!(on_press);
    add_signal!(on_release);
    add_signal!(on_drag);
    add_signal!(on_double_click);
    add_signal!(on_enter);
    add_signal!(on_exit);
    add_signal!(on_middle_press);
    // TODO: maybe add the on_move event later


    if let Ok(on_scroll) = data.get::<mlua::String>("on_scroll") {
        mouse_area_widget = mouse_area_widget.on_scroll(move |delta| {
            if let ScrollDelta::Pixels{ x, y } = delta {
                if y > 0.0 {
                    return AstrumMessages::Msg((on_scroll.to_string_lossy(), "{ direction = 'up' }".to_string()));
                } else {
                    return AstrumMessages::Msg((on_scroll.to_string_lossy(), "{ direction = 'down' }".to_string()));
                }
            }
            AstrumMessages::AnimationTick
        });
    }

    mouse_area_widget
}
