#![feature(trait_alias)]

use std;

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

use self::cursive::event::{Callback, Event, EventResult, Key};

use self::cursive::traits::*;
use self::cursive::Cursive;
use self::cursive::{
    theme::{Color, ColorStyle},
    view::ViewWrapper,
    views::{
        CircularFocus, DebugView, EditView, LinearLayout, NamedView,
        OnEventView, Panel, ResizedView, SelectView, TextContent, TextView,
        ViewRef,
    },
};

use crate::cursive::CursiveExt;
use crate::models::errors::Result as WeirdResult;

use crate::models::{
    config::GitGlobalConfig, light_table::LightTable, repo::Repo,
    repo_tag::RepoTag, result::GitGlobalResult,
};
use itertools::Itertools;
use std::borrow::BorrowMut;

// use std::vec::IntoIter;
use std::iter::Iterator;

fn fetch_all_tags<'a>(light_table: &'a mut LightTable) -> &'a mut Vec<RepoTag> {
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
    let _TEXT_VIEW: String = String::from("text-view");
    let REPO_FIELD: String = String::from("repo-field");
    let TAG_DISPLAY: String = String::from("tag-display");
    let TAG_POOL: String = String::from("tag-pool");
    let NEW_TAG: String = String::from("new-tag");
    let FILE_FILTER: String = String::from("file-filter");

    let focus_ring: Ring<String> = ring![
        REPO_FIELD.clone(),
        TAG_DISPLAY.clone(),
        TAG_POOL.clone(),
        NEW_TAG.clone(),
        FILE_FILTER.clone()
    ];
    let foci: Foci = Foci::new(focus_ring);
    let foci1 = Rc::new(foci);
    let foci2 = Rc::clone(&foci1);
    let foci3 = Rc::clone(&foci1);
    let foci4 = Rc::clone(&foci1);

    let gc = GitGlobalConfig::new();
    let global_table = LightTable::new_from_ggc(gc, path_filter);
    let mut _g = (*global_table).borrow_mut();
    _g.reset_all_tags();
    drop(_g);
    let edit_ref = Rc::clone(&global_table);
    let repo_ref = Rc::clone(&global_table);
    let repo_tag_ref = Rc::clone(&global_table);
    let all_tags_ref = Rc::clone(&global_table);
    let repo_filter_ref = Rc::clone(&global_table);
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
    let error_view_id = error_view_inner.with_name(DEBUG_VIEW);
    let _error_view = error_view_id.max_height(20);

    let text_view_inner = TextView::new("Begin...");
    let text_view_id = text_view_inner.with_name("text-view");
    let text_view = text_view_id;

    // =================================================
    //  NEW TAG EDITOR
    // - Box where we can add new tags
    // =================================================
    let td = TAG_DISPLAY.clone();
    let tp = TAG_POOL.clone();
    let nt = NEW_TAG.clone();
    let new_tag_inner: EditView =
        EditView::new().on_submit_mut(move |s, new_text| {
            let mut light_table = (*edit_ref).borrow_mut();
            let _repo_tags = fetch_all_tags(&mut light_table);
            let new_tag = RepoTag::new(new_text);
            if light_table.add_tag(&new_tag) {
                let mut dd: ViewRef<SelectView<usize>> =
                    s.find_name(&td).unwrap();
                &dd.clear();
                &dd.add_all(light_table.selectify_tags(light_table.repo_index));

                let mut ee: ViewRef<SelectView<usize>> =
                    s.find_name(&tp).unwrap();
                &ee.clear();
                &ee.add_all(light_table.retags());

                let mut s_view: ViewRef<EditView> =
                    s.find_name(&nt).expect("Could not find view");
                s_view.set_content("");
            }
        });
    let new_tag_style = new_tag_inner.style(ColorStyle::new(
        Color::parse("#FFF").unwrap(),
        Color::parse("#444").unwrap(),
    ));
    let nt = NEW_TAG.clone();
    let new_tag_id = (new_tag_style).with_name(&nt);
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

            let mut dd: ViewRef<SelectView<usize>> = s.find_name(&td).unwrap();
            &dd.clear();
            &dd.add_all((*repo_ref).borrow().selectify_tags(*ss));

            let mut _light_table = (*repo_ref).borrow_mut();
            _light_table.retags();
        });
    let repo_selector_id = repo_selector_inner.with_name(&rp);
    let repo_selector_tmp = foci1.make_event_layer(
        &mut siv,
        vec![Event::Key(Key::Left), Event::Key(Key::Right)],
        repo_selector_id,
    );
    let repo_selector =
        repo_selector_tmp.scrollable().min_width(20).max_height(10);

    // =================================================
    //  TAGS DISPLAYER
    // - Shows the tags associated with the currently selected file
    // =================================================
    let rr = Rc::clone(&repo_tag_ref);
    let _rf = REPO_FIELD.clone();
    let td = TAG_DISPLAY.clone();
    let tp = TAG_POOL.clone();
    let tags_displayer_inner: SelectView<usize> = SelectView::new().with_all({
        let _current_repo = (*rr).borrow().repo_index;
        (*rr).borrow().selectify_tags(_current_repo)
    });
    let tags_displayer_id = tags_displayer_inner.with_name(&td);
    let tags_displayer_tmp = foci2.make_event_layer(
        &mut siv,
        vec![Event::Key(Key::Left), Event::Key(Key::Right)],
        tags_displayer_id,
    );
    let tags_displayer_outer = tags_displayer_tmp.min_width(20).max_height(10);
    let tags_displayer = OnEventView::new(tags_displayer_outer)
        .on_event(Event::Key(Key::Esc), |s| {
            s.focus_name("repo-field").expect("...")
        })
        .on_event(Event::Key(Key::Backspace), move |s| {
            // note: we can find our own view here but maybe because we are wrapped in an `OnEventView`
            let mut this: ViewRef<SelectView<usize>> =
                s.find_name(&td).unwrap();
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

            let _light_table: &mut LightTable = &mut (*repo_tag_ref)
                .try_borrow_mut()
                .expect("Mut Borrow 3 failed");

            let _current_repo: usize = _light_table.repo_index;
            let current_repo: &mut Repo = _light_table
                // .repos
                .filtered_repos
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
                s.find_name(&tp).unwrap();
            (*all_tag_view).clear();
            (*all_tag_view).add_all(_light_table.retags());

            // LOG STUFF
            // let mut tt: ViewRef<TextView> = s.find_name("text-view").unwrap();
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
    // - Shows the "global tag pool" of all tags associated
    // with all repos
    // =================================================
    let td = TAG_DISPLAY.clone();
    let tags_pool_inner: SelectView<usize> = SelectView::new()
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
                // .repos
                .filtered_repos
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
            //     s.find_name("tag_pool").expect("Could not find myself");
            // let _current_tag_name = this.get_item(*ss).unwrap().0;
            // let _current_tag = RepoTag::new(_current_tag_name);

            debug!("**** - current tag  index {}", *ss);
            // debug!("**** - current tag {:#?}", _current_tag);

            if current_repo.tags.contains(&_current_tag) {
                return;
            }

            current_repo.tags.push(_current_tag.clone());

            let mut dd: ViewRef<SelectView<usize>> = s.find_name(&td).unwrap();
            &dd.clear();
            &dd.add_all(_light_table.selectify_tags(_current_repo));
        });

    let tags_pool_id = tags_pool_inner.with_name(&(&TAG_POOL).clone());
    let tags_pool_tmp = foci3.make_event_layer(
        &mut siv,
        vec![Event::Key(Key::Left), Event::Key(Key::Right)],
        tags_pool_id,
    );
    let tags_pool = tags_pool_tmp
        .on_event_inner(Event::Key(Key::Esc), |_s1, _k| {
            let cb = Callback::from_fn(|siv: &mut Cursive| {
                siv.focus_name("repo-field")
                    .expect("failed to focus on 'repo-field'");
            });
            return Some(EventResult::Consumed(Some(cb)));
        })
        .scrollable();

    // =================================================
    // FILE FILTER
    // =================================================
    let rf = REPO_FIELD.clone();
    let file_filter_inner =
        EditView::new().on_submit(move |s: &mut Cursive, ss: &str| {
            let _light_table: &mut LightTable = &mut (*repo_filter_ref)
                .try_borrow_mut()
                .expect("Mut Borrow 4 failed");
            _light_table.repo_filter = ss.to_owned();
            let filtered = _light_table.rerepos();
            let mut repo_box: ViewRef<SelectView<usize>> = s
                .find_name(&rf)
                .expect(&format!("name find failed for {}", &rf));
            repo_box.clear();
            repo_box.add_all(filtered);
        });
    let file_filter_id = file_filter_inner.with_name(FILE_FILTER.clone());
    let file_filter_tmp = foci4.make_event_layer(
        &mut siv,
        vec![Event::Key(Key::Left), Event::Key(Key::Right)],
        file_filter_id,
    );
    let file_filter = file_filter_tmp;

    // =================================================
    // MAIN LAYOUT
    // =================================================

    let top_layout = LinearLayout::horizontal()
        .child(Panel::new(repo_selector).title("Files"))
        .child(Panel::new(tags_displayer).title("File Tags"));

    let first_layer = LinearLayout::vertical()
        .child(top_layout)
        .child(Panel::new(tags_pool).title("Available Tags"))
        .child(Panel::new(new_tag).title("New Tag"))
        .child(Panel::new(file_filter).title("File Filter"))
        // .child(Panel::new(error_view))
        .child(Panel::new(text_view));

    // =================================================
    // MAIN WINDOW
    // =================================================
    siv.add_layer(first_layer);
    // siv.add_layer(CircularFocus::new(first_layer, false, true));
    // #[rock]
    siv.add_global_callback('q', move |s1| {
        trace!("agg1");
        let mut lighttable = (*final_ref)
            .try_borrow_mut()
            .expect("final lighttable failed");
        // NOTE - Merge in any changes from filtered/display list of repos
        lighttable.repo_filter_update();
        let more_reps = lighttable.repos.clone();
        let more_tags = lighttable.tags.clone();
        save_repos_and_quit(s1, more_reps, more_tags);
        trace!("agg2");
    });
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
    //         siv.find_name(TAG_DISPLAY).unwrap();
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
        let mut gc = GitGlobalConfig::new();
        // NOTE: This is bloody stupid - needing to clone repos
        // gc.efficient_repos_update(reps.clone());
        // gc.save_repos_and_tags(reps, tags);
        gc.update_repos_and_tags(reps, tags);
        s.cb_sink()
            .send(Box::new(|siv: &mut Cursive| siv.quit()))
            .expect("thread send failed");
    }

    Ok(GitGlobalResult::new(&vec![]))
}

fn debug_write_file(messages: Vec<String>, _log_file: &str) {
    let strs_join: String = messages.as_slice().join("\n");
    let _file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("tmp_out")
        .unwrap()
        .write_all(strs_join.as_ref());
}
