use std;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
extern crate cursive;

use std::fs::OpenOptions;
use std::io::Write;

use self::cursive::event::{Callback, Event, EventResult, Key};
use self::cursive::logger;
use self::cursive::logger::{log, Record};
use self::cursive::traits::*;
use self::cursive::views::{
    BoxView, DebugView, EditView, IdView, LinearLayout, OnEventView, Panel,
    SelectView, TextContent, TextView, ViewRef,
};
use self::cursive::Cursive;
use self::cursive::{Printer, XY};
use itertools::Itertools;
use repo::errors::Result as WeirdResult;
use repo::{
    save_repos_and_tags, GitGlobalConfig, GitGlobalResult, Repo, RepoTag,
};
use std::borrow::BorrowMut;
use std::cell::Ref;

// use std::vec::IntoIter;
use std::iter::{IntoIterator, Iterator};

type RMut = Rc<RefCell<TextContent>>;
type RcResult = Rc<GitGlobalResult>;
type RcRcResult = Rc<RefCell<GitGlobalResult>>;

// type RcRepo<'a> = Rc<RefCell<&'a Repo>>;
// type RcRepoTag<'a> = Rc<RefCell<&'a RepoTag>>;
// type RcVecRepoTag<'a> = Rc<RefCell<&'a Vec<RepoTag>>>;
// type RcVecRepo<'a> = Rc<RefCell<&'a Vec<Repo>>>;
type RcRef<V> = Rc<RefCell<V>>;
type RcRepo = Rc<RefCell<Repo>>;
type RcRepoTag = Rc<RefCell<RepoTag>>;
type RcVecRepoTag = Rc<RefCell<Vec<RepoTag>>>;
type RcVecRepo = Rc<RefCell<Vec<Repo>>>;

#[derive(PartialEq, Eq, Clone, Debug)]
struct LightTable {
    repos: Vec<Repo>,
    repo_index: usize,
    tag_index: usize,
    tags: Vec<RepoTag>,
}

impl LightTable {
    pub fn new(
        repos: Vec<Repo>,
        repo_index: usize,
        tag_index: usize,
        tags: Vec<RepoTag>,
    ) -> LightTable {
        LightTable {
            repos,
            repo_index,
            tag_index,
            tags,
        }
    }
    pub fn new_from_rc(
        repos: Vec<Repo>,
        repo_index: usize,
        tag_index: usize,
        tags: Vec<RepoTag>,
    ) -> Rc<RefCell<LightTable>> {
        Rc::new(RefCell::new(Self::new(repos, repo_index, tag_index, tags)))
    }

    pub fn selectify_repos(&self) -> Vec<(&str, usize)> {
        self.repos
            .iter()
            .enumerate()
            .map(|(i, r)| (r.path.as_str(), i))
            .collect::<Vec<(&str, usize)>>()
    }

    pub fn selectify_tags(&self, index: usize) -> Vec<(&str, usize)> {
        self.repos
            .iter()
            .nth(index)
            .expect("ERROR - index requested outside of repos bounds")
            .tags
            .iter()
            .enumerate()
            .map(|(i, t)| (t.name.as_str(), i))
            .collect::<Vec<(&str, usize)>>()
    }

    pub fn all_the_tags(&self) -> Vec<(String, usize)> {
        let mut r = self
            .repos
            .iter()
            .flat_map(|r| r.tags.iter().map(|t| t.name.clone()))
            .chain::<Vec<String>>(
                vec!["haskell", "ml", "rust", "apple", "web dev"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
            )
            .unique()
            .enumerate()
            .map(|(i, t)| (t, i))
            .collect::<Vec<(String, usize)>>();
        r.sort();
        r
    }

    pub fn retags(&mut self) -> Vec<(String, usize)> {
        self.reset_all_tags();
        self.tags_as_list()
    }

    pub fn tags_as_list(&self) -> Vec<(String, usize)> {
        self.tags
            .iter()
            .map(|r| r.name.clone())
            .enumerate()
            .map(|(i, t)| (t, i))
            .collect()
    }

    pub fn reset_all_tags(&mut self) {
        let mut _tmp: Vec<(RepoTag)> = self
            .repos
            .iter()
            .flat_map(|r| r.tags.clone())
            .chain::<Vec<RepoTag>>(
                vec!["haskell", "ml", "rust", "apple", "web dev"]
                    .into_iter()
                    .map(RepoTag::new)
                    // .map(String::from)
                    .collect(),
            )
            .unique()
            .collect::<Vec<RepoTag>>();
        _tmp.sort();
        self.tags = _tmp;
    }

    pub fn all_tags(&self) -> Vec<(String, usize)> {
        vec!["haskell", "ml", "rust", "apple", "web dev"]
            .iter()
            .map(|t| RepoTag::new(t))
            .enumerate()
            .map(|(i, t)| (t.name, i))
            .collect::<Vec<(String, usize)>>()
    }
}

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

#[allow(dead_code)]
type SelRepoList<'a> =
    std::iter::Zip<std::vec::IntoIter<&'a str>, std::vec::IntoIter<Repo>>;

#[allow(dead_code)]
type SelRepoList2 = std::iter::Zip<String, Repo>;

type SelTagList<'a> =
    std::iter::Zip<std::vec::IntoIter<&'a str>, std::vec::IntoIter<String>>;

fn selectify_strings<'a>(tags_1: &'a Vec<String>) -> SelTagList<'a> {
    let tags_2: Vec<&'a str> = tags_1.iter().map(AsRef::as_ref).collect();
    return tags_2.into_iter().zip(tags_1.to_vec());
}

fn selectify_rc_tags<'a>(rctags: &'a RcVecRepoTag) -> Vec<String> {
    return rc_borr!(rctags)
        .iter()
        .map(|r| r.name.clone())
        .collect::<Vec<String>>();
}

fn selectify_repos(repos: &RcVecRepo) -> Vec<(String, Repo)> {
    return RefCell::borrow_mut(&repos)
        .clone()
        .into_iter()
        .map(|r| (r.path.clone(), r))
        .collect();
}

/// General selectifier for RC types
fn selectify_rc_things<R>(
    // fn selectify_rc_things<R, T>(
    things: &Rc<RefCell<Vec<R>>>,
    map_fn: impl Fn(R) -> (String, R), // note: This gives a Sized error when used with `dyn` instead of `impl`
) -> Vec<(String, R)>
where
    R: Clone,
    // T: IntoIterator<
    //     Item = (String, R),
    //     // IntoIter = ::std::vec::IntoIter<(String, R)>,
    // >,
{
    return RefCell::borrow_mut(&things)
        .clone()
        .into_iter()
        .map(map_fn)
        // .collect::<T>();
        .collect();
    // let strs: Vec<String> = RefCell::borrow_mut(things.deref())
    //     .iter()
    //     .map(|f| format!("{:?}", f))
    //     .collect();
    // return strs.into_iter().zip(things.into_iter()).collect();
}

fn selectify_rc_things_backwards<R>(
    things: &Rc<RefCell<Vec<R>>>,
    map_fn: impl Fn(R) -> (R, String), // note: This gives a Sized error when used with `dyn` instead of `impl`
) -> Vec<(R, String)>
where
    R: Clone,
{
    return RefCell::borrow_mut(&things)
        .clone()
        .into_iter()
        .map(map_fn)
        .collect();
    // let strs: Vec<String> = RefCell::borrow_mut(things.deref())
    //     .iter()
    //     .map(|f| format!("{:?}", f))
    //     .collect();
    // return strs.into_iter().zip(things.into_iter()).collect();
}

fn selectify_things_two<T>(
    things: Vec<T>,
    map_fn: impl Fn(T) -> (String, T),
) -> Vec<(String, T)>
where
    T: std::fmt::Debug,
{
    // let strs: Vec<String> = things.into_iter().map(map_fn).collect();
    let strs = things.into_iter().map(map_fn).collect();
    // let strs: Vec<String> = things.iter().map(|f| format!("{:?}", f)).collect();
    // return strs.into_iter().zip(things.into_iter()).collect();
    return strs;
}

fn selectify_things<T>(things: Vec<T>) -> Vec<(String, T)>
where
    T: std::fmt::Debug,
{
    let strs: Vec<String> = things.iter().map(|f| format!("{:?}", f)).collect();
    return strs.into_iter().zip(things.into_iter()).collect();
    // return things.into_iter().zip(strs.iter()).collect();

    // return RefCell::borrow_mut(&repos)
    //     .clone()
    //     .into_iter()
    //     .map(|r| (r.path.clone(), r))
    //     // .map(|r| (r.path.clone(), Rc::new(RefCell::new(r))))
    //     .collect();
}

pub fn go<'a>() -> WeirdResult<GitGlobalResult> {
    // logger::init();

    let gc = GitGlobalConfig::new();
    let mut reps: Vec<Repo> = gc.get_cached_repos();
    let mut fake_more_tags: Vec<RepoTag> =
        ["haskell", "ml", "rust", "apple", "web dev"]
            .to_owned()
            .into_iter()
            .map(|&t| RepoTag::new(t))
            .collect();
    let global_table = LightTable::new_from_rc(reps, 0, 0, fake_more_tags);
    let mut _g = (*global_table).borrow_mut();
    _g.reset_all_tags();
    drop(_g);
    let repo_ref = Rc::clone(&global_table);
    let repo_tag_ref = Rc::clone(&global_table);
    let all_tags_ref = Rc::clone(&global_table);
    // =================================================
    //  TAKE 2
    // =================================================
    trace!("go");

    let mut siv = Cursive::default();
    let main_theme = siv.load_theme_file("assets/style.toml").unwrap();

    // https://github.com/gyscos/Cursive/issues/179

    // VIEWS
    let error_view_inner = DebugView::new();
    let error_view_id = error_view_inner.with_id("debug-view");
    let error_view = error_view_id.max_height(20);

    let text_view_inner = TextView::new("Begin...");
    let text_view_id = text_view_inner.with_id("text-view");
    let text_view = text_view_id;

    // =================================================
    //  REPO SELECTOR
    // =================================================
    let repo_selector_inner: SelectView<usize> = SelectView::new()
        .with_all((*Rc::clone(&repo_ref)).borrow().selectify_repos())
        .on_select(move |s: &mut Cursive, ss: &usize| {
            (*repo_ref).borrow_mut().repo_index = *ss;

            let mut dd: ViewRef<SelectView<usize>> =
                s.find_id("tag-display").unwrap();
            &dd.clear();
            &dd.add_all((*repo_ref).borrow().selectify_tags(*ss));

            let mut tt: ViewRef<TextView> = s.find_id("text-view").unwrap();
            let mut content = String::new();
            let mut _light_table = (*repo_ref).borrow_mut();
            _light_table.retags();
            let _content =
                vec![format!("Current Tags:\n{:#?}", _light_table.tags)]
                    .iter()
                    .for_each(|s| content.push_str(s));
            &tt.set_content(content);
        });

    let repo_selector_id = repo_selector_inner.with_id("repo-field");
    let repo_selector =
        repo_selector_id.scrollable().min_width(20).max_height(10);

    // =================================================
    //  TAGS DISPLAYER
    // =================================================
    let rr = Rc::clone(&repo_tag_ref);
    let tags_displayer_inner: SelectView<usize> = SelectView::new().with_all({
        let _current_repo = (*rr).borrow().repo_index;
        (*rr).borrow().selectify_tags(_current_repo)
    });
    let tags_displayer_id = tags_displayer_inner.with_id("tag-display");
    let tags_displayer_outer = tags_displayer_id.min_width(20).max_height(10);
    let tags_displayer = OnEventView::new(tags_displayer_outer)
        .on_event(Event::Key(Key::Esc), |s| {
            s.focus_id("repo-field").expect("...")
        })
        .on_event(Event::Key(Key::Backspace), move |s| {
            // note: we can find our own view here but maybe because we are wrapped in an `OnEventView`
            let mut this: ViewRef<SelectView<usize>> =
                s.find_id("tag-display").unwrap();
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
                s.find_id("tag-pool").unwrap();
            (*all_tag_view).clear();
            (*all_tag_view).add_all(_light_table.retags());

            // LOG STUFF
            let mut tt: ViewRef<TextView> = s.find_id("text-view").unwrap();
            let mut content = String::new();
            let _content = vec![
                format!("Deleted Tag:\n{:#?}", deleted_tag),
                // format!("Current Tag:\n{:#?}", _current_tag),
                format!("Current Tags:\n{:#?}", _light_table.tags),
            ]
            .iter()
            .for_each(|s| content.push_str(s));
            &tt.set_content(content);
        });

    // =================================================
    //  TAGS POOL
    // =================================================
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

            let mut dd: ViewRef<SelectView<usize>> =
                s.find_id("tag-display").unwrap();
            &dd.clear();
            &dd.add_all(_light_table.selectify_tags(_current_repo));

            let mut tt: ViewRef<TextView> = s.find_id("text-view").unwrap();
            let mut content = String::new();
            // we need to get rid of previous immutable ref to lighttable to call retagss
            let tmp_tag = _current_tag.clone();
            drop(_current_tag);
            _light_table.retags();
            let _content = vec![
                format!("Current Tag:\n{:#?}", tmp_tag),
                // format!("Current Tag:\n{:#?}", _current_tag),
                format!("Current Tags:\n{:#?}", _light_table.tags),
            ]
            .iter()
            .for_each(|s| content.push_str(s));
            &tt.set_content(content);
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
    let tags_pool = tags_pool_inner.with_id("tag-pool");

    // Main Window
    siv.add_layer(
        LinearLayout::vertical()
            .child(
                LinearLayout::horizontal()
                    .child(Panel::new(repo_selector))
                    .child(Panel::new(tags_displayer)),
                // .child(Panel::new(tags_displayer)),
            )
            .child(
                // sel_view
                Panel::new(
                    OnEventView::new(tags_pool)
                        .on_event_inner(Event::Key(Key::Esc), |s1, k| {
                            let cb = Callback::from_fn(|siv: &mut Cursive| {
                                siv.focus_id("repo-field")
                                    .expect("failed to focus on 'repo-field'");
                            });
                            return Some(EventResult::Consumed(Some(cb)));
                        })
                        // NOTE: Due to fucking annoying design this has to come
                        // after/outside `OnEventView` - otherwise we never get to unwrap
                        // properly
                        .scrollable(),
                ),
            )
            .child(Panel::new(error_view))
            .child(Panel::new(text_view)),
    );
    // #[rock]
    siv.add_global_callback('q', move |s1| {
        s1.quit();
        trace!("agg1");
        // let more_reps = rreps.clone();
        // let more_tags = config_tags.clone();
        // save_repos_and_quit(s1, more_reps, more_tags);
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
    //         siv.find_id("tag-display").unwrap();
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
        reps: RcVecRepo,
        tags: RcVecRepoTag,
    ) {
        // fn save_repos_and_quit(s: &mut Cursive, reps: RcVecRepo, tags: RcVecRepoTag, repsmo: *const Vec<Repo>) {

        trace!("srq1: {}", Rc::strong_count(&reps));
        // let ireps = Rc::try_unwrap(reps).expect("we have the repos");
        // let itags = Rc::try_unwrap(tags).expect("we have the tags");
        trace!("srq2");

        // let tmp = &ireps.clone();
        trace!("srq3");
        let irepst = RefCell::borrow(&reps);
        let ireps = irepst.deref();
        let itagst = RefCell::borrow(&tags);
        // borrow(&tags);
        let itags = itagst.deref();
        // save_repos_and_tags(ireps.into_inner(), itags.into_inner());
        save_repos_and_tags(ireps.clone(), itags.clone());

        // s.quit();
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
