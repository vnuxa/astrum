use iced::{Element, Length};


pub fn lua_text_widget<'a>(
    data: mlua::Table
) -> iced::widget::Text<'a> {

    let widget_content: mlua::String = data.get("content").unwrap();
    let mut text_widget = iced::widget::text(widget_content.to_str().unwrap());


    // covers Fill and Shrink
    if let Ok(width) = data.get::<_, mlua::String>("width") {
        text_widget = text_widget.width(match width.to_str().unwrap() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(width) = data.get::<_, mlua::Table>("width") {
        // covers FillPortion(i32) and Fixed(u32)
        let width_type: mlua::String = width.get(1).unwrap();
        let width_contents: mlua::Number = width.get(2).unwrap();

        text_widget = text_widget.width(match width_type.to_str().unwrap() {
            "fill_portion" => Length::FillPortion(width_contents as u16),
            "fixed" => Length::Fixed(width_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(height) = data.get::<_, mlua::String>("height") {
        text_widget = text_widget.height(match height.to_str().unwrap() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(height) = data.get::<_, mlua::Table>("height") {
        // covers FillPortion(i32) and Fixed(u32)
        let height_type: mlua::String = height.get(1).unwrap();
        let height_contents: mlua::Number = height.get(2).unwrap();

        text_widget = text_widget.height(match height_type.to_str().unwrap() {
            "fill_portion" => Length::FillPortion(height_contents as u16),
            "fixed" => Length::Fixed(height_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(horizontal_alignment) = data.get::<_, mlua::String>("horizontal_alignment") {
        text_widget = text_widget.horizontal_alignment(match horizontal_alignment.to_str().unwrap() {
            "center" => iced::alignment::Horizontal::Center,
            "right" => iced::alignment::Horizontal::Right,
            _ => iced::alignment::Horizontal::Left
        });
    }

    if let Ok(vertical_alignment) = data.get::<_, mlua::String>("vertical_alignment") {
        text_widget = text_widget.vertical_alignment(match vertical_alignment.to_str().unwrap() {
            "center" => iced::alignment::Vertical::Center,
            "bottom" => iced::alignment::Vertical::Bottom,
            _ => iced::alignment::Vertical::Top,
        });
    }


    text_widget
}
