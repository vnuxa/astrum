use cosmic::{cosmic_theme::palette::{Alpha, Srgba}, iced::{color, font::{Family, Style, Weight}, Font, Length}, iced_core::text::LineHeight, iced_winit::graphics::core::id::Id};

use crate::{astrum_binds::style::text_input::lua_text_input_style, astrum_core::app::main::AstrumMessages};

use super::make_static_str;

pub fn make_text_input_widget<'a>(
    data: mlua::Table
) -> cosmic::widget::TextInput<'a, AstrumMessages> {
    let placeholder_text: mlua::String = data.get("placeholder").expect("text input does not have placeholder text");
    let value_text: mlua::String = data.get("value").expect("text input does not have value text");

    let mut text_input_widget: cosmic::widget::TextInput<AstrumMessages> = cosmic::widget::text_input(placeholder_text.to_string_lossy(), value_text.to_string_lossy());

    if let Ok(on_input) = data.get::<mlua::String>("on_input") {
        text_input_widget = text_input_widget.on_input(move |text| {
            return AstrumMessages::Msg((
                on_input.to_string_lossy(),
                format!("{{ text = '{text}' }}", text = text)
            ));
        });
    }

    if let Ok(on_toggle_edit) = data.get::<mlua::String>("on_toggle_edit") {
        text_input_widget = text_input_widget.on_toggle_edit(move |focus| {
            return AstrumMessages::Msg((
                on_toggle_edit.to_string_lossy(),
                format!("{{ focused = '{boolean}' }}", boolean = focus)
            ));
        });
    }

    if let Ok(on_submit) = data.get::<mlua::String>("on_submit") {
        text_input_widget = text_input_widget.on_submit(move |_|AstrumMessages::Msg((on_submit.to_string_lossy(), "{}".to_string())));
    } else if let Ok(on_submit) = data.get::<mlua::Table>("on_submit") {
        text_input_widget = text_input_widget.on_submit(move |_|AstrumMessages::Msg(
            (
                on_submit.get::<mlua::String>("signal_name").unwrap().to_string_lossy(),
                on_submit.get::<mlua::String>("signal_data").unwrap().to_string_lossy(),
            )
        ));
    }
    if let Ok(id) = data.get::<mlua::String>("id") {
        unsafe {
            text_input_widget = text_input_widget.id(Id::new(make_static_str(&id.to_string_lossy())));
        }
    }


    if let Ok(padding) = data.get::<mlua::Number>("padding") {
        text_input_widget = text_input_widget.padding(padding as f32);
    } else  if let Ok(padding) = data.get::<mlua::Table>("padding") {
        let mut padding_list: Vec<f32> = Vec::new();
        for pair in padding.pairs::<mlua::Number, f32>() {
            let (_key, value): (mlua::Number, f32) = pair.unwrap();
            padding_list.push(value);
        }
        match padding_list.len() {
            2 => {
                text_input_widget = text_input_widget.padding(TryInto::<[f32; 2]>::try_into(padding_list).unwrap());
            },
            4 => {
                text_input_widget = text_input_widget.padding(TryInto::<[f32; 4]>::try_into(padding_list).unwrap());
            },
            _ => {
                panic!("Text input has unsupported padding array size!");
            }
        };
    }

    if let Ok(width) = data.get::<mlua::String>("width") {
        text_input_widget = text_input_widget.width(match width.to_string_lossy().as_str() {
            "fill" => Length::Fill,
            _ => Length::Shrink, // since shrink is default
        });

    } else if let Ok(width) = data.get::<mlua::Table>("width") {
        // covers FillPortion(i32) and Fixed(u32)
        let width_type: mlua::String = width.get(1).unwrap();
        let width_contents: mlua::Number = width.get(2).unwrap();

        text_input_widget = text_input_widget.width(match width_type.to_string_lossy().as_str() {
            "fill_portion" => Length::FillPortion(width_contents as u16),
            "fixed" => Length::Fixed(width_contents as f32),
            _ => Length::Shrink
        });
    }


    if data.get::<bool>("always_active").unwrap_or(false) {
        text_input_widget = text_input_widget.always_active();
    }

    if data.get::<bool>("password").unwrap_or(false) {
        text_input_widget = text_input_widget.password();
    }

    if let Ok(line_height) = data.get::<mlua::Table>("line_height") {
        let first_param: mlua::String = line_height.get(1).unwrap();
        let second_param: mlua::Number = line_height.get(2).unwrap();

        text_input_widget = text_input_widget.line_height(match first_param.to_string_lossy().as_str() {
            "relative" => {
                LineHeight::Relative(second_param as f32)
            },
            "absolute" => {
                LineHeight::Absolute(cosmic::iced::Pixels(second_param as f32))
            }
            &_ => unimplemented!("Text input line height not supported")
        });
    }

    if let Ok(size) = data.get::<mlua::Number>("size") {
        text_input_widget = text_input_widget.size(size as f32);
    }

        if let Ok(font_settings) = data.get::<mlua::Table>("font") {
        // let font_settings = Rc::new(font_settings);
        // let mut current_font = Font::default();
        let mut font_family: Option<Family> = None;
        let mut font_weight: Option<Weight> = None;
        let mut font_style: Option<Style> = None;

        if let Ok(font_name) = font_settings.get::<mlua::String>("name") {
            unsafe {
                font_family = Some(Family::Name(make_static_str(&*font_name.to_str().unwrap())));
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
                "normal" => Style::Normal,
                "italic" => Style::Italic,
                "oblique" => Style::Oblique,
                _ => Style::Normal,
            });
        }

        text_input_widget= text_input_widget.font(Font {
            family: font_family.unwrap_or_default(),
            weight: font_weight.unwrap_or_default(),
            style: font_style.unwrap_or_default(),
            ..Default::default()
        });
    }

    if let Ok(style) = data.get::<mlua::Table>("style") {
        text_input_widget = text_input_widget.style(lua_text_input_style(style));
    }

    text_input_widget
}
