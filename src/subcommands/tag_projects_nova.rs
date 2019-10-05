use std;
// use std::any::Any;
// use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
// use std::iter::Zip;
use std::ops::Deref;
use std::rc::Rc;
extern crate cursive;
// use itertools::rciter;
// use itertools::Itertools;

use std::fs::OpenOptions;
use std::io::Write;

// use macros::rc_mut;

use self::cursive::event::{Callback, Event, EventResult, Key};
use self::cursive::traits::*;
use self::cursive::views::{
    EditView, IdView, LinearLayout, OnEventView, Panel, SelectView,
    TextContent, ViewRef,
};
use self::cursive::Cursive;
use repo::errors::Result as WeirdResult;
use repo::{
    save_repos_and_tags, GitGlobalConfig, GitGlobalResult, Repo, RepoTag,
};
// use mut_static::MutStatic;
use std::cell::Ref;
// use std::iter::FromIterator;
// use take_mut;

// use std::fmt;

type RMut = Rc<RefCell<TextContent>>;
type RcResult = Rc<GitGlobalResult>;
type RcRcResult = Rc<RefCell<GitGlobalResult>>;
type RcRepo = Rc<RefCell<Repo>>;
type RcRepoTag = Rc<RefCell<RepoTag>>;
type RcVecRepoTag = Rc<RefCell<Vec<RepoTag>>>;
type RcVecRepo = Rc<RefCell<Vec<Repo>>>;

/// Not sure if I use this here
struct TagStatus {
    repos: RcVecRepo,
    current_repo: RcRepo,
    current_tags: RcVecRepoTag,
}

impl TagStatus {
    pub fn new_from_rc(
        repos: RcVecRepo,
        repo: RcRepo,
        tags: RcVecRepoTag,
    ) -> TagStatus {
        TagStatus {
            repos: repos,
            current_repo: repo,
            current_tags: tags,
        }
    }
}

pub fn repo_2_name<'a>(s: &'a str) -> &'a str {
    s.rsplit("/").collect::<Vec<&str>>().first().unwrap()
}

pub fn go<'a>() -> WeirdResult<GitGlobalResult> {
    // note a pointer
    let uc = GitGlobalConfig::new();
    // let user_config = Box::new(&uc);
    // let uRepos: Box<&GitGlobalConfig> = user_config.clone();

    let mut reps: Vec<Repo> = uc
        .get_cached_repos()
        .into_iter()
        // .take(5)
        .collect();
    let repsreps = &reps as *const Vec<Repo>;
    let results = uc.get_cached_results();
    let result_tags: Vec<RepoTag> = results
        .all_tags()
        .into_iter()
        .cloned()
        // .map(|&x| x)
        // .map(AsRef::asref)
        .collect();

    // NOTE: unsafe
    // let cur: [Repo] = reps.borrow();
    let mut cur = reps.as_mut_ptr();
    let c3po = &cur as *const *mut Repo;
    // let c3po = &cur as *const *mut Repo as *mut *mut Repo;
    let mut cur2 = reps.as_mut_ptr();
    let mut cur3 = reps.as_mut_ptr();
    let mut rcur = Rc::new(RefCell::new(cur2));

    let strs = vec![
        format!("--------------------------------------------"),
        format!("INITIALS"),
        format!("&cur:                {:?}", (&cur as *const *mut Repo)),
        format!("cur:                 {:?}", (cur)),
        format!("reps:                {:?}", (reps.as_mut_ptr())),
        format!("cur3:                {:?}", (cur3)),
        format!("c3po:                {:?}", (c3po)),
        format!("------------------------------------------\n"),
    ];
    debug_write_file(strs, "tmp_out");

    // let pos: usize = 0;
    // let posptr = &pos as *mut usize;
    // let mut cur_ptr = reps.as_mut_ptr();

    let rct = reps.clone();
    let repo_names = &rct.iter().map(|x| x.path.clone()).zip(rct.iter());

    let rreps = Rc::new(RefCell::new(reps));
    let ttags = Rc::new(RefCell::new(result_tags));
    // NOTE: This is just until we have some actually tagged repos
    let config_tags = Rc::new(RefCell::new(uc.tags));

    trace!("go");

    let mut siv = Cursive::default();
    siv.load_theme_file("assets/style.toml").unwrap();

    // https://github.com/gyscos/Cursive/issues/179

    #[allow(dead_code)]
    type SelRepoList<'a> =
        std::iter::Zip<std::vec::IntoIter<&'a str>, std::vec::IntoIter<Repo>>;

    #[allow(dead_code)]
    type SelRepoList2 = std::iter::Zip<String, Repo>;

    type SelTagList<'a> = std::iter::Zip<
        // Vec<&'a str>,
        // Vec<String>
        // std::iter::Map<&'a str>,
        std::vec::IntoIter<&'a str>,
        std::vec::IntoIter<String>, // std::slice::Iter<&'a, &str>,
                                    // std::vec::IntoIter<&'a str>,
                                    // itertools::RcIter<std::vec::IntoIter<String>>
    >;

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

    pub use std::vec::IntoIter;

    // type SelRepIter = Zip<IntoIter<String>, IntoIter<RcRepo>>;
    type SelRepIter<'a> = &'a Vec<(String, RcRepo)>;
    // type SelRepIter = Vec<(String, RcRepo)>;
    // type SelRepIter<'a> = Vec<(String, &'a RcRepo)>;

    fn selectify_repos(repos: RcVecRepo) -> Vec<(String, Repo)> {
        return RefCell::borrow_mut(&repos)
            // .deref()
            // .borrow_mut()
            .clone()
            .into_iter()
            .map(|r| (r.path.clone(), r))
            // .map(|r| (r.path.clone(), Rc::new(RefCell::new(r))))
            .collect();
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
        .with_all(selectify_repos(rreps.clone()))
        .on_select(move |s: &mut Cursive, ss: &Repo| {
            let rcin: Ref<Vec<Repo>> = rreps_1.deref().borrow();
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
                let mut ptr_cpy_four =
                    &cur as *const *mut Repo as *mut *mut Repo;

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
                let strs = vec![
                    format!("--------------------------------------------"),
                    format!("BEGIN"),
                    format!("sss_real:                  {:?}", (sss_real)),
                    format!("*sss_real:                 {:?}", (*sss_real)),
                    format!(
                        "&cur:                      {:?}",
                        (&cur as *const *mut Repo)
                    ),
                    format!("cur:                       {:?}", (cur)),
                    format!("*cur:                      {:?}", (*cur)),
                    format!("ptr_cpy_two:               {:?}", (ptr_cpy_two)),
                    format!("*ptr_cpy_two:              {:?}", (*ptr_cpy_two)),
                    format!("ptr_cpy_three:             {:?}", (ptr_cpy_three)),
                    format!(
                        "*ptr_cpy_three:            {:?}",
                        (*ptr_cpy_three)
                    ),
                    format!("ptr_cpy_four:              {:?}", (ptr_cpy_four)),
                    format!("*ptr_cpy_four:             {:?}", (*ptr_cpy_four)),
                    format!("c3po:                      {:?}", (c3po)),
                    format!("*c3po:                     {:?}", (*c3po)),
                    // format!("c3p4*:                     {:?}", (c3p4)),
                    // format!("*c3p4:                     {:?}", (*c3p4)),
                    // format!("fake3:                     {:?}", (fake3)),
                    format!("--------------------------------------------\n"),
                ];
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
                        },
                    );
            }
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
    let tags_displayer = OnEventView::new(
        SelectView::new()
            .with_all(selectify_strings(unsafe {
                &(**c3po).get_tags()
                // .tags
                // .clone()
                // .into_iter()
                // // .map(String::from)
                // .map(|x| x.name)
                // .collect::<Vec<String>>()
            }))
            .with_id("tag-display")
            .min_width(20)
            .max_height(10),
    )
    .on_event(Event::Key(Key::Esc), |s| {
        s.focus_id("repo-field").expect("...")
    })
    .on_event(Event::Key(Key::Backspace), move |s| {
        let mut this: ViewRef<SelectView> = s.find_id("tag-display").unwrap();
        // this.clear();
        if let Some(id) = this.selected_id() {
            let name = this.selection().unwrap();
            let cb = this.remove_item(id);
            cb(s);
            unsafe {
                &(**c3po).untag(&name);
            }
        }
        // toggle_bg(*this);
    });
    let ct = selectify_rc_tags(&config_tags.clone());
    // let fo_c = rreps.clone();
    let mut rcur1 = rcur.clone();
    let tags_pool: IdView<SelectView> = SelectView::new()
        .with_all(selectify_strings(&ct))
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

                if (**c3po).has_tag(ss) {
                    return;
                }

                (**c3po)
                    // (*cur)
                    .tags
                    .push(RepoTag::new(ss));

                updated_display_tags(s, &(**c3po));
                // let strs = vec!(
                debug_file!(
                    "tmp_out",
                    format!("--------------------------------------------"),
                    format!("ADDING A TAG"),
                    format!("ss:                  {:?}", (ss)),
                    format!(
                        "&cur:                {:?}",
                        (&cur as *const *mut Repo)
                    ),
                    format!("cur:                 {:?}", (cur)),
                    format!("*cur:                {:?}", (*cur)),
                    format!("cur3:                {:?}", (cur3)),
                    format!("c3po:                {:?}", (c3po)),
                    format!("*c3po:               {:?}", (*c3po)),
                    format!("------------------------------------------\n")
                );
                // );
                // debug_write_file(strs, "tmp_out");
            }
        })
        .with_id("tag-pool");

    // Main Window
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
            ),
    );
    // #[rock]
    siv.add_global_callback('q', move |s1| {
        // s1.quit();
        trace!("agg1");
        let more_reps = rreps.clone();
        let more_tags = config_tags.clone();
        // let more_tags = ttags.clone();
        save_repos_and_quit(s1, more_reps, more_tags);
        // save_repos_and_quit(s1, more_reps, more_tags, cur);
        // save_repos_and_quit(s1, more_reps.clone(), ttags.clone(), repsreps);
        trace!("agg2");
        // debug!("Current Repo Name: {:?},  Tags {:?}", unsafe {
        //                 &(*cur).path
        //             }, unsafe {
        //                 &(*cur).tags
        //             });
        // debug!("REPOS PPPPPP: {:?}",
        //     unsafe {
        //         &*(more_reps.as_ptr())
        //     });
        // debug!("cur is {:?}, orig ref ptr is {:?}", cur, repsreps);
        // debug!("original ref: {:?}", unsafe {
        //     &(*repsreps)
        // });
        // let strs = vec!(
        //     format!("--------------------------------------------"),
        //     format!("FINALS"),
        //     format!("cur:                 {:?}", (cur)),
        //     format!("&cur:                {:?}", (&cur as *const *mut Repo)),
        //     format!("reps:                {:?}", (more_reps.as_ptr())),
        //     format!("cur3:                {:?}", (cur3)),
        //     format!("------------------------------------------\n"),
        // );
        // debug_write_file(strs, "tmp_out");

        // RefCell::into_inner(rreps.clone().get_mut()));
        // debug!("REPOS ARE: {:?}", rreps.clone().borrow_mut().into_inner());
        // GitGlobalConfig::new().cache_repos(&rreps.deref().borrow());
    });
    siv.run();
    debug!(
        "Current Repo Name: {:?},  Tags {:?}",
        unsafe { &(*cur).path },
        unsafe { &(*cur).tags }
    );

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
            v.add_all(selectify_strings(unsafe {
                // &(*cur)
                &r
                    // .as_ref()
                    .tags
                    .clone()
                    .into_iter()
                    // .map(String::from)
                    .map(|x| x.name)
                    .collect::<Vec<String>>()
            }));
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

    // fn toggle_bg(sel: SelectView) {
    //     let xy = XY {
    //         x: 0,
    //         y: 0,
    //     };
    //     let printer = Printer {
    //         size: sel.required_size(xy)
    //     };
    //     sel.draw(printer.with_color(ColorStyle::primary, || {
    //         printer.print()
    //     }));
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
        unsafe {
            trace!("srq4");
            // let check_tags: Vec<String> = all_tags(&(*repsmo))
            // let check_tags = all_tags(&(**c4))
            // let check_tags = all_tags(&tmp.borrow())
            //     .iter()
            //     .map(|x| format!("TTAA: {}", x.name))
            //     .collect();
            trace!("srq5");
            let beg: Vec<String> = vec![
                format!("--------------------------------------------"),
                format!("QUITTING TIME"),
            ];
            trace!("srq6");
            let end: Vec<String> =
                vec![format!("--------------------------------------------")];
            trace!("srq7");
            // let ttt = Itertools::kmerge(vec![
            //             beg.into_iter(),
            //             check_tags.into_iter(),
            //             end.into_iter()
            // ].into_iter()).collect();
            trace!("srq8");
            // debug_write_file(ttt, "tmp_out");
            trace!("srq9");
            // debug_write_file(check_tags, "tmp_out");
        }
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
