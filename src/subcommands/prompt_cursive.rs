extern crate cursive;

use self::cursive::Cursive;
// use cursive::views::{Dialog, TextView};
use self::cursive::align::HAlign;
use self::cursive::event::EventResult;
use self::cursive::traits::*;
use self::cursive::views::{Dialog, OnEventView, SelectView, TextView};

use crate::models::errors::Result as WeirdResult;

use crate::models::{config::GitGlobalConfig, result::GitGlobalResult};

#[derive(Debug)]
struct Selectable<'a> {
    pub selections: [&'a str; 3],
    pub selected: usize,
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
    let mut _sel = Selectable {
        selections: ["Choice 1", "Choice 2", "Choice 3"],
        selected: 0,
    };

    let mut select = SelectView::new().h_align(HAlign::Center);

    let _content = "
    Choice 1
    Choice 2
    Choice 3";

    let mut gc = GitGlobalConfig::new();
    let repos = gc.get_repos();
    let r_tags: Vec<&str> = repos
        .iter()
        .map(|r| r.path().rsplit('/').nth(0).expect("repo has name?"))
        .collect();
    // let r_tags: Vec<&str> = global_git.tag_names();

    // select.add_all_str(content.lines());
    select.add_all_str(r_tags);

    // Sets the callback for when "Enter" is pressed.
    select.set_on_submit(show_next_window);

    // Let's override the `j` and `k` keys for navigation
    let select = OnEventView::new(select)
        .on_pre_event_inner('k', |s, _k| {
            s.select_up(1);
            Some(EventResult::Consumed(None))
        })
        .on_pre_event_inner('j', |s, _k| {
            s.select_down(1);
            Some(EventResult::Consumed(None))
        });

    let mut siv = Cursive::default();

    // Let's add a BoxView to keep the list at a reasonable size
    // (it can scroll anyway).
    siv.add_layer(
        Dialog::around(select.fixed_size((20, 10)))
            .title("Where are you from?"),
    );

    siv.run();
    Ok(GitGlobalResult::new(&vec![]))
}

// Let's put the callback in a separate function to keep it clean,
// but it's not required.
fn show_next_window(siv: &mut Cursive, city: &str) {
    siv.pop_layer();
    let text = format!("{} is a great city!", city);
    siv.add_layer(
        Dialog::around(TextView::new(text)).button("Quit", |s| s.quit()),
    );
}
