use std::io;
use std::sync::mpsc;
use std::thread;

use tui::backend::TermionBackend;
use tui::layout::{Constraint::Percentage, Direction, Layout};
use tui::widgets::{Block, Borders, SelectableList, Widget};
use tui::Terminal;
// use tui::layout::{Group, Size, Direction};
use tui::style::{Color, Modifier, Style};
extern crate termion;
use self::termion::event;
use self::termion::input::TermRead;

use crate::repo::errors::Result as WeirdResult;

use crate::repo::GitGlobalResult;
// use super::super::{GitGlobalResult, RepoTag, get_repos, get_tagged_repos};

enum Event {
    Input(event::Key),
}

#[derive(Debug)]
struct Selectable<'a> {
    pub selections: Vec<&'a str>,
    pub selected: usize,
    // pub selected: &'a mut usize,
}

impl<'a> Selectable<'a> {
    pub fn inc(&mut self) -> usize {
        if self.selected < (self.selections.len() - 1) {
            self.selected = self.selected + 1;
        } else {
            self.selected;
        }
        self.selected
    }

    pub fn dec(&mut self) -> usize {
        if self.selected >= 1 {
            self.selected = self.selected - 1;
        } else {
            self.selected;
        }
        self.selected
    }
}

pub fn go() -> WeirdResult<GitGlobalResult> {
    let mut terminal = init().expect("Failed initialization");

    let mut sel = Selectable {
        selections: vec!["Choice 1", "Choice 2", "Choice 3"],
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
        let _size = terminal.size().unwrap();
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
                    break;
                }
                event::Key::Up => {
                    sel.dec();
                }
                event::Key::Down => {
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
        draw(&mut terminal, &sel).expect("Draw fail");
    }
    terminal.clear()?;
    terminal.show_cursor().expect("Show Cursur fail");

    println!("Selected was {}", sel.selections[sel.selected]);

    // Make type sig match
    Ok(GitGlobalResult::new(&vec![]))
}

fn init() -> Result<Terminal<TermionBackend<io::Stdout>>, io::Error> {
    let stdout = io::stdout();
    // let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    Terminal::new(backend)
}

// fn draw(t: &mut Terminal<TermionBackend>) -> () {
fn draw(
    term: &mut Terminal<TermionBackend<io::Stdout>>,
    sel: &Selectable,
) -> Result<(), io::Error> {
    let size = term.size()?;

    term.draw(|mut t| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [Percentage(10), Percentage(80), Percentage(10)].as_ref(),
            )
            .split(size);
        // .render(t, &size, |t, chunks| {
        Block::default()
            .title("Block")
            .borders(Borders::ALL)
            .render(&mut t, chunks[0]);
        SelectableList::default()
            .block(
                Block::default()
                    .title("Choose One of these")
                    .borders(Borders::ALL),
            )
            .items(&sel.selections)
            .select(Some(sel.selected))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .render(&mut t, chunks[1]);
        Block::default()
            .title("Block 2")
            .borders(Borders::ALL)
            .render(&mut t, chunks[2]);
    })
}
