use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::ops::{Deref, DerefMut};

extern crate cursive;

use self::cursive::Cursive;
use self::cursive::align::HAlign;
use self::cursive::event::EventResult;
use self::cursive::{
    traits::*,
    view::Selector
    };
use self::cursive::views::{
    Dialog,
    EditView,
    LinearLayout,
    ListView,
    OnEventView,
    SelectView,
    TextContent,
    TextView
};
use core::errors::Result as WeirdResult;

use core::{GitGlobalConfig, RepoTag, GitGlobalResult, get_repos};

use mut_static::MutStatic;

type RMut = Rc<RefCell<TextContent>>;

// mk_cursive = cursive::default;
// let mk_cursive = cursive::ncurses;

lazy_static! {
    pub static ref STAT_TAG: MutStatic<Vec<&'static str>> = {
        return MutStatic::from(vec![]);
    };

    pub static ref STAT_TC: MutStatic<TextContent> = {
        return MutStatic::from(TextContent::new("New TextContent"));
    };
}


pub fn go<'a, 'b>() -> WeirdResult<GitGlobalResult> {
    let mut user_config = GitGlobalConfig::new();

    trace!("go");

    debug!("ADD TAGS -  GOOOO: did we get here - 0");
    let mut siv = Cursive::default();
    debug!("ADD TAGS -  GOOOO: did we get here - 1");
    let tags = Vec::<&str>::new();
    debug!("ADD TAGS -  GOOOO: did we get here - 1");

    // static mut tagBag: Vec<&str> = vec![];
    // NOTE: No real idea why this works but nothing works without it
    // - see https://stackoverflow.com/a/28521985/935470
    let _seen_cell = RefCell::new(tags);

    let mut_content = TextContent::new(
        user_config.tag_names()
            .join("\n")
            // .for_each(|&x| String::from(x).push_str("\n"))
            // .map(|&x| String::from(x).push_str("\n"))
            // .map(|&x| x.append("\n"))
            // .collect::<String>()
    );
    let mut_con = Rc::new(RefCell::new(mut_content));
    let m2_con = &mut_con.clone();
    let m3_con = Rc::clone(&mut_con);
    let m4_con = Rc::clone(&mut_con);

    debug!("ADD TAGS: did we get here - 3");

    let edit_cb = move |s: &mut Cursive, name: &str| {
        debug!("edit_cb was called...");

        let nut_con = m3_con.clone();
        let mut b1 = nut_con.borrow_mut();
        show_next_screen(s, &name.clone().deref(), &mut b1);
    };

    siv.add_layer(
        LinearLayout::vertical()
            .child(
                Dialog::new()
                    .title("Add a Tag...")
                    .padding((1, 1, 1, 0))
                    .content(
                        EditView::new()
                            // .on_submit(show_popup)
                            .on_submit(edit_cb)
                            .with_id("tag")
                            .fixed_width(20),
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

                        let mut my_vec = STAT_TAG.write().unwrap();
                        my_vec.push("name");
                        let mut _my_tc = STAT_TC.write().unwrap();
                        let nut_con = mut_con.clone();
                        let mut b1 = nut_con.borrow_mut();
                        show_next_screen(s, &name.clone().deref(), &mut b1);
                    }).with_id("dialog"),
            )
            .child(
                TextView::new_with_content(
                    m2_con.borrow()
                        .deref()
                        .clone()
                ).with_id("tag_list")
            )
    );

    siv.run();
    debug!("ADD TAGS: called - 33");
    Ok(GitGlobalResult::new(&vec![]))
}

fn save_tags_and_quit(s: &mut Cursive, tags: &RMut) {
// fn save_tags_and_quit(s: &mut Cursive, user_config: &mut GitGlobalConfig, tags: &RMut) {
    let mut user_config = GitGlobalConfig::new();

    trace!("save_tags_and_quit");
    debug!("wtf???");

    let tag_list: String = tags
        .borrow()
        .deref()
        .get_content()
        .source()
        .to_string();

    s.call_on_id("tag",
        |view: &mut EditView|
            {
                let po = &tag_list.clone();
                view.set_content(po.to_string());
            }).expect("final unwrap...");
    let tag_list_list: Vec<String> = tag_list
        .lines()
        .skip(1)
        .map(|s| s.to_string())
        .collect();
    debug!("About to print tags");
    debug!("tags are: {:?}", &tag_list_list);
    user_config.add_tags(
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
        c.append("\n");
        c.append(name);
        s.call_on_id("tag",
            |view: &mut EditView|
                {
                    view.set_content("")
                    // view.set_cursor(0)
                }).expect("failure");
        // s.focus_id("tag").unwrap();
        s.focus(&Selector::Id("tag")).expect("thing");
    }
}


fn show_popup(s: &mut Cursive, name: &str) {
    trace!("show_popup");
    if name.is_empty() {
        s.add_layer(Dialog::info("Please enter a name!"));
    } else {
        // c.set_content(name);
        let _content = format!("Hello {}!", name);
        s.call_on_id("tag",
            |view: &mut EditView|
                {
                    view.set_content("")
                    // view.set_cursor(0)
                }).expect("show content");
    }
}

