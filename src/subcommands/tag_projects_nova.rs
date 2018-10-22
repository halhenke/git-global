use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::ops::{Deref, DerefMut};
use std::iter::Zip;
use std::any::Any;
use std;
use std::borrow::{
    Borrow,
    BorrowMut};
extern crate cursive;
use itertools::{rciter};

use self::cursive::Cursive;
use self::cursive::align::HAlign;
use self::cursive::event::{Callback, Event, EventResult, Key};
use self::cursive::{
    traits::*,
    view::Selector
    };
use std::iter::FromIterator;
use self::cursive::{
    views::{
        EditView,
        IdView,
        Layer,
        LinearLayout,
        ListView,
        Menubar,
        MenuPopup,
        OnEventView,
        Panel,
        ScrollView,
        SelectView,
        TextContent,
        TextView,
        }};
use core::errors::Result as WeirdResult;
use core::{GitGlobalConfig, Repo, RepoTag, GitGlobalResult, get_repos};
use mut_static::MutStatic;
use take_mut;
use std::cell::Ref;
type RMut = Rc<RefCell<TextContent>>;

use std::fmt;

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

struct TagStatus {
    repos: RcVecRepo,
    currentRepo: RcRepo,
    currentTags: RcVecRepoTag,
}

impl TagStatus {
    pub fn new_from_rc(repos: RcVecRepo, repo: RcRepo, tags: RcVecRepoTag) -> TagStatus {
        TagStatus {
            repos: repos,
            currentRepo: repo,
            currentTags: tags,
        }
    }
}

pub fn repo_2_name<'a>(s: &'a str) -> &'a str {
    s.rsplit("/")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
}

pub fn go<'a>() -> WeirdResult<GitGlobalResult> {
    // note a pointer
    let uc = GitGlobalConfig::new();
    // let user_config = Box::new(&uc);
    // let uRepos: Box<&GitGlobalConfig> = user_config.clone();

    let reps: Vec<Repo> = uc.get_cached_repos()
        .into_iter()
        .take(5)
        .collect();
    let results = uc.get_cached_results();
    let result_tags: Vec<RepoTag> = results.all_tags()
        .into_iter()
        .cloned()
        // .map(|&x| x)
        // .map(AsRef::asref)
        .collect();
        // .clone()
        // .into_iter()
        // .cloned()
        // .map(|rt| rt.clone())
        // .collect();

    // NOTE: unsafe
    // let cur: [Repo] = reps.borrow();
    let cur = reps.as_ptr();

    let rct = reps.clone();
    let repoNames = &rct
        .iter()
        .map(|x| x.path.clone())
        .zip(rct.iter());

    let rreps = Rc::new(RefCell::new(reps));
    let ttags = Rc::new(RefCell::new(result_tags));

    // let sleps = rreps.clone();

    // static mut current: &Repo = || {
    //     rreps.into_inner()[0]
    // };
    // let current = &rreps.deref().borrow_mut()[0] as *mut Repo;
    // let curr: Rc<&Repo> = Rc::new(
    //     &rreps
    //         .clone()
    //         .deref()
    //         // .borrow()
    //         // .iter()
    //         // .nth(0)
    //         // .unwrap()
    //         // .borrow()[0]
    //         .get_mut()[0]
    //         // .into_inner()[0]
    //     // rreps.borrow::<Ref<Repo>>()[0]
    // );



    // let uRepos: GitGlobalConfig = *(&user_config).downcast::<GitGlobalConfig>().expect("yo");
    // let uRepos: Vec<Repo> = user_config.downcast::<GitGlobalConfig>().get_cached_resuts();
    // let results1 = Rc::new(RefCell::new(user_config.get_cached_results().repos));
    // let results2 = Rc::new(RefCell::new(user_config.get_cached_results().repos.remove(0)));
    // let results3 = Rc::new(RefCell::new(user_config.get_cached_results().repos.remove(0).tags));
    // // let rr1 = &results.clone();
    // let rr2 = &results.clone();
    // let rr3 = results.clone();
    // let rc1 = Rc::new(RefCell::new(rr1.borrow().repos));
    // let rc2 = Rc::new(RefCell::new(rr2.borrow().repos[0]));
    // let rc3 = Rc::new(RefCell::new(rr3.borrow().repos[0].tags));


    // let rc1 = results.clone();
    // NOTE: Think i must clone once to get proper lifetime...
    // let rc1 = Rc::new(RefCell::new(&results.repos));
    // let rc11 = &rc1.clone();
    // let rc2 = Rc::new(RefCell::new(&results.repos[0]));
    // let rc3 = Rc::new(RefCell::new(&results.repos[0].tags));

    // let status = TagStatus::new_from_rc(
    //     results1,
    //     results2,
    //     results3
    // );

    // let mut_stat = Rc::new(RefCell::new(&status));
    // let stat_1 = Rc::clone(&mut_stat);
    // let stat_2 = Rc::clone(&mut_stat);

    trace!("go");

    let mut siv = Cursive::default();
    siv.load_theme_file("assets/style.toml").unwrap();

    // https://github.com/gyscos/Cursive/issues/179
    let mut_content = TextContent::new(
        vec!("aaaa", "bbbb")
        // user_config.tag_names()
            .join("\n")
    );


    type SelRepoList<'a> = std::iter::Zip<std::vec::IntoIter<&'a str>, std::vec::IntoIter<Repo>>;

    type SelRepoList2 = std::iter::Zip<String, Repo>;

    type SelTagList<'a> = std::iter::Zip<
        // Vec<&'a str>,
        // Vec<String>
        // std::iter::Map<&'a str>,
        std::vec::IntoIter<&'a str>,
        std::vec::IntoIter<String>
        // std::slice::Iter<&'a, &str>,
        // std::vec::IntoIter<&'a str>,
        // itertools::RcIter<std::vec::IntoIter<String>>
    >;

fn selectify<'a>(tags_1: &'a Vec<String>) -> SelTagList<'a> {
    let tags_2: Vec<&'a str> = tags_1
        .iter()
        .map(AsRef::as_ref)
        .collect();
    return tags_2
        .into_iter()
        .zip(
            tags_1.to_vec()
        )
}


    pub use std::vec::IntoIter;

    // type SelRepIter = Zip<IntoIter<String>, IntoIter<RcRepo>>;
    type SelRepIter<'a> = &'a Vec<(String, RcRepo)>;
    // type SelRepIter = Vec<(String, RcRepo)>;
    // type SelRepIter<'a> = Vec<(String, &'a RcRepo)>;

    fn selectify_repos(repos: RcVecRepo) -> Vec<(String, Repo)> {
        return repos
            .deref()
            .borrow_mut()
            .clone()
            .into_iter()
            .map(|r| (r.path.clone(), r))
            // .map(|r| (r.path.clone(), Rc::new(RefCell::new(r))))
            .collect()
    }

    // // /// Turn a Vector of Repos into a Zip suitable for display in a SelectList
    // // // fn selectify_repos<'a>(repos: Vec<Repo>) -> Vec<(String, Repo)> {
    // // // fn selectify_repos<'a>(repos: &'a Vec<Repo>) -> Vec<(String, &Repo)> {
    // // // fn selectify_repos<'a>(repos: RcVecRepo<'a>) -> Vec<(String, &Repo)> {
    // // fn selectify_repos(repos: RcVecRepo) -> Vec<(String, RcRepo)> {
    // // fn selectify_repos<'a>(repos: RcVecRepo) -> SelRepoList2 {
    // fn selectify_repos<'a, TT>(repos: RcVecRepo) -> TT
    //     // where   TT: IntoIterator,
    //     //         TT::Item: (String, RcRepo)
    //     // where       TT: IntoIterator<Item = (String, RcRepo)>,
    //     where       TT: std::iter::FromIterator<(String, &'a Repo)>
    // {
    //     repos.deref()
    //         // .as_ptr()
    //         // .borrow()
    //         .deref()
    //         .borrow_mut()
    //         // .repos
    //         // .into_iter()
    //         .iter()
    //         .map(|r| (r.name().to_string(), r) )
    //         .collect()
    // }

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
        .with_all(selectify_repos(
            rreps.clone()
            // mut_stat
                // .clone()
                // .deref()
                // .into_inner()
                // .borrow()
                // .repos
            // rs
            // &rrrrr.deref().repos
            // results.repos.clone() &&
            // rc11.clone()
            // Rc::clone(&rc1)
        ))
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
            // let s = status.currentRepo
            //     .deref()
            //     .borrow_mut()
            //     .deref()
            //     .borrow_mut();
            // s = ss.borrow_mut();
            let tmp = RefCell::new(cur);
            *tmp.borrow_mut() = ss;
            // unsafe {
            //     cur = ss;
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
    let tags_displayer  = OnEventView::new(
        SelectView::new()
            .with_all(selectify(
                // &vec!("hoo", "lah", "laa")
                unsafe {
                    &(*cur)
                        // .as_ref()
                        .tags
                        .clone()
                        .into_iter()
                        // .map(String::from)
                        .map(|x| x.name)
                        .collect::<Vec<String>>()

                }
            ))
            .min_width(20)
            .with_id("tag-display")
    ).on_event(Event::Key(Key::Esc), |s|
        s.focus_id("repo-field").expect("...")
    );
    let tags_pool: IdView<SelectView> = SelectView::new()
    // let tags_pool: IdView<ScrollView<SelectView>> = SelectView::new()
            .with_all(selectify(
                // vec!("more", "stuff")
                {
                    // let sss: Rc<RefCell<Vec<RepoTag>>> = ttags.to_owned();
                    let sss: RefMut<Vec<RepoTag>> =
                        ttags
                        // .clone()
                        .deref()
                        // .to_owned()
                        // .into_inner();
                        .borrow_mut();
                        // .cloned();
                    &sss
                        .iter()
                        .cloned()
                        .map(|r| r.name.clone())
                        .collect()
                    // assert!(ttags, ttags.clone())
                    // let sss: Vec<RepoTag> =
                    // // let sss: Borrow<Ref<Vec<RepoTag>>> =
                    //     // ttags
                    //     ttags
                    //     .into_inner();

                    // let salmon: Vec<RepoTag> =
                    // // let salmon: Vec<String> =
                    //     // Vec::from_iter(
                    //         // ttags.clone()
                    //     // Rc::clone(
                    //         ttags.to_owned()
                    //         // .borrow()
                    //         // .deref()
                    //         .into_inner()
                    //         .iter()
                    //         // .into_iter()
                    //         .cloned()
                    //         // .map(|x| x.name)
                    //     // );
                    //         .collect();
                        // vec!("car")
                }
            // user_config.tags
            //     .iter()
            //     .map(|r| r.name.as_str())
            //     .collect()
        ))
        // .scrollable()
        .with_id("tag-pool");

    /// Main Window
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
                    // .on_event_inner(Event::Key(Key::Backspace), |s1| {
                    //     delete_tag(&mut s1.get_mut())
                    // })
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
