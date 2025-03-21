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


    pub struct Circle {
        pos: Point,
        dragging: bool
    }

    impl Circle {
        pub fn new(pos: Point) -> Self {
            Self { 
                pos: Point::new(0.0, 0.0),
                dragging: false,
            }
        }
    }

    pub fn circle(pos: Point) -> Circle {
        Circle::new(pos)
    }

    impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Circle
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

    impl<Message, Theme, Renderer> From<Circle>
        for Element<'_, Message, Theme, Renderer>
    where
        Renderer: renderer::Renderer,
    {
        fn from(circle: Circle) -> Self {
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

struct Example {
    pos: Point,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadiusChanged(f32),
}

impl Example {
    fn new() -> Self {
        Example {pos: Point::new(20.0,20.0) }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::RadiusChanged(radius) => {
            }
        }
    }

    fn view(&self) -> Element<Message> {
        circle(
            self.pos
        )
            .into()
    }
}

impl Default for Example {
    fn default() -> Self {
        Self::new()
    }
}
