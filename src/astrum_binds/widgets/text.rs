use cosmic::{cosmic_theme::palette::{Alpha, Srgba}, iced::{color, font::{Family, Style as FontStyle, Weight}, Font, Length}, iced_widget::text::StyleFn, Theme};
use cosmic::iced_core::widget::text::Style;

use crate::astrum_binds::style::from_colors;

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

    // if let Ok(style) = data.get::<mlua::Table>("style"){

        // text_widget = text_widget.color(cosmic::iced::Color { r: 1.0, g: 0.0, b: 0.0, a: 0.0 });
    // }

    // if let Ok(style) = data.get::<mlua::Table>("style") {
        // text_widget = text_widget.style()

        // cosmic::iced_widget::text::S
        // text_widget = text_widget.style(cosmic::iced_winit::graphics::core::widget::text::Style )
        // text_widget = text_widget.style(move |_theme| cosmic::iced_widget::text::Style { color: Some(from_colors(style.get::<mlua::Table>("text_color").unwrap())) });
        // text_widget = text_widget.style(move |_theme| cosmic::theme::style::Text::Color(color!(0x0000ff)));
        // text_widget = text_widget.style(move |_theme| cosmic::iced_winit::graphics::core::widget::text::Style { color: Some(cosmic::iced_core::Color::new(1.0, 0.0, 0.0, 1.0).into()) });
        // text_widget = text_widget.style(move |_theme| cosmic::iced_widget::text::Style {
        //     color: Some(Alpha {
        //         color: Srgba::new(171u8, 193, 35, 128),
        //         alpha: 1.0
        //     })
        // });
        // text_widget.class(class)

        // text_widget.color(cosmic::iced_winit::graphics::core::Color::WHITE);
        // text_widget = text_widget.color(from_colors(style.get::<mlua::Table>("text_color").expect("Failed to get text_color for text widget")));
        // text_widget = text_widget.class((Box::new(move |_theme| Style { color: Some(color!(0x0000ff)) }) as StyleFn<'a, Theme>).into());

        // text_widget = text_widget.class(Box::new(|_theme| color!(0x0000ff)) as StyleFn<'a, Theme>);
        // text_widget = text_widget.color(color!(0x0000ff));
    // }

    if let Ok(font_settings) = data.get::<mlua::Table>("font") {
        let mut font_family: Option<Family> = None;
        let mut font_weight: Option<Weight> = None;
        let mut font_style: Option<FontStyle> = None;

        if let Ok(font_name) = font_settings.get::<mlua::String>("name") {
            // for some reason font name is static???
            // and its either i leak memory or use unsafe code
            unsafe {
                font_family = Some(Family::Name(make_static_str(font_name.to_string_lossy().as_str())));
            }
        }
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
        if let Ok(style) = font_settings.get::<mlua::String>("style") {
            font_style = Some(match style.to_string_lossy().as_str() {
                "normal" => FontStyle::Normal,
                "italic" => FontStyle::Italic,
                "oblique" => FontStyle::Oblique,
                _ => FontStyle::Normal,
            });
        }

        text_widget = text_widget.font(Font {
            family: font_family.unwrap_or_default(),
            weight: font_weight.unwrap_or_default(),
            style: font_style.unwrap_or_default(),
            ..Default::default()
        });
    }
    if let Ok(size) = data.get::<mlua::Number>("size") {
        text_widget = text_widget.size(size as f32);
    }

    text_widget

}
