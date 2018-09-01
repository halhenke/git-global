use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::ops::{Deref, DerefMut};

extern crate cursive;

use self::cursive::Cursive;
use self::cursive::align::HAlign;
use self::cursive::event::{Callback, Event, EventResult, Key};
use self::cursive::{
    traits::*,
    view::Selector
    };
use self::cursive::views::{
    Dialog,
    EditView,
    LinearLayout,
    ListView,
    MenuPopup,
    OnEventView,
    SelectView,
    TextContent,
    TextView
};
use core::errors::Result as WeirdResult;
use core::{GitGlobalConfig, RepoTag, GitGlobalResult, get_repos};
use mut_static::MutStatic;
use take_mut;

type RMut = Rc<RefCell<TextContent>>;

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
// pub fn go<'a, 'b>() -> WeirdResult<GitGlobalResult<'a>> {
    let user_config = GitGlobalConfig::new();

    trace!("go");

    debug!("ADD TAGS -  GOOOO: did we get here - 0");
    let mut siv = Cursive::default();
    debug!("ADD TAGS -  GOOOO: did we get here - 1");
    debug!("ADD TAGS -  GOOOO: did we get here - 1");

    siv.load_theme_file("assets/style.toml").unwrap();

    // https://github.com/gyscos/Cursive/issues/179
    let mut_content = TextContent::new(
        user_config.tag_names()
            .join("\n")
            // .for_each(|&x| String::from(x).push_str("\n"))
            // .map(|&x| String::from(x).push_str("\n"))
            // .map(|&x| x.append("\n"))
            // .collect::<String>()
    );
    let sel_tags_1: Vec<&str> = user_config.tag_names();
    let sel_tags_2: Vec<String> = user_config.tag_names()
        .into_iter()
        .map(|x| String::from(x))
        .collect();
    // let sel_tags_1: Vec<String> = user_config.tag_names()
        // .into_iter()
        // .map(|x| String::from(x))
        // .collect();
    // let sel_tags_2: Vec<String> = sel_tags_1.clone();
    let sel_tags = sel_tags_1.into_iter().zip(sel_tags_2.into_iter());
        // .collect::<Vec<String>>();
    // let sel_tags = user_config.tag_names().iter()
    //         .zip(*user_config.tag_names().collect::<String>());

    // NOTE: We want to make these "upfront" otherwise we woulc clone on every callback - prob not a big deal actually
    // If we make borrows here then we cant do borrow_muts later which is what we need
    let mut_con = Rc::new(RefCell::new(mut_content));
    // let immut_con = Rc::clone(&mut_con).borrow()
    let m2_con = Rc::clone(&mut_con);
    // let m2_con = &mut_con.clone();
    let m3_con = Rc::clone(&mut_con);
    let m4_con = Rc::clone(&mut_con);

    debug!("ADD TAGS: did we get here - 3");

    // Need to wrap this to make it usable in the static closures/callbacks
    // Need to keep a list of new tags as i have to display both pre-existing tags and new ones so i need to store these separately
    // let new_tags: Vec<&str>;
    // let new_tags: &mut Vec<String> = &mut vec!();
    // let new_tags: &mut Vec<&str>;
    let mut new_tags: Vec<String> = Vec::new();
    // let fake_tags = &new_tags;

    // let edit_cb = move |s: &mut Cursive, name: &str| {
    let edit_cb = move |s: &mut Cursive, name: &str| {
        debug!("edit_cb was called...");
        // let nut_con = m3_con.clone();
        let mut b1 = m2_con.borrow_mut();
        // &new_tags.push(String::from(name));
        take_mut::take(&mut new_tags, |mut new_tags| {
            new_tags.push(String::from(name));
            // new_tags.push("Hola");
            // new_tags.push(&name.clone());
            new_tags
        });
        // show_next_screen(s, &name.clone().deref(), &mut nut_con.borrow_mut());
        // show_next_screen(s, &name.clone().deref(), m3_con.clone().borrow_mut());
        show_next_screen(s, &name.clone().deref(), &mut b1);
    };

    let e_view = EditView::new()
        // .on_submit(show_popup)
        .on_submit_mut(edit_cb)
        .with_id("tag")
        .fixed_width(20);
    // let t_view  = TextView::new_with_content(
    //     m3_con.borrow()
    //     // Rc::clone(&mut_con)
    //         .deref()
    //         .clone())
    //     .with_id("tag_list");
    // let mut sel_view = SelectView::new()
    //     .with_all(
    //         sel_tags
    //     )
    //     .with_id("tag_list");

    siv.add_layer(
        LinearLayout::vertical()
            .child(
                Dialog::new()
                    .title("Add a Tag...")
                    .padding((1, 1, 1, 0))
                    .content(
                        e_view
                    )
                    .button("q", move |s: &mut Cursive| {
                        debug!("q was called...");
                        save_tags_and_quit(s, &m4_con);
                        // save_tags_and_quit(s, &mut user_config, &m4_con);
                    })
                    .button("Ok", move |s: &mut Cursive| {
                        let name = s.call_on_id(
                            "tag",
                            |view: &mut EditView| view.get_content(),
                        ).unwrap();
                        debug!("OK was called...");
                        let nut_con = mut_con.clone();
                        let mut b1 = nut_con.borrow_mut();
                        show_next_screen(s, &name.clone().deref(), &mut b1);
                    }).with_id("dialog")
            )
            // .child(
            //     t_view
            // )
            .child(
                // sel_view
                OnEventView::new(
                    SelectView::new()
                        .with_all(
                            sel_tags
                        )
                        .with_id("tag_list")
                )
                // .on_event(Event::Key::Del).has_callback()
                // .on_event_inner('p', |mut s1| {
                .on_event_inner(Event::Key(Key::Backspace), |s1| {
                    // s.pop_layer();
                    // s1.get_inner().add_item("bolo", "yolo")
                    // s1.get_mut().add_item("bolo", "yolo".to_string());
                    // s1.get_mut().select_up(1);


                    delete_tag(&mut s1.get_mut())
                    // let sel = s1.get_mut();
                })
                // .on_event(Event::Key::Del)::with_cb(
                // )
            )
    );

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
