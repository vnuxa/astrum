use cosmic::{app::{command::message, Message}, cctk::sctk::shell::wlr_layer::{Anchor, KeyboardInteractivity, Layer}, iced::window::Id, iced_runtime::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings, iced_winit::commands::layer_surface::{destroy_layer_surface, get_layer_surface}, Element, Task};
use log::debug;
use mlua::OwnedFunction;

use crate::astrum_binds::widgets::process_lua_element;

use super::main::{lua_runtime_error, AstrumMessages};

#[derive(Clone)]
pub struct Window {
    id: Option<Id>,
    window_logic: OwnedFunction, //view logic
    internal_id: String,
    settings: WindowSettings,
    // pub input_values: HashMap<String, String> // identifier and the text input value itself
}

#[derive(Debug, Clone)]
pub struct WindowSettings {
    pub popup: bool,
    pub namespace: String,
    pub anchors: Anchor,
    pub exclusion_zone: i32,
    pub keymode: KeyboardInteractivity,
    pub layer: Option<Layer>
}

impl Window {
    pub fn init(settings: WindowSettings, window_logic: OwnedFunction) -> (Self, Task<Message<AstrumMessages>>) {
        let mut id: Option<Id> = None;
        let mut task: Task<Message<AstrumMessages>> = Task::none();
        {
            let settings = settings.clone();
            if !settings.popup {
                id = Some(Id::unique());
                task = get_layer_surface(SctkLayerSurfaceSettings {
                    id: id.unwrap(),
                    keyboard_interactivity: settings.keymode,
                    namespace: settings.namespace,
                    exclusive_zone: settings.exclusion_zone,
                    layer: settings.layer.unwrap_or(Layer::Top),
                    anchor: settings.anchors,
                    // size_limits: Limits::NONE.height(30.0),
                    size: Some((None, Some(settings.exclusion_zone as u32))), // might have to
                    // think about popups
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
            },
            task
        )

    }

    pub fn run_window(&self) -> Element<AstrumMessages> {
        let element_data = match self.window_logic.call::<(), mlua::Table>(()) {
            Ok(data) => Some(data),
            Err(error) => {
                lua_runtime_error(error);
                None
            },
        };

        let lua_element = process_lua_element(element_data.unwrap()).expect("Failed to process widgets");

        lua_element.into()
        // "hello there this is not work".into()
    }

    pub fn get_id(&self) -> Option<Id> {
        self.id
    }
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            popup: false,
            namespace: "astrum-window".into(),
            anchors: Anchor::TOP.union(Anchor::LEFT).union(Anchor::RIGHT),
            exclusion_zone: 32,
            keymode: KeyboardInteractivity::None,
            layer: None // use default layer
        }
    }
}


pub fn make_window_settings<'a>(
    window_name: String,
    lua_window: mlua::Table
) -> WindowSettings {
    let mut settings = WindowSettings::default();
    settings.namespace = window_name;

    debug!("got a window request");
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
        settings.exclusion_zone = exclusive_zone as i32;
    }

    if let Ok(keymode) = lua_window.get::<_, mlua::String>("keymode") {
        settings.keymode = match keymode.to_str().unwrap() {
            "on_demand" => KeyboardInteractivity::OnDemand,
            "exclusive" => KeyboardInteractivity::Exclusive,
            _ => KeyboardInteractivity::None,
        };
    }

    if let Ok(layer) = lua_window.get::<_, mlua::String>("layer") {
        settings.layer = Some(match layer.to_str().unwrap() {
            "top" => Layer::Top,
            "bottom" => Layer::Bottom,
            "background" => Layer::Background,
            _ => unimplemented!("Window layer not supported, are you sure its not a typo?")
        });
    }

    debug!("output window settings: {:?}", settings);
    settings
}

pub fn close_window(id: Id) -> Task<Message<AstrumMessages>> {
    destroy_layer_surface(id)
}
