use std::fmt;
use git2;

/// A git repository, represented by the full path to its base directory.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Repo {
    pub path: String,
    pub tags: Vec<RepoTag>,
}

impl Repo {
    pub fn new(path: String) -> Repo {
        Repo {
            path: path,
            tags: vec![],
        }
    }

    /// Returns the full path to the repo as a `String`.
    pub fn path(&self) -> &str {
        self.path.as_str()
    }

    /// Returns the `git2::Repository` equivalent of this repo.
    pub fn as_git2_repo(&self) -> Option<git2::Repository> {
        git2::Repository::open(&self.path).ok()
    }

    pub fn tag(&mut self, tag: &str) -> () {
        self.tags.push(RepoTag::new(tag));
    }
}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct RepoTag {
    pub name: String
}

impl RepoTag {
    pub fn new(name: &str) -> RepoTag {
        RepoTag {
            name: name.to_string()
        }
    }
}

impl fmt::Display for RepoTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RepoTag: {}", self.name)
    }
}
