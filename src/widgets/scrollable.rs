use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use cosmic::iced::{Element, Length};
use cosmic::iced_widget::scrollable::{Direction, Properties};
use cosmic::iced_widget::Scrollable;
use cosmic::widget::scrollable;
use cosmic::Renderer;

use crate::app::WindowMessages;

use super::process_lua_element;

pub fn lua_scrollable_widget(
    data: mlua::Table,
    // mut identifiers: Rc<HashMap<String, String>>
) -> Scrollable<WindowMessages, cosmic::Theme>{
    let widget_child: mlua::Table = data.get("child").unwrap();

    let mut scrollable_widget = scrollable(process_lua_element(widget_child).unwrap());

    if let Ok(width) = data.get::<_, mlua::String>("width") {
        scrollable_widget = scrollable_widget.width(match width.to_str().unwrap() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(width) = data.get::<_, mlua::Table>("width") {
        // covers FillPortion(i32) and Fixed(u32)
        let width_type: mlua::String = width.get(1).unwrap();
        let width_contents: mlua::Number = width.get(2).unwrap();

        scrollable_widget = scrollable_widget.width(match width_type.to_str().unwrap() {
            "fill_portion" => Length::FillPortion(width_contents as u16),
            "fixed" => Length::Fixed(width_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(height) = data.get::<_, mlua::String>("height") {
        scrollable_widget = scrollable_widget.height(match height.to_str().unwrap() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(height) = data.get::<_, mlua::Table>("height") {
        // covers FillPortion(i32) and Fixed(u32)
        let height_type: mlua::String = height.get(1).unwrap();
        let height_contents: mlua::Number = height.get(2).unwrap();

        scrollable_widget = scrollable_widget.height(match height_type.to_str().unwrap() {
            "fill_portion" => Length::FillPortion(height_contents as u16),
            "fixed" => Length::Fixed(height_contents as f32),
            _ => Length::Shrink
        });
    }


    if let Ok(direction) = data.get::<_, mlua::Table>("direction") {
        if let Ok(scroll_type) = direction.get(1) {
            let scroll_type: mlua::String = scroll_type;

            fn get_propreties(scroll_propreties: mlua::Table) -> Properties {
                let propreties: Properties = Properties::new();

                if let Ok(width) = scroll_propreties.get::<_, mlua::Integer>("width") {
                    propreties.width(width as f32);
                }
                if let Ok(margin) = scroll_propreties.get::<_, mlua::Integer>("margin") {
                    propreties.margin(margin as f32);
                }
                if let Ok(scrollers_width) = scroll_propreties.get::<_, mlua::Integer>("scrollers_width") {
                    propreties.scroller_width(scrollers_width as f32);
                }
                if let Ok(alignment) = scroll_propreties.get::<_, mlua::String>("alignment") {
                    propreties.alignment(match alignment.to_str().unwrap() {
                        "end" => cosmic::iced_widget::scrollable::Alignment::End,
                        _ => cosmic::iced_widget::scrollable::Alignment::Start,
                    });
                }

                propreties
            }

            let scroll_direction = match scroll_type.to_str().unwrap() {
                "vertical" => Direction::Vertical(get_propreties(direction.get::<_, mlua::Table>(2).unwrap())),
                "horizontal" => Direction::Horizontal(get_propreties(direction.get::<_, mlua::Table>(2).unwrap())),
                "both" => {
                    let both = direction.get::<_, mlua::Table>(2).unwrap();
                    Direction::Both { vertical: get_propreties(both.get("vertical").unwrap()), horizontal: get_propreties(both.get("horizontal").unwrap()) }
                }
                _ => unimplemented!("Scroll direction not supported")
            };

            scrollable_widget = scrollable_widget.direction(scroll_direction);
        }
    }


    scrollable_widget
}
