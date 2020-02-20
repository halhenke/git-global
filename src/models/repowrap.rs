use crate::models::{repo::Repo, repo_tag::RepoTag};
use itertools::Itertools;
use std::collections::{hash_map::RandomState, hash_set::HashSet};
use std::hash::{Hash, Hasher};
use std::iter::{Extend, FromIterator};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, PartialOrd, Ord, Clone)]
pub struct RepoWrap(Repo);

impl From<Repo> for RepoWrap {
    fn from(r: Repo) -> Self {
        RepoWrap(r)
    }
}

impl PartialEq for RepoWrap {
    fn eq(&self, other: &Self) -> bool {
        self.0.path == other.0.path
    }
}

impl Eq for RepoWrap {}

/// I need Hash to only rely on path and not tags for
/// equality/difference/union etc of Sets to work
impl Hash for RepoWrap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.path.hash(state);
    }
}

// Because of orphan trait rule i have to either
// - define my own trait
// - add another local type to wrap/Vec and define from on that
// - just do a function...

pub trait MyFrom<T> {
    fn from(t: T) -> Self;
}

impl MyFrom<Vec<Repo>> for Vec<RepoWrap> {
    fn from(repos: Vec<Repo>) -> Self {
        repos.into_iter().map(RepoWrap).collect()
    }
}

pub trait MyInto<T> {
    fn into(self) -> T;
}

impl MyInto<Vec<Repo>> for Vec<RepoWrap> {
    fn into(self) -> Vec<Repo> {
        self.into_iter().map(|rw| rw.0).collect()
    }
}

// pub trait Mergeable<T> // where
//     Self = T,
pub trait Mergeable {
    fn merge_other(&mut self, other: Self) -> Self;
}

impl Mergeable for Vec<RepoWrap> {
    fn merge_other(&mut self, other: Self) -> Self {
        let me: HashSet<RepoWrap, RandomState> =
            HashSet::from_iter(self.clone());
        let notme: HashSet<RepoWrap, RandomState> = HashSet::from_iter(other);
        let mut keepers: HashSet<RepoWrap, RandomState> =
            me.difference(&notme).cloned().collect();
        keepers.extend(notme);
        let mut keepers: Vec<RepoWrap> = keepers.into_iter().collect();
        keepers.sort();
        return keepers;
        // unimplemented!
        // ANCHOR Hey - this is where we go
    }
}

// impl RepoWrap {
//     pub merge_wraps()
// }
