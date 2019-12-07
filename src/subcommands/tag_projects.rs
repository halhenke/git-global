#![feature(trait_alias)]

use std;
use std::any::Any;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
extern crate cursive;

// use crate::models::Foci;
use crate::models::focus_ring::Foci;
// use crate::models::focus_ring::{
//     DEBUG_VIEW, NEW_TAG, REPO_FIELD, TAG_DISPLAY, TAG_POOL, TEXT_VIEW,
// };
use ring_queue::Ring;
use std::fs::OpenOptions;
use std::io::Write;

use self::cursive::event::{Callback, Event, EventResult, EventTrigger, Key};
use self::cursive::logger;
use self::cursive::logger::{log, Record};
use self::cursive::traits::*;
use self::cursive::Cursive;
use self::cursive::{
    theme::{Color, ColorStyle},
    view::ViewWrapper,
    views::{
        BoxView, CircularFocus, DebugView, EditView, IdView, LinearLayout,
        OnEventView, Panel, SelectView, TextContent, TextView, ViewRef,
    },
};
use self::cursive::{Printer, XY};
use crate::models::errors::Result as WeirdResult;
use crate::models::{
    light_table::{LightTable, RcVecRepo, RcVecRepoTag},
    save_repos_and_tags, GitGlobalConfig, GitGlobalResult, Repo, RepoTag,
};
use itertools::Itertools;
use std::borrow::BorrowMut;
use std::cell::Ref;

// use std::vec::IntoIter;
use std::iter::{IntoIterator, Iterator};

fn fetch_all_tags<'a>(light_table: &'a mut LightTable) -> &mut Vec<RepoTag> {
    let _current_repo: usize = light_table.repo_index;
    let current_repo: &mut Repo = light_table
        .repos
        .get_mut(_current_repo)
        .expect("ERROR - repo index out of bounds");
    &mut current_repo.tags
}

pub fn repo_2_name<'a>(s: &'a str) -> &'a str {
    s.rsplit("/").collect::<Vec<&str>>().first().unwrap()
}

pub fn go<'a>(path_filter: Option<String>) -> WeirdResult<GitGlobalResult> {
    // logger::init();

    let DEBUG_VIEW: String = String::from("debug-view");
    let TEXT_VIEW: String = String::from("text-view");
    let REPO_FIELD: String = String::from("repo-field");
    let TAG_DISPLAY: String = String::from("tag-display");
    let TAG_POOL: String = String::from("tag-pool");
    let NEW_TAG: String = String::from("new-tag");

    let mut focus_ring: Ring<String> = ring![
        REPO_FIELD.clone(),
        TAG_DISPLAY.clone(),
        TAG_POOL.clone(),
        NEW_TAG.clone()
    ];
    // let mut focus_ring: Ring<&str> =
    //     ring![REPO_FIELD, TAG_DISPLAY, TAG_POOL, NEW_TAG];
    let foci: Foci = Foci::new(focus_ring);
    let foci1 = Rc::new(foci);
    let foci2 = Rc::clone(&foci1);
    let foci3 = Rc::clone(&foci1);

    let gc = GitGlobalConfig::new();
    let global_table = LightTable::new_from_ggc(gc, path_filter);
    let mut _g = (*global_table).borrow_mut();
    _g.reset_all_tags();
    drop(_g);
    let edit_ref = Rc::clone(&global_table);
    let repo_ref = Rc::clone(&global_table);
    let repo_tag_ref = Rc::clone(&global_table);
    let all_tags_ref = Rc::clone(&global_table);
    let final_ref = Rc::clone(&global_table);
    // =================================================
    //  TAKE 2
    // =================================================
    trace!("go");

    let mut siv = Cursive::default();
    let _main_theme = siv.load_theme_file("assets/style.toml").unwrap();

    // https://github.com/gyscos/Cursive/issues/179

    // VIEWS
    let error_view_inner = DebugView::new();
    let error_view_id = error_view_inner.with_id(DEBUG_VIEW);
    let error_view = error_view_id.max_height(20);

    let text_view_inner = TextView::new("Begin...");
    let text_view_id = text_view_inner.with_id("text-view");
    let text_view = text_view_id;

    // =================================================
    //  NEW TAG EDITOR
    // =================================================
    let td = TAG_DISPLAY.clone();
    let tp = TAG_POOL.clone();
    let nt = NEW_TAG.clone();
    let mut new_tag_inner: EditView =
        EditView::new().on_submit_mut(move |s, new_text| {
            let mut light_table = (*edit_ref).borrow_mut();
            let repo_tags = fetch_all_tags(&mut light_table);
            let new_tag = RepoTag::new(new_text);
            if light_table.add_tag(&new_tag) {
                let mut dd: ViewRef<SelectView<usize>> =
                    s.find_id(&td).unwrap();
                &dd.clear();
                &dd.add_all(light_table.selectify_tags(light_table.repo_index));

                let mut ee: ViewRef<SelectView<usize>> =
                    s.find_id(&tp).unwrap();
                &ee.clear();
                // &dd.add_all(light_table.selectify_tags(_current_repo));
                &ee.add_all(light_table.retags());

                let mut s_view: ViewRef<EditView> =
                    s.find_id(&nt).expect("Could not find view");
                // let content = (*s_view).get_content().clone();
                s_view.set_content("");
            }
        });
    let new_tag_style = new_tag_inner.style(ColorStyle::new(
        Color::parse("#FFF").unwrap(),
        Color::parse("#444").unwrap(),
    ));
    let nt = NEW_TAG.clone();
    let new_tag_id = (new_tag_style).with_id(&nt);
    let new_tag = new_tag_id.max_height(10);

    // =================================================
    //  REPO SELECTOR
    // =================================================
    let rp = REPO_FIELD.clone();
    let td = TAG_DISPLAY.clone();
    let repo_selector_inner: SelectView<usize> = SelectView::new()
        .with_all((*Rc::clone(&repo_ref)).borrow().selectify_repos())
        .on_select(move |s: &mut Cursive, ss: &usize| {
            (*repo_ref).borrow_mut().repo_index = *ss;

            let mut dd: ViewRef<SelectView<usize>> = s.find_id(&td).unwrap();
            &dd.clear();
            &dd.add_all((*repo_ref).borrow().selectify_tags(*ss));

            // let mut tt: ViewRef<TextView> = s.find_id("text-view").unwrap();
            // let mut content = String::new();
            let mut _light_table = (*repo_ref).borrow_mut();
            _light_table.retags();
            // let _content =
            //     vec![format!("Current Tags:\n{:#?}", _light_table.tags)]
            //         .iter()
            //         .for_each(|s| content.push_str(s));
            // &tt.set_content(content);
        });
    // trait Arrow {}
    // impl Arrow
    let repo_selector_id = repo_selector_inner.with_id(&rp);
    let repo_selector_tmp = foci1.make_event_layer(
        &mut siv,
        vec![Event::Key(Key::Left), Event::Key(Key::Right)],
        // EventTrigger::arrows(),
        // EventTrigger::from(Key::Right).or(Key::Left),
        // Key::Right,
        repo_selector_id,
    );
    let repo_selector =
        repo_selector_tmp.scrollable().min_width(20).max_height(10);
    // repo_selector_id.scrollable().min_width(20).max_height(10);
    // drop(repo_selector_tmp);
    // =================================================
    //  TAGS DISPLAYER
    // =================================================
    let rr = Rc::clone(&repo_tag_ref);
    let rf = REPO_FIELD.clone();
    let td = TAG_DISPLAY.clone();
    let tp = TAG_POOL.clone();
    let tags_displayer_inner: SelectView<usize> = SelectView::new().with_all({
        let _current_repo = (*rr).borrow().repo_index;
        (*rr).borrow().selectify_tags(_current_repo)
    });
    let tags_displayer_id = tags_displayer_inner.with_id(&td);
    let tags_displayer_tmp = foci2.make_event_layer(
        &mut siv,
        vec![Event::Key(Key::Left), Event::Key(Key::Right)],
        // EventTrigger::arrows(),
        // EventTrigger::from(Key::Right).or(Key::Left),
        // Key::Right,
        tags_displayer_id,
    );
    let tags_displayer_outer = tags_displayer_tmp.min_width(20).max_height(10);
    let tags_displayer = OnEventView::new(tags_displayer_outer)
        .on_event(Event::Key(Key::Esc), |s| {
            s.focus_id("repo-field").expect("...")
        })
        .on_event(Event::Key(Key::Backspace), move |s| {
            // note: we can find our own view here but maybe because we are wrapped in an `OnEventView`
            let mut this: ViewRef<SelectView<usize>> = s.find_id(&td).unwrap();
            let i: usize = this.selected_id().expect("Couldnt get selected id");
            let deleted_tag: String = String::from(
                (*this)
                    .get_item_mut(i)
                    .expect("Could not get Tag.name to be deleted")
                    .0
                    .source(),
            );
            (*this).remove_item(i);
            // NOTE: Do I need to reindex either list here?

            // let _light_table = (*repo_tag_ref).borrow_mut();
            let _light_table: &mut LightTable = &mut (*repo_tag_ref)
                .try_borrow_mut()
                .expect("Mut Borrow 3 failed");

            let _current_repo: usize = _light_table.repo_index;
            let current_repo: &mut Repo = _light_table
                .repos
                .get_mut(_current_repo)
                .expect("ERROR - repo index out of bounds");
            let _current_tags: &mut Vec<RepoTag> = &mut (current_repo).tags;
            let _current_tag_index = _current_tags
                .iter()
                .position(|rt| rt.name == deleted_tag)
                .expect("did not find the index of the current tag");
            _current_tags.remove(_current_tag_index);

            // UPDATE ALL TAGS
            let mut all_tag_view: ViewRef<SelectView<usize>> =
                s.find_id(&tp).unwrap();
            (*all_tag_view).clear();
            (*all_tag_view).add_all(_light_table.retags());

            // LOG STUFF
            // let mut tt: ViewRef<TextView> = s.find_id("text-view").unwrap();
            // let mut content = String::new();
            // let _content = vec![
            //     format!("Deleted Tag:\n{:#?}", deleted_tag),
            //     // format!("Current Tag:\n{:#?}", _current_tag),
            //     format!("Current Tags:\n{:#?}", _light_table.tags),
            // ]
            // .iter()
            // .for_each(|s| content.push_str(s));
            // &tt.set_content(content);
        });

    // =================================================
    //  TAGS POOL
    // =================================================
    let td = TAG_DISPLAY.clone();
    let tags_pool_inner: SelectView<usize> = SelectView::new()
        // .with_all(selectify_strings(&ct))
        // .with_all((*Rc::clone(&repo_ref)).borrow().selectify_repos())
        .with_all(
            (*Rc::clone(&all_tags_ref))
                .try_borrow_mut()
                .expect("tags pool initial borrow failed")
                .retags(),
        )
        .on_submit(move |s: &mut Cursive, ss: &usize| {
            let _light_table: &mut LightTable = &mut (*all_tags_ref)
                .try_borrow_mut()
                .expect("Mut Borrow 3 failed");
            debug!("**** - _light_table index {:#?}", _light_table);

            let _current_repo: usize = _light_table.repo_index;
            debug!("**** - current repo index {}", _current_repo);

            let current_repo: &mut Repo = _light_table
                .repos
                .get_mut(_current_repo)
                .expect("ERROR - repo index out of bounds");

            debug!("**** - current repo {:#?}", current_repo);

            let _current_tags: &Vec<RepoTag> = &_light_table.tags;
            let _current_tag: &RepoTag = _current_tags.get(*ss).expect(
                "Couldnt get current tag from current repos tags vector",
            );

            // note: Cant get our own view without a panic...
            // this is either because the id view is defined later in the file or we have to call from an `OnEventView` wrapper
            // let mut this: ViewRef<SelectView<usize>> =
            //     s.find_id("tag_pool").expect("Could not find myself");
            // let _current_tag_name = this.get_item(*ss).unwrap().0;
            // let _current_tag = RepoTag::new(_current_tag_name);

            debug!("**** - current tag  index {}", *ss);
            // debug!("**** - current tag {:#?}", _current_tag);

            if current_repo.tags.contains(&_current_tag) {
                return;
            }

            current_repo.tags.push(_current_tag.clone());

            let mut dd: ViewRef<SelectView<usize>> = s.find_id(&td).unwrap();
            &dd.clear();
            &dd.add_all(_light_table.selectify_tags(_current_repo));

            // let mut tt: ViewRef<TextView> = s.find_id("text-view").unwrap();
            // let mut content = String::new();
            // // we need to get rid of previous immutable ref to lighttable to call retagss
            // let tmp_tag = _current_tag.clone();
            // drop(_current_tag);
            // _light_table.retags();
            // let _content = vec![
            //     format!("Current Tag:\n{:#?}", tmp_tag),
            //     // format!("Current Tag:\n{:#?}", _current_tag),
            //     format!("Current Tags:\n{:#?}", _light_table.tags),
            // ]
            // .iter()
            // .for_each(|s| content.push_str(s));
            // &tt.set_content(content);
        });
    // .on_submit(move |s: &mut Cursive, ss: &RepoTag| {
    //     // (*tp_repo).replace_with(|rt| {
    //     //     rt.tags.push(ss.clone());
    //     //     rt.clone()
    //     // });
    //     (*tp_repo).borrow_mut().tags.push(ss.clone());
    //     (*tp_tags).borrow_mut().push(ss.clone());
    //     // (*tp_tags).replace_with(|rt| {
    //     //     rt.push(ss.clone());
    //     //     rt.clone()
    //     // });
    //     println!("And we print");
    //     let _out: Vec<String> = (*tp_tags)
    //         .borrow()
    //         .iter()
    //         .cloned()
    //         .map(String::from)
    //         .collect();
    //     let _form: String =
    //         format!("\nAdd tag to repo {}:\n", &tp_repo.borrow().path);
    //     debug_write_file(
    //         // vec![String::from("add Repo: "), r.path, "\n"],
    //         vec![_form]
    //             // vec!["We printed this", "and this"]
    //             .into_iter()
    //             .map(String::from)
    //             .chain(_out)
    //             .collect(),
    //         "tmp_out",
    //     );
    //     updated_display_tags(s, &((*tp_repo).borrow().deref()))
    // });
    let tags_pool_id = tags_pool_inner.with_id(&(&TAG_POOL).clone());
    let tags_pool_tmp = foci3.make_event_layer(
        &mut siv,
        vec![Event::Key(Key::Left), Event::Key(Key::Right)],
        // EventTrigger::arrows(),
        // EventTrigger::from(Key::Right).or(Key::Left),
        // Key::Right,
        tags_pool_id,
    );
    let tags_pool = tags_pool_tmp
        .on_event_inner(Event::Key(Key::Esc), |s1, k| {
            let cb = Callback::from_fn(|siv: &mut Cursive| {
                siv.focus_id("repo-field")
                    .expect("failed to focus on 'repo-field'");
            });
            return Some(EventResult::Consumed(Some(cb)));
        })
        .scrollable();

    let top_layout = LinearLayout::horizontal()
        .child(Panel::new(repo_selector))
        .child(Panel::new(tags_displayer));
    // .child(Panel::new(tags_displayer)),
    //

    // let rp = REPO_FIELD.clone().as_str();
    let first_layer = LinearLayout::vertical()
        .child(top_layout)
        .child(
            // sel_view
            Panel::new(
                tags_pool
                // OnEventView::new(tags_pool)
                //     .on_event_inner(Event::Key(Key::Esc), |s1, k| {
                //         let cb = Callback::from_fn(|siv: &mut Cursive| {
                //             siv.focus_id("repo-field")
                //                 .expect("failed to focus on 'repo-field'");
                //         });
                //         return Some(EventResult::Consumed(Some(cb)));
                //     })
                //     // NOTE: Due to fucking annoying design this has to come
                //     // after/outside `OnEventView` - otherwise we never get to unwrap
                //     // properly
                //     .scrollable(),
            ),
        )
        .child(Panel::new(new_tag))
        // .child(Panel::new(error_view))
        .child(Panel::new(text_view));

    // Main Window
    siv.add_layer(first_layer);
    // siv.add_layer(CircularFocus::new(first_layer, false, true));
    // #[rock]
    siv.add_global_callback('q', move |s1| {
        // s1.quit();
        trace!("agg1");
        // let more_reps = rreps.clone();
        // let more_tags = config_tags.clone();
        let lighttable = (*final_ref)
            .try_borrow_mut()
            .expect("final lighttable failed");
        let more_reps = lighttable.repos.clone();
        let more_tags = lighttable.tags.clone();
        // save_repos_and_quit(s1, more_reps, more_tags);
        save_repos_and_quit(s1, more_reps, more_tags);
        trace!("agg2");
    });
    // debug_buffer.append_elements(["hey"]);
    siv.run();
    debug!("ADD TAGS: called - 33");

    // fn updated_display_tags(siv: &mut Cursive, r: &Repo) {
    //     println!("updated_display_tags called");
    //     // debug_write_file(
    //     //     // vec![String::from("add Repo: "), r.path, "\n"],
    //     //     vec!["add Repo: ", &r.path, "\n"]
    //     //         // vec!["And then i went to", "the beach"]
    //     //         .into_iter()
    //     //         .map(String::from)
    //     //         .collect(),
    //     //     "tmp_out",
    //     // );

    //     let mut dd: ViewRef<SelectView<RepoTag>> =
    //         siv.find_id(TAG_DISPLAY).unwrap();
    //     &dd.clear();
    //     // &dd.add_all(selectify_rc_things(&rs_tags, |t| (t.name.clone(), t)));
    //     // let t_tags: Vec<RepoTag> = r.get_tags();
    //     let tags =
    //         selectify_things_two(r.tags.clone(), |t| (t.name.clone(), t));
    //     // let tags = &r
    //     //     .get_tags()
    //     //     .into_iter()
    //     //     .map(String::from)
    //     //     .collect::<Vec<String>>();
    //     &dd.add_all(tags);
    // }

    // fn toggle_bg(sel: SelectView) {
    //     let xy = XY { x: 0, y: 0 };
    //     let printer = Printer {
    //         offset: Vec2::new(0, 0),
    //         content_offset: Vec2::new(0, 0),
    //         enabled: true,
    //         output_size: sel.required_size(xy),
    //         focused: false,
    //         size: sel.required_size(xy),
    //         theme: &main_theme,
    //     };
    //     sel.draw(printer.with_color(ColorStyle::primary, || printer.print()));
    // }

    /// Final behaviour - for some reason this only works inside this block
    fn save_repos_and_quit(
        s: &mut Cursive,
        reps: Vec<Repo>,
        tags: Vec<RepoTag>,
    ) {
        save_repos_and_tags(reps, tags);
        s.cb_sink()
            .send(Box::new(|siv: &mut Cursive| siv.quit()))
            .expect("thread send failed");
    }

    Ok(GitGlobalResult::new(&vec![]))
}

fn debug_write_file(messages: Vec<String>, log_file: &str) {
    let strs_join: String = messages.as_slice().join("\n");
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("tmp_out")
        .unwrap()
        .write_all(strs_join.as_ref());
}
