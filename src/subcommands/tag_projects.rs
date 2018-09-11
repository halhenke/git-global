use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::ops::{Deref, DerefMut};
use std::iter::Zip;
use std;
extern crate cursive;

use self::cursive::Cursive;
use self::cursive::align::HAlign;
use self::cursive::event::{Callback, Event, EventResult, Key};
use self::cursive::{
    traits::*,
    view::Selector
    };
use self::cursive::{
    view::{
        View,
        ViewWrapper,
    },
    views::{
        BoxView,
        Dialog,
        EditView,
        IdView,
        LinearLayout,
        ListView,
        MenuPopup,
        OnEventView,
        Panel,
        ScrollView,
        SelectView,
        TextContent,
        TextView,
        }};
use core::errors::Result as WeirdResult;
use core::{GitGlobalConfig, RepoTag, GitGlobalResult, get_repos};
use mut_static::MutStatic;
use take_mut;

type RMut = Rc<RefCell<TextContent>>;

// #[derive(Debug)]
// IdView
use std::fmt;
// impl fmt::Debug for IdView<V> {
// // impl Debug<V> for IdView<V>
//     // where V: View {
// // impl fmt::Debug for IdView<V> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "IdView");
//     }
// }

// mk_cursive = cursive::default;
// let mk_cursive = cursive::ncurses;

// pub fn delete_tag(siv: &mut Cursive, sel: &mut SelectView) {
pub fn delete_tag(sel: &mut SelectView) -> Option<EventResult> {
    // match Some(id) {
    match sel.selected_id() {
        Some(id) => {
        // if let Some(id) = sel.selected_id() {
            let tag: String = sel.get_item(id).unwrap().1.clone();
            let cb: Callback = Callback::from_fn(
                move |siv: &mut Cursive| {
                    siv.add_layer(Dialog::around(
                        TextView::new(format!("Delete tag: {}?", tag)))
                            .button("No", |s| {
                                s.pop_layer();
                            })
                            .button("Yes", move |s| {
                                s.call_on_id("tag_list", |v: &mut SelectView| {
                                    v.remove_item(id);
                                });
                                s.pop_layer();
                            }));
            });
            Some(EventResult::Consumed(Some(cb)))
        },
        None => {
            None
        }
    }

}


pub fn go<'a, 'b>() -> WeirdResult<GitGlobalResult> {
    let user_config = GitGlobalConfig::new();

    trace!("go");

    let mut siv = Cursive::default();
    siv.load_theme_file("assets/style.toml").unwrap();

    // https://github.com/gyscos/Cursive/issues/179
    let mut_content = TextContent::new(
        user_config.tag_names()
            .join("\n")
    );
    let sel_tags_1: Vec<&str> = user_config.tag_names();
    let sel_tags_2: Vec<String> = user_config.tag_names()
        .into_iter()
        .map(|x| String::from(x))
        .collect();
    let sel_tags = sel_tags_1.into_iter().zip(sel_tags_2.into_iter());

    type sel_tag_list<'a> = std::iter::Zip<std::vec::IntoIter<&'a str>, std::vec::IntoIter<String>>;

    fn selectify(tags_1: Vec<&str>) -> sel_tag_list {
        let tags_2: Vec<String> = tags_1
            .clone()
            .into_iter()
            .map(|x| String::from(x))
            .collect();
        return tags_1
            .into_iter()
            .zip(tags_2.into_iter())
    }

    debug!("ADD TAGS: did we get here - 3");
    let mut new_tags: Vec<String> = Vec::new();
    let edit_cb = move |s: &mut Cursive, name: &str| {
        debug!("edit_cb was called...");
        // let nut_con = m3_con.clone();
        // let mut b1 = m2_con.borrow_mut();
        // &new_tags.push(String::from(name));
        take_mut::take(&mut new_tags, |mut new_tags| {
            new_tags.push(String::from(name));
            new_tags
        });
    };

    let e_view = EditView::new()
        // .on_submit(show_popup)
        .on_submit_mut(edit_cb)
        .with_id("tag")
        .fixed_width(20);
    let repo_selector = SelectView::new()
        .with_all(selectify(
            user_config.get_cached_repos()
                .iter()
                .map(|r| r.path.as_str())
                // .map(|p| p.rsplit("/"))
                .map(|p|
                    *p.rsplit("/")
                        .collect::<Vec<&str>>()
                        .first()
                        .unwrap()
                )
                // .flatten()
                .take(5)
                .collect()
        ))
        .min_width(20)
        .with_id("repo-field");
    let tags_displayer: IdView<BoxView<SelectView>> = SelectView::new()
        .with_all(selectify(
            vec!("hoo", "lah", "laa")
        ))
        .min_width(20)
        .with_id("tag-display");
    let tags_pool: IdView<SelectView> = SelectView::new()
    // let tags_pool: IdView<ScrollView<SelectView>> = SelectView::new()
            .with_all(selectify(
            user_config.tags
                .iter()
                .map(|r| r.name.as_str())
                .collect()
        ))
        // .scrollable()
        .with_id("tag-pool");

    siv.add_layer(
        LinearLayout::vertical()
            .child(
                LinearLayout::horizontal()
                    .child(Panel::new(repo_selector))
                    .child(Panel::new(tags_displayer))
            )
            .child(
                // sel_view
                Panel::new(

                    OnEventView::new(
                        tags_pool
                    )
                    .on_event_inner(Event::Key(Key::Backspace), |s1| {
                        delete_tag(&mut s1.get_mut())
                    })
                    // NOTE: Due to fucking annoying design this has to come
                    // after/outside `OnEventView` - otherwise we never get to unwrap
                    // properly
                    .scrollable()
                    // .on_event(Event::Key::Del)::with_cb(
                    // )
                )
            )
    );
    siv.add_global_callback('q', |s1| {
        s1.quit()
    });
    siv.run();
    debug!("ADD TAGS: called - 33");

    // println!("new tags is {:?}", &fake_tags);
    Ok(GitGlobalResult::new(&vec![]))
}

fn save_tags_and_quit(s: &mut Cursive, tags: &RMut) {
// fn save_tags_and_quit(s: &mut Cursive, user_config: &mut GitGlobalConfig, tags: &RMut) {
    let mut user_config = GitGlobalConfig::new();
    trace!("save_tags_and_quit");
    debug!("wtf???");
    let mut t_list: Vec<String> = Vec::new();
    s.call_on_id("tag_list",
        |tl: &mut SelectView| {
            error!("tag count is {}", tl.len());
            let count = tl.len();
            for i in 0..count  {
                t_list.push(tl.get_item(i).unwrap().0.to_string())
            }
        }
    );
    let tag_list: String = tags
        .borrow()
        .deref()
        .get_content()
        .source()
        .to_string();
    s.call_on_id("tag",
        |view: &mut EditView| {
            let po = &tag_list.clone();
            view.set_content(po.to_string());
        }
    ).expect("final unwrap...");
    let tag_list_list = t_list;
    debug!("About to print tags");
    debug!("tags are: {:?}", &tag_list_list);
    // user_config.add_tags(
    //     tag_list_list
    // );
    user_config.replace_tags(
        tag_list_list
    );
    user_config.write_tags();
    s.cb_sink()
        .send(Box::new(|siv: &mut Cursive| siv.quit()));
}

fn show_next_screen(s: &mut Cursive, name: &str, c: &mut TextContent) {
    trace!("show_next_screen");
    if name.is_empty() {
        s.add_layer(Dialog::info("Please enter a name!"));
    } else {
        trace!("show_next_screen 2");
        s.call_on_id("tag_list",
            |view: &mut SelectView|
                view.add_item_str(name)
        ).expect("failure");
        s.call_on_id("tag",
            |view: &mut EditView|
                {
                    view.set_content("")
                    // view.set_cursor(0)
                }).expect("failure");
        // // s.focus_id("tag").unwrap();
        s.focus(&Selector::Id("tag")).expect("thing");
    }
}
