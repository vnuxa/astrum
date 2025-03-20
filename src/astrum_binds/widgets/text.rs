use cosmic::{cosmic_theme::palette::{Alpha, Srgba}, iced::{color, font::{Family, Style as FontStyle, Weight}, Font, Length}, iced_core::text::LineHeight, iced_widget::text::StyleFn, Theme};
use cosmic::iced_core::widget::text::Style;

use crate::astrum_binds::style::{from_colors, text::lua_text_style};

use super::make_static_str;


pub fn make_text_widget<'a>(
    data: mlua::Table
) -> cosmic::widget::Text<'a, cosmic::Theme>
{
    let widget_content: mlua::String = data.get("content").unwrap();

    let mut text_widget: cosmic::widget::Text<cosmic::Theme, cosmic::Renderer> = cosmic::widget::text(widget_content.to_string_lossy());

    // covers Fill and Shrink
    if let Ok(width) = data.get::<mlua::String>("width") {
        text_widget = text_widget.width(match width.to_string_lossy().as_str() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(width) = data.get::<mlua::Table>("width") {
        // covers FillPortion(i32) and Fixed(u32)
        let width_type: mlua::String = width.get(1).unwrap();
        let width_contents: mlua::Number = width.get(2).unwrap();

        text_widget = text_widget.width(match width_type.to_string_lossy().as_str() {
            "fill_portion" => Length::FillPortion(width_contents as u16),
            "fixed" => Length::Fixed(width_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(height) = data.get::<mlua::String>("height") {
        text_widget = text_widget.height(match height.to_string_lossy().as_str() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(height) = data.get::<mlua::Table>("height") {
        // covers FillPortion(i32) and Fixed(u32)
        let height_type: mlua::String = height.get(1).unwrap();
        let height_contents: mlua::Number = height.get(2).unwrap();

        text_widget = text_widget.height(match height_type.to_string_lossy().as_str() {
            "fill_portion" => Length::FillPortion(height_contents as u16),
            "fixed" => Length::Fixed(height_contents as f32),
            _ => Length::Shrink
        });
    }

    if let Ok(horizontal_alignment) = data.get::<mlua::String>("align_x") {
        text_widget = text_widget.align_x(match horizontal_alignment.to_string_lossy().as_str() {
            "center" => cosmic::iced::alignment::Horizontal::Center,
            "right" => cosmic::iced::alignment::Horizontal::Right,
            _ => cosmic::iced::alignment::Horizontal::Left
        });
    }

    if let Ok(vertical_alignment) = data.get::<mlua::String>("align_y") {
        text_widget = text_widget.align_y(match vertical_alignment.to_string_lossy().as_str() {
            "center" => cosmic::iced::alignment::Vertical::Center,
            "bottom" => cosmic::iced::alignment::Vertical::Bottom,
            _ => cosmic::iced::alignment::Vertical::Top,
        });
    }

    if let Ok(style) = data.get::<mlua::Table>("style") {
        text_widget = text_widget.class(lua_text_style(style));
    }

    if let Ok(font_settings) = data.get::<mlua::Table>("font") {
        let mut font_family: Option<Family> = None;
        let mut font_weight: Option<Weight> = None;
        let mut font_style: Option<FontStyle> = None;

        if let Ok(font_name) = font_settings.get::<mlua::String>("name") {
            unsafe {
                font_family = Some(Family::Name(make_static_str(&*font_name.to_str().unwrap())));
            }
        }
        // println!("before weight");
        if let Ok(weight) = font_settings.get::<mlua::String>("weight") {
            font_weight = Some(match weight.to_string_lossy().as_str() {
                "thin" => Weight::Thin,
                "extra_light" => Weight::ExtraLight,
                "light" => Weight::Light,
                "normal" => Weight::Normal,
                "medium" => Weight::Medium,
                "semibold" => Weight::Semibold,
                "bold" => Weight::Bold,
                "extrabold" => Weight::ExtraBold,
                "black" => Weight::Black,
                _ => Weight::Normal
            });
        }
        // println!("post weight");
        if let Ok(style) = font_settings.get::<mlua::String>("style") {
            // println!("got style: {}", style.to_str().unwrap().to_string());
            font_style = Some(match style.to_string_lossy().as_str() {
                "italic" => FontStyle::Italic,
                "oblique" => FontStyle::Oblique,
                _ => FontStyle::Normal,
            });
        }

        text_widget = text_widget.font(Font {
            // family: Family::Name("EPSON 正楷書体Ｍ"),
            // family: Family::Name("Torus"),
            family: font_family.unwrap(),
            weight: font_weight.unwrap_or_default(),
            style: font_style.unwrap_or_default(),
            ..Default::default()
        });
    }
    if let Ok(size) = data.get::<mlua::Number>("size") {
        text_widget = text_widget.size(size as f32);
    }

    if let Ok(height) = data.get::<mlua::Number>("line_height") {
        text_widget = text_widget.line_height(LineHeight::Absolute(cosmic::iced::Pixels((height as f32))));
    }

    text_widget

}

// impl<'a> From<StyleFn<'a, cosmic::Theme>> for cosmic::theme::Text {
//
//     fn from(value: StyleFn<'a, cosmic::Theme>) -> Self {
//         Self::Custom(*value)
//     }
// }
//
// impl From<cosmic::theme::Text> for StyleFn<'a, Theme> {
//     fn from(value: cosmic::theme::Text) -> Self {
//         Box::new(value)
//     }
// }
