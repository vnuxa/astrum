// have lua contexts here as well!!!


use std::{borrow::{Borrow, BorrowMut}, cell::RefCell, collections::HashMap, io::Read, ops::Deref, path::PathBuf};

use iced::{event::listen, subscription::{self, run}, wayland::layer_surface::Anchor, widget::{button, container, row, shader::wgpu::MaintainBase, text}, Application};
use mlua::{Lua, OwnedFunction, OwnedTable, Value};

use crate::{config::lua::make_lua_context, services::{self, hyprland::{self, HyprlandListener}}, widgets::process_lua_element};

#[derive(Debug, Clone)]
pub enum WindowMessages {
    Msg((String, String)) // Signal_name and signal_data
}

impl Application for MainWindow {
    type Executor = iced::executor::Default;
    // type Flags<'lua> = Flags<'lua>;
    // type Flags = mlua::Value<'lua>;
    type Flags = (RefCell<mlua::Lua>, String);
    type Theme = iced::Theme;
    type Message = WindowMessages;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {

        // println!("file is");
        // println!("{}", s);

        // let config = flags;
        // println!("the type name {}", config.to_string().unwrap());
        //
        // let mut view_logic: Option<mlua::Function> = None;
        // let mut update_logic: Option<mlua::Function> = None;
        // match config {
        //     mlua::Value::Table(table) => {
        //         if let Ok(logic) = table.get("view_logic") {
        //             let logic: mlua::Function = logic;
        //             view_logic = Some(logic);
        //         }
        //         if let Ok(logic) = table.get("update_logic") {
        //             let logic: mlua::Function = logic;
        //             update_logic = Some(logic);
        //         }
        //         println!("table is {}", table.len().unwrap());
        //     },
        //     _ => {
        //         println!("this is ran");
        //     }
        // }
        // TODO: move the entire lua thingy a bove
        // into the ::new logic
        //
        // why?
        // well new gets run only once so whats the difference
        // and if flags implements some weird datatype, i can make it so that it has clone!!
        // let lua = self.lua.as_ref().unwrap();
        let mut update_logic: Option<OwnedFunction> = None;
        let mut view_logic: Option<OwnedFunction> = None;
        let mut requested_signals: Option<OwnedTable> = None;
        {
            let config_binding = flags.0.borrow();
            let config: mlua::Value = config_binding.load(flags.1)
                // .set_name(path.display())
                .eval().expect("evaluating the lua context failed");


            match config {
                mlua::Value::Table(table) => {
                    if let Ok(logic) = table.get("update_logic") {
                        let logic: mlua::Function = logic;
                        update_logic = Some(logic.into_owned());
                        // let signal_info: Option<mlua::Table> =
                        // match lua.load(signal_data).eval().expect("failed to evaluate signal_data") {
                        //     Value::Table(table_data) => Some(table_data),
                        //     _ => None,
                        // };
                        // logic.call::<_, (mlua::String, mlua::Table)>(
                        //     (
                        //         lua.create_string(&signal_name).expect("failed to create string"),
                        //         signal_info.expect("signal data is not a table!")
                        //     )
                        // );
                    }
                    if let Ok(logic) = table.get("view_logic") {
                        let logic: mlua::Function = logic;
                        view_logic = Some(logic.into_owned());
                    }
                    if let Ok(signals) = table.get("requested_signals")  {
                        let signals: mlua::Table = signals;
                        requested_signals = Some(signals.into_owned());
                    }
                },
                _ => {  }
            }
        }
        (
            MainWindow {
                update_logic,
                view_logic,
                requested_signals,
                lua: flags.0
                // lua: Some(flags.0)
                // lua: Some(flags.0),
                // config_string: Some(flags.1),
                // lua: Some(lua)
            },

            iced::Command::none()
        )
    }

    fn title(&self, id: iced::window::Id) -> String {
        String::from("astrum")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {

        if let WindowMessages::Msg((signal_name, signal_data)) = message  {
            let lua = self.lua.borrow();
            if let Some(logic) = &self.update_logic {
                let signal_info: Option<mlua::Table> =
                match lua.load(signal_data).eval().expect("failed to evaluate signal_data") {
                    Value::Table(table_data) => Some(table_data),
                    _ => None,
                };
                logic.call::<_, (mlua::String, mlua::Table)>(
                    (
                        lua.create_string(&signal_name).expect("failed to create string"),
                        signal_info.expect("signal data is not a table!")
                    )
                );
            }
            // self.up

            // let lua = self.lua.as_ref().unwrap();
            // let config: mlua::Value = lua.load(self.config_string.as_ref().unwrap())
            //     // .set_name(path.display())
            //     .eval().expect("evaluating the lua context failed");
            //
            // match config {
            //     mlua::Value::Table(table) => {
            //         if let Ok(logic) = table.get("update_logic") {
            //             let logic: mlua::Function = logic;
            //             let signal_info: Option<mlua::Table> =
            //                 match lua.load(signal_data).eval().expect("failed to evaluate signal_data") {
            //                     Value::Table(table_data) => Some(table_data),
            //                     _ => None,
            //                 };
            //             logic.call::<_, (mlua::String, mlua::Table)>(
            //                 (
            //                     lua.create_string(&signal_name).expect("failed to create string"),
            //                     signal_info.expect("signal data is not a table!")
            //                 )
            //             );
            //         }
            //     },
            //     _ => {  }
            // }
        }

        // if let Some(logic) = &self.update_logic {
        // }

        iced::Command::none()
    }

    fn view(
        &self,
        id: iced::window::Id,
    ) -> iced::Element<Self::Message> {

        if let Some(view_logic) = &self.view_logic {
            let element_data = view_logic.call::<(), mlua::Table>(()).unwrap();
            let lua_element = process_lua_element(element_data).expect("Could not process view logic, are you sure you are returning a widget?");

            return lua_element.into()
        }
        // let config: mlua::Value = self.lua.as_ref().unwrap().load(self.config_string.as_ref().unwrap())
        //     // .set_name(path.display())
        //     .eval().expect("evaluating the lua context failed");
        // println!("redraw!");
        //
        // match config {
        //     mlua::Value::Table(table) => {
        //         if let Ok(logic) = table.get("view_logic") {
        //             let logic: mlua::Function = logic;
        //             let wow = logic.call::<(), mlua::Table>(()).unwrap(); // TODO: fix
        //             let test = process_lua_element(wow).expect("Could not process view logic, are you sure you are returning a widget?");
        //             return test.into();
        //
        //             // for pair in wow.pairs::<mlua::String, mlua::Value>() {
        //             //     let (key, value) = pair.expect("wow the key value thing failed");
        //             //     println!("key is {}, value is {}", key.to_str().unwrap(), value.type_name());
        //             //
        //             // }
        //         }
        //     },
        //     _ => {  }
        // }
        "wow it didnt work".into()

        // button("hai")
        //     .on_press(WindowMessages::Msg(("button_press".to_string(), "{ button_text = 'hai' }".to_string())))
        //     .into()
        // let row = row!{
        //     button( text(self.num) )
        // };
        // let content = container(row)
        //     .center_x()
        //     .center_y()
        //     .width(iced::Length::Fill)
        //     // .height(iced::Length::Fill)
        //     .into();
        //
        // content
    }
    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let mut services = Vec::new();
        let lua = self.lua.borrow();
        println!("Subscriptions are ran again..");

        if let Some(requested) = &self.requested_signals {
            let signals = requested.to_ref();

            if let Ok(signal_table) = signals.get::<mlua::String, mlua::Table>(lua.create_string("hyprland").unwrap()) {
                let signal_table: mlua::Table = signal_table;

                let mut listener_specifics: HashMap<String, bool> = HashMap::new();
                for pair in signal_table.pairs::<mlua::Integer, mlua::String>() {
                    let (key, value) = pair.unwrap();

                    match value.to_str().unwrap() {
                        "workspaces" => listener_specifics.insert("workspaces".to_string(), true),
                        &_ => { None },
                    };
                }

                services.push(
                    hyprland::listen_workspaces(listener_specifics)
                );
            }
            if let Ok(signal_table) = signals.get::<mlua::String, mlua::Table>(lua.create_string("mpris").unwrap()) {
                let signal_table: mlua::Table = signal_table;

                let mut listener_specifics: HashMap<String, bool> = HashMap::new();
                for pair in signal_table.pairs::<mlua::Integer, mlua::String>() {
                    let (key, value) = pair.unwrap();

                    match value.to_str().unwrap() {
                        "playing" => listener_specifics.insert("playing".to_string(), true),
                        "paused" => listener_specifics.insert("paused".to_string(), true),
                        "stopped" => listener_specifics.insert("stopped".to_string(), true),
                        "volume_changed" => listener_specifics.insert("volume_changed".to_string(), true),
                        "looping_changed" => listener_specifics.insert("looping_changed".to_string(), true),
                        "shuffle_toggled" => listener_specifics.insert("shuffle_toggled".to_string(), true),
                        "track_changed" => listener_specifics.insert("track_changed".to_string(), true),
                        &_ => { None },
                    };
                }

                services.push(
                    services::mpris::listen_first_player(listener_specifics)
                );
            }

            if let Ok(signal_table) = signals.get::<mlua::String, mlua::Table>(lua.create_string("calls").unwrap()) {
                let signal_table: mlua::Table = signal_table;

                let mut listener_specifics: HashMap<String, bool> = HashMap::new();
                for pair in signal_table.pairs::<mlua::Integer, mlua::String>() {
                    let (key, value) = pair.unwrap();

                    listener_specifics.insert(value.to_str().unwrap().to_string(), true);
                }
                services.push(
                    services::calls::listen_to_calls(listener_specifics)
                );
            }


        }
        subscription::Subscription::batch(services)
    }
}

struct MainWindow {
    // view_logic: Option<mlua::Function<'lua>>,
    // update_logic: Option<mlua::Function<'lua>>,
    lua: RefCell<Lua>,
    // config_string: Option<String>,

    update_logic: Option<OwnedFunction>,
    view_logic: Option<OwnedFunction>,

    requested_signals: Option<OwnedTable>
}

pub fn create_app(
    config_path: PathBuf
) -> anyhow::Result<()> {

    let path = config_path.as_path();
    let mut file = std::fs::File::open(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).expect("writing to file failed");
    let mut lua = make_lua_context(path).expect("making the lua context failed");
    // let table = lua.create_table().unwrap();


    // let config: mlua::Value = lua.load(s.trim_start_matches('\u{FEFF}'))
    //     // .set_name(path.display())
    //     .eval().expect("evaluating the lua context failed");
    // println!("Hello, world!");
    MainWindow::run(
        iced::Settings {
            antialiasing: true,
            exit_on_close_request: false, // maybe popups??? // customizable

            initial_surface: iced::wayland::InitialSurface::LayerSurface(iced::wayland::actions::layer_surface::SctkLayerSurfaceSettings {
                id: iced::window::Id::MAIN,
                keyboard_interactivity: iced::wayland::layer_surface::KeyboardInteractivity::None, // customizable
                namespace: "astrum-window".into(), // changable
                layer: iced::wayland::layer_surface::Layer::Top,
                // maybe only height is chanagable
                // some presets like "Full" to make it automatically use the full monitors size??
                exclusive_zone: 32,// customizable
                anchor: Anchor::TOP.union(Anchor::LEFT).union(Anchor::RIGHT), // customizable
                ..Default::default()
            }),
            // flags: lua,
            // flags: config,
            flags: (RefCell::new(lua), s),
            id: None,
            fonts: Default::default(), // TODO: make it customizable by user
            default_font: Default::default(),
            default_text_size: 14.into(),
        }
    );

    Ok(())
}
