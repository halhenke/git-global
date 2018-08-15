// use std::io;
// use std::sync::mpsc;
// use std::thread;
// use std::collections::HashMap;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::ops::{Deref, DerefMut};

extern crate cursive;


use self::cursive::Cursive;
// use cursive::views::{Dialog, TextView};
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

lazy_static! {
    pub static ref STAT_TAG: MutStatic<Vec<&'static str>> = {
        return MutStatic::from(vec![]);
    };

    pub static ref STAT_TC: MutStatic<TextContent> = {
        return MutStatic::from(TextContent::new("New TextContent"));
    };


    // static ref tagBag: Vec<&str> = Vec::new();
}

struct TagCursive {
    siv: Cursive,
    tags: Vec<String>,
}

impl TagCursive {
    pub fn new() -> TagCursive {
        TagCursive {
            siv: Cursive::new(),
            tags: vec![],
        }
    }
}

// struct TagCursive<'a> {
//     siv: &'a Cursive,
//     tags: &'a Vec<&'a str>,
// }

// impl<'a> TagCursive<'a> {
//     pub fn new() -> TagCursive<'a> {
//         TagCursive {
//             siv: &Cursive::new(),
//             tags: &vec![],
//         }
//     }
// }

// fn start(arg: Type) -> RetType {
//     unimplemented!();
// }

pub fn go<'a, 'b>() -> WeirdResult<GitGlobalResult> {

    println!("ADD TAGS -  GOOOO: did we get here - 0");

    let mut siv = Cursive::new();

    println!("ADD TAGS -  GOOOO: did we get here - 1");

    let mut tags = Vec::<&str>::new();

    println!("ADD TAGS -  GOOOO: did we get here - 1");

    // static mut tagBag: Vec<&str> = vec![];
    // NOTE: No real idea why this works but nothing works without it
    // - see https://stackoverflow.com/a/28521985/935470
    let _seen_cell = RefCell::new(tags);
    let content = TextContent::new("Original");
    let _seen_content = Rc::new(RefCell::new(content));
    // let seen_content = RefCell::new(content);

    // let shared = Rc::new(&TextContent::new("Original"));

    let mut_content = TextContent::new("Original");
    let mut_con = Rc::new(RefCell::new(mut_content));
    let m2_con = &mut_con.clone();
    // let m3_con = m2Con.clone();
    let m3_con = Rc::clone(&mut_con);
    let m4_con = Rc::clone(&mut_con);

    // let fuck = (&seen_content).borrow();
    // let seen_more = RefCell::new(&seen_content);
    // let other_text = (&seen_content).borrow();
    // let other_content = Rc::clone(&seen_content);

    let mut more_content  = TextContent::new("Original");
    let _box_content = Box::new(more_content);

    // STAT_TC.set(TextContent::new("hello")).unwrap();

    println!("ADD TAGS: did we get here - 3");


    // let tags = &mut Vec::<&str>::new();
    // const mut tags: Vec<&str> = vec![];
    // static mut tags: Vec<&str> = Vec::<&str>::new();

    // let mut cursor = TagCursive::new();
    // let mut siv = cursor.siv;
    // let tags = cursor.tags;

    let editCB = move |s: &mut Cursive, name: &str| {
        // let name = s.call_on_id(
        //     "tag",
        //     |view: &mut EditView| view.get_content(),
        // ).unwrap();
        println!("editCB was called...");

        let nutCon = m3_con.clone();
        let mut b1 = nutCon.borrow_mut();
        show_next_screen(s, &name.clone().deref(), &mut b1);
    };
    // let ecb = Rc::new(editCB);
    // let ec2 = ecb.clone();
    // let ec3 = Rc::clone(&ecb);
    // // let ec2 = Rc::clone(&ecb);
    // // let ec3 = Rc::clone(&ecb);


    // siv.tags = Vec::new();
    // let mut tags = Vec::new();

    // Create a dialog with an edit text and a button.
    // The user can either hit the <Ok> button,
    // or press Enter on the edit text.
    siv.add_layer(
    // cursor.siv.add_layer(
        LinearLayout::vertical()
            .child(
                Dialog::new()
                    .title("Add a Tag...")
                    .padding((1, 1, 1, 0))
                    .content(
                        EditView::new()
                            // .on_submit(show_popup)
                            .on_submit(editCB)
                            // .on_submit(Rc::try_unwrap(ec2).expect())
                            .with_id("tag")
                            .fixed_width(20),
                    )
                    .button("q", move |s: &mut Cursive| {
                        // s.quit()
                        println!("q was called...");
                        // println!("we are going with {:?}", &m4Con);
                        save_tags_and_quit(s, &m4_con);
                    })
                    // .button("Ok", |s| {
                    // .button("Ok", |s: &mut Cursive| {
                    .button("Ok", move |s: &mut Cursive| {
                        let name = s.call_on_id(
                            "tag",
                            |view: &mut EditView| view.get_content(),
                        ).unwrap();

                        println!("OK was called...");


                        // let mut tag = STAT_TAG.push("name");
                        let mut my_vec = STAT_TAG.write().unwrap();
                        my_vec.push("name");
                        let mut my_tc = STAT_TC.write().unwrap();
                        // my_tc.append("name");
                        // my_tc.append(Rc::try_unwrap(name).unwrap());

                        // &shared.clone();
                        // Rc::try_unwrap(&shared.clone()).unwrap_or(
                        //     &TextContent::new("Hey Man")
                        // );

                        let nutCon = mut_con.clone();
                        let mut b1 = nutCon.borrow_mut();
                        show_next_screen(s, &name.clone().deref(), &mut b1);
                        // &editCB(s, &name.clone().deref());


                        // let mut borrowed = seen_cell.borrow_mut();
                        // borrowed.push("fuck");
                        // tags.push("fuck");

                        // let mut borrowed_content = seen_content.borrow_mut();

                        // show_next_screen(s, &name, borrowed_content);
                        // show_next_screen(s, &name, boxContent);
                        // show_next_screen(s, &name, &mut boxContent.deref_mut());

                        // show_popup(s, &name);


                        // &tags.push(&String::from("fuck"));
                        // &tags.push(&String::from("fuck"));

                        // &cursor.tags.push(String::from("fuck"));
                        // tags.push(name.clone());
                        // let t = &mut tags;
                        // t.push(name);
                        // show_popup_tags(s, &name, &mut tags);
                    }).with_id("dialog"),
            )
            .child(
                TextView::new_with_content(
                    // TextContent::new("Hey Man")
                    m2_con.borrow().deref().clone()
                ).with_id("tagList")
            )
    );

    siv.run();
    println!("ADD TAGS: called - 33");

    // let gc = GitGlobalConfig::new();
    // println!("ADD TAGS: called - 34");
    // gc.write_tags();

    Ok(GitGlobalResult::new(&vec![]))
}

fn save_tags_and_quit(s: &mut Cursive, tags: &RMut) {
    println!("wtf???");
    let mut user_config = GitGlobalConfig::new();
    let mut tagList: String = tags
        .borrow()
        .deref()
        .get_content()
        .source()
        .to_string();
    // println!("i have some tags");

    s.call_on_id("tag",
        |view: &mut EditView|
            {
                &tagList.push_str("abra-cadabra");
                let po = &tagList.clone();
                view.set_content(po.to_string());
                // view.append("abra-cadabra");
                // view.set_cursor(0)
            }).expect("final unwrap...");
    let tag_list_list = tagList
        .lines()
        .skip(1)
        // .by_ref()
        .map(|s| s.to_string())
        .collect();
    println!("About to print tags");
    println!("tags are: {:?}", &tag_list_list);
    user_config.add_tags(
        tag_list_list
        // tagList.split("\n").skip(1).collect()
        // tagList.lines().skip(1).collect::Vec<String>()
    );
    user_config.write_tags();
    // user_config.print_tags();
    // s.quit();
    s.cb_sink()
        .send(Box::new(|siv: &mut Cursive| siv.quit()))
        .expect("Dont fail here");
}

fn show_next_screen(s: &mut Cursive, name: &str, c: &mut TextContent) {
// fn show_next_screen(s: &mut Cursive, name: &str, mut c: TextContent) {
// fn show_popup(s: &mut Cursive, name: &str) {
// fn show_next_screen(s: &mut Cursive, name: &str, mut c: RefMut<TextContent>) {
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
        // s.focus_id("dialog").unwrap();
        // s.call_on_id(
        //     "tag",
        //     |view: &mut EditView| view.set_cursor(0),
        // ).unwrap();
    }
}


fn show_popup(s: &mut Cursive, name: &str) {
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
        // s.pop_layer();
        // s.add_layer(Dialog::around(TextView::new(content))
        //     .button("Quit", |s| s.quit()));
    }
}

