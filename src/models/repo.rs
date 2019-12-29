use git2;
use std::fmt;
// use std::iter::FromIterator;
use crate::models::repo_tag::RepoTag;
use itertools::Itertools;
use std::hash::Hash;
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
    fn tags_from_repos(&self, repos: Vec<Repo>) -> Vec<RepoTag>;
    fn recalculate_tags(&self) -> Vec<RepoTag>;
    fn reset_tags(&mut self);
    // Generic Functions
    fn merge_things<T: Clone + Eq + Hash + Ord>(
        &self,
        things_one: Vec<T>,
        things_two: Vec<T>,
    ) -> Vec<T>;
    fn delete_things<T: Clone + Eq + Hash + Ord>(
        &self,
        things_one: Vec<T>,
        things_two: Vec<T>,
    ) -> Vec<T>;
    // Doing Stuff
    fn merge_repos(&self, repos: Vec<Repo>) -> Vec<Repo>;
    fn merge_tags(&self, tags: Vec<RepoTag>) -> Vec<RepoTag>;
    fn update_repos(
        &self,
        add_repos: Vec<Repo>,
        delete_repos: Vec<Repo>,
    ) -> Vec<Repo>;
    fn update_tags(
        &self,
        add_tags: Vec<RepoTag>,
        delete_tags: Vec<RepoTag>,
    ) -> Vec<RepoTag>;
    fn merge_repos_and_tags(
        &self,
        repos: Vec<Repo>,
        tags: Vec<RepoTag>,
    ) -> (Vec<Repo>, Vec<RepoTag>);
    fn merge_and_delete_repos_and_tags(
        &self,
        add_repos: Vec<Repo>,
        delete_repos: Vec<Repo>,
        add_tags: Vec<RepoTag>,
        delete_tags: Vec<RepoTag>,
    ) -> (Vec<Repo>, Vec<RepoTag>);
    fn update_merge_repos_and_tags(
        &mut self,
        repos: Vec<Repo>,
        tags: Vec<RepoTag>,
    );
    fn update_and_delete_merge_repos_and_tags(
        &mut self,
        add_repos: Vec<Repo>,
        delete_repos: Vec<Repo>,
        add_tags: Vec<RepoTag>,
        delete_tags: Vec<RepoTag>,
    );
    // fn proper_repos_update(&mut self, repos: Vec<Repo>) -> Vec<Repo>;
    fn proper_repos_update(
        &mut self,
        original: Vec<Repo>,
        updates: Vec<Repo>,
    ) -> Vec<Repo>;

    fn proper_repos_self_update(&mut self, updates: Vec<Repo>);
    fn efficient_repos_update(&mut self, updates: Vec<Repo>);
}

impl Updatable for crate::models::config::GitGlobalConfig {
    /// functional version to extract tags from a given Vec of repos
    fn tags_from_repos(&self, repos: Vec<Repo>) -> Vec<RepoTag> {
        repos.into_iter().flat_map(|r| r.tags).collect()
    }

    /// Get tags for existing list of Repos and merge with default tags
    /// Returns tags but DOES NOT update self.tags
    /// NOTE: Not sure about that - maybe should separate update and calculate
    fn recalculate_tags(&self) -> Vec<RepoTag> {
        // Get tags for existing list of Repos
        let existing_tags = self.tags_from_repos(self.repos.clone());
        // Merge with default tags
        let mut new_tags: Vec<RepoTag> =
            self.merge_things(self.default_tags.clone(), existing_tags);
        new_tags.sort();
        // self.tags = new_tags.clone();
        new_tags
    }

    /// Basically run recalculate_tags and then assign the result
    /// to self.tags
    fn reset_tags(&mut self) {
        self.tags = self.recalculate_tags();
    }

    fn merge_things<T>(&self, things_one: Vec<T>, things_two: Vec<T>) -> Vec<T>
    where
        T: Clone + Eq + Hash + Ord,
    {
        let mut new_things: Vec<T> = vec![things_one, things_two]
            .into_iter()
            .concat()
            .into_iter()
            .unique()
            .collect::<Vec<T>>();
        new_things.sort();
        new_things
    }

    fn delete_things<T>(&self, things_one: Vec<T>, things_two: Vec<T>) -> Vec<T>
    where
        T: Clone + Eq + Hash + Ord,
    {
        // let mut new_things: Vec<T> = vec![things_one, things_two]
        let mut new_things: Vec<T> = things_one
            .into_iter()
            // .concat()
            .filter(|v| !things_two.contains(v))
            // .into_iter()
            .unique()
            .collect::<Vec<T>>();
        new_things.sort();
        new_things
    }

    fn merge_repos(&self, repos: Vec<Repo>) -> Vec<Repo> {
        self.merge_things(self.repos.clone(), repos)
    }

    fn merge_tags(&self, tags: Vec<RepoTag>) -> Vec<RepoTag> {
        self.merge_things(self.recalculate_tags(), tags)
    }

    fn update_repos(
        &self,
        add_repos: Vec<Repo>,
        delete_repos: Vec<Repo>,
    ) -> Vec<Repo> {
        let tmp_repos = self.merge_repos(add_repos);
        self.delete_things(tmp_repos, delete_repos)
    }

    fn update_tags(
        &self,
        add_tags: Vec<RepoTag>,
        delete_tags: Vec<RepoTag>,
    ) -> Vec<RepoTag> {
        let tmp_tags = self.merge_tags(add_tags);
        self.delete_things(tmp_tags, delete_tags)
    }

    /// This will
    /// - merge new repos with existing repos,
    /// - update self.tags based on tags applied to new repose and the default tag list
    /// - merge another set of tags into tags
    /// NOTE: Not sure this makes sense
    /// 1. Why would i have tags in my list outside of default tags and those associated with repos
    /// 2. I cant delete repos with this method - only add more
    fn merge_repos_and_tags(
        &self,
        repos: Vec<Repo>,
        tags: Vec<RepoTag>,
    ) -> (Vec<Repo>, Vec<RepoTag>) {
        let new_repos = self.merge_repos(repos);
        let new_tag_total = self.merge_tags(tags);
        (new_repos, new_tag_total)
    }

    fn merge_and_delete_repos_and_tags(
        &self,
        add_repos: Vec<Repo>,
        delete_repos: Vec<Repo>,
        add_tags: Vec<RepoTag>,
        delete_tags: Vec<RepoTag>,
    ) -> (Vec<Repo>, Vec<RepoTag>) {
        let new_repos = self.update_repos(add_repos, delete_repos);
        let new_tag_total = self.update_tags(add_tags, delete_tags);
        (new_repos, new_tag_total)
    }

    fn update_merge_repos_and_tags(
        &mut self,
        repos: Vec<Repo>,
        tags: Vec<RepoTag>,
    ) {
        self.repos = self.merge_repos(repos);
        // Need to do this after merge_repos and before merge_tags
        // so that we have all tags (and default tags) to update from
        self.reset_tags();
        self.tags = self.merge_tags(tags);
    }

    fn update_and_delete_merge_repos_and_tags(
        &mut self,
        add_repos: Vec<Repo>,
        delete_repos: Vec<Repo>,
        add_tags: Vec<RepoTag>,
        delete_tags: Vec<RepoTag>,
    ) {
        self.repos = self.update_repos(add_repos, delete_repos);
        self.reset_tags();
        self.tags = self.update_tags(add_tags, delete_tags);
    }

    /// Tried to do this with a mutable reference but you cant borrow self and self.repos more than oncee
    fn proper_repos_update(
        &mut self,
        mut original: Vec<Repo>,
        updates: Vec<Repo>,
    ) -> Vec<Repo> {
        for repo in updates {
            if let Some(r) = original.iter_mut().find(|r| r.path == repo.path) {
                r.tags = repo.tags;
            }
        }
        original
    }

    /// This is the same as proper_repos_self_update but it doesnt require a clone on repos - couldnt find a way to do that without reimplementing
    fn efficient_repos_update(&mut self, updates: Vec<Repo>) {
        for repo in updates {
            if let Some(r) = self.repos.iter_mut().find(|r| r.path == repo.path)
            {
                r.tags = repo.tags;
            }
        }
        self.reset_tags();
    }

    fn proper_repos_self_update(&mut self, updates: Vec<Repo>) {
        // let repos: &Vec<Repo> = self.proper_repos_update(self.repos, updates);
        self.repos = self.proper_repos_update(self.repos.clone(), updates);
        self.reset_tags();
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
        vec_from_vecs(s, Box::new(|s: &str| Repo::new(s.to_owned())))
    }

    fn repotags_from_vecs(s: Vec<&str>) -> Vec<RepoTag> {
        vec_from_vecs(s, Box::new(|s: &str| RepoTag::new(s)))
    }

    #[test]
    pub fn test_merge_repos_and_tags() {
        let mut gc = GitGlobalConfig::new();
        let tags1: Vec<RepoTag> =
            repotags_from_vecs(vec!["apple", "os x", "denite"]);
        let tags2: Vec<RepoTag> = vec!["apple", "os windows", "haskell"]
            .into_iter()
            .map(RepoTag::new)
            .collect();
        let repo1: Vec<Repo> =
            repos_from_vecs(vec!["/hal/code/1", "/hal/code/2", "/hal/code/3"]);
        let repo2: Vec<Repo> =
            repos_from_vecs(vec!["/hal/code/1", "/hal/code/4"]);
        // PRE-SORTED REPOS
        let repo_final: Vec<Repo> = repos_from_vecs(vec![
            "/hal/code/1",
            "/hal/code/2",
            "/hal/code/3",
            "/hal/code/4",
        ]);
        gc.repos = repo1;
        let (r_out, t_out) =
            gc.merge_repos_and_tags(repo2.clone(), tags2.clone());
        assert_eq!(r_out, repo_final, "repo comparison failed!");
        // UNSORTED REPOS
        let repo1: Vec<Repo> =
            repos_from_vecs(vec!["/hal/code/2", "/hal/code/3", "/hal/code/1"]);
        gc.repos = repo1;
        let (r_out, t_out) =
            gc.merge_repos_and_tags(repo2.clone(), tags2.clone());
        assert_eq!(
            r_out, repo_final,
            "repo comparison failed for unsorted data!"
        );
        // UNEQUAL REPOS
        let repo1: Vec<Repo> =
            repos_from_vecs(vec!["/hal/code/2", "/hal/code/8"]);
        gc.repos = repo1;
        let (r_out, t_out) =
            gc.merge_repos_and_tags(repo2.clone(), tags2.clone());
        assert_ne!(r_out, repo_final, "repo comparison succeeded when it should have failed due to not equal inputs!");
    }

    #[test]
    pub fn test_efficient_updates() {
        let mut gc = GitGlobalConfig::new();
        let gc_repos: Vec<Repo> = repos_from_vecs(vec![
            "/hal/code/1",
            "/hal/code/2",
            "/hal/code/3",
            "/hal/code/4",
        ]);
        let mut test_rep_1 = Repo::new("/hal/code/2".to_owned());
        let mut test_rep_2 = Repo::new("/hal/code/3".to_owned());
        test_rep_1.tags =
            vec![RepoTag::new("bat-bath"), RepoTag::new("birdface!")];
        test_rep_2.tags = vec![RepoTag::new("camelo"), RepoTag::new("ordinaf")];
        gc.repos = gc_repos;
        gc.efficient_repos_update(vec![test_rep_1.clone(), test_rep_2.clone()]);
        if let Some(res_1) = gc.repos.iter().find(|r| r.path == test_rep_1.path)
        {
            ic!(res_1.tags);
            ic!(res_1.path);
            ic!(gc.repos);
            assert!(res_1.tags == test_rep_1.tags);
        } else {
            panic!("res_1 tags are not equal")
        }
        if let Some(res_2) = gc.repos.iter().find(|r| r.path == test_rep_2.path)
        {
            ic!(res_2.tags);
            ic!(res_2.path);
            ic!(gc.tags);
            assert!(res_2.tags == test_rep_2.tags);
        } else {
            panic!("res_2 tags are not equal")
        }
        gc.cache_repos(&gc.repos);
    }
}
