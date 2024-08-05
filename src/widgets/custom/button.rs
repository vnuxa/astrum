// Copyright 2019 H�ctor Ram�n, Iced contributors
// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: MIT

//! Allow your users to perform actions by pressing a button.
//!
//! A [`Button`] has some local [`State`].

use cosmic::cosmic_theme::THEME_MODE_ID;
use cosmic::widget::Id;
use cosmic::iced::{keyboard, Command};

use cosmic::iced_core::event::{self, Event};
use cosmic::iced_core::renderer::{self, Quad, Renderer};
use cosmic::iced_core::touch;
use cosmic::iced_core::widget::tree::{self, Tree};
use cosmic::iced_core::widget::Operation;
use cosmic::iced_core::{layout, svg};
use cosmic::iced_core::{mouse, Border};
use cosmic::iced_core::{overlay, Shadow};
use cosmic::iced_core::{
    Background, Clipboard, Color, Layout, Length, Padding, Point, Rectangle, Shell, Vector, Widget,
};
use cosmic::iced_renderer::core::widget::{operation, OperationOutputWrapper};

// use cosmic::theme::THEME;
use cosmic::theme::Theme;

// pub use super::style::{Appearance, StyleSheet};
use cosmic::widget::button::{Appearance, StyleSheet};

/// Internally defines different button widget variants.
enum Variant<Message> {
    Normal,
    Image {
        close_icon: svg::Handle,
        on_remove: Option<Message>,
    },
}

/// A generic button which emits a message when pressed.
#[allow(missing_debug_implementations)]
#[must_use]
pub struct Button<'a, Message> {
    id: Id,
    content: cosmic::Element<'a, Message>,
    on_press: Option<Message>,
    on_press_down: Option<Message>,
    on_scroll_up: Option<Message>, // custom
    on_scroll_down: Option<Message>, // custom
    width: Length,
    height: Length,
    padding: Padding,
    selected: bool,
    style: cosmic::theme::Button,
    variant: Variant<Message>,
}

impl<'a, Message> Button<'a, Message> {
    /// Creates a new [`Button`] with the given content.
    pub fn new(content: impl Into<cosmic::Element<'a, Message>>) -> Self {
        Self {
            id: Id::unique(),
            content: content.into(),
            on_press: None,
            on_press_down: None,
            on_scroll_up: None,
            on_scroll_down: None,
            width: Length::Shrink,
            height: Length::Shrink,
            padding: Padding::new(5.0),
            selected: false,
            style: cosmic::theme::Button::default(),
            variant: Variant::Normal,
        }
    }

    pub fn new_image(
        content: impl Into<cosmic::Element<'a, Message>>,
        on_remove: Option<Message>,
    ) -> Self {
        Self {
            id: Id::unique(),
            content: content.into(),
            on_press: None,
            on_press_down: None,
            on_scroll_up: None,
            on_scroll_down: None,
            width: Length::Shrink,
            height: Length::Shrink,
            padding: Padding::new(5.0),
            selected: false,
            style: cosmic::theme::Button::default(),
            variant: Variant::Image {
                on_remove,
                close_icon: cosmic::widget::icon::from_name("window-close-symbolic")
                    .size(8)
                    .icon()
                    .into_svg_handle()
                    .unwrap_or_else(|| {
                        let bytes: &'static [u8] = &[];
                        cosmic::iced_core::svg::Handle::from_memory(bytes)
                    }),
            },
        }
    }

    /// Sets the [`Id`] of the [`Button`].
    pub fn id(mut self, id: Id) -> Self {
        self.id = id;
        self
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

    /// Sets the message that will be produced when the [`Button`] is pressed and released.
    ///
    /// Unless `on_press` or `on_press_down` is called, the [`Button`] will be disabled.
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
    ///
    /// Unless `on_press` or `on_press_down` is called, the [`Button`] will be disabled.
    pub fn on_press_down(mut self, on_press: Message) -> Self {
        self.on_press_down = Some(on_press);
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

    /// Sets the widget to a selected state.
    ///
    /// Displays a selection indicator on image buttons.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;

        self
    }

    /// Sets the style variant of this [`Button`].
    pub fn style(mut self, style: cosmic::theme::Button) -> Self {
        self.style = style;
        self
    }

}

impl<'a, Message: 'a + Clone> Widget<Message, cosmic::Theme, cosmic::Renderer>
    for Button<'a, Message>
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
        tree.diff_children(std::slice::from_mut(&mut self.content));
    }

    fn size(&self) -> cosmic::iced_core::Size<Length> {
        cosmic::iced_core::Size::new(self.width, self.height)
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &cosmic::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout(
            renderer,
            limits,
            self.width,
            self.height,
            self.padding,
            |renderer, limits| {
                self.content
                    .as_widget()
                    .layout(&mut tree.children[0], renderer, limits)
            },
        )
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &cosmic::Renderer,
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
        let state = tree.state.downcast_mut::<State>();
        operation.focusable(state, Some(&self.id));
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &cosmic::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        if let Variant::Image {
            on_remove: Some(on_remove),
            ..
        } = &self.variant
        {
            // Capture mouse/touch events on the removal button
            match event {
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    if let Some(position) = cursor.position() {
                        if removal_bounds(layout.bounds(), 4.0).contains(position) {
                            shell.publish(on_remove.clone());
                            return event::Status::Captured;
                        }
                    }
                }

                _ => (),
            }
        }

        if self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        ) == event::Status::Captured
        {
            return event::Status::Captured;
        }

        update(
            self.id.clone(),
            event,
            layout,
            cursor,
            shell,
            &self.on_press,
            &self.on_press_down,
            &self.on_scroll_up,
            &self.on_scroll_down,
            || tree.state.downcast_mut::<State>(),
        )
    }

    #[allow(clippy::too_many_lines)]
    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut cosmic::Renderer,
        theme: &cosmic::Theme,
        renderer_style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let content_layout = layout.children().next().unwrap();

        let mut headerbar_alpha = None;

        let is_enabled = self.on_press.is_some() || self.on_press_down.is_some();
        let is_mouse_over = cursor.position().is_some_and(|p| bounds.contains(p));

        let state = tree.state.downcast_ref::<State>();

        let styling = if !is_enabled {
            theme.disabled(&self.style)
        } else if is_mouse_over {
            if state.is_pressed {
                if !self.selected && matches!(self.style, cosmic::theme::Button::HeaderBar) {
                    headerbar_alpha = Some(0.8);
                }

                theme.pressed(state.is_focused, self.selected, &self.style)
            } else {
                if !self.selected && matches!(self.style, cosmic::theme::Button::HeaderBar) {
                    headerbar_alpha = Some(0.8);
                }

                theme.hovered(state.is_focused, self.selected, &self.style)
            }
        } else {
            if !self.selected && matches!(self.style, cosmic::theme::Button::HeaderBar) {
                headerbar_alpha = Some(0.75);
            }

            theme.active(state.is_focused, self.selected, &self.style)
        };

        let mut icon_color = styling.icon_color.unwrap_or(renderer_style.icon_color);

        // Menu roots should share the accent color that icons get in the header.
        let mut text_color = if matches!(self.style, cosmic::theme::Button::MenuRoot) {
            icon_color
        } else {
            styling.text_color.unwrap_or(renderer_style.text_color)
        };

        if let Some(alpha) = headerbar_alpha {
            icon_color.a = alpha;
            text_color.a = alpha;
        }

        draw::<_, cosmic::Theme>(
            renderer,
            bounds,
            &styling,
            |renderer, _styling| {
                self.content.as_widget().draw(
                    &tree.children[0],
                    renderer,
                    theme,
                    &renderer::Style {
                        icon_color,
                        text_color,
                        scale_factor: renderer_style.scale_factor,
                    },
                    content_layout,
                    cursor,
                    &bounds,
                );
            },
            matches!(self.variant, Variant::Image { .. }),
        );

        if let Variant::Image {
            close_icon,
            on_remove,
        } = &self.variant
        {
            let mut parent_bounds = bounds;
            parent_bounds.y -= 8.0;
            parent_bounds.width += 16.0;
            parent_bounds.height += 16.0;

            renderer.with_layer(parent_bounds, |renderer| {
                let selection_background = theme.selection_background();

                // let c_rad = THEME.with(|t| t.borrow().cosmic().corner_radii);
                let c_rad = theme.borrow().cosmic().corner_radii;

                // NOTE: Workaround to round the border of the unselected, unhovered image.
                if !self.selected && !is_mouse_over {
                    let mut bounds = bounds;
                    bounds.x -= 2.0;
                    bounds.y -= 2.0;
                    bounds.width += 4.0;
                    bounds.height += 4.0;
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds,
                            border: Border {
                                width: 2.0,
                                color: cosmic::theme::active().current_container().base.into(),
                                radius: 9.0.into(),
                            },
                            shadow: Shadow::default(),
                        },
                        Color::TRANSPARENT,
                    );
                }

                if self.selected {
                    renderer.fill_quad(
                        Quad {
                            bounds: Rectangle {
                                width: 24.0,
                                height: 20.0,
                                x: bounds.x + styling.border_width,
                                y: bounds.y + (bounds.height - 20.0 - styling.border_width),
                            },
                            border: Border {
                                radius: [
                                    c_rad.radius_0[0],
                                    c_rad.radius_s[1],
                                    c_rad.radius_0[2],
                                    c_rad.radius_s[3],
                                ]
                                .into(),
                                ..Default::default()
                            },
                            shadow: Shadow::default(),
                        },
                        selection_background,
                    );

                    cosmic::iced_core::svg::Renderer::draw(
                        renderer,
                        object_select().clone(),
                        Some(icon_color),
                        Rectangle {
                            width: 16.0,
                            height: 16.0,
                            x: bounds.x + 5.0 + styling.border_width,
                            y: bounds.y + (bounds.height - 18.0 - styling.border_width),
                        },
                    );
                }

                if on_remove.is_some() {
                    if let Some(position) = cursor.position() {
                        if bounds.contains(position) {
                            let bounds = removal_bounds(layout.bounds(), 4.0);
                            renderer.fill_quad(
                                renderer::Quad {
                                    bounds,
                                    shadow: Shadow::default(),
                                    border: Border {
                                        radius: c_rad.radius_m.into(),
                                        ..Default::default()
                                    },
                                },
                                selection_background,
                            );

                            cosmic::iced_core::svg::Renderer::draw(
                                renderer,
                                close_icon.clone(),
                                Some(icon_color),
                                Rectangle {
                                    width: 16.0,
                                    height: 16.0,
                                    x: bounds.x + 4.0,
                                    y: bounds.y + 4.0,
                                },
                            );
                        }
                    }
                }
            });
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &cosmic::Renderer,
    ) -> mouse::Interaction {
        mouse_interaction(layout, cursor, self.on_press.is_some())
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &cosmic::Renderer,
    ) -> Option<overlay::Element<'b, Message, cosmic::Theme, cosmic::Renderer>> {
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

impl<'a, Message: Clone + 'a> From<Button<'a, Message>> for cosmic::Element<'a, Message> {
    fn from(button: Button<'a, Message>) -> Self {
        Self::new(button)
    }
}

/// The local state of a [`Button`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(clippy::struct_field_names)]
pub struct State {
    is_hovered: bool,
    is_pressed: bool,
    is_focused: bool,
}

impl State {
    /// Creates a new [`State`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns whether the [`Button`] is currently focused or not.
    pub fn is_focused(self) -> bool {
        self.is_focused
    }

    /// Returns whether the [`Button`] is currently hovered or not.
    pub fn is_hovered(self) -> bool {
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

/// Processes the given [`Event`] and updates the [`State`] of a [`Button`]
/// accordingly.
#[allow(clippy::needless_pass_by_value)]
pub fn update<'a, Message: Clone>(
    _id: Id,
    event: Event,
    layout: Layout<'_>,
    cursor: mouse::Cursor,
    shell: &mut Shell<'_, Message>,
    on_press: &Option<Message>,
    on_press_down: &Option<Message>,
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
        | Event::Touch(touch::Event::FingerPressed { .. }) => {
            if on_press.is_some() || on_press_down.is_some() {
                let bounds = layout.bounds();

                if cursor.is_over(bounds) {
                    let state = state();

                    state.is_pressed = true;

                    if let Some(on_press_down) = on_press_down {
                        shell.publish(on_press_down.clone());
                    }

                    return event::Status::Captured;
                }
            }
        }
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
        | Event::Touch(touch::Event::FingerLifted { .. }) => {
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
            } else if on_press_down.is_some() {
                let state = state();
                state.is_pressed = false;
            }
        }
        Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
            if let Some(on_press) = on_press.clone() {
                let state = state();
                if state.is_focused && key == keyboard::Key::Named(keyboard::key::Named::Enter) {
                    state.is_pressed = true;
                    shell.publish(on_press);
                    return event::Status::Captured;
                }
            }
        }
        Event::Touch(touch::Event::FingerLost { .. }) | Event::Mouse(mouse::Event::CursorLeft) => {
            let state = state();
            state.is_hovered = false;
            state.is_pressed = false;
        }
        _ => {}
    }

    event::Status::Ignored
}

#[allow(clippy::too_many_arguments)]
pub fn draw<Renderer: cosmic::iced_core::Renderer, Theme>(
    renderer: &mut Renderer,
    bounds: Rectangle,
    styling: &cosmic::widget::button::Appearance,
    draw_contents: impl FnOnce(&mut Renderer, &Appearance),
    is_image: bool,
) where
    Theme: cosmic::widget::style::StyleSheet,
{
    let doubled_border_width = styling.border_width * 2.0;
    let doubled_outline_width = styling.outline_width * 2.0;

    if styling.outline_width > 0.0 {
        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: bounds.x - styling.border_width - styling.outline_width,
                    y: bounds.y - styling.border_width - styling.outline_width,
                    width: bounds.width + doubled_border_width + doubled_outline_width,
                    height: bounds.height + doubled_border_width + doubled_outline_width,
                },
                border: Border {
                    width: styling.outline_width,
                    color: styling.outline_color,
                    radius: styling.border_radius,
                },
                shadow: Shadow::default(),
            },
            Color::TRANSPARENT,
        );
    }

    if styling.background.is_some() || styling.border_width > 0.0 {
        if styling.shadow_offset != Vector::default() {
            // TODO: Implement proper shadow support
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: bounds.x + styling.shadow_offset.x,
                        y: bounds.y + styling.shadow_offset.y,
                        width: bounds.width,
                        height: bounds.height,
                    },
                    border: Border {
                        radius: styling.border_radius,
                        ..Default::default()
                    },
                    shadow: Shadow::default(),
                },
                Background::Color([0.0, 0.0, 0.0, 0.5].into()),
            );
        }

        // Draw the button background first.
        if let Some(background) = styling.background {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        radius: styling.border_radius,
                        ..Default::default()
                    },
                    shadow: Shadow::default(),
                },
                background,
            );
        }

        // Then button overlay if any.
        if let Some(overlay) = styling.overlay {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        radius: styling.border_radius,
                        ..Default::default()
                    },
                    shadow: Shadow::default(),
                },
                overlay,
            );
        }

        // Then draw the button contents onto the background.
        draw_contents(renderer, styling);

        let mut clipped_bounds = bounds;
        clipped_bounds.height += styling.border_width;

        renderer.with_layer(clipped_bounds, |renderer| {
            // NOTE: Workaround to round the border of the hovered/selected image.
            if is_image {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds,
                        border: Border {
                            width: styling.border_width,
                            color: cosmic::theme::active().current_container().base.into(),
                            radius: 0.0.into(),
                        },
                        shadow: Shadow::default(),
                    },
                    Color::TRANSPARENT,
                );
            }

            // Finish by drawing the border above the contents.
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border {
                        width: styling.border_width,
                        color: styling.border_color,
                        radius: styling.border_radius,
                    },
                    shadow: Shadow::default(),
                },
                Color::TRANSPARENT,
            );
        });
    } else {
        draw_contents(renderer, styling);
    }
}

/// Computes the layout of a [`Button`].
pub fn layout<Renderer>(
    renderer: &Renderer,
    limits: &layout::Limits,
    width: Length,
    height: Length,
    padding: Padding,
    layout_content: impl FnOnce(&Renderer, &layout::Limits) -> layout::Node,
) -> layout::Node {
    let limits = limits.width(width).height(height);

    let mut content = layout_content(renderer, &limits.shrink(padding));
    let padding = padding.fit(content.size(), limits.max());
    let size = limits
        .shrink(padding)
        .resolve(width, height, content.size())
        .expand(padding);

    content = content.move_to(Point::new(padding.left, padding.top));

    layout::Node::with_children(size, vec![content])
}

/// Returns the [`mouse::Interaction`] of a [`Button`].
#[must_use]
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
        Self::is_focused(*self)
    }

    fn focus(&mut self) {
        Self::focus(self);
    }

    fn unfocus(&mut self) {
        Self::unfocus(self);
    }
}

fn removal_bounds(bounds: Rectangle, offset: f32) -> Rectangle {
    Rectangle {
        x: bounds.x + bounds.width - 12.0 - offset,
        y: bounds.y - 12.0 + offset,
        width: 24.0,
        height: 24.0,
    }
}


use std::borrow::Borrow;
use std::sync::OnceLock;

/// Static `svg::Handle` to the `object-select-symbolic` icon.
pub fn object_select() -> &'static cosmic::widget::svg::Handle {
    static SELECTION_ICON: OnceLock<cosmic::widget::svg::Handle> = OnceLock::new();

    SELECTION_ICON.get_or_init(|| {
        cosmic::widget::icon::from_name("object-select-symbolic")
            .size(16)
            .icon()
            .into_svg_handle()
            .unwrap_or_else(|| {
                let bytes: &'static [u8] = &[];
                cosmic::iced_core::svg::Handle::from_memory(bytes)
            })
    })
}
