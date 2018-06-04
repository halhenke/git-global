use std::io;
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;

use tui::Terminal;
use tui::backend::RawBackend;
use tui::widgets::{Widget, Block, Borders, SelectableList};
use tui::layout::{Group, Size, Direction};
use tui::style::{Style, Color, Modifier};
extern crate termion;
use self::termion::event;
use self::termion::input::TermRead;

use errors::Result as WeirdResult;

use super::super::{GitGlobalResult};
// use super::super::{GitGlobalResult, RepoTag, get_repos, get_tagged_repos};

enum Event {
    Input(event::Key),
}

#[derive(Debug)]

struct Selectable<'a> {
    pub selections: [&'a str; 3],
    pub selected: usize,
    // pub selected: &'a mut usize,
}

impl<'a> Selectable<'a> {
    pub fn inc(&mut self) -> usize {
        if self.selected < (self.selections.len() - 1) {
            self.selected = self.selected + 1;
        }
        else {
            self.selected;
        }
        self.selected
    }

    pub fn dec(&mut self) -> usize {
        if self.selected >= 1 {
            self.selected = self.selected - 1;
        }
        else {
            self.selected;
        }
        self.selected
    }
}

// mod sel {
//     // let hal;
//     // let mename = Hal;
//     // let selections = [
//     //     "Choice 1",
//     //     "Choice 2",
//     //     "Choice 3",
//     //     ];
// }

// let Selections = HashMap::from_list([
//     ("Choice 1", 1)
// ])

// enum Selection {
//     Choice1,
//     Choice2,
//     Choice3,
// }

pub fn go() -> WeirdResult<GitGlobalResult> {
    let mut terminal = init().expect("Failed initialization");
    // let mut selected = 0;
    // let selections = [
    //     "Choice 1",
    //     "Choice 2",
    //     "Choice 3",
    // ];
    let mut sel = Selectable{
        selections: [
            "Choice 1",
            "Choice 2",
            "Choice 3",
        ],
        selected: 0,
    };


    // Channels
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();

    // Input
    thread::spawn(move || {
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            input_tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
    });

    // Draw
    terminal.clear()?;
    terminal.hide_cursor()?;
    draw(&mut terminal, &sel).expect("Failed to draw");

    // Event Loop
    loop {
        let size = terminal.size().unwrap();
        // if app.size != size {
        //     terminal.resize(size).unwrap();
        //     app.size = size;
        // }

        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => match input {
                event::Key::Char('q') => {
                    break;
                }
                event::Key::Char('\n') => {
                    // app.messages.push(app.input.drain(..).collect());
                    break;
                }
                event::Key::Up => {
                    // sel.selected = (sel.selected - 1) % 3;
                    sel.dec();
                }
                event::Key::Down => {
                    // sel.selected = (sel.selected + 1) % 3;
                    sel.inc();
                }
                // event::Key::Char(c) => {
                //     // app.input.push(c);
                //     break;
                // }
                event::Key::Backspace => {
                    // app.input.pop();
                    break;
                }
                _ => {}
            },
        }
        draw(&mut terminal, &sel);
    }
    terminal.clear()?;
    terminal.show_cursor();

    println!("Selected was {}", sel.selections[sel.selected]);

    // Make type sig match
    Ok(GitGlobalResult::new(&vec![]))
}

fn init() -> Result<Terminal<RawBackend>, io::Error> {
    let backend = RawBackend::new()?;
    Terminal::new(backend)
}

// fn draw(t: &mut Terminal<RawBackend>) -> () {
fn draw(t: &mut Terminal<RawBackend>, sel: & Selectable) -> Result<(), io::Error> {

    let size = t.size()?;

    Group::default()
        .direction(Direction::Vertical)
        .margin(1)
        .sizes(&[Size::Percent(10), Size::Percent(80), Size::Percent(10)])
        .render(t, &size, |t, chunks| {
            Block::default()
                .title("Block")
                .borders(Borders::ALL)
                .render(t, &chunks[0]);
            SelectableList::default()
                .block(
                    Block::default()
                        .title("Choose One of these")
                        .borders(Borders::ALL)
                )
                .items(&sel.selections)
                .select(sel.selected)
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().modifier(Modifier::Italic))
                .highlight_symbol(">>")
                .render(t, &chunks[1]);
            Block::default()
                .title("Block 2")
                .borders(Borders::ALL)
                .render(t, &chunks[2]);
        });

    t.draw()
}
