//TODO  Learn how canvas works in iced
// - Make a basic running GUI program
// - Successfully make a Grid that implements program
// - Use this GridProgram implementation to make a running application


use iced::Application;
use iced::executor;
use iced::widget::button;
use iced::widget::Container;
use iced::widget::container;
use iced::widget::canvas::*;
use iced::Theme;
use iced::Rectangle;
use iced::Sandbox;
use iced::Color;
use iced::Command;
use iced::widget::Row;
use iced::window::Settings;


#[derive(Debug, Clone, Copy)]
enum Message {
    Enlarge
}
struct Circle{
    radius: f32
}

impl iced::widget::canvas::Program<Message> for Circle{
    type State = ();

    fn draw(&self, state: &Self::State, theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry>{

        let mut frame = Frame::new(bounds.size());

        let circle = Path::circle(frame.center(), self.radius);

        frame.fill(&circle,Color::BLACK);

        vec![frame.into_geometry()]
    }

    
    
}

struct MyApplication{
   radius_value: f32
}


impl Application for MyApplication{
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn title(&self) -> String {
        String::from("Circle")
    }

    fn new(flags: Self::Flags) -> (Self,Command<Self::Message>) {
        (
            Self {radius_value: 50.0},
            Command::none()
        )
    }

    fn view(&self) -> iced::Element<Self::Message> {
       let canvas =  Canvas::new(Circle {radius: self.radius_value});
       let button: iced::widget::Button<Message> = button("+").on_press(Message::Enlarge);
       let row = Row::new().push(button).push(canvas);
       container(row).into()
        
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match(message) {
            Message::Enlarge => {
                self.radius_value+=1.0;
            } 
        }
     Command::none()

    }
}

fn main(){
    MyApplication::run(iced::Settings::default());
}
