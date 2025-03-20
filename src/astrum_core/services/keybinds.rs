use std::{hash::Hash, sync::{Arc, Mutex}};

use cosmic::{iced::{futures::Stream, keyboard::{key::{Code, Named, Physical}, Key, Modifiers}, Subscription}, iced_core::{event, keyboard}, iced_futures::{boxed_stream, subscription::{from_recipe, EventStream, Recipe}, MaybeSend}, iced_winit::winit::keyboard::NamedKey, widget::menu::key_bind::Modifier};
use cosmic::iced::advanced::graphics::futures::event::listen_raw;
use cosmic::iced::advanced::graphics::futures::subscription;

use lazy_static::lazy_static;
use mlua::Table;

use crate::astrum_core::{app::main::{AstrumMessages, StringOrNum}, HOME_DIR};

#[derive(Clone)]
pub struct LuaKeybind {
    pub signal_name: String,
    pub modifiers: String,
    pub key: String,
}

lazy_static!{
    pub static ref ALL_KEYBINDS: Arc<Mutex<Vec<LuaKeybind>>> = Arc::new(Mutex::new(Vec::new()));
}

fn code_to_string(code: Named) -> String {
    match code {
        Named::Alt => "alt",
        Named::Backspace => "backspace",
        Named::CapsLock => "caps_lock",
        Named::Control => "ctrl",
        Named::Enter => "enter",
        Named::Super => "super",
        Named::Shift => "shift",
        Named::Space => "space",
        Named::Tab => "tab",
        Named::Delete => "delete",
        Named::End => "end",
        Named::Help => "help",
        Named::Home => "home",
        Named::Insert => "insert",
        Named::PageUp => "page_up",
        Named::PageDown => "page_down",
        Named::ArrowDown => "arrow_down",
        Named::ArrowUp => "arrow_up",
        Named::ArrowLeft => "arrow_left",
        Named::ArrowRight => "arrow_right",
        Named::NumLock => "numlock",
        Named::Escape => "escape",
        Named::Pause => "pause",
        Named::PrintScreen => "print_screen",
        Named::ScrollLock => "scroll_lock",
        Named::Meta => "meta", // no clue what this key is
        Named::F1 => "f1",
        Named::F2 => "f2",
        Named::F3 => "f3",
        Named::F4 => "f4",
        Named::F5 => "f5",
        Named::F6 => "f6",
        Named::F7 => "f7",
        Named::F8 => "f8",
        Named::F9 => "f9",
        Named::F10 => "f10",
        Named::F11 => "f11",
        Named::F12 => "f12",
        _ => "unidentified"
    }.to_string()
}

fn modifier_to_string(modifier: Modifier) -> String {
    match modifier {
        Modifier::Shift => "shift",
        Modifier::Super => "super",
        Modifier::Alt => "alt",
        Modifier::Ctrl => "ctrl",
    }.to_string()
}
// fn make_modifiers(modifiers: Modifiers, key: String) -> bool {
//     if &key == "" {
//         return Vec::new();
//     }
//     // let mut modifiers = Vec::new();
//     for modifier in key.split(",") {
//         match modifier {
//             "super" => modifiers.push(Modifier::Super),
//             "ctrl" => modifiers.push(Modifier::Ctrl),
//             "alt" => modifiers.push(Modifier::Alt),
//             "shift" => modifiers.push(Modifier::Shift),
//             &_ => unimplemented!("unknown modifier")
//         }
//     }
//
//     modifiers
// }
fn make_modifiers(key: &String) -> u32 {
    if key == "" {
        return 0b000;
    }

    let mut modifiers: u32 = 0b000;
    for modifier in key.split(",") {
        // println!("modifier: {:?}", modifier);
        match modifier {
            "super" => modifiers += (0b100 << 9),
            "alt" => modifiers += (0b100 << 6),
            "ctrl" => modifiers += (0b100 << 3),
            "shift" => modifiers += 0b100,
            &_ => unimplemented!("unknown modifier")
        }
    }


    // let mut modifiers = Vec::new();
    // for modifier in key.split(",") {
    //     match modifier {
    //         "super" => modifiers.push(Modifier::Super),
    //         "ctrl" => modifiers.push(Modifier::Ctrl),
    //         "alt" => modifiers.push(Modifier::Alt),
    //         "shift" => modifiers.push(Modifier::Shift),
    //         &_ => unimplemented!("unknown modifier")
    //     }
    // }

    modifiers
}






pub fn keybinds_service_channel() -> Subscription<AstrumMessages> {
    listen_raw(|event, status, _| {
        if event::Status::Ignored != status {
            return None;
        }



        match event {
            event::Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                // physical_key,
                modifiers,
                ..
            }) => {
                // println!("got keybind event: {:?} and {:?}", modifiers, key);
                // println!("physical code: {:?}", physical_key);
                let animation_storage = ALL_KEYBINDS.lock().unwrap();
                for object in animation_storage.iter() {
                    match key {
                        Key::Named(named) => {
                            if object.key != code_to_string(named) {
                                continue;
                            }
                        },

                        Key::Character(ref character) => {
                            if character.to_ascii_lowercase() != object.key {
                                continue;
                            }
                        },
                        Key::Unidentified => continue
                    }
                    if modifiers.contains(Modifiers::from_bits(make_modifiers(&object.modifiers)).expect("unknown bits")) {
                        return Some(AstrumMessages::SubscriptionRequest(("keybinds".to_string(), StringOrNum::String(object.signal_name.clone()), "{}".to_string())));
                    }
                }
                None
            }
            // match physical_key {
            //     _ => {},
            // },
            _ => None,

        }
    })

}
