use iced::advanced::widget::Id;
use iced::Command;
use iced::event::{self, Event};
use iced::advanced::layout;
use iced::mouse;
use iced::overlay;
use iced::advanced::widget::tree::{self, Tree};
use iced::advanced::widget::Operation;
use iced::{
    Background, advanced::Clipboard, Color, Element, advanced::Layout, Length, Padding, Rectangle,
    advanced::Shell, Size, advanced::Widget,
};

use iced::advanced::widget::operation;
use iced::advanced::widget::OperationOutputWrapper;

use iced::widget::button::{Appearance, StyleSheet};

/// A generic widget that produces a message when pressed.
///
/// ```no_run
/// # type Button<'a, Message> =
/// #     iced_widget::Button<'a, Message, iced_widget::style::Theme, iced_widget::renderer::Renderer>;
/// #
/// #[derive(Clone)]
/// enum Message {
///     ButtonPressed,
/// }
///
/// let button = Button::new("Press me!").on_press(Message::ButtonPressed);
/// ```
///
/// If a [`Button::on_press`] handler is not set, the resulting [`Button`] will
/// be disabled:
///
/// ```
/// # type Button<'a, Message> =
/// #     iced_widget::Button<'a, Message, iced_widget::style::Theme, iced_widget::renderer::Renderer>;
/// #
/// #[derive(Clone)]
/// enum Message {
///     ButtonPressed,
/// }
///
/// fn disabled_button<'a>() -> Button<'a, Message> {
///     Button::new("I'm disabled!")
/// }
///
/// fn enabled_button<'a>() -> Button<'a, Message> {
///     disabled_button().on_press(Message::ButtonPressed)
/// }
/// ```
#[allow(missing_debug_implementations)]
pub struct Button<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: StyleSheet,
    Renderer: iced::advanced::Renderer,
{
    content: Element<'a, Message, Theme, Renderer>,
    id: Id,
    on_press: Option<Message>,
    on_scroll_up: Option<Message>,
    on_scroll_down: Option<Message>,
    width: Length,
    height: Length,
    padding: Padding,
    style: Theme::Style,
}


impl<'a, Message, Theme, Renderer> Button<'a, Message, Theme, Renderer>
where
    Theme: StyleSheet,
    Renderer: iced::advanced::Renderer,
{
    /// Creates a new [`Button`] with the given content.
    pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        let content = content.into();
        let size = content.as_widget().size_hint();

        Button {
            content,
            id: Id::unique(),
            on_press: None,
            on_scroll_up: None,
            on_scroll_down: None,
            width: size.width.fluid(),
            height: size.height.fluid(),
            padding: Padding::new(5.0),
            style: Theme::Style::default(),
        }
    }

    /// Sets the width of the [`Button`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Button`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the [`Padding`] of the [`Button`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the message that will be produced when the [`Button`] is pressed.
    ///
    /// Unless `on_press` is called, the [`Button`] will be disabled.
    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_press = Some(on_press);
        self
    }

    /// Sets the message that will be produced when the [`Button`] is scrolled upwards.
    pub fn on_scroll_up(mut self, on_scroll: Message) -> Self {
        self.on_scroll_up = Some(on_scroll);
        self
    }

    /// Sets the message that will be produced when the [`Button`] is scrolled downwards.
    pub fn on_scroll_down(mut self, on_scroll: Message) -> Self {
        self.on_scroll_down = Some(on_scroll);
        self
    }

    /// Sets the message that will be produced when the [`Button`] is pressed,
    /// if `Some`.
    ///
    /// If `None`, the [`Button`] will be disabled.
    pub fn on_press_maybe(mut self, on_press: Option<Message>) -> Self {
        self.on_press = on_press;
        self
    }

    /// Sets the style variant of this [`Button`].
    pub fn style(mut self, style: impl Into<Theme::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the [`Id`] of the [`Button`].
    pub fn id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Button<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: StyleSheet,
    Renderer: 'a + iced::advanced::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.content))
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout(limits, self.width, self.height, self.padding, |limits| {
            self.content.as_widget().layout(
                &mut tree.children[0],
                renderer,
                limits,
            )
        })
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<OperationOutputWrapper<Message>>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.content.as_widget().operate(
                &mut tree.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        if let event::Status::Captured = self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        ) {
            return event::Status::Captured;
        }

        update(
            self.id.clone(),
            event,
            layout,
            cursor,
            shell,
            &self.on_press,
            &self.on_scroll_up,
            &self.on_scroll_down,
            || tree.state.downcast_mut::<State>(),
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        renderer_style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let content_layout = layout.children().next().unwrap();

        let styling = draw(
            renderer,
            bounds,
            cursor,
            self.on_press.is_some(),
            theme,
            &self.style,
            || tree.state.downcast_ref::<State>(),
        );

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            &iced::advanced::renderer::Style {
                icon_color: styling
                    .icon_color
                    .unwrap_or(renderer_style.icon_color),
                text_color: styling.text_color,
                scale_factor: renderer_style.scale_factor,
            },
            content_layout,
            cursor,
            &bounds,
        );
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        mouse_interaction(layout, cursor, self.on_press.is_some())
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
        )
    }

    fn id(&self) -> Option<Id> {
        Some(self.id.clone())
    }

    fn set_id(&mut self, id: Id) {
        self.id = id;
    }
}

impl<'a, Message, Theme, Renderer> From<Button<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: StyleSheet + 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(button: Button<'a, Message, Theme, Renderer>) -> Self {
        Self::new(button)
    }
}

/// The local state of a [`Button`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    is_hovered: bool,
    is_pressed: bool,
    is_focused: bool,
}

impl State {
    /// Creates a new [`State`].
    pub fn new() -> State {
        State::default()
    }

    /// Returns whether the [`Button`] is currently focused or not.
    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    /// Returns whether the [`Button`] is currently hovered or not.
    pub fn is_hovered(&self) -> bool {
        self.is_hovered
    }

    /// Focuses the [`Button`].
    pub fn focus(&mut self) {
        self.is_focused = true;
    }

    /// Unfocuses the [`Button`].
    pub fn unfocus(&mut self) {
        self.is_focused = false;
    }
}

// INFO: changed this from the regular button!

/// Processes the given [`Event`] and updates the [`State`] of a [`Button`]
/// accordingly.
pub fn update<'a, Message: Clone>(
    _id: Id,
    event: Event,
    layout: Layout<'_>,
    cursor: mouse::Cursor,
    shell: &mut Shell<'_, Message>,
    on_press: &Option<Message>,
    on_scroll_up: &Option<Message>,
    on_scroll_down: &Option<Message>,
    state: impl FnOnce() -> &'a mut State,
) -> event::Status {
    match event {
        Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
            if on_scroll_up.is_some() | on_scroll_down.is_some(){
                let bounds = layout.bounds();

                if cursor.is_over(bounds) {
                    if let mouse::ScrollDelta::Lines { x, y } = delta {

                        if y == 1.0 && on_scroll_up.is_some(){
                            shell.publish(on_scroll_up.clone().unwrap());
                        } else if y == -1.0 && on_scroll_down.is_some() {
                            shell.publish(on_scroll_down.clone().unwrap());
                        }
                        return event::Status::Captured;
                    }

                }
            }
        }
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
        | Event::Touch(iced::touch::Event::FingerPressed { .. }) => {
            if on_press.is_some() {
                let bounds = layout.bounds();

                if cursor.is_over(bounds) {
                    let state = state();

                    state.is_pressed = true;

                    return event::Status::Captured;
                }
            }
        }
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
        | Event::Touch(iced::touch::Event::FingerLifted { .. }) => {
            if let Some(on_press) = on_press.clone() {
                let state = state();

                if state.is_pressed {
                    state.is_pressed = false;

                    let bounds = layout.bounds();

                    if cursor.is_over(bounds) {
                        shell.publish(on_press);
                    }

                    return event::Status::Captured;
                }
            }
        }
        Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. }) => {
            if let Some(on_press) = on_press.clone() {
                let state = state();
                if state.is_focused
                    && matches!(
                        key,
                        iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter)
                    )
                {
                    state.is_pressed = true;
                    shell.publish(on_press);
                    return event::Status::Captured;
                }
            }
        }
        Event::Touch(iced::touch::Event::FingerLost { .. })
        | Event::Mouse(mouse::Event::CursorLeft) => {
            let state = state();
            state.is_hovered = false;
            state.is_pressed = false;
        }
        _ => {}
    }

    event::Status::Ignored
}

/// Draws a [`Button`].
pub fn draw<'a, Theme, Renderer: iced::advanced::Renderer>(
    renderer: &mut Renderer,
    bounds: Rectangle,
    cursor: mouse::Cursor,
    is_enabled: bool,
    theme: &Theme,
    style: &Theme::Style,
    state: impl FnOnce() -> &'a State,
) -> Appearance
where
    Theme: StyleSheet,
{
    let is_mouse_over = cursor.is_over(bounds);

    let styling = if !is_enabled {
        theme.disabled(style)
    } else if is_mouse_over {
        let state = state();

        if state.is_pressed {
            theme.pressed(style)
        } else {
            theme.hovered(style)
        }
    } else {
        theme.active(style)
    };

    if styling.background.is_some()
        || styling.border.width > 0.0
        || styling.shadow.color.a > 0.0
    {
        renderer.fill_quad(
            iced::advanced::renderer::Quad {
                bounds,
                border: styling.border,
                shadow: styling.shadow,
            },
            styling
                .background
                .unwrap_or(Background::Color(Color::TRANSPARENT)),
        );
    }

    styling
}

/// Computes the layout of a [`Button`].
pub fn layout(
    limits: &layout::Limits,
    width: Length,
    height: Length,
    padding: Padding,
    layout_content: impl FnOnce(&layout::Limits) -> layout::Node,
) -> layout::Node {
    layout::padded(limits, width, height, padding, layout_content)
}

/// Returns the [`mouse::Interaction`] of a [`Button`].
pub fn mouse_interaction(
    layout: Layout<'_>,
    cursor: mouse::Cursor,
    is_enabled: bool,
) -> mouse::Interaction {
    let is_mouse_over = cursor.is_over(layout.bounds());

    if is_mouse_over && is_enabled {
        mouse::Interaction::Pointer
    } else {
        mouse::Interaction::default()
    }
}

/// Produces a [`Command`] that focuses the [`Button`] with the given [`Id`].
pub fn focus<Message: 'static>(id: Id) -> Command<Message> {
    Command::widget(operation::focusable::focus(id))
}

impl operation::Focusable for State {
    fn is_focused(&self) -> bool {
        State::is_focused(self)
    }

    fn focus(&mut self) {
        State::focus(self)
    }

    fn unfocus(&mut self) {
        State::unfocus(self)
    }
}
