//A button with added drag and drop functionality
mod button {
    use iced::advanced::layout::{self, Layout};
    use iced::border::{self, Border};
    use iced::advanced::renderer;
    use iced::advanced::widget::{self, Operation, Widget};
    use iced::advanced::widget::tree::{self, Tree};
    use iced::theme::palette;
    use iced::overlay;
    use iced::mouse;
    use iced::mouse::Cursor;
    use iced::Event;
    use iced::Point;
    use iced::event::Status as IcedStatus;
    use iced::advanced::{Shell, Clipboard};
    use iced::{Background, Color, Element, Length, Padding, Rectangle,
        Shadow, Size, Theme, Vector};
    

    //This struct represents the state of the widget. Must be given a pos on construction
    pub struct Button<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
    where 
        Renderer: renderer::Renderer,
        Theme: Catalog,
    {
        content: Element<'a, Message, Theme, Renderer>,
        on_press: Option<OnPress<'a, Message>>,
        width: Length,
        height: Length,
        padding: Padding,
        clip: bool,
        class: Theme::Class<'a>,
        status: Option<Status>,
        dragging: bool
    }

    enum OnPress<'a, Message> {
        Direct(Message),
        Closure(Box<dyn Fn() -> Message + 'a>),
    }

    impl<'a, Message: Clone> OnPress<'_, Message> { //maybe remove 'a
        fn get(&self) -> Message {
            match self {
                OnPress::Direct(message) => message.clone(),
                OnPress::Closure(f) => f(),
            }
        }
    }

    impl<'a, Message, Theme, Renderer> Button<'a, Message, Theme, Renderer>
    where 
        Renderer: renderer::Renderer, //is advanced::Renderer is core::Renderer in docs???
        Theme: Catalog,
    {
        pub fn new(
            content: impl Into<Element<'a, Message, Theme, Renderer>>,
            pos: Point
        ) -> Self {
            let content = content.into();
            let size = content.as_widget().size_hint();

            Button { 
                content,
                on_press: None,
                width: size.width.fluid(),
                height: size.height.fluid(),
                padding: DEFAULT_PADDING,
                clip: false,
                class: Theme::default(),
                status: None,
                dragging: false,
            }
        }

        //Sets the message that will be produced when the Button is pressed
        pub fn on_press(mut self, on_press: Message) -> Self {
            self.on_press = Some(OnPress::Direct(on_press));
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
        /// Sets whether the contents of the [`Button`] should be clipped on
        /// overflow.
        pub fn clip(mut self, clip: bool) -> Self {
            self.clip = clip;
            self
        }

        /// Sets the style of the [`Button`].
        #[must_use]
        pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
            where
            Theme::Class<'a>: From<StyleFn<'a, Theme>>,
            {
                self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
                self
            }

        /// Sets the style class of the [`Button`].
        #[must_use]
        pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
            self.class = class.into();
            self
        }

    }
    //Store stuff in a State struct rather than the Button struct, as we have to update the state
    //based
    #[derive(Debug, Clone, Copy, PartialEq, Default)] 
    struct State {
        is_pressed: bool,
        pos: Point,
    }

    /*
    pub fn button<'a, Message>(pos: Point) -> Button<'a, Message> {
        Button::new(pos)
    }
    */

    impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> 
        for Button<'a, Message, Theme, Renderer>
    where
        Renderer: renderer::Renderer + 'a,//renderer is part of iced_core
        Message: Clone + 'a,
        Theme: Catalog,
        {

            fn tag(&self) -> tree::Tag {
                tree::Tag::of::<State>()
            }

            fn state(&self) -> tree::State {
                tree::State::new(State::default())
            }

            fn children(&self) -> Vec<Tree> {
                vec![Tree::new(&self.content)]
            }

            fn diff(&self, tree: &mut Tree) {
                tree.diff_children(std::slice::from_ref(&self.content));
            }
            fn size(&self) -> Size<Length> {
                Size {
                    width: Length::Shrink,
                    height: Length::Shrink,
                }
            }

            fn layout(
                &self,
                tree: &mut widget::Tree,
                renderer: &Renderer,
                limits: &layout::Limits,
            ) -> layout::Node {
                let state = tree.state.downcast_ref::<State>();
                let x =
                layout::padded(
                    limits,
                    self.width,
                    self.height,
                    self.padding,
                    |limits| {
                        self.content
                            .as_widget()
                            .layout(&mut tree.children[0], renderer, limits,)
                            
                    },

                );
                x.move_to(state.pos)
                // x.move_to(self.pos)
                // let size = Size::new(100.0, 100.0);
                // let node = layout::Node::new(size);
                // node.move_to(self.pos)
            }
            fn operate(
                &self,
                tree: &mut Tree,
                layout: Layout<'_>,
                renderer: &Renderer,
                operation: &mut dyn Operation,
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
                cursor: Cursor,
                _renderer: &Renderer,
                _clipboard: &mut dyn Clipboard,
                shell: &mut Shell<'_, Message>,
                _viewport: &Rectangle,
            ) -> IcedStatus {
                match event {
                    Event::Mouse(mouse_event) => match mouse_event {
                        iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left) => {
                            if self.on_press.is_some() {
                                let bounds = layout.bounds();
                                if cursor.is_over(bounds) {
                                    let state = tree.state.downcast_mut::<State>();
                                    state.is_pressed = true;
                                    self.dragging = true;
                                    return IcedStatus::Captured;
                                }
                            } 
                            IcedStatus::Ignored 
                                // dbg!("{}",cursor.position().unwrap());
                        },
                        iced::mouse::Event::ButtonPressed(iced::mouse::Button::Right) => {
                            dbg!(layout.bounds());
                            IcedStatus::Ignored
                        },
                        iced::mouse::Event::CursorMoved { position } => {
                            if self.dragging {
                                let state = tree.state.downcast_mut::<State>();
                                state.pos = position;
                                // self.pos = position;
                                shell.invalidate_layout(); //Redraws layout every time an event is
                                                           //triggered
                                // println!("we dragging");
                            }
                            IcedStatus::Captured
                        },
                        iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left) => {
                            if let Some(on_press) = self.on_press.as_ref().map(OnPress::get) {
                                let state = tree.state.downcast_mut::<State>();
                                if state.is_pressed {
                                    state.is_pressed = false;
                                    let bounds = layout.bounds();
                                    if cursor.is_over(bounds) {
                                        shell.publish(on_press);
                                    }
                                    return IcedStatus::Captured;
                                }
                            }
                            //self.pos = cursor.position().expect("Idk");
                            // shell.invalidate_layout();
                            self.dragging = false;
                            IcedStatus::Captured
                        }
                        _ => IcedStatus::Ignored,
                    },

                    _ => IcedStatus::Ignored,
                }
            }
            /*
            fn draw(
                &self,
                _state: &widget::Tree,
                renderer: &mut Renderer,
                _theme: &Theme,
                _style: &renderer::Style,
                layout: Layout<'_>,
                _cursor: mouse::Cursor,
                _viewport: &Rectangle,
                ) {
                renderer.fill_quad(
                renderer::Quad {
                bounds: layout.bounds(),
                border: border::rounded(0.0),
                ..renderer::Quad::default()
                },
                Color::BLACK,
                );
                }
                */
            fn draw(
                &self,
                tree: &Tree,
                renderer: &mut Renderer,
                theme: &Theme,
                _style: &renderer::Style,
                layout: Layout<'_>,
                cursor: mouse::Cursor,
                viewport: &Rectangle,
            ) {
                let bounds = layout.bounds();
                let content_layout = layout.children().next().unwrap();
                let is_mouse_over = cursor.is_over(bounds);

                let status = if self.on_press.is_none() {
                    Status::Disabled
                } else if is_mouse_over {
                    let state = tree.state.downcast_ref::<State>();

                    if state.is_pressed {
                        Status::Pressed
                    } else {
                        Status::Hovered
                    }
                } else {
                    Status::Active
                };

                let style = theme.style(&self.class, status);

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

                let viewport = if self.clip {
                    bounds.intersection(viewport).unwrap_or(*viewport)
                } else {
                    *viewport
                };

                self.content.as_widget().draw(
                    &tree.children[0],
                    renderer,
                    theme,
                    &renderer::Style {
                        text_color: style.text_color,
                    },
                    content_layout,
                    cursor,
                    &viewport,
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
                let is_mouse_over = cursor.is_over(layout.bounds());

                if is_mouse_over && self.on_press.is_some() {
                    mouse::Interaction::Pointer
                } else {
                    mouse::Interaction::default()
                }
            }

            
            fn overlay<'b>(
                &'b mut self,
                tree: &'b mut Tree,
                layout: Layout<'_>,
                renderer: &Renderer,
                translation: Vector,
            ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
                self.content.as_widget_mut().overlay(
                    &mut tree.children[0],
                    layout.children().next().unwrap(),
                    renderer,
                    translation,
                )
            }


        }

    impl<'a, Message, Theme, Renderer> From<Button<'a, Message, Theme, Renderer>>
        for Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'a,
        Theme: Catalog + 'a,
        Renderer: renderer::Renderer + 'a,
    {
        fn from(button: Button<'a, Message, Theme, Renderer>) -> Self {
            Self::new(button)
        }
    }

    pub(crate) const DEFAULT_PADDING: Padding = Padding {
        top: 5.0,
        bottom: 5.0,
        right: 10.0,
        left: 10.0,
    };

    /// The possible status of a [`Button`].
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Status {
        /// The [`Button`] can be pressed.
        Active,
        /// The [`Button`] can be pressed and it is being hovered.
        Hovered,
        /// The [`Button`] is being pressed.
        Pressed,
        /// The [`Button`] cannot be pressed.
        Disabled,
    }
    /// The style of a button.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Style {
        /// The [`Background`] of the button.
        pub background: Option<Background>,
        /// The text [`Color`] of the button.
        pub text_color: Color,
        /// The [`Border`] of the buton.
        pub border: Border,
        /// The [`Shadow`] of the butoon.
        pub shadow: Shadow,
    }

    impl Style {
        /// Updates the [`Style`] with the given [`Background`].
        pub fn with_background(self, background: impl Into<Background>) -> Self {
            Self {
                background: Some(background.into()),
                ..self
            }
        }
    }

    impl Default for Style {
        fn default() -> Self {
            Self {
                background: None,
                text_color: Color::BLACK,
                border: Border::default(),
                shadow: Shadow::default(),
            }
        }
    }

    /// The theme catalog of a [`Button`].
    pub trait Catalog {
        /// The item class of the [`Catalog`].
        type Class<'a>;

        /// The default class produced by the [`Catalog`].
        fn default<'a>() -> Self::Class<'a>;

        /// The [`Style`] of a class with the given status.
        fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
    }

    /// A styling function for a [`Button`].
    pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

    impl Catalog for Theme {
        type Class<'a> = StyleFn<'a, Self>;

        fn default<'a>() -> Self::Class<'a> {
            Box::new(primary)
        }

        fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
            class(self, status)
        }
    }

    /// A primary button; denoting a main action.
    pub fn primary(theme: &Theme, status: Status) -> Style {
        let palette = theme.extended_palette();
        let base = styled(palette.primary.strong);

        match status {
            Status::Active | Status::Pressed => base,
            Status::Hovered => Style {
                background: Some(Background::Color(palette.primary.base.color)),
                ..base
            },
            Status::Disabled => disabled(base),
        }
    }

    /// A secondary button; denoting a complementary action.
    pub fn secondary(theme: &Theme, status: Status) -> Style {
        let palette = theme.extended_palette();
        let base = styled(palette.secondary.base);

        match status {
            Status::Active | Status::Pressed => base,
            Status::Hovered => Style {
                background: Some(Background::Color(palette.secondary.strong.color)),
                ..base
            },
            Status::Disabled => disabled(base),
        }
    }

    /// A success button; denoting a good outcome.
    pub fn success(theme: &Theme, status: Status) -> Style {
        let palette = theme.extended_palette();
        let base = styled(palette.success.base);

        match status {
            Status::Active | Status::Pressed => base,
            Status::Hovered => Style {
                background: Some(Background::Color(palette.success.strong.color)),
                ..base
            },
            Status::Disabled => disabled(base),
        }
    }

    /// A danger button; denoting a destructive action.
    pub fn danger(theme: &Theme, status: Status) -> Style {
        let palette = theme.extended_palette();
        let base = styled(palette.danger.base);

        match status {
            Status::Active | Status::Pressed => base,
            Status::Hovered => Style {
                background: Some(Background::Color(palette.danger.strong.color)),
                ..base
            },
            Status::Disabled => disabled(base),
        }
    }

    /// A text button; useful for links.
    pub fn text(theme: &Theme, status: Status) -> Style {
        let palette = theme.extended_palette();

        let base = Style {
            text_color: palette.background.base.text,
            ..Style::default()
        };

        match status {
            Status::Active | Status::Pressed => base,
            Status::Hovered => Style {
                text_color: palette.background.base.text.scale_alpha(0.8),
                ..base
            },
            Status::Disabled => disabled(base),
        }
    }

    fn styled(pair: palette::Pair) -> Style {
        Style {
            background: Some(Background::Color(pair.color)),
            text_color: pair.text,
            border: border::rounded(2),
            ..Style::default()
        }
    }

    fn disabled(style: Style) -> Style {
        Style {
            background: style
                .background
                .map(|background| background.scale_alpha(0.5)),
            text_color: style.text_color.scale_alpha(0.5),
            ..style
        }
    }
}

use iced::widget::{center, Container, column, slider, text};
use iced::{ Center, Element};
use iced::Point;

pub fn main() -> iced::Result {
    iced::run("Custom Widget - Iced", App::update, App::view)
}

#[derive(Default)]
struct App {
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadiusChanged,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::RadiusChanged => {
                println!("Main call, button was pressed!");
            }
        }
    }
    fn view(&self) -> Element<Message> {
        let b = button::Button::new("test", Point::new(300.0, 300.0));
        b.on_press(Message::RadiusChanged)
            .into()
    }
}
