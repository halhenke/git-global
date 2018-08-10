use std::io;
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;

extern crate cursive;

use self::cursive::Cursive;
// use cursive::views::{Dialog, TextView};
use self::cursive::align::HAlign;
use self::cursive::event::EventResult;
use self::cursive::traits::*;
use self::cursive::views::{Dialog, OnEventView, SelectView, TextView};

use errors::Result as WeirdResult;


use super::super::{GitGlobalConfig, RepoTag, GitGlobalResult, get_repos};

#[derive(Debug)]
struct Selectable<'a> {
    pub selections: [&'a str; 3],
    pub selected: usize,
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


pub fn go() -> WeirdResult<GitGlobalResult> {
    let mut _sel = Selectable{
        selections: [
            "Choice 1",
            "Choice 2",
            "Choice 3",
        ],
        selected: 0,
    };

    // let r_tags: &Vec<RepoTag> = global_git.tag_names();

    // let mut siv = Cursive::default();

    // let content = "Press Q to quit the application.\n\nPress P to open the \
    //                popup.";

    // siv.add_global_callback('q', |s| s.quit());

    // // Let's wrap the view to give it a recognizable ID, so we can look for it.
    // // We add the P callback on the textview only (and not globally),
    // // so that we can't call it when the popup is already visible.
    // siv.add_layer(
    //     OnEventView::new(
    //         TextView::new(content)
    //             .with_id("text"))
    //             .on_event('p', |s| show_popup(s)),
    // );

    let mut select = SelectView::new().h_align(HAlign::Center);

    // Read the list of cities from separate file, and fill the view with it.
    // (We include the file at compile-time to avoid runtime read errors.)

    // let content = include_str!("../assets/cities.txt");
    // select.add_all_str(content.lines());

    // let content = [
    //     String::from("Choice 1"),
    //     String::from("Choice 2"),
    //     String::from("Choice 3"),
    // ];

    let _content = "
    Choice 1
    Choice 2
    Choice 3";

    let _global_git = GitGlobalConfig::new();
    let repos = get_repos();
    let r_tags: Vec<&str> = repos
        .iter()
        .map(|r| r.path())
        .collect();
    // let r_tags: Vec<&str> = global_git.tag_names();

    // select.add_all_str(content.lines());
    select.add_all_str(r_tags);

    // Sets the callback for when "Enter" is pressed.
    select.set_on_submit(show_next_window);

    // Let's override the `j` and `k` keys for navigation
    let select = OnEventView::new(select)
        .on_pre_event_inner('k', |s| {
            s.select_up(1);
            Some(EventResult::Consumed(None))
        })
        .on_pre_event_inner('j', |s| {
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
