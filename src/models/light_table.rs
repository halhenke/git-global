//! A Useful struct when dealing with cursive callbacks and Selectviews
//! [`SelectView`]: ../../../cursive/views/select_view/struct.SelectView.html

use crate::models::repo::Filterable;
use crate::models::{config::GitGlobalConfig, repo::Repo, repo_tag::RepoTag};
use itertools::Itertools;
use std::ops::Deref;
use std::{cell::RefCell, rc::Rc};

/// Used to pass around info in PromptCursive
/// where we must use a shared struct with
/// Interior Mutability to model UI state
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LightTable<'a> {
    pub repos: Vec<Repo>,
    // TODO: - Should this just be a reference?
    pub filtered_repos: Vec<&'a Repo>,
    pub repo_index: usize,
    pub tag_index: usize,
    pub tags: Vec<RepoTag>,
    pub default_tags: Vec<RepoTag>,
    pub repo_filter: String,
}

impl LightTable<'_> {
    // impl<'a> LightTable<'a> {
    /// Standard init function
    pub fn new(
        repos: Vec<Repo>,
        filtered_repos: Vec<&Repo>,
        repo_index: usize,
        tag_index: usize,
        tags: Vec<RepoTag>,
        default_tags: Vec<RepoTag>,
        repo_filter: String,
    ) -> LightTable {
        LightTable {
            repos,
            filtered_repos,
            repo_index,
            tag_index,
            tags,
            default_tags,
            repo_filter,
        }
    }

    /// Returns a [`LightTable`] wrapped in an <Rc<RefCell>>
    /// This is the most useful form for the purposes of working
    /// with the Cursive library
    pub fn new_from_rc(
        // pub fn new_from_rc<'a>(
        repos: Vec<Repo>,
        filtered_repos: Vec<&Repo>,
        repo_index: usize,
        tag_index: usize,
        tags: Vec<RepoTag>,
        default_tags: Vec<RepoTag>,
        repo_filter: String,
    ) -> Rc<RefCell<LightTable<'_>>> {
        Rc::new(RefCell::new(Self::new(
            repos,
            filtered_repos,
            repo_index,
            tag_index,
            tags,
            default_tags,
            repo_filter,
        )))
    }

    /// Construct a LightTable from a [`GitGlobalConfig`] (LightTable is probably slightly superfluous as a new data structure actually...)
    pub fn new_from_ggc<'a>(
        gc: GitGlobalConfig,
        path_filter: Option<String>,
    ) -> Rc<RefCell<LightTable<'a>>> {
        // ) -> Rc<RefCell<LightTable<'_>>> {
        // ) -> Rc<RefCell<LightTable<'a>>> {
        let reps: Vec<Repo> = if let Some(pf) = path_filter {
            // NOTE: Adds an extra clone to use filter_paths ¯\_(ツ)_/¯
            gc.get_cached_repos().filter_paths(&pf)
        // .into_iter()
        // .filter(|r| r.path.contains(&pf))
        // .collect()
        } else {
            gc.get_cached_repos()
        };
        Self::new_from_rc(
            // reps.clone(),
            // TODO: - Should this just be a reference?
            reps,
            reps.iter().collect(),
            0,
            0,
            vec![],
            gc.default_tags,
            "".to_owned(),
        )
    }

    // /// chainable function to apply a simple filter to
    // /// the [`Repo`] paths so that the `tags` field
    // /// now contains only those repos matching the filter
    // pub fn filter_repos<'b: 'c, 'c>(
    //     &'b mut self,
    //     path_filter: &'c str,
    // ) -> &'b mut Self {
    //     // self.filtered_repos = self.repos.filter_paths(path_filter);

    //     self.filtered_repos = self
    //         // let filtered_repos: Vec<&'a Repo> = self
    //         .repos
    //         .iter()
    //         .filter(|r| r.path.contains(&path_filter))
    //         // .collect::<Vec<&'a Repo>>();
    //         .collect();
    //     // .clone()
    //     // .into_iter()
    //     // .filter(|r| r.path.contains(&path_filter))
    //     // .collect();
    //     self
    // }

    /// Helper function for the Cursive [`SelectView`](../../../cursive/views/select_view/struct.SelectView.html).
    ///
    /// Return the `repos` field as a [`Vec`] of label, index tuples suitable for input to a [`SelectView`](../../../cursive/views/select_view/struct.SelectView.html) in `Cursive`
    pub fn selectify_repos(&self) -> Vec<(&str, usize)> {
        self.repos
            .iter()
            .enumerate()
            .map(|(i, r)| (r.path.as_str(), i))
            .collect::<Vec<(&str, usize)>>()
    }

    /// Helper function for the Cursive [`SelectView`](../../../cursive/views/select_view/struct.SelectView.html)
    ///
    /// Return a `Vec` of (label, index) pairs from the tags associated with the currently selected [`Repo`]
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

    /// Helper function for the Cursive `SelectView`.
    ///
    /// Returns a [`Vec`] of (label, index) pairs from all the tags currently assigned to one or more Repos *plus* the set of `default_tags` that are globally assigned by [`GitGlobalConfig`]
    pub fn all_the_tags(&self) -> Vec<(String, usize)> {
        let mut r = self
            .repos
            .iter()
            .flat_map(|r| r.tags.iter().map(|t| t.name.clone()))
            .chain::<Vec<String>>(
                self.default_tags.iter().map(|r| r.name.clone()).collect(),
            )
            .unique()
            .enumerate()
            .map(|(i, t)| (t, i))
            .collect::<Vec<(String, usize)>>();
        r.sort();
        r
    }

    /// Running this will both "refresh/recalculate" the list of
    /// tags and return them in a form you can use with a [`SelectView`](../../../cursive/views/select_view/struct.SelectView.html)
    pub fn retags(&mut self) -> Vec<(String, usize)> {
        self.reset_all_tags();
        self.tags_as_list()
    }

    pub fn rerepos<'b>(&mut self) -> Vec<(String, usize)> {
        let s = self.repo_filter.clone();
        // TODO: Try to figure out a sensible way to leave the
        // index the same if possible
        // let old_repos = self.filtered_repos.clone()

        // self.filter_repos(&s);

        // TODO: Should this be in filter_repos?
        self.repo_index = 0;
        // self.repos
        //     .filter_paths(&self.repo_filter)
        self.filtered_repos
            .iter()
            .enumerate()
            .map(|(i, r)| (r.path.clone(), i))
            // .collect::<Vec<(&str, usize)>>()
            .collect::<Vec<(String, usize)>>()
    }

    /// Take our current list of tags and put them into the form of a `Vec` of (tag name, index) pairs.
    pub fn tags_as_list(&self) -> Vec<(String, usize)> {
        self.tags
            .iter()
            .map(|r| r.name.clone())
            .enumerate()
            .map(|(i, t)| (t, i))
            .collect()
    }

    /// Helper function for the Cursive [`SelectView`](../../../cursive/views/select_view/struct.SelectView.html).
    ///
    /// Recalculate the list of tags available to choose from based on the list of `repos` and the prepopulated `default_tags`
    pub fn reset_all_tags(&mut self) {
        let mut _tmp: Vec<RepoTag> = self
            .repos
            .iter()
            .flat_map(|r| r.tags.clone())
            .chain::<Vec<RepoTag>>(self.default_tags.clone())
            .unique()
            .collect::<Vec<RepoTag>>();
        _tmp.sort();
        self.tags = _tmp;
    }

    /// This should go - it just returns a bunch of fake tags
    // pub fn all_tags(&self) -> Vec<(String, usize)> {
    //     vec!["haskell", "ml", "rust", "apple", "web dev"]
    //         .iter()
    //         .map(|t| RepoTag::new(t))
    //         .enumerate()
    //         .map(|(i, t)| (t.name, i))
    //         .collect::<Vec<(String, usize)>>()
    // }

    /// Will add a tag to the current_repo *only if it is not already there*, and will recalculate the total tag list.
    pub fn add_tag(&mut self, rt: &RepoTag) -> bool {
        let current_repo = self
            .repos
            .get_mut(self.repo_index)
            .expect("could not get current repo");
        if current_repo.tags.contains(rt) {
            return false;
        }
        current_repo.tags.push(rt.clone());
        self.retags();
        return true;
    }
}

use std::convert::From;

impl<'a> From<GitGlobalConfig> for LightTable<'a> {
    fn from(gc: GitGlobalConfig) -> Self {
        let repos = gc.get_cached_repos();
        LightTable::new(
            repos,
            repos.iter().collect(),
            0,
            0,
            gc.tags,
            gc.default_tags,
            "".to_owned(),
        )
    }
}

pub type RcVecRepoTag = Rc<RefCell<Vec<RepoTag>>>;
pub type RcVecRepo = Rc<RefCell<Vec<Repo>>>;

type SelTagList<'a> =
    std::iter::Zip<std::vec::IntoIter<&'a str>, std::vec::IntoIter<String>>;

// =================================================
//  Selectify  Functions
// =================================================

// fn selectify_strings<'a>(tags_1: &'a Vec<String>) -> SelTagList<'a> {
//     let tags_2: Vec<&'a str> = tags_1.iter().map(AsRef::as_ref).collect();
//     return tags_2.into_iter().zip(tags_1.to_vec());
// }

// fn selectify_rc_tags<'a>(rctags: &'a RcVecRepoTag) -> Vec<String> {
//     return rc_borr!(rctags)
//         .iter()
//         .map(|r| r.name.clone())
//         .collect::<Vec<String>>();
// }

// fn selectify_repos(repos: &RcVecRepo) -> Vec<(String, Repo)> {
//     return RefCell::borrow_mut(&repos)
//         .clone()
//         .into_iter()
//         .map(|r| (r.path.clone(), r))
//         .collect();
// }

// /// General selectifier for RC types
// fn selectify_rc_things<R>(
//     // fn selectify_rc_things<R, T>(
//     things: &Rc<RefCell<Vec<R>>>,
//     map_fn: impl Fn(R) -> (String, R), // note: This gives a Sized error when used with `dyn` instead of `impl`
// ) -> Vec<(String, R)>
// where
//     R: Clone,
//     // T: IntoIterator<
//     //     Item = (String, R),
//     //     // IntoIter = ::std::vec::IntoIter<(String, R)>,
//     // >,
// {
//     return RefCell::borrow_mut(&things)
//         .clone()
//         .into_iter()
//         .map(map_fn)
//         // .collect::<T>();
//         .collect();
//     // let strs: Vec<String> = RefCell::borrow_mut(things.deref())
//     //     .iter()
//     //     .map(|f| format!("{:?}", f))
//     //     .collect();
//     // return strs.into_iter().zip(things.into_iter()).collect();
// }

// fn selectify_rc_things_backwards<R>(
//     things: &Rc<RefCell<Vec<R>>>,
//     map_fn: impl Fn(R) -> (R, String), // note: This gives a Sized error when used with `dyn` instead of `impl`
// ) -> Vec<(R, String)>
// where
//     R: Clone,
// {
//     return RefCell::borrow_mut(&things)
//         .clone()
//         .into_iter()
//         .map(map_fn)
//         .collect();
//     // let strs: Vec<String> = RefCell::borrow_mut(things.deref())
//     //     .iter()
//     //     .map(|f| format!("{:?}", f))
//     //     .collect();
//     // return strs.into_iter().zip(things.into_iter()).collect();
// }

// fn selectify_things_two<T>(
//     things: Vec<T>,
//     map_fn: impl Fn(T) -> (String, T),
// ) -> Vec<(String, T)>
// where
//     T: std::fmt::Debug,
// {
//     // let strs: Vec<String> = things.into_iter().map(map_fn).collect();
//     let strs = things.into_iter().map(map_fn).collect();
//     // let strs: Vec<String> = things.iter().map(|f| format!("{:?}", f)).collect();
//     // return strs.into_iter().zip(things.into_iter()).collect();
//     return strs;
// }

// fn selectify_things<T>(things: Vec<T>) -> Vec<(String, T)>
// where
//     T: std::fmt::Debug,
// {
//     let strs: Vec<String> = things.iter().map(|f| format!("{:?}", f)).collect();
//     return strs.into_iter().zip(things.into_iter()).collect();
//     // return things.into_iter().zip(strs.iter()).collect();
// }
