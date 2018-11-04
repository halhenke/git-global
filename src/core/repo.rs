use std::fmt;
use git2;
use std::path::Path;
use std::iter::FromIterator;

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

    pub fn untag(&mut self, tag: &str) -> () {
        let id_match = self.tags
            .iter()
            .position(|x| x.name == tag);
        if let Some(id) = id_match {
            self.tags.remove(id);
        }
    }

    pub fn get_tags(&self) -> Vec<String> {
        return self
            .tags
            .clone()
            .into_iter()
            .map(|x| String::from(x))
            .collect()
    }
}
// type VecRep = Vec<Repo>;

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

// impl FromIterator<RepoTag> for Vec<RepoTag> {
//     fn from_iter<I: IntoIterator<Item=RepoTag>>(iter: I) -> Vec<RepoTag> {
//         let v = Vec::new();
//         for i in iter {
//             v.push(i)
//         }
//         v
//     }
// }

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
