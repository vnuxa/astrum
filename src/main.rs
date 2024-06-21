use iced::{
    executor,
    wayland::{
        actions::layer_surface::SctkLayerSurfaceSettings,
        layer_surface::{Anchor, KeyboardInteractivity, Layer},
        InitialSurface,
    },
    window::Id, Application, Command, Executor, Font, Settings, Theme,
    Message, Theme, Renderer, Element
};

pub struct App;


impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;
    // application ID for window manager
    // const APP_ID: &'static str = "vnuxa.astrum";
    //
    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self, Command::none())
    }

    fn title(&self, id: Id) -> String {
        String::from("astrum")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none() // TODO:  this is where application logic is at, make the user change these
    }

    fn view(&self) -> Element<Message> {
        let content: Element<_> = text!("wow this works!").size(40).into();
        center(content).into()
    }
}


const HEIGHT: u32 = 34;

fn main() {


    // TODO: make most of these settings cusstomizable by user
    App::run(Settings {
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
