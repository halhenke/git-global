use git2;
use std::fmt;
// use std::iter::FromIterator;
use crate::models::Repo;
use std::path::Path;

/// All tags from a Vec of repos
pub fn all_tags<'a>(reps: &Vec<Repo>) -> Vec<RepoTag> {
    let mut v: Vec<RepoTag> = Vec::new();
    // reps.iter()
    // .map(|x| &x.tags)
    // .collect()
    // let mut vi = v.ter();
    for r in reps {
        // (&vi).chain(r.tags.iter());
        // &v.append(r.tags.iter())
        let mut t: Vec<RepoTag> = r.tags.clone();
        &v.append(&mut t);
    }
    // return v.iter().flatten().collect();
    // return vi.collect();
    return v;
}

#[derive(
    Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug,
)]
pub struct RepoTag {
    pub name: String,
}

/// Basically a wrapper around a string ¯\_(ツ)_/¯
impl RepoTag {
    pub fn new(name: &str) -> RepoTag {
        RepoTag {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for RepoTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RepoTag: {}", self.name)
    }
}

// impl fmt::Display for Vec<RepoTag> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // write!(f, "RepoTag: {}", self.name)
//         // unimplemented!()
//     }
// }

/// RepoTag is basically a wrapper around a string
impl From<String> for RepoTag {
    fn from(name: String) -> RepoTag {
        Self { name }
    }
}

impl From<RepoTag> for String {
    fn from(repo: RepoTag) -> String {
        repo.name
    }
}

pub fn does_this_work(reps: Vec<RepoTag>) -> Vec<String> {
    // return reps.into();
    reps.into_iter()
        .map(|x| x.into()) //make use of from implementation...
        .collect()
}
