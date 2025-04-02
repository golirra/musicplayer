//A button with added drag and drop functionality
#![allow(warnings)]
pub mod button {
    use std::fmt;
    use iced::advanced::layout::{self, Layout};
    use iced::border::{self, Border};
    use iced::advanced::renderer;
    use iced::advanced::widget::{self, operation::{Outcome, Scrollable}, Id, Operation, Widget};
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
        Message: Clone,
        Renderer: renderer::Renderer,
        Theme: Catalog,
    {
        id: Option<Id>,
        content: Element<'a, Message, Theme, Renderer>,
        on_press: Option<Message>,
        on_drop: Option<Box<dyn Fn(Point, Rectangle) -> Message + 'a>>,
        on_drag: Option<Box<dyn Fn(Point, Rectangle) -> Message + 'a>>,
        width: Length,
        height: Length,
        padding: Padding,
        clip: bool,
        class: Theme::Class<'a>,
        status: Option<Status>,
        dragging: bool
    }

    enum OnDrop<Message> {
        Direct(Message),
    }

    impl<Message: Clone> OnDrop<Message> {
        fn get(&self) -> Message {
            match self {
                OnDrop::Direct(message) => message.clone(),
            }
        }
    }

    enum CustomEvent<Message> {
        IcedEvent(Message),
    }

    impl<'a, Message, Theme, Renderer> Button<'a, Message, Theme, Renderer>
    where 
        Message: Clone,
        Renderer: renderer::Renderer, //is advanced::Renderer is core::Renderer in docs???
        Theme: Catalog,
    {
        pub fn new(
            content: impl Into<Element<'a, Message, Theme, Renderer>>,
        ) -> Self {
            let content = content.into();
            let size = content.as_widget().size_hint();

            Button { 
                id: None,
                content,
                on_press: None,
                on_drop: None,
                on_drag: None,
                width: size.width.fluid(),
                height: size.height.fluid(),
                padding: DEFAULT_PADDING,
                clip: false,
                class: Theme::default(),
                status: None,
                dragging: false,
            }
        }

        pub fn id(mut self) -> Self {
            self.id = Some(Id::new("TEST"));
            self
        }

        pub fn on_press(mut self, message: Message) -> Self {
            self.on_press = Some(message);
            self
        }

        pub fn on_drop<F>(mut self, message: F) -> Self 
        where
            F: Fn(Point, Rectangle) -> Message + 'a,
        {
            self.on_drop = Some(Box::new(message));
            self
        }
        
        pub fn on_drag<F>(mut self, message: F) -> Self
        where
            F: Fn(Point, Rectangle) -> Message + 'a,
        {
            self.on_drag = Some(Box::new(message));
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
        overlay_bounds: Rectangle,
    }

    pub fn button<'a, Message, Theme, Renderer>(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        ) -> Button<'a, Message, Theme, Renderer>
    where
        Message: Clone,
        Renderer: renderer::Renderer,
        Theme: Catalog,
    {
        Button::new(content)
    }
    pub fn print_bounds() -> impl Operation<()> {
        struct PrintBounds;
        
        impl Operation<()> for PrintBounds {

            fn container(
                &mut self,
                id: Option<&Id>,
                bounds: Rectangle,
                operate_on_children: &mut dyn FnMut(&mut dyn Operation<()>)
            ) {
                println!("Button {:?} has bounds {:?}", id, bounds);
                //NOTE: calling operate_on_children attempts to keep traversing the widget tree
                operate_on_children(self);
            }

            fn finish(&self) -> Outcome<()> {
                Outcome::None
            }

            fn scrollable(
                &mut self,
                _state: &mut dyn Scrollable,
                _id: Option<&Id>,
                bounds: Rectangle,
                _content_bounds: Rectangle,
                translation: Vector,
            ) {
                println!("Scrollable widget has bounds {:?} and translation {:?}", bounds, translation);
            }
        }

        PrintBounds
    }


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
                let content_node = self
                    .content
                    .as_widget()
                    .layout(&mut tree.children[0], renderer, limits);
                content_node

            }




            fn operate(
                &self,
                tree: &mut Tree,
                layout: Layout<'_>,
                renderer: &Renderer,
                operation: &mut dyn Operation,
            ) {
                println!("o");
                //NOTE:read as_ref() documentation if confused

                operation.container(self.id.as_ref(), layout.bounds(), &mut |operation| {
                    self.content
                        .as_widget()
                        .operate(&mut tree.children[0], layout, renderer, operation);
                });
            }

            fn on_event(
                &mut self,
                tree: &mut Tree,
                event: Event,
                layout: Layout<'_>,
                cursor: Cursor,
                renderer: &Renderer,
                _clipboard: &mut dyn Clipboard,
                shell: &mut Shell<'_, Message>,
                _viewport: &Rectangle,
            ) -> IcedStatus {
                let mut operation = print_bounds();

                // if let Some(on_drop) = self.on_drop.as_deref() {
                    let state = tree.state.downcast_mut::<State>();

                    match event {
                        Event::Mouse(mouse_event) => match mouse_event {
                            iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left) => {
                                if cursor.is_over(layout.bounds()) {
                                    state.pos = layout.bounds().position();
                                    state.overlay_bounds.width = layout.bounds().width;
                                    state.overlay_bounds.height = layout.bounds().height;
                                    self.dragging = true;
                                    self.operate(tree, layout, renderer, &mut operation);
                                    return IcedStatus::Captured;
                                    if let Some(on_press) = self.on_press.clone() {
                                        shell.publish(on_press);
                                    }
                                    return IcedStatus::Captured;
                                }
                                return IcedStatus::Ignored;
                            },
                            iced::mouse::Event::ButtonPressed(iced::mouse::Button::Right) => {
                                return IcedStatus::Ignored;
                            },
                            iced::mouse::Event::CursorMoved { position } => {
                                    state.overlay_bounds.x = position.x;
                                    state.overlay_bounds.y = position.y;
                            },
                            iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left) => {
                                self.dragging = false;//overlay display relies on dragging=true

                            },
                            _ => {
                                return IcedStatus::Ignored;
                            },
                        }//end mouse event match
                        _ => { //other arms (keyboard, window, touch)
                            return IcedStatus::Ignored;
                        },
                    }
                // }
                return IcedStatus::Ignored;
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
                    layout,
                    cursor,
                    &viewport,
                );
            }

            //Just changes how the cursor looks
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
                let state: &mut State = tree.state.downcast_mut::<State>();
                let mut children = tree.children.iter_mut();
                if self.dragging {
                    return Some(overlay::Element::new(Box::new(Overlay {
                        content: &self.content,
                        tree: children.next().unwrap(),
                        overlay_bounds: state.overlay_bounds,
                    })));
                } else {
                    None
                }

                // self.content.as_widget_mut().overlay(
                //     &mut tree.children[0],
                //     layout.children().next().unwrap(),
                //     renderer,
                //     translation,
                // )
            }
        }

    //Implementing the From trait allows Button to be treated as a generic Element
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

    struct Overlay<'a, 'b, Message, Theme, Renderer>
    where
        Renderer: renderer::Renderer,
    {
        content: &'b Element<'a, Message, Theme, Renderer>,
        tree: &'b mut Tree,
        overlay_bounds: Rectangle,
    }

    impl<'a, 'b, Message, Theme, Renderer> iced::advanced::Overlay<Message, Theme, Renderer>
        for Overlay<'a, 'b, Message, Theme, Renderer>
    where
        Renderer: renderer::Renderer,
    {
        //Responsible for where the widget overlay is placed
        fn layout(&mut self, renderer: &Renderer, _bounds: Size) -> layout::Node {
            // dbg!(self.overlay_bounds.position());
            Widget::<Message, Theme, Renderer>::layout(
                self.content.as_widget(),
                self.tree,
                renderer,
                &layout::Limits::new(Size::ZERO, self.overlay_bounds.size()),
            )
                .move_to(self.overlay_bounds.position())

        }

        //How the button's overlay looks
        fn draw(
            &self,
            renderer: &mut Renderer,
            theme: &Theme,
            inherited_style: &renderer::Style,
            layout: Layout<'_>,
            cursor_position: mouse::Cursor,
        ) {
            Widget::<Message, Theme, Renderer>::draw(
                self.content.as_widget(),
                self.tree,
                renderer,
                theme,
                inherited_style,
                layout,
                cursor_position,
                &Rectangle::with_size(Size::INFINITY),
            );
        }

        fn is_over(&self, _layout: Layout<'_>, _renderer: &Renderer, _cursor_position: Point) -> bool {
            false
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
                background: Some(Background::Color(Color::BLACK)),
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
    //NOTE: place to actually set button theme. Note the default fn for the button 
    //above, Box::new(primary)
    pub fn primary(theme: &Theme, status: Status) -> Style {
        let palette = theme.extended_palette();
        let base = styled(palette.primary.strong);

        match status {
            Status::Active | Status::Pressed => base,
            Status::Hovered => Style {
                background: Some(Background::Color(Color::BLACK)),
                // background: Some(Background::Color(palette.primary.base.color)),
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



use iced::widget::{center, container, Stack, column, row, Column, slider, text, Text};
use iced::widget::container::Id as CId;
use iced::advanced::widget::Id;
use iced::{ Center, Element, Length};
use iced::Point;
use iced::advanced::overlay;
use button::button;

pub fn main() -> iced::Result {
    iced::run("Custom Widget - Iced", App::update, App::view)
}

#[derive(Default)]
struct App {
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadiusChanged,
    Test,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::RadiusChanged => {
                println!("Main call, button was pressed!");
            },
            Message::Test => {
                println!("Test on drop");
            },
        }
    }
    fn view(&self) -> Element<Message> {
        let header = container(text("Test Zones").size(20))
            .center(Length::Fill)
            .padding(10.0)
            .width(Length::Fill)
            .height(Length::Fixed(50.0));
        let c = button(container("test").id(CId::new("C")))
            .height(50)
            .id()
            .width(100)
            .on_press(Message::RadiusChanged)
            // .on_drop(Message::Test)
            .clip(false);
        let a = container("A")
            .height(500)
            .width(400)
            .clip(false)
            .style(container::bordered_box);
        let b = container("B")
            .height(500)
            .width(400)
            .clip(false)
            .style(container::bordered_box);
        column![
            header,
            row![
                c,
                a,
                b,
            ].padding(10.0),
        ].into()

        // Column::new()
        //     .push(b)
        //     .push(x)
        //     .spacing(50)
        //     .into()
    }
}
// use iced::{
//     advanced::widget::{operate, Id},
//     advanced::{graphics::futures::MaybeSend },
//     task::Task, Rectangle,
// };
//
// pub fn zones_on_point<T, MF>(
//     msg: MF,
//     point: Point,
//     options: Option<Vec<Id>>,
//     depth: Option<usize>,
// ) -> Task<T>
// where
//     T: Send + 'static,
//     MF: Fn(Vec<(Id, Rectangle)>) -> T + MaybeSend + Sync + Clone + 'static,
// {
//     operate(drop::find_zones(
//         move |bounds| bounds.contains(point),
//         options,
//         depth,
//     ))
//     .map(move |id| msg(id))
// }
//
// pub fn find_zones<Message, MF, F>(
//     msg: MF,
//     filter: F,
//     options: Option<Vec<Id>>,
//     depth: Option<usize>,
// ) -> Task<Message>
// where
//     Message: Send + 'static,
//     MF: Fn(Vec<(Id, Rectangle)>) -> Message + MaybeSend + Sync + Clone + 'static,
//     F: Fn(&Rectangle) -> bool + Send + 'static,
// {
//     operate(drop::find_zones(filter, options, depth)).map(move |id| msg(id))
// }
