use iced::{
    executor, wayland::{
        actions::layer_surface::SctkLayerSurfaceSettings,
        layer_surface::{Anchor, KeyboardInteractivity, Layer},
        InitialSurface,
    }, widget::{row, text}, window::Id, Application, Command, Element, Font, Length, Pixels, Settings, Theme
};

pub struct Window;


pub enum Message {
    None,

}

// impl Application for Window {
//     type Executor = executor::Default;
//     type Flags = ();
//     type Message = ();
//     type Theme = Theme;
//     // application ID for window manager
//     // const APP_ID: &'static str = "vnuxa.astrum";
//     //
//     fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
//         (Self, Command::none())
//     }
//
//     fn title(&self, id: Id) -> String {
//         String::from("astrum")
//     }
//
//     fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
//         Command::none() // TODO:  this is where application logic is at, make the user change these
//     }
//
//     fn view(&self, id: Id) -> Element<Self::Message> {
//         let content: Element<_> = text("wow this works!").width(Length::Shrink).into();
//         content
//     }
// }

macro_rules! app {
    // or maybe for messages, use expr ?
    ($t: ty, $messages:ident, $update_logic:expr, $view_logic:expr) => {
        impl Application for $t {
            type Executor = executor::Default;
            type Flags = ();
            type Message = $messages

            fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
                (Self, Command::none())
            }

            fn title(&self, id: Id) -> String {
                String::from("astrum")
            }

            fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
                // closures
                $update_logic(self, message)
            }

            fn view(&self, id: Id) -> Element<Self::Message> {
                $view_logic(self, id)
            }
        }
    }
}


pub struct App_settings {
    height: u32,
    keyboard_interactivity: Option<KeyboardInteractivity>,
    anchor: Anchor,
    font: Option<Font>,
    exit_on_close_request: bool,
    default_text_size: Option<Pixels>,

}


// maybe make this in to a macro that has a closure??
pub fn run_app<app_struct> (
    app: app_struct,
    app_settings: App_settings
)
    where app_struct: iced::Application,
{
    app::run(Settings {
        antialiasing: true,
        exit_on_close_request: app_settings.exit_on_close_request, // maybe popups???

        initial_surface: InitialSurface::LayerSurface(SctkLayerSurfaceSettings {
            id: Id::MAIN,
            keyboard_interactivity: app_settings.keyboard_interactivity.unwrap_or(KeyboardInteractivity::None),
            namespace: "astrum".into(), // changable
            layer: Layer::Top,
            size: Some((None, Some(app_settings.height))),
            // maybe only height is chanagable
            // some presets like "Full" to make it automatically use the full monitors size??
            exclusive_zone: app_settings.height as i32, // dont know if this is a temporary fix
            anchor: app_settings.anchor,
            ..Default::default()
        }),
        flags: (),
        id: None,
        fonts: Default::default(), // TODO: make it customizable by user
        default_font: Font::default(),
        default_text_size: app_settings.default_text_size.unwrap_or(14.into()),
    })
}

const HEIGHT: u32 = 34;

fn main() {
    // TODO: make most of these settings cusstomizable by user
    Window::run(Settings {
        antialiasing: true,
        exit_on_close_request: false,

        // TODO: make the user change these settings!!
        initial_surface: InitialSurface::LayerSurface(SctkLayerSurfaceSettings {
            id: Id::MAIN,
            keyboard_interactivity: KeyboardInteractivity::None, // TODO: make the user change this
            namespace: "astrum".into(), // maybe user can change it?
            layer: Layer::Top,
            size: Some((None, Some(HEIGHT))), // make the user be able to change these, maybe make
            // some presets like "Full" to make it automatically use the full monitors size?? would
            exclusive_zone: HEIGHT as i32, // same as above ^
            anchor: Anchor::TOP.union(Anchor::LEFT).union(Anchor::RIGHT), // anchors, maybe make it
            // a function or something or a table of bools or a table of strings to know whcih ones
            // to use
            ..Default::default()
            // also somehow import custom user settings too

        }),
        flags: (),
        id: None,
        fonts: Default::default(), // TODO: make it customizable by user
        default_font: Font::default(),
        default_text_size: 14.into(),
    })
    .unwrap();
}
