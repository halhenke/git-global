use std;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
extern crate cursive;

use std::fs::OpenOptions;
use std::io::Write;

use self::cursive::event::{Callback, Event, EventResult, Key};
use self::cursive::traits::*;
use self::cursive::views::{
    DebugView, TextView,
    BoxView, EditView, IdView, LinearLayout, OnEventView, Panel, SelectView,
    TextContent, ViewRef,
};
use self::cursive::Cursive;
use self::cursive::{Printer, XY};
use self::cursive::logger;
use self::cursive::logger::{Record, log};
use repo::errors::Result as WeirdResult;
use repo::{
    save_repos_and_tags, GitGlobalConfig, GitGlobalResult, Repo, RepoTag,
};
use std::cell::Ref;
use std::borrow::BorrowMut;

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

    // pub fn all_tags(&self) -> Vec<(RepoTag, usize)> {
    // pub fn all_tags(&self) -> Vec<(&str, usize)> {
    pub fn all_tags(&self) -> Vec<(String, usize)> {
        // let fake_more_tags =
        // let fake_more_tags: Vec<(usize, RepoTag)> =
        // let fake_more_tags: Vec<RepoTag> =
        vec!["haskell", "ml", "rust", "apple", "web dev"]
            // .to_owned()
            // .into_iter()
            .iter()
            .map(|t| RepoTag::new(t))
            // .into_iter()
            //         .collect();
            // let fuck_more_tags = fake_more_tags.clone();
            // return fuck_more_tags
            //     // .iter()
            //     .into_iter()
            //     // .clone()
            //     // .to_owned()
            .enumerate()
            // return fake_more_tags;
            // .map(move |(i, t)| (t.name.as_str(), i))
            // // .map(|(i, t)| (t.name.as_str(), i))
            // .collect::<Vec<(&str, usize)>>();
            //     .collect();
            // let fs: Vec<(&str, usize)> = fake_more_tags
            // .to_owned()
            // .iter()
            .map(|(i, t)| (t.name, i))
            // .map(|(i, t)| (t.name.as_str(), i))
            .collect::<Vec<(String, usize)>>()
        // .for_each(|(i, t)| (t.name.as_str(), i))
        // .map(|(i, t)| (t, i))
        // // .clone()
        // .collect::<Vec<(RepoTag, usize)>>()
        // .collect::<Vec<(RepoTag, usize)>>()
        // .iter()
        // // .enumerate()
        // .map(|(t, i)| (t.name.as_str(), *i))
        // .collect::<Vec<(&str, usize)>>()
        // return fs.to_owned();
    }
}

/// Not sure if I use this here
#[derive(PartialEq, Eq, Clone, Debug)]
struct TagStatus {
    // struct TagStatus<'a> {
    repos: Vec<Repo>,
    tags: Vec<RepoTag>,
    current_repo: Repo,
    // current_repo: &'a Repo,
    repo_tags: Vec<RepoTag>,
}

impl TagStatus {
    pub fn new_from_rc(
        repos: Vec<Repo>,
        tags: Vec<RepoTag>,
        current_repo: Repo,
        // current_repo: i32,
        repo_tags: Vec<RepoTag>,
    ) -> TagStatus {
        TagStatus {
            repos: repos,
            tags,
            current_repo,
            // current_repo: &repos[current_repo],
            repo_tags,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct TagStatusRC {
    // struct TagStatusRC {
    repos: RcVecRepo,
    tags: RcVecRepoTag,
    current_repo: RcRepo,
    // current_repo: &'a Repo,
    repo_tags: RcVecRepoTag,
}

impl TagStatusRC {
    pub fn new_from_rc(
        repos: RcVecRepo,
        tags: RcVecRepoTag,
        current_repo: RcRepo,
        // current_repo: &'a Repo,
        repo_tags: RcVecRepoTag,
    ) -> TagStatusRC {
        TagStatusRC {
            repos: repos,
            tags,
            current_repo,
            // current_repo: &repos[current_repo],
            repo_tags,
        }
    }
}
// impl TagStatusRC<'_> {
//     pub fn new_from_rc<'a>(
//         repos: RcVecRepo<'a>,
//         tags: RcVecRepoTag<'a>,
//         current_repo: RcRepo<'a>,
//         // current_repo: &'a Repo,
//         repo_tags: RcVecRepoTag<'a>,
//     ) -> TagStatusRC<'a> {
//         TagStatusRC {
//             repos: repos,
//             tags,
//             current_repo,
//             // current_repo: &repos[current_repo],
//             repo_tags,
//         }
//     }
// }

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
        // return rctags
        //     .deref()
        //     .borrow_mut()
        .iter()
        .map(|r| r.name.clone())
        .collect::<Vec<String>>();
}

// type SelRepIter<'a> = &'a Vec<(String, RcRepo)>;

fn selectify_repos(repos: &RcVecRepo) -> Vec<(String, Repo)> {
    return RefCell::borrow_mut(&repos)
        .clone()
        .into_iter()
        .map(|r| (r.path.clone(), r))
        // .map(|r| (r.path.clone(), Rc::new(RefCell::new(r))))
        .collect();
}

/// General selectifier for RC types
fn selectify_rc_things<R>(
    // fn selectify_rc_things<R, T>(
    things: &Rc<RefCell<Vec<R>>>,
    map_fn: impl Fn(R) -> (String, R), // note: This gives a Sized error when used with `dyn` instead of `impl`
                                       // ) -> T
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
    let repo_ref = Rc::clone(&global_table);
    let repo_tag_ref = Rc::clone(&global_table);
    let all_tags_ref = Rc::clone(&global_table);
    // =================================================
    //  TAKE 2
    // =================================================
    println!("Fuck");
    debug!("Fuck");
    ic!("Fuck");
    // note a pointer
    // let debug_buffer = vec![];
    let uc = GitGlobalConfig::new();
    let mut repos: Vec<Repo> = uc.get_cached_repos();
    let results = uc.get_cached_results();
    println!("Fuck 2");
    let existing_tags: Vec<RepoTag> =
        results.all_tags().into_iter().cloned().collect();
    // let fake_tags: dyn IntoIterator<
    //     Item = &str,
    //     IntoIter = std::vec::Vec<String>,
    // > = ["haskell", "ml", "rust"].to_owned().into_iter();
    println!("Fuck 3");
    let safe_fake_tags: Vec<String> = vec!["haskell", "ml", "rust"]
        .iter()
        .map(|&s| String::from(s))
        // .map(|s| String::from(*s))
        .collect();
    let fake_tags = ["haskell", "ml", "rust"]
        .to_owned()
        .into_iter()
        .map(|&t| RepoTag::new(t))
        .collect();
    let fake_more_tags: Vec<RepoTag> =
        ["haskell", "ml", "rust", "apple", "web dev"]
            .to_owned()
            .into_iter()
            .map(|&t| RepoTag::new(t))
            .collect();
    ic!(4);
    // .map(RefCell::new) // note: theres no need for mutable tags
    // .map(Rc::new)
    // .collect();
    let all_tags = {
        if existing_tags.is_empty() {
            fake_tags
        } else {
            existing_tags
        }
    };
    ic!(5);
    debug!("Before initial repo");
    let initial_repo = repos[0].clone();
    debug!("After initial repo");
    let initial_tags = initial_repo.tags.clone();
    // let mut globals =
    //     TagStatus::new_from_rc(repos, all_tags, initial_repo, initial_tags);
    // let mut_globals = Rc::new(RefCell::new(globals));
    ic!(6);

    let globals_rc = TagStatusRC::new_from_rc(
        Rc::new(RefCell::new(repos)),
        Rc::new(RefCell::new(all_tags)),
        Rc::new(RefCell::new(initial_repo)),
        Rc::new(RefCell::new(initial_tags)),
        // rcref!(repos),
        // rcref!(all_tags),
        // rcref!(initial_repo),
        // rcref!(initial_tags),
    );

    // let rct = reps.clone();
    // let repo_names = &rct.iter().map(|x| x.path.clone()).zip(rct.iter());
    // let mut cur2 = reps.as_mut_ptr();
    // let mut cur3 = reps.as_mut_ptr();
    // let mut rcur = Rc::new(RefCell::new(cur2));

    // let rreps = Rc::new(RefCell::new(reps));
    // let ttags = Rc::new(RefCell::new(result_tags));
    // NOTE: This is just until we have some actually tagged repos
    // let config_tags = Rc::new(RefCell::new(uc.tags));

    trace!("go");

    let mut siv = Cursive::default();
    let main_theme = siv.load_theme_file("assets/style.toml").unwrap();

    // https://github.com/gyscos/Cursive/issues/179

    // debug!("ADD TAGS: did we get here - 3");
    let mut new_tags: Vec<String> = Vec::new();

    // VIEWS
    let e_view = EditView::new().with_id("tag").fixed_width(20);
    // let repo_selector: SelectView<RcRepo> = SelectView::new();
    // let rreps_1 = Rc::clone(&rreps);
    // let mut rcur2 = Rc::clone(&rcur);
    // let repo_selector: SelectView<Repo> = SelectView::new()

    let error_view_inner
     = DebugView::new();
     let error_view_id = error_view_inner.with_id("debug-view");
     let error_view = error_view_id
        .max_height(20);

    let text_view_inner
     = TextView::new("Begin...");
     let text_view_id = text_view_inner.with_id("text-view");
     let text_view = text_view_id;

    // REPO SELECTOR
    let rs_tags = Rc::clone(&globals_rc.repo_tags);
    let rs_repo = Rc::clone(&globals_rc.current_repo);
    // TAGS DISPLAY
    let td_repo = Rc::clone(&globals_rc.current_repo);
    // TAGS_POOL
    let tp_repo = Rc::clone(&globals_rc.current_repo);
    let tp_tags = Rc::clone(&globals_rc.repo_tags);

    // =================================================
    //  TAKE 2
    // =================================================
    let repo_selector_inner: SelectView<usize> = SelectView::new()
        .with_all((*Rc::clone(&repo_ref)).borrow().selectify_repos())
        .on_select(move |s: &mut Cursive, ss: &usize| {
            // let new_tags = Rc::clone(&globals_rc.repo_tags);
            // (*rs_tags).replace(ss.tags.clone());
            // *rs_tags.borrow_mut() = ss.tags;
            // (*rs_repo).replace(ss.clone());
            // *rs_repo.borrow_mut() = ss;
            (*repo_ref).borrow_mut().repo_index = *ss;

            let mut dd: ViewRef<SelectView<usize>> =
                s.find_id("tag-display").unwrap();
            &dd.clear();
            &dd.add_all((*repo_ref).borrow().selectify_tags(*ss));
        });
    // =================================================
    //  TAKE 1
    // =================================================
    // let repo_selector_inner: SelectView<Repo> = SelectView::new()
    //     .with_all(selectify_rc_things(&globals_rc.repos, |r| {
    //         (r.path.clone(), r)
    //     }))
    //     // .with_all(selectify_things(
    //     //     // RefCell::borrow(&*mut_globals).repos.iter().collect(),
    //     //     {
    //     //         // (*mut_globals).borrow().clone().repos.iter().collect(),
    //     //     },
    //     // ))
    //     .on_select(move |s: &mut Cursive, ss: &Repo| {
    //         // let new_tags = Rc::clone(&globals_rc.repo_tags);
    //         (*rs_tags).replace(ss.tags.clone());
    //         // *rs_tags.borrow_mut() = ss.tags;
    //         (*rs_repo).replace(ss.clone());
    //         // *rs_repo.borrow_mut() = ss;

    //         let mut dd: ViewRef<SelectView<RepoTag>> =
    //             s.find_id("tag-display").unwrap();
    //         &dd.clear();
    //         &dd.add_all(selectify_rc_things(&rs_tags, |t| (t.name.clone(), t)));
    //         // &dd.insert_item(
    //         //     0,
    //         //     String::from("hola"),
    //         //     // "fuck".to_string(),
    //         //     RepoTag::from("hola".to_string()),
    //         // );
    //         // let dd: &IdView<SelectView<RepoTag>> =
    //         //     &**(&s.find_id("tag-display").unwrap());
    //         // let ddd = views::IdView::<V>::get_mut
    //         // let ddd = dd.borrow_mut();
    //         // dd.wtf();
    //     });
    // .on_submit(|s: &mut Cursive, r: &Repo| {
    //     // Lets focus on these tags for now
    //     s.focus_id("tag-pool").expect("...")
    //     // s.focus_id("tag-display").expect("...")
    // })
    // .item("hey", 4)
    let repo_selector_id = repo_selector_inner.with_id("repo-field");
    let repo_selector =
        repo_selector_id.scrollable().min_width(20).max_height(10);
    // let tags_displayer: IdView<BoxView<SelectView>> = OnEventView()
    // let tags_displayer: SelectView<RepoTag> = OnEventView::new(
    // =================================================
    //  TAKE 2
    // =================================================
    let rr = Rc::clone(&repo_tag_ref);
    let tags_displayer_inner: SelectView<usize> = SelectView::new()
        // .with_all_str(vec!["Gladly", "my", "dear"])
        .with_all({
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
            // let mut dd: ViewRef<SelectView<usize>> =
            //     s.find_id("tag-display").unwrap();

            let mut this: ViewRef<SelectView<usize>> =
                s.find_id("tag-display").unwrap();
            // &dd.clear();
            // &dd.add_all(_light_table.selectify_tags(_current_repo));
            let i: usize = this.selected_id().expect("Couldnt get selected id");
            (*this).remove_item(i);

            // let _light_table = (*repo_tag_ref).borrow_mut();
            let _light_table: &mut LightTable = &mut (*repo_tag_ref).try_borrow_mut().expect("Mut Borrow 3 failed");
            // let repos = _light_table.repos;

            let _current_repo: usize = _light_table.repo_index;
            let current_repo: &mut Repo = _light_table
                .repos
                .get_mut(_current_repo)
                .expect("ERROR - repo index out of bounds");
            let _current_tags: &mut Vec<RepoTag> = &mut (current_repo).tags;
            // let _current_tags: &Vec<RepoTag> = &_light_table
            //     .tags;
            _current_tags.remove(i);

            //     // this.clear();
            //     if let Some(id) = this.selected_id() {
            //         let name = this.selection().unwrap();
            //         let cb = this.remove_item(id);
            //         cb(s);
            //     }
        });
    // =================================================
    //  TAKE 1
    // =================================================
    // let tags_displayer_inner: SelectView<RepoTag> = SelectView::new()
    //     // .with_all_str(vec!["Gladly", "my", "dear"])
    //     .with_all(selectify_rc_things(&globals_rc.repo_tags, |t| {
    //         (t.name.clone(), t)
    //     }));
    // // .with_all(selectify_strings(
    // //     &RefCell::borrow(&*mut_globals)
    // //         .tags
    // //         .iter()
    // //         .map(|t| t.name.clone())
    // //         .collect(),
    // // ))
    // // .item("hey", 4)
    // let tags_displayer_id = tags_displayer_inner.with_id("tag-display");
    // let tags_displayer_outer = tags_displayer_id.min_width(20).max_height(10);
    // let tags_displayer = OnEventView::new(tags_displayer_outer)
    //     .on_event(Event::Key(Key::Esc), |s| {
    //         s.focus_id("repo-field").expect("...")
    //     })
    //     .on_event(Event::Key(Key::Backspace), move |s| {
    //         let mut this: ViewRef<SelectView> =
    //             s.find_id("tag-display").unwrap();
    //         //     // this.clear();
    //         //     if let Some(id) = this.selected_id() {
    //         //         let name = this.selection().unwrap();
    //         //         let cb = this.remove_item(id);
    //         //         cb(s);
    //         //     }
    //     });

    // let extra_ref = Rc::clone(&all_tags_ref);

    // =================================================
    //  TAKE 2
    // =================================================
    let tags_pool_inner: SelectView<usize> = SelectView::new()
        // .with_all(selectify_strings(&ct))
        // .with_all((*Rc::clone(&repo_ref)).borrow().selectify_repos())
        .with_all((*Rc::clone(&all_tags_ref)).try_borrow().expect("Borrow 1 failed").all_tags())
        .on_submit(move |s: &mut Cursive, ss: &usize| {
            // let all_tags = Rc::clone(&all_tags_ref);
            // let plain_borrow = (*all_tags_ref).try_borrow().expect("Borrow 2 failed");
            // debug!("**** - plain_borrow {:#?}", plain_borrow);

            let _light_table: &mut LightTable = &mut (*all_tags_ref).try_borrow_mut().expect("Mut Borrow 3 failed");
            debug!("**** - _light_table index {:#?}", _light_table);

            let _current_repo: usize = _light_table.repo_index;
            // let _current_repo = plain_borrow.repo_index;
            debug!("**** - current repo index {}", _current_repo);

            // let _current_repo = (*all_tags_ref).borrow().repo_index;


            let current_repo: &mut Repo = _light_table
                .repos
                // .get(_current_repo)
                .get_mut(_current_repo)
                .expect("ERROR - repo index out of bounds");

            debug!("**** - current repo {:#?}", current_repo);

                // let dd = &s.find_id()

            // *(*dd.borrow_mut()).borrow_mut().clear();
            // View::clear(dd);
            // &dd.
            // let r = Record{
            //     level: log::Level::Info,
            //     time: "27",
            //     message: format!("Fuck {}", &current_repo.path)};
            // logger::log(r);
                // info!("Fuck {}", &current_repo.path);

            // let _current_tag: &Vec<RepoTag> = &(&current_repo)
            //     .tags;
            let _current_tags: &Vec<RepoTag> = &_light_table
                .tags;

            let _current_tag: &RepoTag = _current_tags.get(*ss)
                .expect("ERROR - tags index out of bounds");

            // let _current_tag: &RepoTag = (&current_repo)
            //     .tags
            //     .clone()
            //     .get(*ss)
            //     .expect("ERROR - tags index out of bounds");
            // debug!("**** - current_tag {:#?}", _current_tag);

            // let _current_tags = &(*all_tags_ref).borrow().tags;
            // let _current_tag = _current_tags.get(*ss);
            // let _current_tag = (&_light_table).tags.get(*ss);
            // let mut _light_table_two = (*extra_ref).borrow_mut();
            // let _current_tag = (&_light_table_two).tags.get(*ss)
            //     .expect("ERROR - repo index out of bounds");

            // let _current_tag = plain_borrow.tags.get(*ss)
            //     .expect("ERROR - repo index out of bounds");

            // let _current_tag = (*all_tags_ref).borrow().tags.get(*ss);

            let mut tt: ViewRef<TextView> =
            s.find_id("text-view").unwrap();
            // &tt.clear();
            &tt.set_content(format!("{:#?}", _current_tag));
            // &tt.set_content(&current_repo.path);

            debug!("**** - current tag  index {}", *ss);
            // debug!("**** - current tag {:#?}", _current_tag);

            current_repo.tags.push(_current_tag.clone());

            let mut dd: ViewRef<SelectView<usize>> =
                s.find_id("tag-display").unwrap();
            &dd.clear();
            &dd.add_all(_light_table.selectify_tags(_current_repo));

            // (*all_tags_ref)
            //     .borrow_mut()
            //     .repos
            //     .get_mut(_current_repo)
            //     .expect("ERROR - repo index out of bounds")
            //     .tags
            //     .push(_current_tag.clone());
            // (*all_tags).borrow_mut().repos
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
    // =================================================
    //  TAKE 1
    // =================================================
    // let tags_pool_inner: SelectView<RepoTag> = SelectView::new()
    //     // .with_all(selectify_strings(&ct))
    //     .with_all(selectify_things_two(fake_more_tags, |t| {
    //         (t.name.clone(), t)
    //     }))
    //     .on_submit(move |s: &mut Cursive, ss: &RepoTag| {
    //         // (*tp_repo).replace_with(|rt| {
    //         //     rt.tags.push(ss.clone());
    //         //     rt.clone()
    //         // });
    //         (*tp_repo).borrow_mut().tags.push(ss.clone());
    //         (*tp_tags).borrow_mut().push(ss.clone());
    //         // (*tp_tags).replace_with(|rt| {
    //         //     rt.push(ss.clone());
    //         //     rt.clone()
    //         // });
    //         println!("And we print");
    //         let _out: Vec<String> = (*tp_tags)
    //             .borrow()
    //             .iter()
    //             .cloned()
    //             .map(String::from)
    //             .collect();
    //         let _form: String =
    //             format!("\nAdd tag to repo {}:\n", &tp_repo.borrow().path);
    //         debug_write_file(
    //             // vec![String::from("add Repo: "), r.path, "\n"],
    //             vec![_form]
    //                 // vec!["We printed this", "and this"]
    //                 .into_iter()
    //                 .map(String::from)
    //                 .chain(_out)
    //                 .collect(),
    //             "tmp_out",
    //         );
    //         updated_display_tags(s, &((*tp_repo).borrow().deref()))
    //         // updated_display_tags(s, &((*rcur1).borrow().deref()))
    //         //     updated_display_tags(s, &fuckRepo)
    //         //     // updated_display_tags(s, &(**c3po));
    //     });
    // let tags_pool = tags_pool_inner.with_id("tag-pool");

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
                    OnEventView::new(
                        tags_pool, //     .on_event(Event::Key(Key::Escape), |s1| {
                                  // })
                    )
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
                    .scrollable(), // .on_event(Event::Key::Del)::with_cb(
                                   // )
                ),
            )
            .child(
                Panel::new(error_view)
            )
            .child(
                Panel::new(text_view)
            )
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
