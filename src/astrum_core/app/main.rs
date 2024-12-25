use std::{borrow::Borrow, cell::RefCell, collections::HashMap, io::Read, path::PathBuf};

use crate::astrum_core::{animations::any_animation_in_progress, lua_context::make_lua_context, services::{self, calls, hyprland::hyprland_service_channel, mpris::mpris_service_channel, system_tray::system_tray_service_channel, time::time_service_channel}};
use color_print::{cprintln, cstr};
use cosmic::{app::{Message, Settings}, iced::{self, window::frames, Subscription}, Element, Task};
use mlua::{OwnedFunction, OwnedTable, Value};

use super::window::{close_window, make_window_settings, Window};


struct AstrumApp {
    windows: HashMap<String, Window>,
    core: cosmic::app::Core,
    update_logic: Option<OwnedFunction>,
    subscription_logic: Option<OwnedFunction>,
    subscription_data: Option<OwnedTable>,
    lua: RefCell<mlua::Lua>,
    config_path: PathBuf,
}

#[derive(Debug, Clone)]
pub enum StringOrNum {
    String(String),
    Num(i64),
}

#[derive(Debug, Clone)]
pub enum AstrumMessages {
    Msg((String, String)), // signal_name and signal_data
    SubscriptionRequest((String, StringOrNum, String)), //subscription_type, subscription_signal (num for time), subscription_data
    LiveReload,
    AnimationTick

}

pub fn start_application(config_path: PathBuf) -> anyhow::Result<()> {
    // Initialize lua context
    let mut lua = make_lua_context(config_path.as_path()).expect("making the lua context failed");

    // get the config paths main file
    let path = config_path.as_path();
    let mut file = std::fs::File::open(path).unwrap();
    let mut source = String::new();
    file.read_to_string(&mut source).expect("reading to file failed");

    cosmic::app::run::<AstrumApp>(
        Settings::default()
            .antialiasing(true)
            .client_decorations(false)
            .debug(false)
            .default_text_size(16.0)
            .scale_factor(1.0)
            .no_main_window(true)
            .exit_on_close(false),

        (RefCell::new(lua), source, config_path)
    );


    // TODO: run new with
    // (RefCell::new(lua), source, config_path)

    Ok(())
}

pub fn lua_runtime_error(error: mlua::Error) {
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

    // std::process::Command::new("pkill").arg("astrum").output().ok();
    panic!("Error while evaluating lua context");
}

fn configure_app(
    lua_context: RefCell<mlua::Lua>,
    source: String,
) -> (
    Option<OwnedFunction>, // update_logic
    Option<OwnedFunction>, // subscription_logic
    Option<OwnedTable>, // subscription_data,
    Option<OwnedTable>, // app_style
    HashMap<String, Window>, // windows
    Vec<Task<Message<AstrumMessages>>>, // commands
    RefCell<mlua::Lua>
) {
    let mut update_logic: Option<OwnedFunction> = None;
    let mut subscription_logic: Option<OwnedFunction> = None;
    let mut subscription_data: Option<OwnedTable> = None;
    let mut app_style: Option<OwnedTable> = None;
    let mut windows: HashMap<String, Window> = HashMap::new();
    let mut commands: Vec<Task<Message<AstrumMessages>>> = Vec::new();


    {
        let config_binding = lua_context.borrow();
        // load the configuration source
        let config: mlua::Value = match config_binding.load(source).eval() {
            Ok(conf) => { conf },
            Err(error) => {
                lua_runtime_error(error);
                mlua::Value::Nil
            },
        };


        match config {
            mlua::Value::Table(table) => {
                if let Ok(logic) = table.get("update_logic") {
                    let logic: mlua::Function = logic;

                    update_logic = Some(logic.into_owned());
                }
                if let Ok(logic) = table.get("windows") {
                    let table_of_windows: mlua::Table = logic;

                    for pairs in table_of_windows.pairs::<mlua::String, mlua::Table>() {
                        let (key,value) = pairs.unwrap();

                        let id = key.to_str().unwrap().to_string();

                        let (window, command) = Window::init(
                            make_window_settings(id.clone(), value.clone()),
                            value.get::<_, mlua::Function>("view").unwrap().into_owned(),
                        );

                        windows.insert(
                            id,
                            window
                        );
                        // println!("got command {:?}", command);
                        commands.push(command);

                    }
                }
                if let Ok(logic) = table.get("subscription_logic") {
                    let logic: mlua::Function = logic;

                    subscription_logic = Some(logic.into_owned());
                }
                if let Ok(subscriptions) = table.get("subscription_messages")  {
                    let subscriptions: mlua::Table = subscriptions;
                    subscription_data = Some(subscriptions.into_owned());
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
        update_logic,
        subscription_logic,
        subscription_data,
        app_style,
        windows,
        commands,
        lua_context
    )

}

impl cosmic::Application for AstrumApp {
    type Executor = cosmic::executor::Default;
    type Message = AstrumMessages;
    type Flags = (RefCell<mlua::Lua>, String, PathBuf);

    const APP_ID: &'static str = "astrum_unstable";

    fn init(core: cosmic::app::Core, flags: Self::Flags) -> (AstrumApp, Task<Message<AstrumMessages>>) {
        let ( update_logic, subscription_logic, subscription_data, style, windows, commands, lua ) = configure_app(flags.0, flags.1);
        (
            Self {
                core,
                windows,
                update_logic,
                subscription_logic,
                subscription_data,
                lua,
                config_path: flags.2,

                // lua: flags.0,
                // update_logic,
                // requested_signals,
                // style: app_style,
            },

            Task::batch(commands)
        )
    }

    // refer to view_window, since this is a multiwindow application
    fn view(&self) -> Element<Self::Message> {
        return cosmic::widget::text("wow the biew logic").into();
        unimplemented!()
    }

    fn view_window(&self, id: iced::window::Id) -> Element<Self::Message> {

        println!("view logic ran! {:?}", id);
        let windows: &HashMap<String, Window> = &self.windows;
        // if id == SurfaceId::MAIN {
            // println!("draw main window pls");
            // return "".into();
        // }
        // return cosmic::widget::text("hello my name is astrum").into();
        for (window_name, window) in windows.iter() {
            if let Some(window_id) = window.get_id() {
                if id == window_id {
                    return window.run_window().into();
                }
            }
        }

        "".into()
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::app::Message<AstrumMessages>> {
        // messagesright now are split into 2
        // user defined view messages
        // and subscription defined view messages
        //
        // the view messages are fine, but the subscription messages i dont like
        // mainly bec ause the subscription messafges have an issue where any single window can
        // react to the same message, and it might be confusing for the user
        // so instead of using App:Subscribe()
        // i should provide a list of
        // {
            // Hyprland = {
                // workspace_change = "hyprland_workspaces_changed"
            // }
            // mpris = {
                // track_changed = "bar_update_thingy"

            // }
        // }
        match message {
            AstrumMessages::Msg((name, data)) => {
                let lua = self.lua.borrow();

                if let Some(logic) = &self.update_logic {
                    let data_evaled: Option<mlua::Table> = match lua.load(data).eval().expect("failed to load signals") {
                        Value::Table(table_data) => Some(table_data),
                        _ => None,
                    };

                    logic.call::<_, (mlua::String, mlua::Table)>(
                        (
                            lua.create_string(&name).expect("failed to create string"),
                            data_evaled.expect("signal msg data is not a table!")
                        )
                    );
                }
                Task::none()
            },
            AstrumMessages::SubscriptionRequest((service, name, data)) => {
                let lua = self.lua.borrow();

                if let Some(logic) = &self.subscription_logic {
                    let data_evaled: Option<mlua::Table> = match lua.load(data).eval().expect("failed to load subscription data") {
                        Value::Table(table_data) => Some(table_data),
                        _ => None,
                    };

                    // since the time service makes it so that the 2nd argument isnt always a
                    // string, i split it into 2 variants
                    match name {
                        StringOrNum::String(name) => {
                            logic.call::<_, (mlua::String, mlua::String, mlua::Table)>(
                                (
                                    lua.create_string(&service).expect("failed to create string"),
                                    lua.create_string(&name).expect("failed to create string"),
                                    data_evaled.expect("subscription data is not a table!")
                                )
                            );
                        },
                        StringOrNum::Num(name) => {
                            logic.call::<_, (mlua::String, mlua::Integer, mlua::Table)>(
                                (
                                    lua.create_string(&service).expect("failed to create string"),
                                    name,
                                    data_evaled.expect("subscription data is not a table!")
                                )
                            );
                        }
                    }

                }
                Task::none()
            },
            AstrumMessages::LiveReload => {
                let path = self.config_path.as_path();
                let mut file = std::fs::File::open(path).unwrap();
                let mut source = String::new();
                file.read_to_string(&mut source).expect("reading to file failed");
                let mut lua = RefCell::new(make_lua_context(path).expect("making the lua context failed"));

                let ( update_logic, subscription_logic, subscription_data, style, windows, mut commands, lua ) = configure_app(lua, source);

                for (id, window) in &self.windows {
                    if let Some(window_id) = window.get_id() {
                        commands.push(close_window(window_id));
                    }
                }

                self.lua = lua;
                self.update_logic = update_logic;
                self.subscription_logic = subscription_logic;
                self.subscription_data = subscription_data;
                // self.style = style;
                self.windows = windows;

                Task::batch(commands)
            },
            AstrumMessages::AnimationTick => {
                println!("animation tick!");
                Task::none()
            }
        }

    }

   fn subscription(&self) -> Subscription<AstrumMessages> {
        let lua = self.lua.borrow();
        let mut services = Vec::new();

        if any_animation_in_progress() {
            services.push(frames().map(|_| AstrumMessages::AnimationTick));
        }

        if let Some(requested_signals) = &self.subscription_data {
            let subscription_data = requested_signals.to_ref();

            // below is searching for what to provide to each service
            //
            // TODO: make iomplementations of services that use less memory
            // or in general jsut try to find memory thingy
            // cuz astrum shouldbt e using 300 mb, if ags uses like 70 then
            // also add greetd support

            // this closure is just abstractions for services that have the same syntax for their
            // signals, ofc not every service will use this exact function
            let mut make_subscribtion = |subscribtion_name: &str, requests_list: Vec<&str>, service_function: fn(Vec<String>) -> Subscription<AstrumMessages>|
                {
                if let Ok(subscribtion_table) = subscription_data.get::<_, mlua::Table>(subscribtion_name) {
                    let mut service_requests: Vec<String> = Vec::new();

                    for pair in subscribtion_table.pairs::<mlua::String, mlua::Table>(){
                        let (key, value) = pair.unwrap();
                        let key_str = key.to_str().unwrap();

                        if requests_list.contains(&key_str){
                            service_requests.push(key_str.to_string());
                        }
                    }

                    services.push(service_function(service_requests));
                }
            };

            make_subscribtion("hyprland", vec!["workspaces", "clients", "active_client"], hyprland_service_channel);
            // make_subscribtion("niri", vec!["workspace_changed"])
            make_subscribtion(
                "mpris",
                vec![
                    "playing",
                    "paused",
                    "stopped",
                    "volume_changed",
                    "looping_changed",
                    "shuffle_toggled",
                    "track_changed"
                ],
                mpris_service_channel
            );
            make_subscribtion("system_tray", vec!["update"], system_tray_service_channel);
            services.push(
                services::live_reloading::live_reload_service_channel(self.config_path.clone())
            );

            if let Ok(calls) = subscription_data.get::<_, mlua::Table>("calls") {
                let mut service_requests: Vec<String> = Vec::new();

                for pair in calls.pairs::<mlua::String, mlua::Table>(){
                    let (key, value) = pair.unwrap();

                    service_requests.push(key.to_str().unwrap().to_string());
                }

                services.push(calls::calls_service_channel(service_requests));
            }
            if let Ok(time_dictionary) = subscription_data.get::<_, mlua::Table>("time") {
                for pair in time_dictionary.pairs::<mlua::Integer, mlua::Table>() {
                    let (key, value) = pair.unwrap();

                    services.push(time_service_channel(key as u64));
                }
            }


            return Subscription::batch(services);
        }
        Subscription::none()
    }


    fn core(&self) -> &cosmic::app::Core {
        &self.core
    }
    fn core_mut(&mut self) -> &mut cosmic::app::Core {
        &mut self.core
    }
}
