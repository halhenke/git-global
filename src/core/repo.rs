use std::fmt;
use git2;
use std::path::Path;

/// A git repository, represented by the full path to its base directory.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
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

    /// Returns the name of the repo as a `String`.
    pub fn name(&self) -> &str {
        Path::new(&self.path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
        // Path::new(self.path).file_stem()
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
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

// impl fmt::Display for Vec<RepoTag> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // write!(f, "RepoTag: {}", self.name)
//         // unimplemented!()
//     }
// }

/// RepoTag is basically a wrapper around a string
impl From<String> for RepoTag {
    fn from(name: String) -> RepoTag {
        Self {
            name
        }
    }
}

impl From<RepoTag> for String {
    fn from(repo: RepoTag) -> String {
        repo.name
    }
}

// impl From<Vec<RepoTag>> for Vec<String> {
//     fn (reps) {
//         reps.iter()
//             .map(|x| x.name)
//             .collect()
//     }
// }
// impl Into<Vec<String>> for Vec<RepoTag> {
//     fn (reps) {

//     }
// }

pub fn does_this_work(reps: Vec<RepoTag>) -> Vec<String> {
    // return reps.into();
    reps.into_iter()
        .map(|x| x.into()) //make use of from implementation...
        .collect()
}
