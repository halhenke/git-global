use std::io;

use tui::Terminal;
use tui::backend::RawBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Group, Size, Direction};
use errors::Result as WeirdResult;

use super::super::{GitGlobalResult, RepoTag, get_repos, get_tagged_repos};


pub fn go() -> WeirdResult<GitGlobalResult> {
    let mut terminal = init().expect("Failed initialization");
    draw(&mut terminal).expect("Failed to draw");
    Ok(GitGlobalResult::new(&vec![]))
}

fn init() -> Result<Terminal<RawBackend>, io::Error> {
    let backend = RawBackend::new()?;
    Terminal::new(backend)
}

fn draw(t: &mut Terminal<RawBackend>) -> Result<(), io::Error> {

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
            Block::default()
                .title("Block 2")
                .borders(Borders::ALL)
                .render(t, &chunks[2]);
        });

    t.draw()
}
