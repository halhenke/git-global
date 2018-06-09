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

/// RepoTag is basically a wrapper around a string
impl From<String> for RepoTag {
    fn from(name: String) -> RepoTag {
        Self {
            name
        }
    }
}

// impl From<String<'a>> for &'a mut RepoTag {
//     fn from(name: String) -> & mut RepoTag {
//         Self {
//             name
//         }
//     }
// }

// use std::iter::{FromIterator};

// impl<'a> FromIterator<RepoTag> for mut Vec<RepoTag> {
//     // fn from_iter(iter: RepoTag) -> &'a mut Vec<RepoTag> {
//     // type IntoIterator = IntoIter<RepoTag>;
//     fn from_iter<I: IntoIterator<Item = RepoTag>>(iter: I) -> mut Vec<RepoTag> {
//     // fn from_iter<T: IntoIterator<RepoTag>>(iter: IntoIterator<RepoTag>) -> &'a mut Vec<RepoTag> {
//         // let ref mut v = Vec::new();
//         // let ref mut v: mut Vec<> = Vec::new();

//         let v = mut Vec::new();
//         // let v: &mut Vec<_> = &mut Vec::new();

//         for i in iter {
//             v.push(i);
//         }
//         return v;
//         // return &'a v;
//         // unimplemented!()
//     }
// }

// impl<'a> FromIterator<RepoTag> for &'a mut Vec<RepoTag> {
//     // fn from_iter(iter: RepoTag) -> &'a mut Vec<RepoTag> {
//     // type IntoIterator = IntoIter<RepoTag>;
//     fn from_iter<I: IntoIterator<Item = RepoTag>>(iter: I) -> &'a mut Vec<RepoTag> {
//     // fn from_iter<T: IntoIterator<RepoTag>>(iter: IntoIterator<RepoTag>) -> &'a mut Vec<RepoTag> {
//         // let ref mut v = Vec::new();
//         // let ref mut v: mut Vec<> = Vec::new();

//         let mut v = Vec::new();
//         // let v: &mut Vec<_> = &mut Vec::new();

//         for i in iter {
//             v.push(i);
//         }
//         return &mut v;
//         // return &'a v;
//         // unimplemented!()
//     }
// }


// #[stable(feature = "rust1", since = "1.0.0")]
// impl<A> FromIterator<A> for VecDeque<A> {
//     fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> VecDeque<A> {
//         let iterator = iter.into_iter();
//         let (lower, _) = iterator.size_hint();
//         let mut deq = VecDeque::with_capacity(lower);
//         deq.extend(iterator);
//         deq
//     }
// }

