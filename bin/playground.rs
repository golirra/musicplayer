mod circle {
    use iced::advanced::layout::{self, Layout};
    use iced::advanced::renderer;
    use iced::advanced::widget::{self, Widget, Tree};
    use iced::border;
    use iced::mouse;
    use iced::mouse::Cursor;
    use iced::Event;
    use iced::Point;
    use iced::event::Status;
    use iced::advanced::{Shell, Clipboard};
    use iced::{Color, Element, Length, Rectangle, Size};

    //This struct represents the state of the widget. Must be given a pos on construction
    pub struct Circle<'a, Message> {
        on_press: Option<OnPress<'a, Message>>,
        pos: Point, //initial position of widget
        dragging: bool
    }

    enum OnPress<'a, Message> {
        Direct(Message),
        Closure(Box<dyn Fn() -> Message + 'a>),
    }

    impl<Message: Clone> OnPress<'_, Message> {
        fn get(&self) -> Message {
            match self {
                OnPress::Direct(message) => message.clone(),
                OnPress::Closure(f) => f(),
            }
        }
    }

    impl<'a, Message> Circle<'a, Message> {
        pub fn new(pos: Point) -> Self {
            Self { 
                on_press: None,
                pos, //can use pos instead of pos: pos so long as theres a var in scope with correct name
                dragging: false,
            }
        }

        pub fn on_press(mut self, on_press: Message) -> Self {
            self.on_press = Some(OnPress::Direct(on_press));
            self
        }
    }

    pub fn circle<'a, Message>(pos: Point) -> Circle<'a, Message> {
        Circle::new(pos)
    }

    impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> 
        for Circle<'a, Message>
    where
        Renderer: renderer::Renderer,
    {
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
        _state: &mut Tree,
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
                    if layout.bounds().contains(cursor.position().unwrap()) {
                        println!("inside the black thing");
                        self.dragging = true;
                        return Status::Captured;
                    }
                    dbg!("{}",cursor.position().unwrap());

                    println!("left button");

                    Status::Captured
                },
                iced::mouse::Event::ButtonPressed(iced::mouse::Button::Right) => {
                    dbg!(layout.bounds());
                    Status::Ignored
                },
                iced::mouse::Event::CursorMoved { position } => {
                    if self.dragging {
                        self.pos = Point::new(
                            cursor.position().unwrap().x - 50.0,
                            cursor.position().unwrap().y - 50.0,
                        );
                        shell.invalidate_layout();
                        println!("we dragging");
                    }
                    Status::Captured
                },
                iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left) => {
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

    impl<'a, Message, Theme, Renderer> From<Circle<'a, Message>>
        for Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'a,
        Renderer: renderer::Renderer,
    {
        fn from(circle: Circle<'a, Message>) -> Self {
            Self::new(circle)
        }
    }
}

use circle::circle;
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
                println!("circle was pressed!");
            }
        }
    }

    fn view(&self) -> Element<Message> {
        circle(Point::new(500.0, 500.0)).on_press(Message::RadiusChanged)
        
            .into()
    }
}


