use std;
use std::any::Any;
use std::borrow::{Borrow, BorrowMut};
use std::cell::{RefCell, RefMut};
use std::iter::Zip;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
extern crate cursive;

use self::cursive::align::HAlign;
use self::cursive::event::{Callback, Event, EventResult, Key};
use self::cursive::Cursive;
use self::cursive::{
    menu::MenuTree,
    view::{View, ViewWrapper},
    views::{
        BoxView, Dialog, EditView, IdView, Layer, LinearLayout, ListView,
        MenuPopup, Menubar, OnEventView, Panel, ScrollView, SelectView,
        TextContent, TextView,
    },
};
use self::cursive::{traits::*, view::Selector};
use mut_static::MutStatic;
use repo::errors::Result as WeirdResult;
use repo::{get_repos, GitGlobalConfig, GitGlobalResult, Repo, RepoTag};
use std::cell::Ref;
use take_mut;
type RMut = Rc<RefCell<TextContent>>;

use std::fmt;

struct TagStatusSimple<'a> {
    repos: &'a Vec<Repo>,
    // currentRepo: usize,
    currentRepo: &'a Repo,
    currentTags: &'a Vec<RepoTag>,
}
impl<'a> TagStatusSimple<'a> {
    //     pub fn new(repos: RcVecRepo, repo: RcRepo, tags: RcVecRepoTag) -> TagStatus {
    // // impl TagStatus {
    // //     pub fn new(repos: Vec<Repo>, repo: Repo, tags: Vec<RepoTag>) -> TagStatus {
    // // impl<'a> TagStatus<'a> {
    // pub fn new(repos: &'a Vec<Repo>, repo: usize, tags: &'a Vec<RepoTag>) -> TagStatusSimple<'a> {
    pub fn new(
        repos: &'a Vec<Repo>,
        repo: &'a Repo,
        tags: &'a Vec<RepoTag>,
    ) -> TagStatusSimple<'a> {
        return TagStatusSimple {
            repos: repos,
            currentRepo: repo,
            currentTags: tags,
        };
    }
}

type RcResult = Rc<GitGlobalResult>;
type RcRcResult = Rc<RefCell<GitGlobalResult>>;
type RcRepo = Rc<RefCell<Repo>>;
type RcRepoTag = Rc<RefCell<RepoTag>>;
type RcVecRepoTag = Rc<RefCell<Vec<RepoTag>>>;
type RcVecRepo = Rc<RefCell<Vec<Repo>>>;
// type RcRepo<'a> = Rc<RefCell<&'a Repo>>;
// type RcRepoTag<'a> = Rc<RefCell<&'a RepoTag>>;
// type RcVecRepoTag<'a> = Rc<RefCell<&'a Vec<RepoTag>>>;
// type RcVecRepo<'a> = Rc<RefCell<&'a Vec<Repo>>>;

lazy_static! {
    // static ref HASHMAP: HashMap<u32, &'static str> = {
    //     let mut m = HashMap::new();
    //     m.insert(0, "foo");
    //     m.insert(1, "bar");
    //     m.insert(2, "baz");
    //     m
    // };
    // let repos = GitGlobalConfig::new().get_cached_repos;
    static ref RESULTS: Vec<Repo> =
        GitGlobalConfig::new().get_cached_repos();

    static ref stat_two: TagStatusSimple<'static> = TagStatusSimple::new(
        // &rep_star.read().unwrap(),
        // rep_star.read().unwrap()[0],
        // &rep_star.read().unwrap()[0].tags);
        &RESULTS,
        // *CURRENT_REPO,
        &RESULTS[0],
        &RESULTS[0].tags);
        // &CURRENT_TAGS);

    // static ref CURRENT_REPO: u16 = 0;
    // static ref CURRENT_REPO: &'static Repo = repos[0];
    // static ref CURRENT_REPO: &'static Repo = &RESULTS[0];
    // static ref CURRENT_REPO: Arc<Mutex<&'static Repo>> = Arc::new(Mutex::new(&RESULTS[0]));
    // static ref CURRENT_REPO: &'static Repo = GitGlobalConfig::new().get_cached_repos().get(0).unwrap();
    // static ref CURRENT_TAGS: &'static Vec<RepoTag> = &RESULTS[0].tags;
    // static ref RESULTS: MutStatic<Vec<Repo>> =
    //     MutStatic::from(
    //         GitGlobalConfig::new().get_cached_repos()
    //     );
}

pub fn repo_2_name<'a>(s: &'a str) -> &'a str {
    s.rsplit("/").collect::<Vec<&str>>().first().unwrap()
}

// note
// - We cant pass a reference to closure
// - We need to wrap in an Rc....
//  - We need to clone the Rc
//  - We cant use references when we clone the Rc
//      - this seems impossible
// - Want to get several references to the same thing
//      - this may be why its impossible
// - repos, repos[0], repos.tags
//  - Cant all own these without an RC

pub fn go<'a, 'b>() -> WeirdResult<GitGlobalResult> {
    // note a pointer
    let uc = GitGlobalConfig::new();
    let user_config = Box::new(&uc);
    // let user_config: Box<Any> = Box::new(GitGlobalConfig::new());
    let uRepos: Box<&GitGlobalConfig> = user_config.clone();
    let rep_star = &RESULTS;

    // let rc1 = results.clone();
    // NOTE: Think i must clone once to get proper lifetime...
    // let rc1 = Rc::new(RefCell::new(&results.repos));
    // let rc11 = &rc1.clone();
    // let rc2 = Rc::new(RefCell::new(&results.repos[0]));
    // let rc3 = Rc::new(RefCell::new(&results.repos[0].tags));

    // let status = TagStatus::new_from_rc(
    //     results1,
    //     results2,
    //     results3);

    // let mut stat_two = TagStatusSimple::new(
    //     // &rep_star.read().unwrap(),
    //     // rep_star.read().unwrap()[0],
    //     // &rep_star.read().unwrap()[0].tags);
    //     rep_star,
    //     // *CURRENT_REPO,
    //     0,
    //     &CURRENT_TAGS);
    //     // rep_star,
    //     // &rep_star[0],
    //     // &rep_star[0].tags);

    // let sss = stat_two.repos.to_vec();

    // let mut_stat = Rc::new(RefCell::new(&status));
    // let stat_1 = Rc::clone(&mut_stat);
    // let stat_2 = Rc::clone(&mut_stat);

    trace!("go");

    let mut siv = Cursive::default();
    siv.load_theme_file("assets/style.toml").unwrap();

    // https://github.com/gyscos/Cursive/issues/179
    let mut_content = TextContent::new(user_config.tag_names().join("\n"));

    type SelTagList<'a> =
        std::iter::Zip<std::vec::IntoIter<&'a str>, std::vec::IntoIter<String>>;

    type SelRepoList<'a> =
        std::iter::Zip<std::vec::IntoIter<&'a str>, std::vec::IntoIter<Repo>>;

    type SelRepoList2 = std::iter::Zip<String, Repo>;

    /// Turn a Vector of tags into a Zip suitable for display in a SelectList
    fn selectify(tags_1: Vec<&str>) -> SelTagList {
        let tags_2: Vec<String> = tags_1
            .clone()
            .into_iter()
            .map(|x| String::from(x))
            .collect();
        return tags_1.into_iter().zip(tags_2.into_iter());
    }

    pub use std::vec::IntoIter;

    // /// Turn a Vector of Repos into a Zip suitable for display in a SelectList
    fn selectify_repos(repos: Vec<Repo>) -> Vec<(String, Repo)> {
        // fn selectify_repos<'a>(repos: &'a Vec<Repo>) -> Vec<(String, &'a Repo)> {
        repos
            .into_iter()
            .map(|r| (r.name().to_string(), r))
            .collect()
    }

    debug!("ADD TAGS: did we get here - 3");
    let mut new_tags: Vec<String> = Vec::new();

    /// VIEWS
    let e_view = EditView::new()
        // .on_submit(show_popup)
        // .on_submit_mut(edit_cb)
        .with_id("tag")
        .fixed_width(20);
    // let repo_selector: SelectView<RcRepo> = SelectView::new();
    let repo_selector = SelectView::new()
        // .with_all(selectify_repos(
        .with_all(selectify_repos(stat_two.repos.clone()))
        // .with_all(selectify(
        //     user_config.get_cached_repos()
        //         .iter()
        //         .map(|r| r.path.as_str())
        //         .map(|p| repo_2_name(p))
        //         .take(5)
        //         .collect()
        // ))
        .on_select(move |s: &mut Cursive, ss| {
            // Rc::clone(&mut_stat)
            // results.repos;
            let sss = &stat_two
                .repos
                .iter()
                // .position(
                .find(|&x| x == ss)
                .unwrap();

            unsafe {
                // CURRENT_REPO = sss;
                stat_two.currentRepo = *sss;
            }
            // stat_two.currentRepo = ss;
            // stat_two.currentRepo = *sss;
            // CURRENT_REPO = sss;
            // let mut tmp = CURRENT_REPO
            //     .borrow()
            //     .get_mut()
            //     .unwrap();
            // *tmp = ss;
            // if let Some(foo) = tmp {
            //     foo = ss;
            // }
            // stat_1;
            // .deref()
            // .get_mut()
            // .borrow_mut()
            // .get_mut()
            // .currentRepo = ss;
            // .currentRepo = ss.clone();
        })
        // .on_submit(|s, r| {
        .on_submit(|s: &mut Cursive, r: &Repo| {
            s.focus_id("tag-display").expect("...")
        })
        .min_width(20)
        .with_id("repo-field");
    // let tags_displayer: IdView<BoxView<SelectView>> = OnEventView()
    let tags_displayer = OnEventView::new(
        SelectView::new()
            .with_all(selectify(vec!["hoo", "lah", "laa"]))
            .min_width(20)
            .with_id("tag-display"),
    )
    .on_event(Event::Key(Key::Esc), |s| {
        s.focus_id("repo-field").expect("...")
    });
    let tags_pool: IdView<SelectView> = SelectView::new()
        // let tags_pool: IdView<ScrollView<SelectView>> = SelectView::new()
        .with_all(selectify(
            user_config.tags.iter().map(|r| r.name.as_str()).collect(),
        ))
        // .scrollable()
        .with_id("tag-pool");

    /// Main Window
    siv.add_layer(
        LinearLayout::vertical()
            .child(
                LinearLayout::horizontal()
                    .child(Panel::new(repo_selector))
                    .child(Panel::new(tags_displayer)),
            )
            .child(
                // sel_view
                Panel::new(
                    OnEventView::new(tags_pool)
                        // .on_event_inner(Event::Key(Key::Backspace), |s1| {
                        //     delete_tag(&mut s1.get_mut())
                        // })
                        // NOTE: Due to fucking annoying design this has to come
                        // after/outside `OnEventView` - otherwise we never get to unwrap
                        // properly
                        .scrollable(), // .on_event(Event::Key::Del)::with_cb(
                                       // )
                ),
            ),
    );
    siv.add_global_callback('q', |s1| s1.quit());
    siv.run();
    debug!("ADD TAGS: called - 33");

    // println!("new tags is {:?}", &fake_tags);
    Ok(GitGlobalResult::new(&vec![]))
}

// /// Final behaviour
// fn save_tags_and_quit(s: &mut Cursive, tags: &RMut) {
// // fn save_tags_and_quit(s: &mut Cursive, user_config: &mut GitGlobalConfig, tags: &RMut) {
//     let mut user_config = GitGlobalConfig::new();
//     trace!("save_tags_and_quit");
//     debug!("wtf???");
//     let mut t_list: Vec<String> = Vec::new();
//     s.call_on_id("tag_list",
//         |tl: &mut SelectView| {
//             error!("tag count is {}", tl.len());
//             let count = tl.len();
//             for i in 0..count  {
//                 t_list.push(tl.get_item(i).unwrap().0.to_string())
//             }
//         }
//     );
//     let tag_list: String = tags
//         .borrow()
//         .deref()
//         .get_content()
//         .source()
//         .to_string();
//     s.call_on_id("tag",
//         |view: &mut EditView| {
//             let po = &tag_list.clone();
//             view.set_content(po.to_string());
//         }
//     ).expect("final unwrap...");
//     let tag_list_list = t_list;
//     debug!("About to print tags");
//     debug!("tags are: {:?}", &tag_list_list);
//     // user_config.add_tags(
//     //     tag_list_list
//     // );
//     user_config.replace_tags(
//         tag_list_list
//     );
//     user_config.write_tags();
//     s.cb_sink()
//         .send(Box::new(|siv: &mut Cursive| siv.quit()));
// }

// fn show_next_screen(s: &mut Cursive, name: &str, c: &mut TextContent) {
//     trace!("show_next_screen");
//     if name.is_empty() {
//         s.add_layer(Dialog::info("Please enter a name!"));
//     } else {
//         trace!("show_next_screen 2");
//         s.call_on_id("tag_list",
//             |view: &mut SelectView|
//                 view.add_item_str(name)
//         ).expect("failure");
//         s.call_on_id("tag",
//             |view: &mut EditView|
//                 {
//                     view.set_content("")
//                     // view.set_cursor(0)
//                 }).expect("failure");
//         // // s.focus_id("tag").unwrap();
//         s.focus(&Selector::Id("tag")).expect("thing");
//     }
// }
