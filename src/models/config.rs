/*!
    Defines the [`GitGlobalConfig`] struct
    At the moment this data structure contains
        - a basedir
        - a list of basedirs
        - a list of [`Tag`]s
        - a list of default_tags
        - a list of actions


*/
// use colored::*;
// use std::env;
use anyhow::Context;
// use anyhow::{Context, Result};
use app_dirs::{app_dir, get_app_dir, AppDataType, AppInfo};
use colored::*;
use config::{Config, ConfigError, Environment, File as CFile, Source, Value};
// use futures::executor::LocalPool;
use crate::models::{
    action::Action,
    errors::{GitGlobalError, Result},
    // errors::GitGlobalError,
    repo::Repo,
    repo::Updatable,
    repo_tag::RepoTag,
    result::GitGlobalResult,
    settings::{Settings, SettingsRaw},
    utils::new_find_repos,
};
use git2;
use std::collections::hash_map::HashMap;
use std::fs::{remove_file, File};
use std::io::{Error, ErrorKind};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
// use std::result::Result as StdResult;
use walkdir::DirEntry;

const APP: AppInfo = AppInfo {
    name: "git-global",
    author: "hal",
};
const CACHE_FILE: &'static str = "repos.txt";
// const TAG_CACHE_FILE: &'static str = "tags.txt";
// const SETTING_BASEDIR: &'static str = "global.basedir";
// const SETTING_IGNORED: &'static str = "global.ignore";
// const SETTINGS_DEFAULT_TAGS: &'static str = "global.default-tags";
// const SETTINGS_DEFAULT_GIT_ACTIONS: &'static str = "global.default-git-actions";
const CONFIG_FILE_NAME: &'static str = ".git_global_config_simple";
const CONFIG_FILE_PROPER: &'static str = "/.git_global_config";
// const ANOTHER: &'static str = dirs::home_dir()
//     .expect("Could not determine home directory.")
//     .to_str()
//     .expect("Could not convert home directory path to string.");
// .to_string();

// const fn goooo() -> String {
//     dirs::home_dir()
//         .expect("Could not determine home directory.")
//         .to_str()
//         .expect("Could not convert home directory path to string.")
//         .to_string()
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrentState {
    pub tags: Vec<RepoTag>,
    pub repos: Vec<Repo>,
    pub actions: Vec<Repo>,
}

// NOTE: Get rid of this if it isnt used soon
impl CurrentState {
    pub fn new() -> Self {
        CurrentState {
            tags: vec![],
            repos: vec![],
            actions: vec![],
        }
    }
}

/// A container for git-global configuration options.
/// By Default these options are gathered from the global `.gitconfig`
/// file and cached repos/tags are retrieved/stored on disk
#[derive(Serialize, Deserialize, Debug)]
pub struct GitGlobalConfig {
    pub basedir: String,
    pub basedirs: Vec<String>,
    pub repos: Vec<Repo>,
    pub current: CurrentState,
    // pub current_repos: Vec<Repo>,
    pub ignored_patterns: Vec<String>,
    // TODO: This should probably not be here - or it should not be relied upon as a source of truth.
    // Instead tags should
    // be derived from those associated with repos and the default tags
    pub tags: Vec<RepoTag>,
    pub default_tags: Vec<RepoTag>,
    pub ignored_repos: Vec<Repo>,
    pub ignored_paths: Vec<String>,
    pub default_repos: Vec<Repo>,
    pub actions: Vec<Action>,
    pub cache_file: PathBuf,
    // pub tags_file: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct RepoTagCache {
    repos: Vec<Repo>,
    tags: Vec<RepoTag>,
}

impl RepoTagCache {
    fn new(repos: &Vec<Repo>, tags: &Vec<RepoTag>) -> RepoTagCache {
        RepoTagCache {
            repos: repos.clone(),
            tags: tags.clone(),
        }
    }
}

impl GitGlobalConfig {
    /// Create a new Git Configuration Settings Object
    /// - has both
    ///     - a set of default tags
    ///     - a set of default actions
    /// if they are defined in the global .gitconfig file
    pub fn new() -> GitGlobalConfig {
        trace!("GitGlobalConfig::new");
        let home_dir = dirs::home_dir()
            .expect("Could not determine home directory.")
            .to_str()
            .expect("Could not convert home directory path to string.")
            .to_string();

        let settings: Settings = GitGlobalConfig::get_parsed_config()
            .expect("Parsing of your Settings file failed");

        assert!(
            &settings.basedir.is_some(),
            "You must provide a basedir in the settings."
        );
        assert!(
            Path::exists(Path::new(&settings.basedir.as_ref().unwrap())),
            "Your provided basedir: {} does not exist",
            &settings.basedir.as_ref().unwrap()
        );
        let cache_file =
            match get_app_dir(AppDataType::UserCache, &APP, "cache") {
                Ok(mut dir) => {
                    dir.push(CACHE_FILE);
                    dir
                }
                Err(_) => panic!("TODO: work without XDG"),
            };
        // let tags_file = match get_app_dir(AppDataType::UserCache, &APP, "cache")
        // {
        //     Ok(mut dir) => {
        //         dir.push(TAG_CACHE_FILE);
        //         dir
        //     }
        //     Err(_) => panic!("TODO: work without XDG"),
        // };

        // NOTE: Handle this earlier
        // if basedir == "" {
        //     unimplemented!();
        // }

        let ggc = GitGlobalConfig {
            basedir: settings.basedir.unwrap(),
            basedirs: settings.basedirs,
            current: CurrentState::new(),
            repos: vec![],
            tags: vec![],
            default_tags: settings.default_tags,
            default_repos: settings.default_repos,
            // default_paths: vec![],
            ignored_repos: settings.ignored_repos,
            ignored_paths: settings.ignored_paths,
            actions: settings.actions,
            ignored_patterns: settings.ignored_patterns,
            cache_file: cache_file,
            // tags_file,
        };
        // TODO: get rid of this
        // ggc.tags = ggc.read_tags().unwrap_or(vec![]);
        ggc
    }

    /// Initialise With Repos
    pub fn new_with_repos() -> Self {
        trace!("GitGlobalConfig::new_with_repos");
        let mut gc = GitGlobalConfig::new();
        gc.get_repos();
        gc
    }

    /// A version that prepopulates some set of tags and/or actions
    pub fn new_with_defaults(tags: Vec<RepoTag>, actions: Vec<Action>) -> Self {
        trace!("GitGlobalConfig::new_with_defaults");
        let mut gcc = Self::new();
        gcc.tags = tags;
        gcc.actions = actions;
        gcc
    }

    fn get_config() -> Result<HashMap<String, Value>> {
        // fn get_config() -> HashMap<String, Value> {
        let mut HOME_CONFIG = dirs::home_dir()
            .expect("Could not determine home directory.")
            .to_str()
            .expect("Could not convert home directory path to string.")
            .to_string();
        HOME_CONFIG.push_str(CONFIG_FILE_PROPER);
        let mut c = Config::default();
        c.merge(CFile::with_name(HOME_CONFIG.as_mut_str()))?
            // .expect("Merge of Configuration File Values failed")
            // .or_else(|e| return Err(e))
            .merge(Environment::with_prefix("GIT_GLOBAL"))?
            // .expect("Merge of Environment Configuration Values failed")
            .collect()
            .context("couldnt get your config file")
        // .expect("Config: Conversion to hashMap Failed")
    }

    fn get_raw_config() -> Result<Config> {
        let mut HOME_CONFIG = dirs::home_dir()
            .expect("Could not determine home directory.")
            .to_str()
            .expect("Could not convert home directory path to string.")
            .to_string();
        HOME_CONFIG.push_str(CONFIG_FILE_PROPER);
        let mut c = Config::default();
        c.merge(CFile::with_name(HOME_CONFIG.as_mut_str()))?
            .merge(Environment::with_prefix("GIT_GLOBAL"))?;
        // .unwrap();
        // .context("Trying to read config from .config filee")
        Ok(c)
    }

    pub fn get_parsed_config() -> Result<Settings> {
        // GitGlobalConfig::get_raw_config().map(|c| c.try_into().unwrap())
        GitGlobalConfig::get_raw_config()?
            .try_into::<SettingsRaw>()
            .map(|sr| Settings::from(sr))
            .context("Parse config into Settings")
    }

    /// Add tags to the [`GitGlobalConfig`] object - Chainable
    pub fn with_tags(&mut self, tags: Vec<RepoTag>) -> &mut Self {
        trace!("GitGlobalConfig::with_tags");
        self.tags = tags;
        self
    }

    /// set actions field as true - Chainable
    pub fn with_actions(&mut self, actions: Vec<Action>) -> &mut Self {
        self.actions = actions;
        self
    }

    /// Returns `true` if this directory entry should be included in scans.
    pub fn filter(&self, entry: &DirEntry) -> bool {
        let entry_path = entry.path().to_str().expect("DirEntry without path.");
        (self.ignored_patterns.len() == 1 && self.ignored_patterns[0] == "")
            || !self
                .ignored_patterns
                .iter()
                .any(|pat| entry_path.contains(pat))
    }

    /// Append Vec of Strings as tags
    pub fn append_tags(&mut self, tags: Vec<String>) -> () {
        let new_repos = &mut tags
            .into_iter()
            .map(|t| t.into())
            // .map(|t| RepoTag::from(t))
            .collect();
        debug!("new_repos is {:?}", new_repos);
        debug!("Before add tags - self.tags is {:?}", self.tags);
        self.tags.append(new_repos);
        self.tags.dedup_by(|a, b| {
            a.name.as_str().eq_ignore_ascii_case(b.name.as_str())
        });
    }

    /// Replace current tags with [`Vec<RepoTag>`] given a [`Vec`] of [`String`]s as input
    pub fn replace_tags(&mut self, tags: Vec<String>) -> () {
        let new_tags = tags.into_iter().map(|t| t.into()).collect();
        self.tags = new_tags;
    }

    #[allow(dead_code)]
    fn tags(&self) -> &Vec<RepoTag> {
        &self.tags
    }

    /// Return tags as a [`Vec`] of [`String`]s
    pub fn tag_names(&self) -> Vec<&str> {
        // pub fn tag_names(&self) -> &Vec<&str> {
        self.tags.iter().map(|g| g.name.as_str()).collect()
    }

    pub fn print_tags(&self) {
        if self.tags.is_empty() {
            println!("You have no tags defined as yet");
            return;
        }
        for tag in &self.tags {
            println!("{}", tag.name);
        }
    }
}

/// GitGlobalConfig is mainly responsible for reading Global Configuration settings, and fetching/caching the list of Repos on the system based on that
impl GitGlobalConfig {
    /// Returns boolean indicating if the cache file exists.
    pub fn has_cache(&self) -> bool {
        self.cache_file.as_path().exists()
    }

    pub fn clear_cache(&self) -> Result<()> {
        let mut f = File::create(&self.cache_file)?;
        let rt: RepoTagCache = RepoTagCache::new(&Vec::new(), &Vec::new());
        let serialized = serde_json::to_string(&rt)?;
        f.write_all(serialized.as_bytes())
            .context("trying to write empty cache to disk")
    }

    /// Remove the cache file
    pub fn remove_cache_file(&self) -> Result<()> {
        remove_file(self.cache_file.as_path())
            .context("just trying to get anyhow to work")
    }

    ///Return the cache file
    pub fn print_cache(
        &self,
    ) -> Result<
        GitGlobalResult,
        // crate::models::errors::GitGlobalError,
    > {
        println!(
            "git-global cache is at {}",
            self.cache_file.as_path().display()
        );
        let v: Vec<Repo> = vec![];
        Ok(GitGlobalResult::new(&v))
    }

    #[allow(dead_code)]
    /// Do we have any repos in the cache?
    fn empty_cache(&self) -> bool {
        self.get_cached_repos().len() == 0
    }

    /// Reads the cache and returns Vec of Tags
    pub fn get_cached_repos_and_tags(
        &self,
    ) -> Result<(Vec<Repo>, Vec<RepoTag>)> {
        trace!("get_cached_repos_and_tags");
        if self.has_cache() {
            let mut f = File::open(&self.cache_file)
                .expect("Could not create cache file.");
            let reader = &mut Vec::new();
            f.read_to_end(reader).expect("Couldnt read.");

            let rtt: RepoTagCache =
                serde_json::from_slice(reader).expect("Could not deserialize");
            Ok((rtt.repos, rtt.tags))
        } else {
            // Try to create the cache directory if the cache *file* doesn't
            // exist; app_dir() handles an existing directory just fine.
            match app_dir(AppDataType::UserCache, &APP, "cache") {
                Ok(_) => {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "Cache Directory exists but no Cache file",
                    ))
                    .context("just trying to get anyhow to work");
                }
                Err(_e) => {
                    return Err(GitGlobalError::from(Error::new(
                        ErrorKind::NotFound,
                        "No Cache Directory exists",
                    )))
                    .context("just trying to get anyhow to work");
                }
            }
        }
    }

    /// Returns the list of repos found in the cache file.
    pub fn get_cached_repos(&self) -> Vec<Repo> {
        trace!("get_cached_repos");
        // debug!("GET CACHED REPOS - 0");
        self.get_cached_repos_and_tags()
            .expect("get_cached_repos_and_tags failed")
            .0
        // .map(|rtt| rtt.0)
    }

    /// Reads the cache and returns Vec of Tags
    pub fn get_cached_tags(&self) -> Vec<RepoTag> {
        trace!("get_cached_tags");
        self.get_cached_repos_and_tags()
            .expect("get_cached_repos_and_tags failed")
            .1
        // .map(|rtt| rtt.1)
    }

    /// Reads cached Repos from disk and returns them as a `GitGlobalResult`
    pub fn get_cached_results(&self) -> GitGlobalResult {
        trace!("get_cached_results");
        GitGlobalResult::new(&self.get_cached_repos())
    }

    /// Reads the currently stored repos from the cache and then writes them along with the in-memory Vec of Tags to disk
    pub fn write_tags(&self) {
        debug!("WRITING TAGS: called");

        if !self.has_cache() {
            // Try to create the cache directory if the cache *file* doesn't
            // exist; app_dir() handles an existing directory just fine.
            match app_dir(AppDataType::UserCache, &APP, "cache") {
                Ok(_) => (),
                Err(e) => panic!("Could not create cache directory: {}", e),
            }
        }
        debug!("WRITING TAGS: called - 2");

        let repos = self.get_cached_repos();

        let mut f = File::create(&self.cache_file)
            .expect("Could not create cache file.");

        debug!("WRITING TAGS: called - 3");

        type RepoTagTuple<'a> = (&'a Vec<Repo>, &'a Vec<RepoTag>);
        let _wowser: RepoTagTuple = (&repos, &self.tags);

        debug!("WRITING TAGS: repos:\n{:?}", &repos);

        let rt: RepoTagCache = RepoTagCache::new(&repos, &self.tags);
        let serialized = serde_json::to_string(&rt).unwrap();

        debug!("WRITING TAGS: SERIALIZED:\n{}", serialized);

        f.write_all(serialized.as_bytes())
            .expect("Problem writing cache file");
    }

    /// Writes the given repo paths to the cache file.
    pub fn cache_repos(&self, repos: &Vec<Repo>) {
        if !self.has_cache() {
            // Try to create the cache directory if the cache *file* doesn't
            // exist; app_dir() handles an existing directory just fine.
            match app_dir(AppDataType::UserCache, &APP, "cache") {
                Ok(_) => (),
                Err(e) => panic!("Could not create cache directory: {}", e),
            }
        }
        let mut f = File::create(&self.cache_file)
            .expect("Could not create cache file.");

        let rt: RepoTagCache = RepoTagCache::new(repos, &self.tags);
        let serialized = serde_json::to_string(&rt).unwrap();

        debug!("CACHING REPOS: SERIALIZED:\n{}", &serialized);

        f.write_all(serialized.as_bytes())
            .expect("Problem writing cache file");
    }

    /// This should be better
    /// - Should have more control over saving repos and tags
    pub fn save_repos_and_tags(
        &mut self,
        repos: Vec<Repo>,
        tags: Vec<RepoTag>,
    ) {
        self.tags = tags;
        self.cache_repos(&repos);
    }

    /// Dont replace all repos
    pub fn update_repos_and_tags(
        &mut self,
        repos: Vec<Repo>,
        _tags: Vec<RepoTag>,
    ) {
        // NOTE: I either need to read repos from cache first
        // or i need to do a kind of merge write to cache afterwards...
        // self.current.tags = tags;
        // self.current.repos = repos;
        self.repos = self.get_cached_repos();
        self.efficient_repos_update(repos);
        self.cache_repos(&self.repos);
    }

    // TODO: using this? - YEP
    pub fn get_tagged_repos(&mut self, tags: &Vec<RepoTag>) -> Vec<Repo> {
        trace!("get_tagged_repos");
        if tags.len() == 0 {
            // println!("NO TAGS");
            return self.get_repos();
        } else {
            debug!("tags!!!! {}", tags.len());
            return self
                .get_repos()
                .into_iter()
                .filter(|x| {
                    tags.iter()
                        .filter(|y| x.tags.iter().any(|t| &t == y))
                        .count()
                        > 0
                })
                .collect();
        }
    }

    /// Returns all known git repos, populating the cache first, if necessary.
    /// TODO: Shouldnt this be a method on GitGlobalConfig?
    /// TODO? Surely this should update the `repos` field?
    pub fn get_repos(&mut self) -> Vec<Repo> {
        trace!("get_repos");
        if !self.has_cache() {
            println!("{}", "You have no cached repos yet...".yellow());
            let repos = new_find_repos();
            self.repos = repos;
            self.cache_repos(&self.repos);
            self.repos.clone()
        } else {
            println!("{}", "You have a cache!".green());
            self.repos = self.get_cached_repos();
            self.repos.clone()
        }
    }

    pub fn clear_tags(&self) -> Result<&str> {
        trace!("clear_tags");
        let repos: Vec<Repo> = self
            .get_cached_repos()
            .into_iter()
            .map(|mut r| {
                r.tags.clear();
                r
            })
            .collect();
        // save_repos_and_tags(repos, vec![]);
        self.cache_repos(&repos);
        Ok("cool")
    }
}

trait Cached {
    fn cache_repos(&self, repos: &Vec<Repo>);
    fn get_cache_repos(&self) -> &Vec<Repo>;
    fn empty_cache(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let hm: Result<HashMap<String, Value>>;
        // let hm: HashMap<String, Value>;
        hm = GitGlobalConfig::get_config();
        // hm = GitGlobalConfig::get_config();
        assert!(hm.is_ok());
        // let hmu = hm.unwrap()
        println!("\n\nCONFIG VALS:");
        for (k, v) in hm.unwrap() {
            ic!((k, v));
        }
        println!("CONFIG VALS END\n\n");
    }

    #[test]
    fn inspect_config() {
        println!("INSPECT CONFIG");
        let config = GitGlobalConfig::get_config().unwrap();
        let more_paths = GitGlobalConfig::get_raw_config()
            .unwrap()
            .get_array("ignored_paths")
            .unwrap();
        let mooofe: Vec<String> = more_paths
            .into_iter()
            .map(|v| v.into_str().unwrap())
            .collect();
        println!("mooofe {:#?}", mooofe);
    }

    use std::collections::hash_map::HashMap;

    #[test]
    fn deserialize_config() {
        // unimplemented!();
        let ignored: Vec<Value> = GitGlobalConfig::get_raw_config()
            .unwrap()
            .get_array("ignored_paths")
            .unwrap();
        // let ignored_deserial: Vec<HashMap<String, String>> = ignored
        let ignored_deserial: Vec<String> = ignored
            .into_iter()
            .map(|v| v.try_into().expect("We tried to convert but"))
            // .map(|v| serde_json::from_value(ignored).unwrap())
            .collect();
        println!("Deserialized ignored_paths to:");
        for v in ignored_deserial {
            println!("Config: {:#?}", v);
        }
        let short_paths: HashMap<String, String> =
            GitGlobalConfig::get_raw_config()
                .unwrap()
                .get_table("path_shortcuts")
                .unwrap()
                .into_iter()
                .map(|(k, v)| (k, v.into_str().unwrap()))
                .collect();
        println!("Deserialized path_shortcuts to:");
        for (k, v) in short_paths {
            println!("Config: {}: {}", k, v);
        }
    }

    #[test]
    fn deserialize_setings() {
        let settings: Settings = GitGlobalConfig::get_parsed_config().unwrap();
        println!("Deserialized Settings to:");
        println!("{:?}", settings);
        println!("Deserialized Settings With Format:");
        println!("{}", settings);
    }
}
