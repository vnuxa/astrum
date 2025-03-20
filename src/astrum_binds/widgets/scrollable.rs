use cosmic::{iced::Length, iced_widget::{scrollable::{Anchor, Direction, Scrollbar}, Scrollable}, widget::scrollable};

use crate::astrum_core::app::main::AstrumMessages;

use super::process_lua_element;


// TODO: ? scrollable theming

pub fn make_scrollable_widget<'a>(
    data: mlua::Table

) -> Scrollable<'a, AstrumMessages, cosmic::Theme>{
    let widget_child: mlua::Table = data.get("child").unwrap();

    let mut scrollable_widget = scrollable(process_lua_element(widget_child).unwrap());

    if let Ok(width) = data.get::<mlua::String>("width") {
        scrollable_widget = scrollable_widget.width(match width.to_string_lossy().as_str() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(width) = data.get::<mlua::Table>("width") {
        // covers FillPortion(i32) and Fixed(u32)
        let width_type: mlua::String = width.get(1).unwrap();
        let width_contents: mlua::Number = width.get(2).unwrap();

        scrollable_widget = scrollable_widget.width(match width_type.to_string_lossy().as_str() {
            "fill_portion" => Length::FillPortion(width_contents as u16),
            "fixed" => Length::Fixed(width_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(height) = data.get::<mlua::String>("height") {
        scrollable_widget = scrollable_widget.height(match height.to_string_lossy().as_str() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(height) = data.get::<mlua::Table>("height") {
        // covers FillPortion(i32) and Fixed(u32)
        let height_type: mlua::String = height.get(1).unwrap();
        let height_contents: mlua::Number = height.get(2).unwrap();

        scrollable_widget = scrollable_widget.height(match height_type.to_string_lossy().as_str() {
            "fill_portion" => Length::FillPortion(height_contents as u16),
            "fixed" => Length::Fixed(height_contents as f32),
            _ => Length::Shrink
        });
    }


    if let Ok(direction) = data.get::<mlua::Table>("direction") {
        if let Ok(scroll_type) = direction.get(1) {
            let scroll_type: mlua::String = scroll_type;
            // println!("got scroll type {:?}", scroll_type);

            fn get_propreties(scroll_propreties: mlua::Table) -> Scrollbar {
                // println!("getting propreties now..");
                let mut scrollbar: Scrollbar = Scrollbar::new();

                if let Ok(width) = scroll_propreties.get::<mlua::Integer>("width") {
                    scrollbar = scrollbar.width(width as f32);
                }
                if let Ok(margin) = scroll_propreties.get::<mlua::Integer>("margin") {
                    scrollbar = scrollbar.margin(margin as f32);
                }
                if let Ok(scrollers_width) = scroll_propreties.get::<mlua::Integer>("scrollers_width") {
                    scrollbar = scrollbar.scroller_width(scrollers_width as f32);
                }
                if let Ok(anchor) = scroll_propreties.get::<mlua::String>("anchor") {
                    scrollbar = scrollbar.anchor(match anchor.to_string_lossy().as_str() {
                        "end" => Anchor::End,
                        _ => Anchor::Start,
                    })
                }

                scrollbar
            }

            let scroll_direction = match scroll_type.to_string_lossy().as_str() {
                "vertical" => Direction::Vertical(get_propreties(direction.get::<mlua::Table>(2).unwrap())),
                "horizontal" => Direction::Horizontal(get_propreties(direction.get::<mlua::Table>(2).unwrap())),
                "both" => {
                    let both = direction.get::<mlua::Table>(2).unwrap();
                    Direction::Both { vertical: get_propreties(both.get("vertical").unwrap()), horizontal: get_propreties(both.get("horizontal").unwrap()) }
                }
                _ => unimplemented!("Scroll direction not supported")
            };

            scrollable_widget = scrollable_widget.direction(scroll_direction);
        }
    }




    scrollable_widget


}
