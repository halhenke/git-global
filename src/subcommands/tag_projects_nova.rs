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
use itertools::Itertools;

use std::fs::{File, OpenOptions};
use std::io::Write;

use macros::rc_mut;

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
        ViewRef
        }};
use core::errors::Result as WeirdResult;
use core::{
    GitGlobalConfig,
    GitGlobalResult,
    Repo,
    RepoTag,
    all_tags,
    save_repos_and_tags,
    get_repos
};
use mut_static::MutStatic;
use take_mut;
use std::cell::Ref;
type RMut = Rc<RefCell<TextContent>>;

use std::fmt;

type RcResult =     Rc<GitGlobalResult>;
type RcRcResult =   Rc<RefCell<GitGlobalResult>>;
type RcRepo =       Rc<RefCell<Repo>>;
type RcRepoTag =    Rc<RefCell<RepoTag>>;
type RcVecRepoTag = Rc<RefCell<Vec<RepoTag>>>;
type RcVecRepo =    Rc<RefCell<Vec<Repo>>>;
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

    let mut reps: Vec<Repo> = uc.get_cached_repos()
        .into_iter()
        // .take(5)
        .collect();
    let repsreps = &reps as *const Vec<Repo>;
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
    let mut cur = reps.as_mut_ptr();
    let c3po = &cur as *const *mut Repo;
    // let c3po = &cur as *const *mut Repo as *mut *mut Repo;
    let mut cur2 = reps.as_mut_ptr();
    let mut cur3 = reps.as_mut_ptr();
    let mut rcur = Rc::new(RefCell::new(cur2));

    let strs = vec!(
        format!("--------------------------------------------"),
        format!("INITIALS"),
        format!("&cur:                {:?}", (&cur as *const *mut Repo)),
        format!("cur:                 {:?}", (cur)),
        format!("reps:                {:?}", (reps.as_mut_ptr())),
        format!("cur3:                {:?}", (cur3)),
        format!("c3po:                {:?}", (c3po)),
        format!("------------------------------------------\n"),
    );
    debug_write_file(strs, "tmp_out");

    // let pos: usize = 0;
    // let posptr = &pos as *mut usize;
    // let mut cur_ptr = reps.as_mut_ptr();

    let rct = reps.clone();
    let repoNames = &rct
        .iter()
        .map(|x| x.path.clone())
        .zip(rct.iter());

    let rreps = Rc::new(RefCell::new(reps));
    let ttags = Rc::new(RefCell::new(result_tags));
    // NOTE: This is just until we have some actually tagged repos
    let config_tags = Rc::new(RefCell::new(uc.tags));

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


    type SelRepoList<'a> = std::iter::Zip<
        std::vec::IntoIter<&'a str>,
        std::vec::IntoIter<Repo>
    >;

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

    fn selectify_strings<'a>(tags_1: &'a Vec<String>) -> SelTagList<'a> {
        let tags_2: Vec<&'a str> = tags_1
            .iter()
            .map(AsRef::as_ref)
            .collect();
        return tags_2
            .into_iter()
            .zip(
                tags_1.to_vec()
            );
    }

    fn selectify_rc_tags<'a>(rctags: &'a RcVecRepoTag) -> Vec<String> {
    // fn selectify_rc_tags<'a, 'b: 'a>(rctags: &'a RcVecRepoTag) -> SelTagList<'b> {
    // fn selectify_rc_tags<'a>(rctags: &'a RefMut<Vec<RepoTag>>) -> SelTagList<'a> {
        // pipe!(
        // let tv: &'b Vec<String> = &rctags
            return rctags
                // .deref()
                // .clone()
                .deref()
                .borrow_mut()
            return rc_borr!(rctags)
            // return rctags
            //     .deref()
            //     .borrow_mut()
                .iter()
                // .cloned()
                // .into_iter()
                .map(|r| r.name.clone())
                .collect::<Vec<String>>();
                // .as_ptr();
                // .as_ref();
        // return selectify_strings(
        //     rctags
        //         .deref()
        //         .borrow_mut()
        //         .iter()
        //         .cloned()
        //         .map(|r| r.name.clone())
        //         .collect::<Vec<String>>().as_ref()
        // );
        // let tv2: &'static Vec<String> = &tv;
        // return selectify_strings(tv);
            // => selectify_strings
        // )
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

    debug!("ADD TAGS: did we get here - 3");
    let mut new_tags: Vec<String> = Vec::new();

    /// VIEWS
    let e_view = EditView::new()
        // .on_submit(show_popup)
        // .on_submit_mut(edit_cb)
        .with_id("tag")
        .fixed_width(20);
    // let repo_selector: SelectView<RcRepo> = SelectView::new();
    let rreps_1 = rreps.clone();
    let mut rcur2 = rcur.clone();
    let repo_selector = SelectView::new()
        .with_all(selectify_repos(
            rreps.clone()
        ))
        .on_select(move |s: &mut Cursive, ss: &Repo| {
            // Rc::clone(&mut_stat)
            // results.repos;
            // let s = status.currentRepo
            //     .deref()
            //     .borrow_mut()
            //     .deref()
            //     .borrow_mut();
            // s = ss.borrow_mut();
            let rcin: Ref<Vec<Repo>> = rreps_1
                .deref()
                .borrow();
            let ss_real = rcin
                .iter()
                // .position(|x| x.path == ss.path)
                .find(|x| x.path == ss.path)
                .unwrap();
                // .as_mut_ptr();
            // let ss_ptr = rcin[ss_real].as_mut_ptr();

            unsafe {
                // pos = ss_real;
                let mut sss_real = ss_real as *const Repo as *mut Repo;
                let mut sss_real_two = ss_real as *const Repo as *mut Repo;
                // let mut ptr_cpy = &mut cur as *mut *mut Repo;
                let mut ptr_cpy_two = cur as *mut Repo;
                let mut ptr_cpy_three = &mut ptr_cpy_two as *mut *mut Repo;
                let mut ptr_cpy_four = &cur as *const *mut Repo as *mut *mut Repo;

                // let fake3 = &cur3 as *const *mut Repo as *mut *mut Repo;
                // (*fake3) = (*fake3).add(1);

                // (&rcur2).replace(sss_real);
                // (*ptr_cpy_three).add(2);
                // *ptr_cpy_four.add(2);
                // ptr_cpy_two = sss_real;
                // (*ptr_cpy_four) = sss_real;
                // (*c3po) = sss_real;
                let c3p4 = c3po as *mut *mut Repo;
                (*c3p4) = sss_real;

                updated_display_tags(s, &(**c3po));

                // (*c3po) = sss_real;

                // *ptr_cpy = sss_real;
                // ptr_cpy = &mut sss_real;
                // cur = sss_real;
                let strs = vec!(
                    format!("sss_real:                  {:?}", (sss_real)),
                    format!("*sss_real:                 {:?}", (*sss_real)),
                    format!("&cur:                      {:?}", (&cur as *const *mut Repo)),
                    format!("cur:                       {:?}", (cur)),
                    format!("*cur:                      {:?}", (*cur)),
                    format!("ptr_cpy_two:               {:?}", (ptr_cpy_two)),
                    format!("*ptr_cpy_two:              {:?}", (*ptr_cpy_two)),
                    format!("ptr_cpy_three:             {:?}", (ptr_cpy_three)),
                    format!("*ptr_cpy_three:            {:?}", (*ptr_cpy_three)),
                    format!("ptr_cpy_four:              {:?}", (ptr_cpy_four)),
                    format!("*ptr_cpy_four:             {:?}", (*ptr_cpy_four)),
                    format!("c3po:                      {:?}", (c3po)),
                    format!("*c3po:                     {:?}", (*c3po)),
                    // format!("c3p4*:                     {:?}", (c3p4)),
                    // format!("*c3p4:                     {:?}", (*c3p4)),
                    // format!("fake3:                     {:?}", (fake3)),
                    format!("\n")
                );
                let strs_join: String = strs.as_slice().join("\n");
                let rcinptr = rcin.as_ptr();
                let file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open("tmp_out")
                    .unwrap()
                    .write_all(
                    // .write_fmt(
                        unsafe {
                            strs_join.as_ref()
                        // format_args!(
                        // "repo is {:?},\n, cur is {:?},\n ptr_cpy is {:?},\n *ptr_cpy is {:?},\n rcin first is {:?}, \n sss_real is {:?},\n cur val is {:?}\n",
                        // ss_real,
                        // cur,
                        // 0, 0,
                        // // ptr_cpy,
                        // // *ptr_cpy,
                        // rcinptr,
                        // sss_real,
                        // &(*cur)
                        // )
                    });
            }

            // unsafe {
            //     // let tmp = RefCell::new(cur);
            //     let sss = ss as *const Repo as *mut Repo;
            //     let sss_real = ss_real as *const Repo as *mut Repo;
            //     // let sss = &mut (*ss) as *mut Repo;
            //     // tmp.replace(sss);
            //     let ptr_cpy = cur as *mut *mut Repo;
            //     // let ptr_cpy = *mut cur_ptr;
            //     *ptr_cpy = sss;
            //     // *ptr_cpy = sss_real;
            //     // tmp.borrow_mut() = ss;
            // }

            // unsafe {
            //     // let x: i32 = 0;
            //     // let x_ref = &x;
            //     let sss = ss as *const Repo as *mut Repo;
            //     // let sss = ss as *const Repo;
            //     // let x_ptr = ss as *mut Repo;
            //     // let x_ptr = ss as *const Repo as *mut i32;
            //     cur = sss;
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
            // Lets focus on these tags for now
            s.focus_id("tag-pool").expect("...")
            // s.focus_id("tag-display").expect("...")
        })
        .scrollable()
        .min_width(20)
        .max_height(10)
        .with_id("repo-field");
    // let tags_displayer: IdView<BoxView<SelectView>> = OnEventView()
    let tags_displayer  = OnEventView::new(
        SelectView::new()
            .with_all(selectify_strings(
                // &vec!("hoo", "lah", "laa")
                //     // .iter()
                //     .into_iter()
                //     // .map(AsRef::as_ref)
                //     .map(String::from)
                //     .collect()
                unsafe {
                    // &(*cur)
                    &(**c3po)
                        .get_tags()
                        // .tags
                        // .clone()
                        // .into_iter()
                        // // .map(String::from)
                        // .map(|x| x.name)
                        // .collect::<Vec<String>>()
                }
            ))
            .with_id("tag-display")
            .min_width(20)
            .max_height(10)

    ).on_event(Event::Key(Key::Esc), |s|
        s.focus_id("repo-field").expect("...")
    ).on_event(Event::Key(Key::Backspace), move |s| {
        let mut this: ViewRef<SelectView> = s.find_id("tag-display").unwrap();
        // this.clear();
        if let Some(id) = this.selected_id() {
            let name = this.selection().unwrap();
            let cb = this.remove_item(id);
            cb(s);
            unsafe {
                &(**c3po)
                    .untag(&name);
            }
        }
    });
    let ct = selectify_rc_tags(&config_tags.clone());
    // let fo_c = rreps.clone();
    let mut rcur1 = rcur.clone();
    let tags_pool: IdView<SelectView> = SelectView::new()
            // .with_all(selectify_rc_tags(
            //     &config_tags.clone()
            // ))
            .with_all(selectify_strings(
                &ct
            ))
            .on_submit(move |s: &mut Cursive, ss| {
                unsafe {
                    // (&rcur2).replace(sss_real);
                    // ptr_cpy_two = sss_real;

                    // let mut ptr_cpy_two = cur as *mut Repo;
                    // (*ptr_cpy_two).
                    //     tags
                    //     .push(RepoTag::new(ss));
                    // let fake3 = &cur3 as *const *mut Repo as *mut *mut Repo;
                    // (*fake3) = (*fake3).add(1);

                    (**c3po)
                    // (*cur)
                        .tags
                        .push(RepoTag::new(ss));

                    updated_display_tags(s, &(**c3po));


                    // let tmp1 = rcur1
                    //     // .borrow_mut()
                    //     .deref();
                    // let tmp2 = tmp1
                    //     // .borrow_mut()
                    //     .deref()
                    //     .borrow_mut();
                    // RefMut::map(tmp2, |x| {
                    //     (**x).tags
                    //     .push(RepoTag::new(ss));
                    //     return x;
                    // });

                    // // &(*cur)
                    // //     .tags
                    // //     // .tags.write(848)
                    // //     // .tags = old_tags
                    // //         // .push(RepoTag::new(ss));
                    // //     .push(RepoTag::new(ss));
                    let strs = vec!(
                        format!("--------------------------------------------"),
                        format!("ADDING A TAG"),
                        format!("ss:                  {:?}", (ss)),
                        format!("&cur:                {:?}", (&cur as *const *mut Repo)),
                        format!("cur:                 {:?}", (cur)),
                        format!("*cur:                {:?}", (*cur)),
                        format!("cur3:                {:?}", (cur3)),
                        format!("c3po:                {:?}", (c3po)),
                        format!("*c3po:               {:?}", (*c3po)),
                        format!("------------------------------------------\n"),
                    );
                    debug_write_file(strs, "tmp_out");
                }
            })

            // .with_all(selectify_strings(
            //     // vec!("more", "stuff")
            //     {
            //         // let sss: Rc<RefCell<Vec<RepoTag>>> = ttags.to_owned();
            //         let sss: RefMut<Vec<RepoTag>> =
            //             config_tags
            //             // ttags
            //             // .clone()
            //             .deref()
            //             // .to_owned()
            //             // .into_inner();
            //             .borrow_mut();
            //             // .cloned();
            //         &sss
            //             .iter()
            //             .cloned()
            //             .map(|r| r.name.clone())
            //             .collect()
            //     }
            // user_config.tags
            //     .iter()
            //     .map(|r| r.name.as_str())
            //     .collect()
        // ))
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
                        //     .on_event(Event::Key(Key::Escape), |s1| {
                        // })
                    )
                    .on_event_inner(Event::Key(Key::Esc), |s1| {
                        let cb = Callback::from_fn(
                            |siv: &mut Cursive| {
                                siv.focus_id("repo-field");
                            }
                        );
                        return Some(EventResult::Consumed(Some(cb)));
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
    let more_reps = rreps.clone();
    siv.add_global_callback('q', move |s1| {
        s1.quit();
        debug!("Current Repo Name: {:?},  Tags {:?}", unsafe {
                        &(*cur).path
                    }, unsafe {
                        &(*cur).tags
                    });
        debug!("REPOS PPPPPP: {:?}",
            unsafe {
                &*(more_reps.as_ptr())
            });
        debug!("cur is {:?}, orig ref ptr is {:?}", cur, repsreps);
        debug!("original ref: {:?}", unsafe {
            &(*repsreps)
        });
        let strs = vec!(
            format!("--------------------------------------------"),
            format!("FINALS"),
            format!("cur:                 {:?}", (cur)),
            format!("&cur:                {:?}", (&cur as *const *mut Repo)),
            format!("reps:                {:?}", (more_reps.as_ptr())),
            format!("cur3:                {:?}", (cur3)),
            format!("------------------------------------------\n"),
        );
        debug_write_file(strs, "tmp_out");

        // RefCell::into_inner(rreps.clone().get_mut()));
        // debug!("REPOS ARE: {:?}", rreps.clone().borrow_mut().into_inner());
        // GitGlobalConfig::new().cache_repos(&rreps.deref().borrow());

    });
    siv.run();
    debug!("Current Repo Name: {:?},  Tags {:?}", unsafe {
                &(*cur).path
            }, unsafe {
                &(*cur).tags
            });

    debug!("ADD TAGS: called - 33");

    // debug!("Current Total Tags {:?}", uc.get_cached_results().all_tags());
    // debug!("Current Total Tags {:?}", ttags.deref().borrow());

    // println!("new tags is {:?}", &fake_tags);
    fn updated_display_tags(siv: &mut Cursive, r: &Repo) {
        // siv.call_on_id("tag-pool", |v: &mut SelectView| {
        let found = siv.call_on_id("tag-display", |v: &mut SelectView| {
            // &siv.focus_id("tag-display");
            // v.select_down(1);
            v.clear();
            // v.add_all_str(
            //     vec!(
            //         "pp",
            //         "oo"
            //     )
            // );
            v.add_all(selectify_strings(
                    unsafe {
                        // &(*cur)
                        &r
                            // .as_ref()
                            .tags
                            .clone()
                            .into_iter()
                            // .map(String::from)
                            .map(|x| x.name)
                            .collect::<Vec<String>>()
                    }
            ));
        });
        // let found: Option<ViewRef<SelectView>> = siv.find_id("tag-display");
        // if let Some(foo) = found {
        //     // found.
        //     siv.call_on_id("tag-pool", |v: &mut SelectView| {
        //         v.select_down(1);
        //     });
        // } else {
        //     siv.call_on_id("tag-pool", |v: &mut SelectView| {
        //         v.clear();
        //     });
        // }
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
