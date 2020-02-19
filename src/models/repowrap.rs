use crate::models::{repo::Repo, repo_tag::RepoTag};
use itertools::Itertools;
use std::hash::Hash;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, PartialOrd, Ord, Hash, Clone)]
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

// pub trait Mergeable<T> // where
//     Self = T,
pub trait Mergeable {
    fn merge_other(&mut self, other: Self) -> Self;
}

impl Mergeable for RepoWrap {
    fn merge_other(&mut self, other: Self) -> Self {
        return other;
        // unimplemented!
        // ANCHOR Hey - this is where we go
    }
}

// impl RepoWrap {
//     pub merge_wraps()
// }
