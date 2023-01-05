//TODO  Learn how canvas works in iced
// - Make a basic running GUI program
// - Successfully make a Grid that implements program
// - Use this GridProgram implementation to make a running application

use iced::executor;
use iced::widget;
use iced::widget::button;
use iced::widget::canvas::*;
use iced::widget::container;
use iced::widget::Container;
use iced::widget::Row;
use iced::mouse;
use iced::window::Settings;
use iced::Application;
use iced::Color;
use iced::Command;
use iced::Length;
use iced::Point;
use iced::Rectangle;
use iced::Sandbox;
use iced::Size;
use iced::Theme;
use grid::*;

#[derive(Debug, Clone, Copy)]
enum Message {
    Enlarge,
}
struct Square {
    colors: (f32,f32,f32),
    
}

impl iced::widget::canvas::Program<Message> for Square {
    type State = ();

    fn draw(
        &self,
        state: &Self::State,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());

        // let circle = Path::circle(frame.center(), self.radius);
        let mut counter: f32 = 0.0;
        while counter < 15.0{
            
            let square: Path = Path::rectangle(Point::new(15.0*counter, 15.0*counter),Size::from([15.0,15.0]));
            frame.fill(&square, Color::from_rgb(self.colors.0, self.colors.1, self.colors.2));
            counter+=1.0;
        }
        

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
            &self,
            _state: &Self::State,
            _bounds: Rectangle,
            _cursor: Cursor,
        ) -> mouse::Interaction {
           
           mouse::Interaction::default()
    }

    fn update(
            &self,
            _state: &mut Self::State,
            _event: Event,
            _bounds: Rectangle,
            _cursor: Cursor,
        ) -> (event::Status, Option<Message>) {
        
        (event::Status::Ignored,None)
    }


}

struct MyApplication {
    app_colors: (f32,f32,f32)
}

impl Application for MyApplication {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn title(&self) -> String {
        String::from("Circle")
    }

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self {app_colors: (0.0,0.0,0.0) }, Command::none())
    }

    fn view(&self) -> iced::Element<Self::Message> {
        let r = self.app_colors.0;
        let g = self.app_colors.1;
        let b = self.app_colors.2;
        let canvas = Canvas::new(Square {colors: (r,g,b)})
        .width(Length::Fill)
        .height(Length::Fill);
        canvas.into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Enlarge => {
                self.app_colors.0+=0.01;
            }
        }
        Command::none()
    }
}

fn main() {
    MyApplication::run(iced::Settings::default()).expect("Error running application");
}
