use itertools::Itertools;
use repo::{errors, GitGlobalError, GitGlobalResult, Repo, RepoTag};
use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LightTable {
    pub repos: Vec<Repo>,
    pub repo_index: usize,
    pub tag_index: usize,
    pub tags: Vec<RepoTag>,
}

impl LightTable {
    pub fn new(
        repos: Vec<Repo>,
        repo_index: usize,
        tag_index: usize,
        tags: Vec<RepoTag>,
    ) -> LightTable {
        LightTable {
            repos,
            repo_index,
            tag_index,
            tags,
        }
    }
    pub fn new_from_rc(
        repos: Vec<Repo>,
        repo_index: usize,
        tag_index: usize,
        tags: Vec<RepoTag>,
    ) -> Rc<RefCell<LightTable>> {
        Rc::new(RefCell::new(Self::new(repos, repo_index, tag_index, tags)))
    }

    pub fn selectify_repos(&self) -> Vec<(&str, usize)> {
        self.repos
            .iter()
            .enumerate()
            .map(|(i, r)| (r.path.as_str(), i))
            .collect::<Vec<(&str, usize)>>()
    }

    pub fn selectify_tags(&self, index: usize) -> Vec<(&str, usize)> {
        self.repos
            .iter()
            .nth(index)
            .expect("ERROR - index requested outside of repos bounds")
            .tags
            .iter()
            .enumerate()
            .map(|(i, t)| (t.name.as_str(), i))
            .collect::<Vec<(&str, usize)>>()
    }

    pub fn all_the_tags(&self) -> Vec<(String, usize)> {
        let mut r = self
            .repos
            .iter()
            .flat_map(|r| r.tags.iter().map(|t| t.name.clone()))
            .chain::<Vec<String>>(
                vec!["haskell", "ml", "rust", "apple", "web dev"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
            )
            .unique()
            .enumerate()
            .map(|(i, t)| (t, i))
            .collect::<Vec<(String, usize)>>();
        r.sort();
        r
    }

    pub fn retags(&mut self) -> Vec<(String, usize)> {
        self.reset_all_tags();
        self.tags_as_list()
    }

    pub fn tags_as_list(&self) -> Vec<(String, usize)> {
        self.tags
            .iter()
            .map(|r| r.name.clone())
            .enumerate()
            .map(|(i, t)| (t, i))
            .collect()
    }

    pub fn reset_all_tags(&mut self) {
        let mut _tmp: Vec<(RepoTag)> = self
            .repos
            .iter()
            .flat_map(|r| r.tags.clone())
            .chain::<Vec<RepoTag>>(
                vec!["haskell", "ml", "rust", "apple", "web dev"]
                    .into_iter()
                    .map(RepoTag::new)
                    // .map(String::from)
                    .collect(),
            )
            .unique()
            .collect::<Vec<RepoTag>>();
        _tmp.sort();
        self.tags = _tmp;
    }

    pub fn all_tags(&self) -> Vec<(String, usize)> {
        vec!["haskell", "ml", "rust", "apple", "web dev"]
            .iter()
            .map(|t| RepoTag::new(t))
            .enumerate()
            .map(|(i, t)| (t.name, i))
            .collect::<Vec<(String, usize)>>()
    }

    pub fn add_tag(&mut self, rt: &RepoTag) -> bool {
        let current_repo = self
            .repos
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
