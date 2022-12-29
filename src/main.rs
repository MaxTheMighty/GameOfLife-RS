//TODO  Learn how canvas works in iced
// - Make a basic running GUI program
// - Successfully make a Grid that implements program
// - Use this GridProgram implementation to make a running application

use iced::*;

#[derive(Debug, Clone, Copy)]
enum Message {
    AddChar
}
struct BasicStruct{
    my_text: String
}

impl Sandbox for BasicStruct{
    type Message = Message;
    //Constructor (sorta)
    fn new() -> BasicStruct {
        BasicStruct {my_text: String::from("Togo is fat")}
    }

    //Returns the title of our program
    fn title(&self) -> String{
        String::from("1")
    }

    //this is where the logic of our program is called. We use update to modify the state of our program using Messages
    fn update(&mut self, message: Self::Message){
        println!("Update called");
        match(message){
            Message::AddChar =>{
                self.my_text.insert(9, 'a')
            }
        }
       
    }

    //returns widgets that will be displayed on our program
    fn view(&self) -> Element<Self::Message>{
       let mut buttons: Vec<iced::widget::Button<Message>> = vec![];
       let mut counter: i32 = 0;
       while(counter < 20){
            counter+=1;
            buttons.push(iced::widget::button("A").on_press(Message::AddChar))
       }

       let mut column =  iced::widget::column![iced::widget::button("A").on_press(Message::AddChar)];
       column = column.push(iced::widget::button("A").on_press(Message::AddChar)); 
       column.into()
       

       
    }
}

fn main() -> iced::Result{
    <BasicStruct as Sandbox>::run(iced::Settings::default())
}