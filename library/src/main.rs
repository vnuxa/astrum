use iced::{
    executor, wayland::{
        actions::layer_surface::SctkLayerSurfaceSettings,
        layer_surface::{Anchor, KeyboardInteractivity, Layer},
        InitialSurface,
    }, widget::{row, text}, window::Id, Application, Command, Element, Font, Length, Settings, Theme
};

mod widgets;

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
                $update_logic(self, message)
            }

            fn view(&self, id: Id) -> Element<Self::Message> {
                $view_logic(self, id)
            }
        }
    }
}
pub fn main() {
    println!("vauu this is great");
}
