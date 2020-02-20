//! A Useful struct when dealing with cursive callbacks and Selectviews
//! [`SelectView`]: ../../../cursive/views/select_view/struct.SelectView.html

use crate::models::repo::Filterable;
use crate::models::repowrap::{Mergeable, MyFrom, MyInto, RepoWrap};
use crate::models::{config::GitGlobalConfig, repo::Repo, repo_tag::RepoTag};
use itertools::Itertools;
use std::collections::{hash_map::RandomState, hash_set::HashSet};
use std::iter::FromIterator;
use std::ops::Deref;
use std::{cell::RefCell, rc::Rc};

/// Used to pass around info in PromptCursive
/// where we must use a shared struct with
/// Interior Mutability to model UI state
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LightTable {
    pub repos: Vec<Repo>,
    // TODO: - Should this just be a reference?
    pub filtered_repos: Vec<Repo>,
    pub repo_index: usize,
    pub tag_index: usize,
    pub tags: Vec<RepoTag>,
    pub default_tags: Vec<RepoTag>,
    pub repo_filter: String,
}

impl LightTable {
    /// Standard init function
    pub fn new(
        repos: Vec<Repo>,
        filtered_repos: Vec<Repo>,
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
        repos: Vec<Repo>,
        filtered_repos: Vec<Repo>,
        repo_index: usize,
        tag_index: usize,
        tags: Vec<RepoTag>,
        default_tags: Vec<RepoTag>,
        repo_filter: String,
    ) -> Rc<RefCell<LightTable>> {
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
    pub fn new_from_ggc(
        gc: GitGlobalConfig,
        path_filter: Option<String>,
    ) -> Rc<RefCell<LightTable>> {
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
            reps.clone(),
            // TODO: - Should this just be a reference?
            reps,
            0,
            0,
            vec![],
            gc.default_tags,
            "".to_owned(),
        )
    }

    /// We want to find the new Vec of filtered_repos but also have to assume that there is an old set that has information we need to add to our complete set of repos (the first time the function is called this should just be an empty set)
    /// This should also be called before we save tags in order to merge any changes from the filtered_list into the repo list that will be saved
    pub fn repo_filter_update(&mut self) {
        let mut old_repos: Vec<RepoWrap> = MyFrom::from(self.repos.clone());
        let old_filtered_repos: Vec<RepoWrap> =
            MyFrom::from(self.filtered_repos.clone());
        self.repos = MyInto::into(old_repos.merge_other(old_filtered_repos));
        self.filtered_repos = self.repos.filter_paths(&self.repo_filter);
    }

    /// chainable function to apply a simple filter to
    /// the [`Repo`] paths so that the `tags` field
    /// now contains only those repos matching the filter
    pub fn filter_repos(&mut self, path_filter: &str) -> &mut Self {
        self.filtered_repos = self.repos.filter_paths(path_filter);
        // .clone()
        // .into_iter()
        // .filter(|r| r.path.contains(&path_filter))
        // .collect();
        self
    }

    // /// We want to find the new Vec of filtered_repos but also have to assume that there is an old set that has information we need to add to our complete set of repos (the first time the function is called this should just be an empty set)
    // /// OK This is fucked - much easier to just copy tags across to original Vector list...
    // pub fn set_filter_repos(&mut self, path_filter: &str) -> Vec<Repo> {
    //     let old_filtered: HashSet<Repo, RandomState> =
    //         HashSet::from_iter(self.filtered_repos.clone());
    //     let old_no_filter: HashSet<Repo, RandomState> =
    //         HashSet::from_iter(self.repos.clone());
    //     let old_filtered_paths: Vec<String> = self
    //         .filtered_repos
    //         .clone()
    //         .into_iter()
    //         .map(|r| r.path)
    //         .collect();
    //     let unfiltered_og: Vec<Repo> = self
    //         .repos
    //         .clone()
    //         .into_iter()
    //         .filter(|r| !old_filtered_paths.contains(&r.path))
    //         .collect();
    //     // let merged = old_no_filter
    //     let filtered = self.repos.filter_paths(path_filter);
    //     let repo_hash: HashSet<Repo, RandomState> =
    //         HashSet::from_iter(filtered);
    //     return vec![];
    // }

    /// Helper function for the Cursive [`SelectView`](../../../cursive/views/select_view/struct.SelectView.html).
    ///
    /// Return the `filtered_repos` field as a [`Vec`] of label, index tuples suitable for input to a [`SelectView`](../../../cursive/views/select_view/struct.SelectView.html) in `Cursive`
    /// NOTE We use filtered_repos as it is the UI version of repo state
    pub fn selectify_repos(&self) -> Vec<(&str, usize)> {
        self.filtered_repos
            .iter()
            .enumerate()
            .map(|(i, r)| (r.path.as_str(), i))
            .collect::<Vec<(&str, usize)>>()
    }

    /// Helper function for the Cursive [`SelectView`](../../../cursive/views/select_view/struct.SelectView.html)
    ///
    /// Return a `Vec` of (label, index) pairs from the tags associated with the currently selected [`Repo`]
    /// NOTE We use filtered_repos as it is the UI version of repo state
    pub fn selectify_tags(&self, index: usize) -> Vec<(&str, usize)> {
        self
            // .repos
            .filtered_repos
            .iter()
            .nth(index)
            .expect("ERROR - index requested outside of repos bounds")
            .tags
            .iter()
            .enumerate()
            .map(|(i, t)| (t.name.as_str(), i))
            .collect::<Vec<(&str, usize)>>()
    }

    // FIXME - How is this different from `reset_all_tags`?
    /// Helper function for the Cursive `SelectView`.
    ///
    /// Returns a [`Vec`] of (label, index) pairs from all the tags currently assigned to one or more Repos *plus* the set of `default_tags` that are globally assigned by [`GitGlobalConfig`]
    /// NOTE We use repos as well as filtered_repos in order to ensure we display all tags
    pub fn all_the_tags(&self) -> Vec<(String, usize)> {
        let mut r = self
            .repos
            .iter()
            .flat_map(|r| r.tags.iter().map(|t| t.name.clone()))
            .chain::<Vec<String>>(
                self.default_tags.iter().map(|r| r.name.clone()).collect(),
            )
            .chain(
                self.filtered_repos
                    .iter()
                    .flat_map(|r| r.tags.iter().map(|t| t.name.clone())),
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

    pub fn rerepos(&mut self) -> Vec<(String, usize)> {
        // let s = self.repo_filter.clone();
        // TODO: Try to figure out a sensible way to leave the
        // index the same if possible
        // let old_repos = self.filtered_repos.clone()
        // self.filter_repos(&s);
        // TODO: Should this be in filter_repos?
        self.repo_filter_update();
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
    // TODO - common methods for getting list of tags from a Vec of Repos
    /// NOTE We use repos as well as filtered_repos in order to ensure we display all tags

    pub fn reset_all_tags(&mut self) {
        let mut _tmp: Vec<RepoTag> = self
            .repos
            .iter()
            .flat_map(|r| r.tags.clone())
            .chain::<Vec<RepoTag>>(self.default_tags.clone())
            .chain(self.filtered_repos.iter().flat_map(|r| r.tags.clone()))
            .unique()
            .collect::<Vec<RepoTag>>();
        _tmp.sort();
        self.tags = _tmp;
    }

    /// Will add a tag to the current_repo *only if it is not already there*, and will recalculate the total tag list.
    pub fn add_tag(&mut self, rt: &RepoTag) -> bool {
        let current_repo = self
            // .repos
            .filtered_repos
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

impl From<GitGlobalConfig> for LightTable {
    fn from(gc: GitGlobalConfig) -> Self {
        let repos = gc.get_cached_repos();
        LightTable::new(
            repos.clone(),
            repos,
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
