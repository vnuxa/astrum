// have lua contexts here as well!!!


use std::{borrow::{Borrow, BorrowMut}, cell::RefCell, collections::HashMap, error::Error, fmt::Debug, io::Read, ops::Deref, path::PathBuf};

use color_print::{cformat, cprintln, cstr};
use cosmic::{app::Core, iced::Subscription, app::Command};
// use iced::{event::listen, subscription::{self, run}, wayland::layer_surface::Anchor, widget::{button, container, row, shader::wgpu::MaintainBase, text}, Application, Command};
use mlua::{Lua, OwnedFunction, OwnedTable, Value};
use window::{close_window, process_lua_window, Window, WindowSettings};

use crate::{animations, config::lua::{get_or_create_module, make_lua_context}, services::{self, hyprland::{self, HyprlandListener}}, style::application::lua_application_style, widgets::process_lua_element};
use cosmic::iced_runtime::core::window::Id as SurfaceId;

pub mod window;

#[derive(Debug, Clone)]
pub enum WindowMessages {
    Msg((String, String)), // Signal_name and signal_data
    ToggleWindow(String), // window identifier/name
    ReloadLua,
    AnimationTick
}

impl cosmic::Application for MainWindow {
    type Executor = cosmic::executor::Default;
    // type Flags<'lua> = Flags<'lua>;
    // type Flags = mlua::Value<'lua>;
    type Flags = (RefCell<mlua::Lua>, String, PathBuf);
    type Message = WindowMessages;
    const APP_ID: &'static str = "astrum"; // dont know how app_ids work, just
    // guessing

    fn init(core: Core, flags: Self::Flags) -> (Self, cosmic::app::Command<WindowMessages>) {
        // cosmic launcher sets the keyboard_nav to false so idk
        // and has the core as mutable

        let mut update_logic: Option<OwnedFunction> = None;
        let mut requested_signals: Option<OwnedTable> = None;
        let mut app_style: Option<OwnedTable> = None;
        let mut windows: HashMap<String, Window> = HashMap::new();
        let mut commands: Vec<Command<Self::Message>> = Vec::new();

        {
            let config_binding = flags.0.borrow();
            let config: mlua::Value = match config_binding.load(flags.1).eval() {
                Ok(conf) => conf,
                Err(error) => {
                    eprintln!("Lua error occured!");
                    if let mlua::Error::RuntimeError(reason) = error {
                        let formatted_error = reason.replace("\n", cstr!("<white>\n│\t</white>"));

                        println!("┌────── Lua error ────────────");
                        cprintln!("<w>│\t</><r>{}</>", formatted_error);
                        println!("└─────────────────────────────");
                    } else {

                        let formatted_error = error.to_string().replace("\n", cstr!("<w>\n│\t</>"));

                        // cprintln!("Error type: <r>{}</>", error.source().unwrap());
                        println!("┌────── Lua error ────────────");
                        cprintln!("<w>│\t</><r>{}</>", formatted_error);
                        println!("└─────────────────────────────");
                    }

                    std::process::Command::new("pkill").arg("astrum").output().ok();
                    panic!("Error while evaluating lua context");



                    mlua::Value::Nil
                }
            };
                // .set_name(path.display())
                //.expect("evaluating the lua context failed");


            match config {
                mlua::Value::Table(table) => {
                    if let Ok(logic) = table.get("update_logic") {
                        let logic: mlua::Function = logic;

                        update_logic = Some(logic.into_owned());
                    }
                    if let Ok(logic) = table.get("windows") {
                        let table_of_windows: mlua::Table = logic;
                        // let table_of_windows: mlua::Table = logic.call::<_, mlua::Table>(()).expect("View logic is expected to be a table");

                        for pairs in table_of_windows.pairs::<mlua::String, mlua::Table>() {
                            let (key,value) = pairs.unwrap();

                            let id = key.to_str().unwrap().to_string();

                            let (window, command) = Window::init(
                                process_lua_window(id.clone(), value.clone()),
                                value.get::<_, mlua::Function>("view").unwrap().into_owned(),
                            );

                            windows.insert(
                                id,
                                window
                            );
                            commands.push(command);

                        }
                    }
                    if let Ok(signals) = table.get("requested_signals")  {
                        let signals: mlua::Table = signals;
                        requested_signals = Some(signals.into_owned());
                    }
                    if let Ok(style) = table.get("style")  {
                        let style: mlua::Table = style;
                        app_style = Some(style.into_owned());
                    }
                },
                _ => {  }
            }
        }

        (
            MainWindow {
                windows,
                update_logic,
                core,
                // view_logic,
                requested_signals,
                style: app_style,
                lua: flags.0,
                config_path: flags.2
                // lua: Some(flags.0)
                // lua: Some(flags.0),
                // config_string: Some(flags.1),
                // lua: Some(lua)
            },

            Command::batch(commands)
        )
    }


    fn core(&self) -> &Core {
        &self.core
    }
    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    // fn title(&self, id: iced::window::Id) -> String {
    //     String::from("astrum")
    // }
    //

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        // println!("update logic ran!");
        match message {
            WindowMessages::Msg((signal_name, signal_data)) => {
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
                Command::none()
            },
            WindowMessages::ToggleWindow(window_name) => {
                if let Some(window) = self.windows.get_mut(&window_name) {
                    return window.toggle();
                }
                Command::none()
            },
            WindowMessages::ReloadLua => {

                let path = self.config_path.as_path();
                let mut file = std::fs::File::open(path).unwrap();
                let mut s = String::new();
                file.read_to_string(&mut s).expect("writing to file failed");
                let mut lua = RefCell::new(make_lua_context(path).expect("making the lua context failed"));

                let mut update_logic: Option<OwnedFunction> = None;
                let mut requested_signals: Option<OwnedTable> = None;
                let mut app_style: Option<OwnedTable> = None;
                let mut windows: HashMap<String, Window> = HashMap::new();
                let mut commands: Vec<Command<Self::Message>> = Vec::new();

                for (id, window) in &self.windows {
                    if let Some(window_id) = window.get_id() {
                        commands.push(close_window(window_id));
                    }
                }

                {
                    let config_binding = lua.borrow();
                    let config: mlua::Value = match config_binding.load(s).eval() {
                        Ok(conf) => conf,
                        Err(error) => {
                            eprintln!("Lua error occured!");
                            if let mlua::Error::RuntimeError(reason) = error {
                                let formatted_error = reason.replace("\n", cstr!("<white>\n│\t</white>"));

                                println!("┌────── Lua error ────────────");
                                cprintln!("<w>│\t</><r>{}</>", formatted_error);
                                println!("└─────────────────────────────");
                            } else {

                                let formatted_error = error.to_string().replace("\n", cstr!("<w>\n│\t</>"));

                                // cprintln!("Error type: <r>{}</>", error.source().unwrap());
                                println!("┌────── Lua error ────────────");
                                cprintln!("<w>│\t</><r>{}</>", formatted_error);
                                println!("└─────────────────────────────");
                            }

                            std::process::Command::new("pkill").arg("astrum").output().ok();
                            panic!("Error while evaluating lua context");



                            mlua::Value::Nil
                        }
                    };


                    match config {
                        mlua::Value::Table(table) => {
                            if let Ok(logic) = table.get("update_logic") {
                                let logic: mlua::Function = logic;

                                update_logic = Some(logic.into_owned());
                            }
                            if let Ok(logic) = table.get("windows") {
                                let table_of_windows: mlua::Table = logic;
                                // let table_of_windows: mlua::Table = logic.call::<_, mlua::Table>(()).expect("View logic is expected to be a table");

                                for pairs in table_of_windows.pairs::<mlua::String, mlua::Table>() {
                                    let (key,value) = pairs.unwrap();

                                    let id = key.to_str().unwrap().to_string();

                                    let (window, command) = Window::init(
                                        process_lua_window(id.clone(), value.clone()),
                                        value.get::<_, mlua::Function>("view").unwrap().into_owned(),
                                    );

                                    windows.insert(
                                        id,
                                        window
                                    );
                                    commands.push(command);

                                }
                            }
                            if let Ok(signals) = table.get("requested_signals")  {
                                let signals: mlua::Table = signals;
                                requested_signals = Some(signals.into_owned());
                            }

                            if let Ok(style) = table.get("style")  {
                                let style: mlua::Table = style;
                                app_style = Some(style.into_owned());
                            }
                        },
                        _ => {  }
                    }
                }

                self.lua = lua;
                self.update_logic = update_logic;
                self.windows = windows;
                self.requested_signals = requested_signals;
                self.style = app_style;

                Command::batch(commands)
            }
            WindowMessages::AnimationTick => {
                // println!("animation tick!!");
                Command::none()
            }
            _ => Command::none()
        }
    }

    fn view(&self) -> cosmic::Element<Self::Message> {
        unimplemented!();
    }
    fn view_window(
        &self,
        id: SurfaceId,
    ) -> cosmic::Element<Self::Message> {
        // println!("view logic ran!");
        let windows: &HashMap<String, Window> = &self.windows;
        if id == SurfaceId::MAIN {
            // println!("draw main window pls");
            return "".into();
        }
        for (window_name, window) in windows.iter() {
            if let Some(window_id) = window.get_id() {
                if id == window_id {
                    return window.run_window().into();
                }
            }
        }

        "".into()
    }

    fn style(&self) -> Option<<cosmic::Theme as cosmic::iced_style::application::StyleSheet>::Style> {

        if let Some(style) = &self.style {
            // let lua = self.lua.borrow();
            // return Some(
            //     cosmic::style::Application::Custom(
            //         Box::new(
            //             |theme| lua_application_style(style.clone())
            //         )
            //     )
            // );
            return Some(lua_application_style(style.clone()))

        }

        None
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        // println!("subscription logic ran!");
        let mut services = Vec::new();
        let lua = self.lua.borrow();
        if animations::any_animation_in_progress() {
            println!("ik anim is in progress");

            services.push(
                cosmic::iced::window::frames().map(|(id, at)| {
                    println!("wayland farmessss wow!!!!!!!!!!!!!!!1");
                    WindowMessages::AnimationTick
                }
                )
                // cosmic::iced_futures::event::listen_raw(|event, _status| {
                //     println!("listening to the future :O");
                //     match event {
                //         cosmic::iced_core::Event::Window(id, cosmic::iced_core::window::Event::RedrawRequested(at))
                //         | cosmic::iced_core::Event::PlatformSpecific(
                //             cosmic::iced_core::event::PlatformSpecific::Wayland(
                //                 cosmic::iced_core::event::wayland::Event::Frame(at, _, id),
                //             ),
                //         ) => {
                //             println!("wayland framesss wow!!!!!!!!!!!!!1");
                //             Some(WindowMessages::AnimationTick)
                //
                //         },
                //         _ => None,
                //     }
                // })
                // cosmic::iced_futures::event::listen_raw(|event, _status| match event {
                //     cosmic::iced_core::Event::Window(id, cosmic::iced_core::window::Event::RedrawRequested(at)) => {
                //         println!("wayland framesss wow!!!!!!!!!!!!!1");
                //         Some(WindowMessages::AnimationTick)
                //         // Some((id, at))
                //     }
                //     _ => {
                //         println!("no wayland frames..........");
                //         None
                //     },
                // })

            );
        }

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
            if let Ok(time_table) = signals.get::<_, mlua::Table>("time") {
                for pair in time_table.pairs::<mlua::Integer, mlua::Number>() {
                    let (key, value) = pair.unwrap();

                    services.push(
                        services::time::listen_to_time(value as u64)
                    );
                }
            }


        }

        let mut window_names: HashMap<String, bool> = HashMap::new();

        for (window_name, _) in self.windows.iter() {
            window_names.insert(window_name.to_owned(), true);
        }

        services.push(
            services::calls::windows_socket(window_names)
        );
        services.push(
            services::hot_reload::hot_reload(self.config_path.clone())
        );
        //     println!("animation in progress!!");
        // cosmic::iced::window::f
        // }

        // for subscription in &services {
        //
        // println!("      services {:?}", subscription);
        // }
        Subscription::batch(services)
    }
}

struct MainWindow {
    // view_logic: Option<mlua::Function<'lua>>,
    // update_logic: Option<mlua::Function<'lua>>,
    lua: RefCell<Lua>,
    // config_string: Option<String>,

    update_logic: Option<OwnedFunction>,
    // view_logic: Option<OwnedFunction>,

    requested_signals: Option<OwnedTable>,
    style: Option<OwnedTable>,
    windows: HashMap<String, Window>,

    core: Core,
    config_path: PathBuf
}

pub fn create_app(
    config_path: PathBuf
) -> anyhow::Result<()> {

    let path = config_path.as_path();
    let mut file = std::fs::File::open(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).expect("writing to file failed");
    let mut lua = make_lua_context(path).expect("making the lua context failed");

    let cosmic_app = cosmic::app::run::<MainWindow>(
        cosmic::app::Settings::default()
            .antialiasing(true)
            .client_decorations(false)
            .debug(false)
            .default_text_size(16.0)
            .scale_factor(1.0)
            .no_main_window(true)
            .exit_on_close(false),

        (RefCell::new(lua), s, config_path),
    );

    Ok(())
}
