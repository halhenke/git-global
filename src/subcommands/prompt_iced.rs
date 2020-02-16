use crate::models::{config::GitGlobalConfig, result::GitGlobalResult};
use anyhow::Result;
// use iced::{button, Application, Button, Column, Command, Settings, Text};
use iced::{
    button, scrollable, settings::Window, Align, Application, Background,
    Button, Color, Column, Command, Container, Element, Length, Scrollable,
    Settings, Text,
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

#[derive(Default)]
struct Counter {
    // The counter value
    value: i32,

    // The local state of the two buttons
    increment_button: button::State,
    decrement_button: button::State,
    list_of_things: scrollable::State,
}

impl Application for Counter {
    type Message = Message;

    fn new() -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("A simple counter")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        // Container::new()
        // .push()
        let b_bg: Background = Background::Color(Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        });
        let col: Column<'_, Message> = Column::new()
            .padding(100)
            .spacing(40)
            .width(Length::Fill)
            // .max_width(500)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    // .padding(100)
                    // .spacing(20)
                    .background(b_bg)
                    .on_press(Message::IncrementPressed),
            )
            .push(
                Container::new(Text::new(self.value.to_string()).size(50))
                    .center_x(),
            )
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            )
            .push(
                Scrollable::new(&mut self.list_of_things)
                    .push(Text::new("A first thing"))
                    .push(Text::new("A second thing")),
            );
        // .into();
        let c = Container::new::<Element<Message>>(col.into())
            // .max_width(1500)
            // .max_height(100)
            .width(Length::Units(400))
            .center_x()
            .center_y();
        c.into()
    }
}


pub fn go() -> Result<GitGlobalResult> {
    let mut gc = GitGlobalConfig::new();
    let w = Window {
        size: (400, 800),
        resizable: true,
    };
    let s = Settings { window: w };
    Counter::run(s);
    // Counter::run(Settings::default());
    let repos = gc.get_repos();
    let mut result = GitGlobalResult::new(&repos);
    return Ok(result);
}
