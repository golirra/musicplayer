//A modified version of a Container that enables drag/drop functionality on items inside it 
#![allow(warnings)]
mod pane {
    use crate::playground;
    use iced::advanced::layout::{self, Layout};
    use iced::border::{self, Border};
    use iced::advanced::{renderer, overlay as core_overlay};
    use iced::advanced::widget::{self, Operation, Widget};
    use iced::advanced::widget::tree::{self, Tree};
    use iced::theme::palette;
    use iced::overlay;
    use iced::mouse;
    use iced::mouse::Cursor;
    use iced::event::Status as IcedStatus;
    use iced::Event;
    use iced::Point;
    use iced::event;
    use iced::Task;
    use iced::Gradient;
    use iced::gradient;
    use iced::task;

    use iced::advanced::{Shell, Overlay, Clipboard};
    use iced::{Background, Color, Element, Length, Padding, Rectangle,
        Shadow, Size, Theme, Vector};
    use iced::alignment;
    use iced::alignment::Alignment;
    use iced::Pixels;
    use iced::color;


    //This struct represents the state of the widget. Must be given a pos on construction
    pub struct Pane<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
    where 
        Renderer: renderer::Renderer,
        Theme: Catalog,
    {
        id: Option<Id>,
        content: Element<'a, Message, Theme, Renderer>,
        on_drop: Option<OnDrop<'a, Message>>,
        width: Length,
        height: Length,
        max_width: f32,
        max_height: f32,
        horizontal_alignment: alignment::Horizontal,
        vertical_alignment: alignment::Vertical,
        padding: Padding,
        clip: bool,
        class: Theme::Class<'a>,
        dragging: bool
    }

    enum OnDrop<'a, Message> {
        Direct(Message),
        Closure(Box<dyn Fn() -> Message + 'a>),
    }

    impl<Message: Clone> OnDrop<'_, Message> {
        fn get(&self) -> Message {
            match self {
                OnDrop::Direct(message) => message.clone(),
                OnDrop::Closure(f) => f(),
            }
        }
    }

    enum CustomEvent {
        IcedEvent(Event),
        Drop,
        None,
    }

    impl<'a, Message, Theme, Renderer> Pane<'a, Message, Theme, Renderer>
    where 
        Renderer: renderer::Renderer, //is advanced::Renderer is renderer::Renderer in docs???
        Theme: Catalog,
    {
        pub fn new(
            content: impl Into<Element<'a, Message, Theme, Renderer>>,
        ) -> Self {
            let content = content.into();
            let size = content.as_widget().size_hint();

            Pane { 
                id: Some(Id::unique()),
                content,
                on_drop: None,
                width: size.width.fluid(),
                height: size.height.fluid(),
                max_width: f32::INFINITY,
                max_height: f32::INFINITY,
                horizontal_alignment: alignment::Horizontal::Left,
                vertical_alignment: alignment::Vertical::Top,
                padding: Padding::ZERO,
                clip: false,
                class: Theme::default(),
                dragging: false,
            }
        }

        pub fn on_drop(mut self, on_drop: Message) -> Self {
            self.on_drop = Some(OnDrop::Direct(on_drop));
            self
        }

        /// Sets the [`Id`] of the [`Pane`].
        pub fn id(mut self, id: Id) -> Self {
            self.id = Some(id);
            self
        }

        /// Sets the [`Padding`] of the [`Pane`].
        pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
            self.padding = padding.into();
            self
        }

        /// Sets the width of the [`Pane`].
        pub fn width(mut self, width: impl Into<Length>) -> Self {
            self.width = width.into();
            self
        }

        /// Sets the height of the [`Pane`].
        pub fn height(mut self, height: impl Into<Length>) -> Self {
            self.height = height.into();
            self
        }

        /// Sets the maximum width of the [`Pane`].
        pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
            self.max_width = max_width.into().0;
            self
        }

        /// Sets the maximum height of the [`Pane`].
        pub fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
            self.max_height = max_height.into().0;
            self
        }

        /// Sets the width of the [`Pane`] and centers its contents horizontally.
        pub fn center_x(self, width: impl Into<Length>) -> Self {
            self.width(width).align_x(alignment::Horizontal::Center)
        }

        /// Sets the height of the [`Pane`] and centers its contents vertically.
        pub fn center_y(self, height: impl Into<Length>) -> Self {
            self.height(height).align_y(alignment::Vertical::Center)
        }

        /// Centers the contents in both the horizontal and vertical axes of the
        /// [`Pane`].
        ///
        /// This is equivalent to chaining [`center_x`] and [`center_y`].
        ///
        /// [`center_x`]: Self::center_x
        /// [`center_y`]: Self::center_y
        pub fn center(self, length: impl Into<Length>) -> Self {
            let length = length.into();

            self.center_x(length).center_y(length)
        }

        /// Aligns the contents of the [`Pane`] to the left.
        pub fn align_left(self, width: impl Into<Length>) -> Self {
            self.width(width).align_x(alignment::Horizontal::Left)
        }

        /// Aligns the contents of the [`Pane`] to the right.
        pub fn align_right(self, width: impl Into<Length>) -> Self {
            self.width(width).align_x(alignment::Horizontal::Right)
        }

        /// Aligns the contents of the [`Pane`] to the top.
        pub fn align_top(self, height: impl Into<Length>) -> Self {
            self.height(height).align_y(alignment::Vertical::Top)
        }

        /// Aligns the contents of the [`Pane`] to the bottom.
        pub fn align_bottom(self, height: impl Into<Length>) -> Self {
            self.height(height).align_y(alignment::Vertical::Bottom)
        }

        /// Sets the content alignment for the horizontal axis of the [`Pane`].
        pub fn align_x(
            mut self,
            alignment: impl Into<alignment::Horizontal>,
        ) -> Self {
            self.horizontal_alignment = alignment.into();
            self
        }

        /// Sets the content alignment for the vertical axis of the [`Pane`].
        pub fn align_y(
            mut self,
            alignment: impl Into<alignment::Vertical>,
        ) -> Self {
            self.vertical_alignment = alignment.into();
            self
        }

        // pub fn overlay(
        //     self,
        //     position:Point,
        //     target_height: f32,
        // )-> overlay::Element<'a, Message,Theme,Renderer> {
        //     overlay::Element::new(Box::new(Overlay::new(
        //                 position,
        //                 self,
        //                 target_height,
        //     )))
        // }

        /// Sets whether the contents of the [`Pane`] should be clipped on
        /// overflow.
        pub fn clip(mut self, clip: bool) -> Self {
            self.clip = clip;
            self
        }

        /// Sets the style of the [`Pane`].
        #[must_use]
        pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
            where
            Theme::Class<'a>: From<StyleFn<'a, Theme>>,
            {
                self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
                self
            }

        /// Sets the style class of the [`Pane`].
        #[must_use]
        pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
            self.class = class.into();
            self
        }

    }
    //Store stuff in a State struct rather than the Pane struct, as we have to update the state
    //based
    #[derive(Debug, Clone, Copy, PartialEq, Default)] 
    struct State {
        is_pressed: bool,
    }

    /*
       pub fn pane<'a, Message>(pos: Point) -> Pane<'a, Message> {
       Pane::new(pos)
       }
       */

    impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
        for Pane<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'a,
        Theme: Catalog,
        Renderer: renderer::Renderer,
    {
        fn tag(&self) -> tree::Tag {
            self.content.as_widget().tag()
        }

        fn state(&self) -> tree::State {
            self.content.as_widget().state()
        }

        fn children(&self) -> Vec<Tree> {
            self.content.as_widget().children()
        }

        fn diff(&self, tree: &mut Tree) {
            self.content.as_widget().diff(tree);
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
            layout(
                limits,
                self.width,
                self.height,
                self.max_width,
                self.max_height,
                self.padding,
                self.horizontal_alignment,
                self.vertical_alignment,
                |limits| self.content.as_widget().layout(tree, renderer, limits),
            )

        }

        fn operate(
            &self,
            tree: &mut Tree,
            layout: Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn Operation,
        ) {
            println!("operating");
            operation.container(
                self.id.as_ref().map(|id| &id.0),
                layout.bounds(),
                &mut |operation| {
                    self.content.as_widget().operate(
                        tree,
                        layout.children().next().unwrap(),
                        renderer,
                        operation,
                    );
                },
            );
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
        ) -> IcedStatus {//IcedStatus is just iced::event::Status
            let mut operation = playground::button::print_bounds();
        


            match event {
                Event::Mouse(mouse_event) => match mouse_event {
                    iced::mouse::Event::ButtonPressed(iced::mouse::Button::Right) => {


                        if cursor.is_over(layout.bounds()) {
                            self.operate(tree, layout, &renderer, &mut operation);
                        }
                        return IcedStatus::Ignored;
                    },
                    // iced::mouse::Event::CursorMoved { position } => {
                    //     return IcedStatus::Ignored;
                    // },
                    iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left) => {
                        if let Some(on_drop) = self.on_drop.as_ref().map(OnDrop::get) {
                            // let state = tree.state.downcast_mut::<State>();
                        }
                        println!("test");
                    },
                    iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left) => {
                        if self.on_drop.is_some() && cursor.is_over(layout.bounds()) {
                            shell.publish(self.on_drop.as_ref().map(OnDrop::get).unwrap());
                        }
                        return IcedStatus::Captured;
                    },
                    _ => {
                        return IcedStatus::Ignored;
                    },
                }, 
                _ => { //handle other events
                    return IcedStatus::Ignored
                },

            }




            self.content.as_widget_mut().on_event(
                tree,
                event,
                layout.children().next().unwrap(),
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            )
        }

        fn mouse_interaction(
            &self,
            tree: &Tree,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            viewport: &Rectangle,
            renderer: &Renderer,
        ) -> mouse::Interaction {
            // println!("test");
            self.content.as_widget().mouse_interaction(
                tree,
                layout.children().next().unwrap(),
                cursor,
                viewport,
                renderer,
            )
        }

        fn draw(
            &self,
            tree: &Tree,
            renderer: &mut Renderer,
            theme: &Theme,
            renderer_style: &renderer::Style,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            viewport: &Rectangle,
        ) {
            let bounds = layout.bounds();
            let style = theme.style(&self.class);

            if let Some(clipped_viewport) = bounds.intersection(viewport) {
                draw_background(renderer, &style, bounds);

                self.content.as_widget().draw(
                    tree,
                    renderer,
                    theme,
                    &renderer::Style {
                        text_color: style
                            .text_color
                            .unwrap_or(renderer_style.text_color),
                    },
                    layout.children().next().unwrap(),
                    cursor,
                    if self.clip {
                        &clipped_viewport
                    } else {
                        viewport
                    },
                );
            }
        }

        fn overlay<'b>(
            &'b mut self,
            tree: &'b mut Tree,
            layout: Layout<'_>,
            renderer: &Renderer,
            translation: Vector,
        ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
            // core_overlay::from_children(
            //     &mut self.children,
            //     tree,
            //     layout,
            //     renderer,
            //     translation,
            // )
            self.content.as_widget_mut().overlay(
                tree,
                layout.children().next().unwrap(),
                renderer,
                translation,
            )
        }
    }

    impl<'a, Message, Theme, Renderer> From<Pane<'a, Message, Theme, Renderer>>
        for Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'a,
        Theme: Catalog + 'a,
        Renderer: renderer::Renderer + 'a,
        {
            fn from(
                pane: Pane<'a, Message, Theme, Renderer>,
            ) -> Element<'a, Message, Theme, Renderer> {
                Element::new(pane)
            }
        }

    /// Computes the layout of a [`Pane`].
    pub fn layout(
        limits: &layout::Limits,
        width: Length,
        height: Length,
        max_width: f32,
        max_height: f32,
        padding: Padding,
        horizontal_alignment: alignment::Horizontal,
        vertical_alignment: alignment::Vertical,
        layout_content: impl FnOnce(&layout::Limits) -> layout::Node,
    ) -> layout::Node {
        layout::positioned(
            &limits.max_width(max_width).max_height(max_height),
            width,
            height,
            padding,
            |limits| layout_content(&limits.loose()),
            |content, size| {
                content.align(
                    Alignment::from(horizontal_alignment),
                    Alignment::from(vertical_alignment),
                    size,
                )
            },
        )
    }

    /// Draws the background of a [`Pane`] given its [`Style`] and its `bounds`.
    pub fn draw_background<Renderer>(
        renderer: &mut Renderer,
        style: &Style,
        bounds: Rectangle,
    ) where
        Renderer: renderer::Renderer,
    {
        if style.background.is_some()
            || style.border.width > 0.0
                || style.shadow.color.a > 0.0
                {
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds,
                            border: style.border,
                            shadow: style.shadow,
                        },
                        style
                        .background
                        .unwrap_or(Background::Color(Color::TRANSPARENT)),
                    );
                }
    }

    /// The identifier of a [`Pane`].
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


    /// Produces a [`Task`] that queries the visible screen bounds of the
    /// [`Pane`] with the given [`Id`].
    pub fn visible_bounds(id: Id) -> Task<Option<Rectangle>> {
        struct VisibleBounds {
            target: widget::Id,
            depth: usize,
            scrollables: Vec<(Vector, Rectangle, usize)>,
            bounds: Option<Rectangle>,
        }

        impl Operation<Option<Rectangle>> for VisibleBounds {
            fn scrollable(
                &mut self,
                _state: &mut dyn widget::operation::Scrollable,
                _id: Option<&widget::Id>,
                bounds: Rectangle,
                _content_bounds: Rectangle,
                translation: Vector,
            ) {
                match self.scrollables.last() {
                    Some((last_translation, last_viewport, _depth)) => {
                        let viewport = last_viewport
                            .intersection(&(bounds - *last_translation))
                            .unwrap_or(Rectangle::new(Point::ORIGIN, Size::ZERO));

                        self.scrollables.push((
                                translation + *last_translation,
                                viewport,
                                self.depth,
                        ));
                    }
                    None => {
                        self.scrollables.push((translation, bounds, self.depth));
                    }
                }
            }

            fn container(
                &mut self,
                id: Option<&widget::Id>,
                bounds: Rectangle,
                operate_on_children: &mut dyn FnMut(
                    &mut dyn Operation<Option<Rectangle>>,
                ),
            ) {
                if self.bounds.is_some() {
                    return;
                }

                if id == Some(&self.target) {
                    match self.scrollables.last() {
                        Some((translation, viewport, _)) => {
                            self.bounds =
                                viewport.intersection(&(bounds - *translation));
                        }
                        None => {
                            self.bounds = Some(bounds);
                        }
                    }

                    return;
                }

                self.depth += 1;

                operate_on_children(self);

                self.depth -= 1;

                match self.scrollables.last() {
                    Some((_, _, depth)) if self.depth == *depth => {
                        let _ = self.scrollables.pop();
                    }
                    _ => {}
                }
            }

            fn finish(&self) -> widget::operation::Outcome<Option<Rectangle>> {
                widget::operation::Outcome::Some(self.bounds)
            }
        }

            //was previously task::widget in the source code which
            //is a type alias for "operate"
            iced::advanced::widget::operate(VisibleBounds {
            target: id.into(),
            depth: 0,
            scrollables: Vec::new(),
            bounds: None,
        })
    }

    /// The appearance of a container.
#[derive(Debug, Clone, Copy, Default)]
    pub struct Style {
        /// The text [`Color`] of the container.
        pub text_color: Option<Color>,
        /// The [`Background`] of the container.
        pub background: Option<Background>,
        /// The [`Border`] of the container.
        pub border: Border,
        /// The [`Shadow`] of the container.
        pub shadow: Shadow,
    }

    impl Style {
        /// Updates the text color of the [`Style`].
        pub fn color(self, color: impl Into<Color>) -> Self {
            Self {
                text_color: Some(color.into()),
                ..self
            }
        }

        /// Updates the border of the [`Style`].
        pub fn border(self, border: impl Into<Border>) -> Self {
            Self {
                border: border.into(),
                ..self
            }
        }

        /// Updates the background of the [`Style`].
        pub fn background(self, background: impl Into<Background>) -> Self {
            Self {
                background: Some(background.into()),
                ..self
            }
        }

        /// Updates the shadow of the [`Style`].
        pub fn shadow(self, shadow: impl Into<Shadow>) -> Self {
            Self {
                shadow: shadow.into(),
                ..self
            }
        }
    }

    impl From<Color> for Style {
        fn from(color: Color) -> Self {
            Self::default().background(color)
        }
    }

    impl From<Gradient> for Style {
        fn from(gradient: Gradient) -> Self {
            Self::default().background(gradient)
        }
    }

    impl From<gradient::Linear> for Style {
        fn from(gradient: gradient::Linear) -> Self {
            Self::default().background(gradient)
        }
    }

    /// The theme catalog of a [`Pane`].
    pub trait Catalog {
        /// The item class of the [`Catalog`].
        type Class<'a>;

        /// The default class produced by the [`Catalog`].
        fn default<'a>() -> Self::Class<'a>;

        /// The [`Style`] of a class with the given status.
        fn style(&self, class: &Self::Class<'_>) -> Style;
    }

    /// A styling function for a [`Pane`].
    pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

    impl<'a, Theme> From<Style> for StyleFn<'a, Theme> {
        fn from(style: Style) -> Self {
            Box::new(move |_theme| style)
        }
    }

    impl Catalog for Theme {
        type Class<'a> = StyleFn<'a, Self>;

        fn default<'a>() -> Self::Class<'a> {
            Box::new(transparent)
        }

        fn style(&self, class: &Self::Class<'_>) -> Style {
            class(self)
        }
    }

    /// A transparent [`Pane`].
    pub fn transparent<Theme>(_theme: &Theme) -> Style {
        Style::default()
    }

    /// A [`Pane`] with the given [`Background`].
    pub fn background(background: impl Into<Background>) -> Style {
        Style::default().background(background)
    }

    /// A rounded [`Pane`] with a background.
    pub fn rounded_box(theme: &Theme) -> Style {
        let palette = theme.extended_palette();

        Style {
            background: Some(palette.background.weak.color.into()),
            border: border::rounded(2),
            ..Style::default()
        }
    }

    /// A bordered [`Pane`] with a background.
    pub fn bordered_box(theme: &Theme) -> Style {
        let palette = theme.extended_palette();

        Style {
            background: Some(palette.background.weak.color.into()),
            border: Border {
                width: 1.0,
                radius: 0.0.into(),
                color: palette.background.strong.color,
            },
            ..Style::default()
        }
    }

    /// A [`Pane`] with a dark background and white text.
    pub fn dark(_theme: &Theme) -> Style {
        Style {
            background: Some(color!(0x111111).into()),
            text_color: Some(Color::WHITE),
            border: border::rounded(2),
            ..Style::default()
        }
    }
}

use iced::widget::{center, row, Container, column, container, Stack, Column, slider, text, Text};
use iced::{ Center, Element, Length};
use iced::Point;
use iced::overlay;
//Import stuff from other test binary
mod playground;
mod lib;
use playground::button;

pub fn main() -> iced::Result {
    iced::run("Custom Widget - Iced", App::update, App::view)
}

#[derive(Default)]
struct App {
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Test,
    Dropped,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Test => {
            },
            Message::Dropped => {
                println!("Dropped received");
            },
        }
    }

    fn view(&self) -> Element<Message> {

        let a: pane::Pane<Message> = pane::Pane::new("A")
            .height(250)
            .width(250)
            .clip(false)
            .on_drop(Message::Dropped)
            .style(pane::bordered_box);
        let b: pane::Pane<Message> = pane::Pane::new("B")
            .height(500)
            .width(500)
            .on_drop(Message::Dropped)
            .style(pane::bordered_box)
            .clip(false);
        let c = button::Button::new("test")
            .on_press(Message::Dropped)
            .clip(false);
        // iced::widget::hover(playlist, b)

        Stack::new()
            .push(
                row![a, b, c]
            )
            .into()
    }
}
