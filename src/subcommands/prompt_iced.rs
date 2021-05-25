use crate::models::{config::GitGlobalConfig, result::GitGlobalResult};
use anyhow::Result;
// use iced::{button, Application, Button, Column, Command, Settings, Text};
use iced::{
    button, executor, scrollable, settings::Settings,
    window::Settings as Window, Align, Application, Background, Button,
    Clipboard, Color, Column, Command, Container, Element, Length, Scrollable,
    Text,
};
use iced_core::Rectangle;
// use iced_native::Rectangle;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

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
    type Executor = executor::Default;
    type Flags = ();

    fn new(flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("A simple counter")
    }

    fn update(
        &mut self,
        message: Message,
        clipboard: &mut Clipboard,
    ) -> Command<Message> {
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
                    // .background(b_bg)
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

#[derive(Default)]
struct RepoList {
    selected: i32,

    // The local state of the two buttons
    increment_button: button::State,
    decrement_button: button::State,
    list_of_things: scrollable::State,
}

impl Application for RepoList {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("A simple counter")
    }

    fn update(
        &mut self,
        message: Message,
        clipboard: &mut Clipboard,
    ) -> Command<Message> {
        match message {
            Message::IncrementPressed => {
                self.selected += 1;
                // let b = Rectangle{

                // }
                // let l = self.list_of_things.offset(bounds, content_bounds)
                // self.list_of_things.scroll_to(percentage, Re, content_bounds)
            }
            Message::DecrementPressed => {
                self.selected -= 1;
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
        let repos: Vec<String> = vec!["Haskell", "Swift", "Rust"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let mut scroller: Scrollable<'_, Message> =
            Scrollable::new(&mut self.list_of_things);
        // scroller = scroller.push(Text::new(repos.get(0).unwrap()));
        // scroller.push(Text::new(repos.get(1).unwrap()));
        for r in repos {
            scroller = scroller.push(Text::new(r));
        }
        // let rc_scroll = Rc::new(scroller);
        // let rc2 = Rc::clone(&rc_scroll);
        // let rc3 = Rc::clone(&rc_scroll);
        // repos.into_iter().for_each(|r| {
        //     // (rc2).push(Text::new(r));
        //     let s2 = scroller;
        //     s2.push(Text::new(r));
        //     scroller = s2;
        //     // (scroller).push(Text::new(r));
        // });
        let col: Column<'_, Message> = Column::new()
            .padding(100)
            .spacing(40)
            .width(Length::Fill)
            // .max_width(500)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Scroll Up"))
                    .on_press(Message::IncrementPressed),
            )
            .push(
                Container::new(Text::new(self.selected.to_string()).size(50))
                    .center_x(),
            )
            .push(
                Button::new(
                    &mut self.decrement_button,
                    Text::new("Scroll Down"),
                )
                .on_press(Message::DecrementPressed),
            )
            .push(scroller);
        // .push(rc3);
        // .push(
        //     Scrollable::new(&mut self.list_of_things)
        //         .push(Text::new("A first thing"))
        //         .push(Text::new("A second thing")),
        // );
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
        always_on_top: false,
        transparent: false,
        size: (400, 800),
        icon: None,
        min_size: None,
        max_size: None,
        resizable: true,
        decorations: false,
    };
    let s = Settings {
        antialiasing: false,
        default_font: None,
        exit_on_close_request: true,
        default_text_size: 12,
        flags: (),
        window: w,
    };
    RepoList::run(s);
    // Counter::run(s);
    let repos = gc.get_repos();
    let mut result = GitGlobalResult::new(&repos);
    return Ok(result);
}
