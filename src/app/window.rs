use std::{cell::RefCell, collections::HashMap, rc::Rc};

// use iced::{wayland::{actions::layer_surface::SctkLayerSurfaceSettings, layer_surface::{Anchor, KeyboardInteractivity, Layer}}, window::Id};
use mlua::OwnedFunction;

use crate::widgets::process_lua_element;

use super::WindowMessages;
use cosmic::{iced::{wayland::{actions::layer_surface::SctkLayerSurfaceSettings, layer_surface::{Anchor, KeyboardInteractivity, Layer}}, window::Id}, app::Command, Element};


pub fn open_window() -> (Id, Command<WindowMessages>) {
    let id = Id::unique();

    // IMPORTANT: MAKE THIS CUSTOMIZABLE
    (
        id,
        cosmic::iced::wayland::layer_surface::get_layer_surface(SctkLayerSurfaceSettings {
            id,
            keyboard_interactivity: cosmic::iced::wayland::layer_surface::KeyboardInteractivity::None,
            namespace: "astrum-window".into(),
            layer: cosmic::iced::wayland::layer_surface::Layer::Top,
            exclusive_zone: 32,
            anchor: Anchor::LEFT.union(Anchor::TOP).union(Anchor::BOTTOM),
            ..Default::default()
        })
    )
}

pub fn close_window(id: Id) -> Command<WindowMessages> {
    cosmic::iced::wayland::layer_surface::destroy_layer_surface(id)
}

#[derive(Clone)]
pub struct WindowSettings {
    pub popup: bool, // If its a popup, then make it so that a function will toggle its visibility,
    pub namespace: String, // automatically gets picked from key
    pub anchors: Anchor,
    pub exclusive_zone: i32,
    pub keymode: KeyboardInteractivity,

    // function controlled from update_logic
}
impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            popup: false,
            namespace: "astrum-window".into(),
            anchors: Anchor::TOP.union(Anchor::LEFT).union(Anchor::RIGHT),
            exclusive_zone: 32,
            keymode: KeyboardInteractivity::None,
        }
    }
}

pub fn process_lua_window<'a>(
    window_name: String,
    lua_window: mlua::Table
) -> WindowSettings {
    let mut settings = WindowSettings::default();
    settings.namespace = window_name;

    if let Ok(anchors) = lua_window.get::<_, mlua::Table>("anchor") {
        let mut anchor_settings: Option<Anchor> = None;

        for pairs in anchors.pairs::<mlua::Integer, mlua::String>() {
            let (key, value) = pairs.unwrap();

            let output = match value.to_str().unwrap() {
                "bottom" => Anchor::BOTTOM,
                "left" => Anchor::LEFT,
                "right" => Anchor::RIGHT,
                _ => Anchor::TOP,
            };

            if anchor_settings.is_none() {
                anchor_settings = Some(output);
            } else {
                anchor_settings = Some(anchor_settings.unwrap().union(output));
            }
        }

        settings.anchors = anchor_settings.unwrap();
    }
    if let Ok(is_popup) = lua_window.get::<_, bool>("is_popup") {
        settings.popup = is_popup;
    }
    if let Ok(exclusive_zone) = lua_window.get::<_, mlua::Integer>("exclusive_zone") {
        settings.exclusive_zone = exclusive_zone as i32
    }
    if let Ok(keymode) = lua_window.get::<_, mlua::String>("keymode") {
        settings.keymode = match keymode.to_str().unwrap() {
            "on_demand" => KeyboardInteractivity::OnDemand,
            "exclusive" => KeyboardInteractivity::Exclusive,
            _ => KeyboardInteractivity::None,
        }
    }

    settings
}


#[derive(Clone)]
pub struct Window {
    id: Option<Id>,
    window_logic: OwnedFunction,
    internal_id: String,
    settings: WindowSettings,
    // pub input_values: HashMap<String, String> // identifier and the text input value itself
}


impl Window {
    pub fn init(settings: WindowSettings, window_logic: OwnedFunction) -> (Self, Command<WindowMessages>) {
        let mut id: Option<Id> = None;
        let mut command = Command::none();
        {
            let settings = settings.clone();
            if !settings.popup {
                id = Some(Id::unique());
                command = cosmic::iced::wayland::layer_surface::get_layer_surface(SctkLayerSurfaceSettings {
                    id: id.unwrap(),
                    keyboard_interactivity: settings.keymode,
                    namespace: settings.namespace.clone(),
                    exclusive_zone: settings.exclusive_zone,
                    layer: Layer::Top,
                    anchor: settings.anchors,
                    ..Default::default()
                });

            }
        }

        (
            Self {
                window_logic,
                internal_id: settings.clone().namespace,
                settings,
                id,
                // input_values: Rc::new(RefCell::new(HashMap::new())),
                // input_values: HashMap::new()
                // data: RefCell::new(WindowData {
                //     input_values: HashMap::new()
                // })
            },
            command
        )
    }

    // IMPORTANT: maybe make it so that when you press escape on the popup, it closes the popup??
    //
    // toggles visibility, if it is a popup
    pub fn toggle(&mut self) -> Command<WindowMessages> {
        if self.id.is_none() {
            self.id = Some(Id::unique());
            return cosmic::iced::wayland::layer_surface::get_layer_surface(SctkLayerSurfaceSettings {
                id: self.id.unwrap(),
                keyboard_interactivity: self.settings.keymode,
                namespace: self.settings.namespace.clone(),
                exclusive_zone: self.settings.exclusive_zone,
                layer: Layer::Overlay,
                anchor: self.settings.anchors,
                ..Default::default()
            })
        } else {
            if let Some(id) = self.id.take() {
                cosmic::iced::wayland::layer_surface::destroy_layer_surface(id)
            } else {
                Command::none()
            }
        }

    }
    // pub fn change_input(&mut self, input_id: String, input_value: String) {
    //     if let None = self.data.borrow().input_values.get(&input_id) {
    //         self.data.borrow_mut().input_values.insert(input_id, input_value);
    //         return;
    //     }
    //
    //     *self.data.borrow_mut().input_values.get_mut(&input_id).unwrap() = input_value.clone();
    // }

    // maybe make predefined settings that somehow get processed and stuff??
    pub fn run_window(&self) -> Element<WindowMessages> {
        let element_data = self.window_logic.call::<(), mlua::Table>(()).unwrap();
        let lua_element = process_lua_element(element_data).expect("Could not process view logic, are you sure you are returning a widget?");

        return lua_element.into()
    }

    pub fn get_id(&self) -> Option<Id> {
        self.id
    }
}
