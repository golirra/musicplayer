mod button {
    use iced::advanced::layout::{self, Layout};
    use iced::advanced::renderer;
    use iced::advanced::widget::{self, Widget};
    use iced::advanced::widget::tree::{self, Tree};
    use iced::border;
    use iced::mouse;
    use iced::mouse::Cursor;
    use iced::Event;
    use iced::Point;
    use iced::event::Status;
    use iced::advanced::{Shell, Clipboard};
    use iced::{Color, Element, Length, Rectangle, Size};

    //This struct represents the state of the widget. Must be given a pos on construction
    pub struct Button<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
    where 
        Renderer: renderer::Renderer,
    {
        content: Element<'a, Message, Theme, Renderer>,
        on_press: Option<OnPress<'a, Message>>,
        pos: Point, //initial position of widget
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
                pos, //shorthand field init
                dragging: false,
            }
        }

        //Sets the message that will be produced when the Button is pressed
        pub fn on_press(mut self, on_press: Message) -> Self {
            self.on_press = Some(OnPress::Direct(on_press));
            self
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    struct State {
        is_pressed: bool,
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
                _tree: &mut widget::Tree,
                _renderer: &Renderer,
                _limits: &layout::Limits,
            ) -> layout::Node {
                let size = Size::new(100.0, 100.0);
                let node = layout::Node::new(size);

                node.move_to(self.pos)
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
            ) -> Status {
                match event {

                    Event::Mouse(mouse_event) => match mouse_event {
                        iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left) => {
                            if self.on_press.is_some() {
                                let bounds = layout.bounds();
                                if cursor.is_over(bounds) {
                                    let state = tree.state.downcast_mut::<State>();
                                    state.is_pressed = true;
                                    self.dragging = true;
                                    return Status::Captured;
                                }
                            } 
                            Status::Ignored 
                                // dbg!("{}",cursor.position().unwrap());
                        },
                        iced::mouse::Event::ButtonPressed(iced::mouse::Button::Right) => {
                            // dbg!(layout.bounds());
                            Status::Ignored
                        },
                        iced::mouse::Event::CursorMoved { position } => {
                            if self.dragging {
                                self.pos = Point::new(
                                    cursor.position().unwrap().x - 50.0,
                                    cursor.position().unwrap().y - 50.0,
                                );
                                shell.invalidate_layout();
                                // println!("we dragging");
                            }
                            Status::Captured
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
                                    return Status::Captured;
                                }
                            }

                            self.dragging = false;
                            Status::Captured
                        }
                        _ => Status::Ignored,
                    },

                    _ => Status::Ignored,
                }
            }
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
        }

    impl<'a, Message, Theme, Renderer> From<Button<'a, Message, Theme, Renderer>>
        for Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'a,
        Theme: 'a,
        Renderer: renderer::Renderer + 'a,
        {
            fn from(button: Button<'a, Message, Theme, Renderer>) -> Self {
                Self::new(button)
            }
        }
}

use iced::widget::{center, Container, column, slider, text};
use iced::{ Center, Element};
use iced::Point;

pub fn main() -> iced::Result {
    iced::run("Custom Widget - Iced", Example::update, Example::view)
}

#[derive(Default)]
struct Example {
    pos: Point,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadiusChanged,
}

impl Example {


    fn update(&mut self, message: Message) {
        match message {
            Message::RadiusChanged => {
                println!("button was pressed!");
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let b = button::Button::new("test", Point::new(0.0, 0.0));
        b.on_press(Message::RadiusChanged)
            .into()
    }
}


