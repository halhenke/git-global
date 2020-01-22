use crate::models::{
    action::Action, errors::Result, repo::Repo, repo::Updatable,
    repo_tag::RepoTag, result::GitGlobalResult, utils::new_find_repos,
};
use anyhow::Context;
// use config::{Environment, File as CFile, Source, Value};
use std::collections::hash_map::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsRaw {
    // BEGINNING: String,
    // OUT: i32,
    basedir: Option<String>,
    basedirs: Option<Vec<String>>,
    // ignored_paths: Option<Vec<HashMap<String, String>>>,
    ignored_paths: Option<Vec<String>>,
    ignored_patterns: Option<Vec<String>>,
    path_shortcuts: Option<HashMap<String, String>>,
    // path_shortcuts: HashMap<String, String>,
    ignored_repos: Option<Vec<Repo>>,
    default_repos: Option<Vec<Repo>>,
    default_tags: Option<Vec<RepoTag>>,
    actions: Option<Vec<Action>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    /// NOTE: Basedir shouldnt be defaulted to an empty string as we need at least one place to scan? This might change if we wish to override at CLI
    pub basedir: Option<String>,
    pub basedirs: Vec<String>,
    // pub ignored_paths: Vec<HashMap<String, String>>,
    pub ignored_paths: Vec<String>,
    pub ignored_patterns: Vec<String>,
    pub path_shortcuts: HashMap<String, String>,
    // path_shortcuts: HashMap<String, String>,
    pub ignored_repos: Vec<Repo>,
    pub default_repos: Vec<Repo>,
    pub default_tags: Vec<RepoTag>,
    pub actions: Vec<Action>,
}

impl From<SettingsRaw> for Settings {
    fn from(sr: SettingsRaw) -> Settings {
        Settings {
            basedir: sr.basedir,
            basedirs: sr.basedirs.unwrap_or(vec![]),
            // ignored_paths: sr.ignored_paths.unwrap_or(vec![]),
            ignored_paths: sr.ignored_paths.unwrap_or(vec![]),
            ignored_patterns: sr.ignored_patterns.unwrap_or(vec![]),
            path_shortcuts: sr.path_shortcuts.unwrap_or(HashMap::new()),
            // path_shortcuts: HashMap<String, String>,
            ignored_repos: sr.ignored_repos.unwrap_or(vec![]),
            default_repos: sr.default_repos.unwrap_or(vec![]),
            default_tags: sr.default_tags.unwrap_or(vec![]),
            actions: sr.actions.unwrap_or(vec![]),
        }
    }
}
