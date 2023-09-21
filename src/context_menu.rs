//! Decorate content and apply alignment.
use iced::advanced::layout;
use iced::advanced::layout::Node;
use iced::advanced::mouse;
use iced::advanced::overlay;
use iced::advanced::renderer::Style;
use iced::advanced::widget::{self, Operation, Tree};
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::event::{Event, Status};
use iced::mouse::{Button, Cursor};
use iced::{Element, Length, Point, Rectangle, Size};

/// An element that shows a menu on right click.
#[allow(missing_debug_implementations)]
pub struct ContextMenu<'a, Message, Renderer = iced::Renderer>
where
    Renderer: iced::advanced::Renderer,
    Message: Clone,
{
    id: Id,
    active: Option<(Id, Point)>,
    on_open: Box<dyn Fn(Id, Point) -> Message + 'a>,
    on_right_click: Message,
    content: Element<'a, Message, Renderer>,
    context_menu_content: Element<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> ContextMenu<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: iced::advanced::Renderer,
{
    /// Creates a [`ContextMenu`].
    pub fn new<T, V, F>(
        id: Id,
        active: Option<(Id, Point)>,
        on_open: F,
        on_close: Message,
        content: T,
        content2: V,
    ) -> Self
    where
        T: Into<Element<'a, Message, Renderer>>,
        V: Into<Element<'a, Message, Renderer>>,
        F: Fn(Id, Point) -> Message + 'a,
    {
        ContextMenu {
            id,
            active,
            on_open: Box::new(on_open),
            on_right_click: on_close,
            content: content.into(),
            context_menu_content: content2.into(),
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for ContextMenu<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: iced::advanced::Renderer,
{
    fn width(&self) -> Length {
        self.content.as_widget().width()
    }

    fn height(&self) -> Length {
        self.content.as_widget().height()
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) -> Node {
        self.content
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        renderer_style: &Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            renderer_style,
            layout,
            cursor,
            viewport,
        );
    }

    fn children(&self) -> Vec<Tree> {
        vec![
            Tree::new(&self.content),
            Tree::new(&self.context_menu_content),
        ]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.content, &self.context_menu_content]);
    }

    fn operate(
        &self,
        state: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        self.content
            .as_widget()
            .operate(&mut state.children[0], layout, renderer, operation);
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> Status {
        let mut propagate = true;
        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(Button::Right) => {
                    propagate = false;
                    if cursor.is_over(layout.bounds()) {
                        let message = (self.on_open)(self.id.clone(), cursor.position().unwrap());
                        shell.publish(message);
                    }
                }
                _ => {}
            },
            _ => {}
        }

        if propagate {
            self.content.as_widget_mut().on_event(
                &mut tree.children[0],
                event,
                layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            )
        } else {
            Status::Captured
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        _layout: Layout<'_>,
        _renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        if let Some(active) = &self.active {
            if active.0 == self.id {
                return Some(overlay::Element::new(
                    active.1,
                    Box::new(Overlay {
                        content: &mut self.context_menu_content,
                        tree: &mut tree.children[1],
                        on_right_click: self.on_right_click.clone(),
                    }),
                ));
            }
        }

        None
    }
}

impl<'a, Message, Renderer> From<ContextMenu<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: 'a + iced::advanced::Renderer,
{
    fn from(context_menu: ContextMenu<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(context_menu)
    }
}

/// The identifier of a [`ContextMenu`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(widget::Id);

impl Id {
    /// Creates a custom [`Id`].
    pub fn new(id: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        Self(widget::Id::new(id))
    }

    /// Creates a unique [`Id`].
    ///
    /// This function produces a different [`Id`] every time it is called.
    pub fn unique() -> Self {
        Self(widget::Id::unique())
    }
}

impl From<Id> for widget::Id {
    fn from(id: Id) -> Self {
        id.0
    }
}

struct Overlay<'a, 'b, Message, Renderer>
where
    Message: Clone,
{
    content: &'b mut Element<'a, Message, Renderer>,
    tree: &'b mut Tree,
    on_right_click: Message,
}

impl<'a, 'b, Message, Renderer> overlay::Overlay<Message, Renderer>
    for Overlay<'a, 'b, Message, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, _bounds: Size, position: Point) -> Node {
        let limits = layout::Limits::new(Size::ZERO, Size::INFINITY)
            .width(Length::Shrink)
            .height(Length::Shrink);

        let child = self
            .content
            .as_widget()
            .layout(self.tree, renderer, &limits);

        let mut node = Node::with_children(Size::INFINITY, vec![child]);
        node.move_to(position);

        node
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &Style,
        layout: Layout<'_>,
        cursor: Cursor,
    ) {
        self.content.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            &layout.bounds(),
        );
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        self.content.as_widget().operate(
            self.tree,
            layout.children().next().unwrap(),
            renderer,
            operation,
        );
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> Status {
        let content_bounds = layout.children().next().unwrap().bounds();
        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(button) => match button {
                    Button::Left | Button::Right => {
                        if !cursor.is_over(content_bounds) {
                            shell.publish(self.on_right_click.clone());
                            return Status::Ignored;
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        self.content.as_widget_mut().on_event(
            self.tree,
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        )
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            self.tree,
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'c>(
        &'c mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, Renderer>> {
        self.content
            .as_widget_mut()
            .overlay(self.tree, layout.children().next().unwrap(), renderer)
    }
}

pub struct ContextMenuState {
    pub active_context_menu: Option<(Id, Point)>,
}

impl ContextMenuState {
    pub fn new() -> Self {
        Self {
            active_context_menu: None,
        }
    }

    pub fn on_right_click(&mut self, id: Id, position: Point) {
        if let Some(active) = self.active_context_menu.clone() {
            if active.0 == id {
                self.active_context_menu = None;
            } else {
                self.active_context_menu = Some((id, position));
            }
        } else {
            self.active_context_menu = Some((id, position));
        }
    }

    pub fn on_close(&mut self) {
        self.active_context_menu = None;
    }

    pub fn active(&self) -> Option<(Id, Point)> {
        self.active_context_menu.clone()
    }
}
