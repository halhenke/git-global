use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::ops::{Deref, DerefMut};
use std::iter::Zip;
use std;
extern crate cursive;

use self::cursive::Cursive;
use self::cursive::align::HAlign;
use self::cursive::event::{Callback, Event, EventResult, Key};
use self::cursive::{
    traits::*,
    view::Selector
    };
use self::cursive::{
    view::{
        View,
        ViewWrapper,
    },
    menu::{
        MenuTree,
    },
    views::{
        BoxView,
        Dialog,
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

type RMut = Rc<RefCell<TextContent>>;

use std::fmt;

// struct TagStatus {
//     repos: Vec<Repo>,
//     currentRepo: Repo,
//     currentTags: Vec<RepoTag>,
struct TagStatus {
    repos: RcVecRepo,
    currentRepo: RcRepo,
//     currentTags: Vec<RepoTag>,
}

type RcResult = Rc<GitGlobalResult>;
type RcRcResult = Rc<RefCell<GitGlobalResult>>;
type RcRepo = Rc<RefCell<Repo>>;
type RcRepoTag = Rc<RefCell<RepoTag>>;
type RcVecRepoTag = Rc<RefCell<Vec<RepoTag>>>;
type RcVecRepo = Rc<RefCell<Vec<Repo>>>;
type RcTS = Rc<RefCell<TagStatus>>;
// type RcRepo<'a> = Rc<RefCell<&'a Repo>>;
// type RcRepoTag<'a> = Rc<RefCell<&'a RepoTag>>;
// type RcVecRepoTag<'a> = Rc<RefCell<&'a Vec<RepoTag>>>;
// type RcVecRepo<'a> = Rc<RefCell<&'a Vec<Repo>>>;


// impl<'a> TagStatus<'a> {
//     pub fn new(repos: RcVecRepo<'a>, repo: RcRepo<'a>, tags: RcVecRepoTag<'a>) -> TagStatus<'a> {
impl TagStatus {
//     pub fn new(repos: RcVecRepo, repo: RcRepo, tags: RcVecRepoTag) -> TagStatus {
// // impl TagStatus {
// //     pub fn new(repos: Vec<Repo>, repo: Repo, tags: Vec<RepoTag>) -> TagStatus {
// // impl<'a> TagStatus<'a> {
// //     pub fn new(repos: &'a Vec<Repo>, repo: &'a Repo, tags: &'a Vec<RepoTag>) -> TagStatus<'a> {
//         return TagStatus {
//             /// Current repos
//             repos: repos,
//             /// Currently selected repo
//             currentRepo: repo,
//             /// List of all tags (gettable from repos)
//             /// ...or list of all tags for this repo?
//             currentTags: tags,
//         }
//     }

    // pub fn new_from_rc(repos: RcVecRepo, repo: RcVecRepo, tags: RcVecRepo) -> TagStatus {
    pub fn new_from_rc(repos: RcRcResult) -> TagStatus {
        TagStatus {
            // repos: Rc::new(RefCell::new(repos.clone().repos)),
            repos: Rc::new(RefCell::new(repos.clone().deref().borrow().deref().repos)),
            currentRepo: Rc::new(RefCell::new(repos.clone().deref().borrow().deref().repos[0])),
            // currentRepo: Rc::new(RefCell::new(repos.deref().borrow().repos[0])),
        }
    }

    // pub fn select_repo(&mut self, repo: RcRepo<'a>) -> &'a mut TagStatus {
    pub fn select_repo(&mut self, repo: RcRepo) -> &mut TagStatus {
    // pub fn select_repo(&mut self, repo: Repo) -> &mut TagStatus {
    // pub fn select_repo(&mut self, repo: Repo) -> &'a TagStatus {
    // pub fn select_repo(&mut self, repo: &'a Repo) -> &'a TagStatus {
        // let t = &(&repo).tags;
        self
            // .deref()
            // .borrow_mut()
            .currentRepo = repo;
        // self.currentTags = repo.tags;
        // self.currentTags = &repo.tags;
        self
    }
}

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

pub fn repo_2_name<'a>(s: &'a str) -> &'a str {
    s.rsplit("/")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
}

pub fn go<'a, 'b>() -> WeirdResult<GitGlobalResult> {
    let user_config = GitGlobalConfig::new();
    let results = Rc::new(RefCell::new(user_config.get_cached_results()));
    // let rr1 = &results.clone();
    // let rr2 = &results.clone();
    let rr3 = results.clone();
    // let rc1 = Rc::new(RefCell::new(rr1.borrow().repos));
    // let rc2 = Rc::new(RefCell::new(rr2.borrow().repos[0]));
    // let rc3 = Rc::new(RefCell::new(rr3.borrow().repos[0].tags));


    // let rc1 = results.clone();
    // NOTE: Think i must clone once to get proper lifetime...
    // let rc1 = Rc::new(RefCell::new(&results.repos));
    // let rc11 = &rc1.clone();
    // let rc2 = Rc::new(RefCell::new(&results.repos[0]));
    // let rc3 = Rc::new(RefCell::new(&results.repos[0].tags));

    let status = Rc::new(RefCell::new(TagStatus::new_from_rc(
        // rr1,
        // rr2,
        rr3)));
        // results.clone(),
        // rc2,
        // rc3);
    // let status = TagStatus::new(rc1, rc2, rc3);

    // let mut_stat = Rc::new(RefCell::new(&status));
    // let stat_1 = Rc::clone(&mut_stat);
    // let stat_2 = Rc::clone(&mut_stat);

    trace!("go");

    let mut siv = Cursive::default();
    siv.load_theme_file("assets/style.toml").unwrap();

    // https://github.com/gyscos/Cursive/issues/179
    let mut_content = TextContent::new(
        user_config.tag_names()
            .join("\n")
    );

    type SelTagList<'a> = std::iter::Zip<std::vec::IntoIter<&'a str>, std::vec::IntoIter<String>>;

    type SelRepoList<'a> = std::iter::Zip<std::vec::IntoIter<&'a str>, std::vec::IntoIter<Repo>>;

    /// Turn a Vector of tags into a Zip suitable for display in a SelectList
    fn selectify(tags_1: Vec<&str>) -> SelTagList {
        let tags_2: Vec<String> = tags_1
            .clone()
            .into_iter()
            .map(|x| String::from(x))
            .collect();
        return tags_1
            .into_iter()
            .zip(tags_2.into_iter())
    }

    /// Turn a Vector of Repos into a Zip suitable for display in a SelectList
    // fn selectify_repos<'a>(repos: Vec<Repo>) -> Vec<(String, Repo)> {
    // fn selectify_repos<'a>(repos: &'a Vec<Repo>) -> Vec<(String, &Repo)> {
    // fn selectify_repos<'a>(repos: RcVecRepo<'a>) -> Vec<(String, &Repo)> {
    fn selectify_repos<'a>(repos: RcVecRepo) -> Vec<(String, &'a Repo)> {
        unimplemented!();
        // repos.deref()
        //     // .as_ptr()
        //     .borrow()
        //     .deref()
        //     // .repos
        //     .into_iter()
        //     .map(|r| (r.name().to_string(), r) )
        //     .collect()
    }

    debug!("ADD TAGS: did we get here - 3");
    let mut new_tags: Vec<String> = Vec::new();
    let edit_cb = move |s: &mut Cursive, name: &str| {
        debug!("edit_cb was called...");
        take_mut::take(&mut new_tags, |mut new_tags| {
            new_tags.push(String::from(name));
            new_tags
        });
    };


    /// VIEWS
    let e_view = EditView::new()
        // .on_submit(show_popup)
        .on_submit_mut(edit_cb)
        .with_id("tag")
        .fixed_width(20);
    let repo_selector: SelectView<RcRepo> = SelectView::new();
    // let repo_selector = SelectView::new()
    //     .with_all(selectify_repos(
    //         // rs
    //         // &rrrrr.deref().repos
    //         // results.repos.clone() &&
    //         // rc11.clone()
    //         // Rc::clone(&rc1)
    //     ))
    //     // .with_all(selectify(
    //     //     user_config.get_cached_repos()
    //     //         .iter()
    //     //         .map(|r| r.path.as_str())
    //     //         .map(|p| repo_2_name(p))
    //     //         .take(5)
    //     //         .collect()
    //     // ))
    //     .on_select(move |s: &mut Cursive, ss| {
    //         // Rc::clone(&mut_stat)
    //         // results.repos;
    //         stat_1;
    //             // .deref()
    //             // .get_mut()
    //             // .borrow_mut()
    //             // .get_mut()
    //             // .currentRepo = ss;
    //             // .currentRepo = ss.clone();
    //     })
    //     // .on_submit(|s, r| {
    //     .on_submit(|s: &mut Cursive, r: &Repo| {
    //         s.focus_id("tag-display").expect("...")
    //     })
    //     .min_width(20)
    //     .with_id("repo-field");
    // let tags_displayer: IdView<BoxView<SelectView>> = OnEventView()
    let tags_displayer  = OnEventView::new(
        SelectView::new()
            .with_all(selectify(
                vec!("hoo", "lah", "laa")
            ))
            .min_width(20)
            .with_id("tag-display")
    ).on_event(Event::Key(Key::Esc), |s|
        s.focus_id("repo-field").expect("...")
    );
    let tags_pool: IdView<SelectView> = SelectView::new()
    // let tags_pool: IdView<ScrollView<SelectView>> = SelectView::new()
            .with_all(selectify(
            user_config.tags
                .iter()
                .map(|r| r.name.as_str())
                .collect()
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
            // .child(
            //     Layer::new(
            //         Menubar::new()
            //         .subtree("Repo", MenuTree::new()
            //             .leaf("First Thing", |_| {})
            //         )
            //     )
            // )
            .child(
                // sel_view
                Panel::new(
                    OnEventView::new(
                        tags_pool
                    )
                    .on_event_inner(Event::Key(Key::Backspace), |s1| {
                        delete_tag(&mut s1.get_mut())
                    })
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

/// Final behaviour
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
