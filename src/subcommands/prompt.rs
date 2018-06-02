use std::io;
use std::sync::mpsc;
use std::thread;

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

pub fn go() -> WeirdResult<GitGlobalResult> {
    let mut terminal = init().expect("Failed initialization");
    let mut selected = 1;

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
    draw(&mut terminal, selected).expect("Failed to draw");

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
                    // app.messages.push(app.input.drain(..).collect());
                    selected = selected - 1;
                }
                event::Key::Down => {
                    // app.messages.push(app.input.drain(..).collect());
                    selected = selected + 1;
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
        draw(&mut terminal, selected);
    }
    terminal.clear()?;
    terminal.show_cursor();


    // Make type sig match
    Ok(GitGlobalResult::new(&vec![]))
}

fn init() -> Result<Terminal<RawBackend>, io::Error> {
    let backend = RawBackend::new()?;
    Terminal::new(backend)
}

// fn draw(t: &mut Terminal<RawBackend>) -> () {
fn draw(t: &mut Terminal<RawBackend>, sel: usize) -> Result<(), io::Error> {

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
                .items(&[
                    "Choice 1",
                    "Choice 2",
                    "Choice 3"
                ])
                .select(sel)
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
