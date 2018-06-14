use std::io;
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;
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
use errors::Result as WeirdResult;

use super::super::{GitGlobalConfig, RepoTag, GitGlobalResult, get_repos};

use mut_static::MutStatic;

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
    let mut siv = Cursive::new();
    let mut tags = Vec::<&str>::new();
    // static mut tagBag: Vec<&str> = vec![];
    // NOTE: No real idea why this works but nothing works without it
    // - see https://stackoverflow.com/a/28521985/935470
    let seen_cell = RefCell::new(tags);
    let content = TextContent::new("Original");
    let seen_content = Rc::new(RefCell::new(content));
    // let seen_content = RefCell::new(content);

    // let shared = Rc::new(&TextContent::new("Original"));

    let mutContent = TextContent::new("Original");
    let mutCon = Rc::new(RefCell::new(mutContent));
    let m2Con = &mutCon.clone();

    // let fuck = (&seen_content).borrow();
    // let seen_more = RefCell::new(&seen_content);
    // let other_text = (&seen_content).borrow();
    // let other_content = Rc::clone(&seen_content);

    let mut moreContent  = TextContent::new("Original");
    let boxContent = Box::new(moreContent);

    // STAT_TC.set(TextContent::new("hello")).unwrap();



    // let tags = &mut Vec::<&str>::new();
    // const mut tags: Vec<&str> = vec![];
    // static mut tags: Vec<&str> = Vec::<&str>::new();

    // let mut cursor = TagCursive::new();
    // let mut siv = cursor.siv;
    // let tags = cursor.tags;




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
                            .on_submit(show_popup)
                            .with_id("tag")
                            .fixed_width(20),
                    )
                    // .button("Ok", |s| {
                    // .button("Ok", |s: &mut Cursive| {
                    .button("Ok", move |s: &mut Cursive| {
                        let name = s.call_on_id(
                            "tag",
                            |view: &mut EditView| view.get_content(),
                        ).unwrap();

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

                        // boxContent.pu
                        let nutCon = mutCon.clone();
                        let mut b1 = nutCon.borrow_mut();
                        // b1.append("hey");
                        // let mut b2 = RefMut::map(b1, move |&mut t| &mut t.append("hellsbells") );
                        show_next_screen(s, &name.clone().deref(), &mut b1);


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
                    // **shared.clone().deref()
                    // *Rc::try_unwrap(shared.clone()).unwrap_or(
                    //     &TextContent::new("Hey Man")
                    // )

                    // {
                    //     let temp = STAT_TC.read().unwrap();
                    //     // let tc: &TextContent = STAT_TC.read().unwrap();
                    //     // let tc: &TextContent = STAT_TC.read().unwrap();
                    //     // tc.clone()
                    //     temp.clone()
                    //     // tc.get_content()
                    //     // tc
                    // }
                    {
                        // let x1 =  Rc::clone(& mutCon);
                        let x1 = m2Con.borrow();
                        // TextContent::new("Hey Man")
                        x1.deref().clone()
                    }

                    // (&seen_content).as_ptr() as TextContent
                    // (&seen_content).into_inner()
                    // (ref (&seen_content).borrow_mut())
                    // (&content).clone()
                    // (&seen_content).borrow()
                    // *fuck
                    // other_content.borrow().deref() as TextContent
                    // other_content.into_inner().clone()
                    // other_content.deref().borrow()
                    // fuck.deref().clone()

                    // content
                    // borrowed_content
                ).with_id("tagList")

                // ListView::new()
                    // .child("-----")
            )
    );

    siv.run();
    // cursor.siv.run();

    Ok(GitGlobalResult::new(&vec![]))
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
                }).unwrap();
        // s.focus_id("tag").unwrap();
        s.focus(&Selector::Id("tag"));
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
        let content = format!("Hello {}!", name);
        // s.pop_layer();
        // s.add_layer(Dialog::around(TextView::new(content))
        //     .button("Quit", |s| s.quit()));
    }
}

// fn show_popup_tags<'a> (s: &mut Cursive, name: &'a str, tags: &'a mut Vec<&'a str>) {
//     let t = tags;
//     t.push(name);


//     if name.is_empty() {
//         s.add_layer(Dialog::info("Please enter a name!"));
//     } else {
//         let content = format!("Hello {}!", name);
//         s.pop_layer();
//         s.add_layer(Dialog::around(TextView::new(content))
//             .button("Quit", |s| s.quit()));
//     }
// }


// // Let's put the callback in a separate function to keep it clean,
// // but it's not required.
// fn show_next_window(siv: &mut Cursive, city: &str) {
//     siv.pop_layer();
//     let text = format!("{} is a great city!", city);
//     siv.add_layer(
//         Dialog::around(TextView::new(text)).button("Quit", |s| s.quit()),
//     );
// }
