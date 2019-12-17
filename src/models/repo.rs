use git2;
use std::fmt;
// use std::iter::FromIterator;
use crate::models::repo_tag::RepoTag;
use itertools::Itertools;
use std::path::Path;

/// A git repository, represented by the full path to its base directory.
#[derive(
    Serialize, Deserialize, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone,
)]
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
        Path::new(&self.path).file_name().unwrap().to_str().unwrap()
        // Path::new(self.path).file_stem()
    }

    /// Returns the `git2::Repository` equivalent of this repo.
    pub fn as_git2_repo(&self) -> git2::Repository {
        git2::Repository::open(&self.path).ok().expect(
            "Could not open {} as a git repo. Perhaps you should run \
             `git global scan` again.",
        )
    }

    /// Returns "short format" status output.
    pub fn get_status_lines(
        &self,
        mut status_opts: git2::StatusOptions,
    ) -> Vec<String> {
        let git2_repo = self.as_git2_repo();
        let statuses = git2_repo
            .statuses(Some(&mut status_opts))
            .expect(&format!("Could not get statuses for {}.", self));
        statuses
            .iter()
            .map(|entry| {
                let path = entry.path().unwrap();
                let status = entry.status();
                let status_for_path = self.get_short_format_status(status);
                format!("{} {}", status_for_path, path)
            })
            .collect()
    }

    /// Translates a file's status flags to their "short format" representation.
    ///
    /// Follows an example in the git2-rs crate's `examples/status.rs`.
    fn get_short_format_status(&self, status: git2::Status) -> String {
        let mut istatus = match status {
            s if s.is_index_new() => 'A',
            s if s.is_index_modified() => 'M',
            s if s.is_index_deleted() => 'D',
            s if s.is_index_renamed() => 'R',
            s if s.is_index_typechange() => 'T',
            _ => ' ',
        };
        let mut wstatus = match status {
            s if s.is_wt_new() => {
                if istatus == ' ' {
                    istatus = '?';
                }
                '?'
            }
            s if s.is_wt_modified() => 'M',
            s if s.is_wt_deleted() => 'D',
            s if s.is_wt_renamed() => 'R',
            s if s.is_wt_typechange() => 'T',
            _ => ' ',
        };
        if status.is_ignored() {
            istatus = '!';
            wstatus = '!';
        }
        if status.is_conflicted() {
            istatus = 'C';
            wstatus = 'C';
        }
        // TODO: handle submodule statuses?
        format!("{}{}", istatus, wstatus)
    }

    // =====================================================
    //  TAGS
    // =====================================================

    pub fn tag(&mut self, tag: &str) -> () {
        self.tags.push(RepoTag::new(tag));
    }

    pub fn has_tag(&mut self, tag: &str) -> bool {
        self.tags.iter().any(|t| t.name == tag)
    }

    pub fn untag(&mut self, tag: &str) -> () {
        let id_match = self.tags.iter().position(|x| x.name == tag);
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
            .collect();
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

pub trait Filterable {
    // pub trait Filterable<T> {
    // let repos: Vec<Repo>;
    // type repos
    fn filter_tags(&self, tags: Vec<RepoTag>) -> Vec<Repo>;
    fn filter_paths(&self, path: String) -> Vec<Repo>;
    // fn filter_tags(&self, tags: Vec<RepoTag>) -> Vec<Repo> {
    //     self.repos
    //         .iter()
    //         .filter(|r| r.path.contains("ug"))
    //         .collect()
    // }
}

impl Filterable for Vec<Repo> {
    fn filter_tags(&self, tags: Vec<RepoTag>) -> Vec<Repo> {
        self.clone()
            .into_iter()
            .filter(|r| r.tags.iter().any(|rt| tags.contains(rt)))
            .collect()
    }
    fn filter_paths(&self, path: String) -> Vec<Repo> {
        self.clone()
            .into_iter()
            .filter(|r| r.path.contains(&path))
            // .filter(|r| r.tags.iter().any(|rt| tags.contains(rt)))
            .collect()
    }
}

pub trait Updatable {
    fn merge_repos(&self, repos: Vec<Repo>) -> Vec<Repo>;
    fn merge_tags(&self, tags: Vec<RepoTag>) -> Vec<RepoTag>;
    fn merge_repos_and_tags(
        &self,
        repos: Vec<Repo>,
        tags: Vec<RepoTag>,
    ) -> (Vec<Repo>, Vec<RepoTag>);
}

impl Updatable for crate::models::config::GitGlobalConfig {
    fn merge_repos(&self, repos: Vec<Repo>) -> Vec<Repo> {
        repos
    }
    fn merge_tags(&self, tags: Vec<RepoTag>) -> Vec<RepoTag> {
        tags
    }
    fn merge_repos_and_tags(
        &self,
        repos: Vec<Repo>,
        tags: Vec<RepoTag>,
    ) -> (Vec<Repo>, Vec<RepoTag>) {
        let merge_func = |r1: &Repo, r2: &Repo| false; // r1.path == r2.path; // && r1.tags == r2.tags;
        let mut new_repos: Vec<Repo> = vec![self.repos.clone(), repos]
            .into_iter()
            .concat()
            .into_iter()
            .unique()
            .collect::<Vec<Repo>>();
        // .sort();
        new_repos.sort();
        // .kmerge()
        // .collect();
        // let new_repos = self
        //     .repos
        //     .clone()
        //     .into_iter()
        //     .merge_by(repos, merge_func)
        //     .collect();
        let new_tag_total = vec![];
        (new_repos, new_tag_total)
    }
}

// /// Because I use this everywhere
// impl Vec<Repo> {
//     pub fn filter_by_path(&self, path_filter: String) -> Vec<Repo> {
//         self.clone()
//     }
// }

#[cfg(test)]
mod tests {
    use super::Updatable;
    use crate::models::config::GitGlobalConfig;
    use crate::models::{repo::Repo, repo_tag::RepoTag};

    fn vec_from_vecs<T>(s: Vec<&str>, f: Box<dyn FnMut(&str) -> T>) -> Vec<T>
// fn vec_from_vecs<T, F>(s: Vec<&str>, f: F) -> Vec<T>
    // where
    //     F: FnOnce(&str) -> T,
    {
        s.into_iter().map(f).collect::<Vec<T>>()
    }

    fn repos_from_vecs(s: Vec<&str>) -> Vec<Repo> {
        // s.into_iter().map(|s| Repo::new(s.to_owned())).collect()
        vec_from_vecs(s, Box::new(|s: &str| Repo::new(s.to_owned())))
        // s.into_iter().map(|s| Repo::new(s.to_owned())).collect()
    }

    fn repotags_from_vecs(s: Vec<&str>) -> Vec<RepoTag> {
        vec_from_vecs(s, Box::new(|s: &str| RepoTag::new(s)))
    }

    #[test]
    pub fn test_merge_repos_and_tags() {
        let mut gc = GitGlobalConfig::new();
        let tags1: Vec<RepoTag> = vec!["apple", "os x", "denite"]
            .into_iter()
            .map(RepoTag::new)
            .collect();
        let tags2: Vec<RepoTag> = vec!["apple", "os windows", "haskell"]
            .into_iter()
            .map(RepoTag::new)
            .collect();
        let repo1: Vec<Repo> =
            vec!["/hal/code/1", "/hal/code/2", "/hal/code/3"]
                .into_iter()
                .map(|s| Repo::new(s.to_owned()))
                .collect();
        let repo2: Vec<Repo> = vec!["/hal/code/1", "/hal/code/4"]
            .into_iter()
            .map(|s| Repo::new(s.to_owned()))
            .collect();
        // PRE-SORTED REPOS
        let repo_final: Vec<Repo> =
            vec!["/hal/code/1", "/hal/code/2", "/hal/code/3", "/hal/code/4"]
                .into_iter()
                .map(|s| Repo::new(s.to_owned()))
                .collect();
        gc.repos = repo1;
        let (r_out, t_out) =
            gc.merge_repos_and_tags(repo2.clone(), tags2.clone());
        assert_eq!(r_out, repo_final, "repo comparison failed!");
        // UNSORTED REPOS
        let repo1: Vec<Repo> =
            vec!["/hal/code/2", "/hal/code/3", "/hal/code/1"]
                .into_iter()
                .map(|s| Repo::new(s.to_owned()))
                .collect();
        gc.repos = repo1;
        let (r_out, t_out) =
            gc.merge_repos_and_tags(repo2.clone(), tags2.clone());
        assert_eq!(
            r_out, repo_final,
            "repo comparison failed for unsorted data!"
        );
        // UNEQUAL REPOS
        let repo1: Vec<Repo> = vec!["/hal/code/2", "/hal/code/8"]
            .into_iter()
            .map(|s| Repo::new(s.to_owned()))
            .collect();
        gc.repos = repo1;
        let (r_out, t_out) =
            gc.merge_repos_and_tags(repo2.clone(), tags2.clone());
        assert_ne!(r_out, repo_final, "repo comparison succeeded when it should have failed due to not equal inputs!");
    }
}
