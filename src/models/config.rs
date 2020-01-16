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
use app_dirs::{app_dir, get_app_dir, AppDataType, AppInfo};
use colored::*;
use config::{Config, ConfigError, Environment, File as CFile, Source, Value};
// use futures::executor::LocalPool;
use crate::models::{
    action::Action, repo::Repo, repo::Updatable, repo_tag::RepoTag,
    result::GitGlobalResult, utils::new_find_repos,
};
use git2;
use std::collections::hash_map::HashMap;
use std::fs::{remove_file, File};
use std::io::{Error, ErrorKind};
use std::io::{Read, Result, Write};
use std::path::{Path, PathBuf};
use walkdir::DirEntry;

const APP: AppInfo = AppInfo {
    name: "git-global",
    author: "hal",
};
const CACHE_FILE: &'static str = "repos.txt";
// const TAG_CACHE_FILE: &'static str = "tags.txt";
const SETTING_BASEDIR: &'static str = "global.basedir";
const SETTING_IGNORED: &'static str = "global.ignore";
const SETTINGS_DEFAULT_TAGS: &'static str = "global.default-tags";
const SETTINGS_DEFAULT_GIT_ACTIONS: &'static str = "global.default-git-actions";
const CONFIG_FILE_NAME: &'static str = ".git_global_config_simple";

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
        let (basedir, basedirs, patterns, default_tags, default_actions) =
            match git2::Config::open_default() {
            Ok(config) => {
                (config.get_string(SETTING_BASEDIR).unwrap_or(home_dir.clone()),
                 config.get_string(SETTING_BASEDIR)
                    .unwrap_or(home_dir.clone())
                    .split(",")
                    .map(|p| p.trim().to_string())
                    .collect(),
                 config.get_string(SETTING_IGNORED)
                     .unwrap_or(String::new())
                     .split(",")
                     .map(|p| p.trim().to_string())
                     .collect(),
                 config.get_string(SETTINGS_DEFAULT_TAGS)
                     .unwrap_or(String::new())
                     .split(",")
                     .map(|p| p.trim().to_string())
                     .map(|rt| RepoTag::new(&rt))
                    //  .map(|rt| RepoTag::new(&rt.to_owned()))
                     .collect::<Vec<RepoTag>>(),
                 config.get_string(SETTINGS_DEFAULT_GIT_ACTIONS)
                     .unwrap_or(String::new())
                     .split(",")
                     .map(|p| p.trim().to_string())
                    //  TODO: Figure out how to handle an Action without a path
                     .map(|ga| Action::NeedsAPathAction(ga.to_owned(), ga.clone(), vec![]))
                     .collect::<Vec<Action>>()
                )
            }
            Err(_) => {
                println!("Hey - you need to setup your git config so I can find stuff");
                panic!("ARRRGH");
            }
            // Err(_) => (home_dir.clone(), vec![home_dir.clone()], Vec::new()),
            // Err(_) => (home_dir, vec![&home_dir], Vec::new()),
        };
        assert!(
            Path::exists(Path::new(&basedir)),
            "Your provided basedir: {} does not exist",
            basedir
        );
        if !Path::exists(Path::new(&basedir)) {
            panic!("Your provided basedir: {} does not exist", basedir);
        }
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

        let config = GitGlobalConfig::get_config();
        let ignored_paths = config
            .unwrap()
            .iter()
            // .iter()
            .find(|(k, v)| k.as_str() == "IGNORED_PATHS")
            .unwrap();

        let ggc = GitGlobalConfig {
            basedir: basedir,
            basedirs: basedirs,
            current: CurrentState::new(),
            repos: vec![],
            tags: vec![],
            default_tags,
            default_repos: vec![],
            // default_paths: vec![],
            ignored_repos: vec![],
            ignored_paths: vec![],
            actions: default_actions,
            ignored_patterns: patterns,
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

    fn get_config() -> std::result::Result<HashMap<String, Value>, ConfigError>
    {
        // fn get_config() -> HashMap<String, Value> {
        let mut c = Config::default();
        c.merge(CFile::with_name(CONFIG_FILE_NAME))?
            // .expect("Merge of Configuration File Values failed")
            // .or_else(|e| return Err(e))
            .merge(Environment::with_prefix("GIT_GLOBAL"))?
            // .expect("Merge of Environment Configuration Values failed")
            .collect()
        // .expect("Config: Conversion to hashMap Failed")
    }

    fn get_raw_config() -> Config {
        // fn get_config() -> HashMap<String, Value> {
        let mut c = Config::default();
        c.merge(CFile::with_name(CONFIG_FILE_NAME))
            .unwrap()
            // .expect("Merge of Configuration File Values failed")
            // .or_else(|e| return Err(e))
            .merge(Environment::with_prefix("GIT_GLOBAL"));
        // .expect("Merge of Environment Configuration Values failed")
        // .collect()
        // .expect("Config: Conversion to hashMap Failed")
        c
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
        println!("Tags:");
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
    }

    /// Remove the cache file
    pub fn remove_cache_file(&self) -> Result<()> {
        remove_file(self.cache_file.as_path())
    }

    ///Return the cache file
    pub fn print_cache(
        &self,
    ) -> std::result::Result<
        GitGlobalResult,
        crate::models::errors::GitGlobalError,
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
                    ));
                }
                Err(_e) => {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "No Cache Directory exists",
                    ));
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
        let hm: std::result::Result<HashMap<String, Value>, ConfigError>;
        // let hm: HashMap<String, Value>;
        hm = GitGlobalConfig::get_config();
        // hm = GitGlobalConfig::get_config();
        assert!(hm.is_ok());
        // let hmu = hm.unwrap()
        println!("\n\nCONFIG VALS:");
        for (k, v) in hm.unwrap() {
            ic!((k, v));
            // ic!(v);
            // println!("key: {}, val: {}", k, v);
        }
        println!("CONFIG VALS END\n\n");
    }

    #[test]
    fn inspect_config() {
        println!("INSPECT CONFIG");
        let config = GitGlobalConfig::get_config().unwrap();
        // let ignored_paths: Vec<_> = config
        //     // .unwrap()
        //     .into_iter()
        //     // .iter()
        //     .find(|(k, v)| k.as_str() == "IGNORED_PATHS")
        //     .map(|(k, v)| {
        //         v.into_array()
        //             .unwrap()
        //             .into_iter()
        //             .map(|a| a.into_table().unwrap())
        //             .collect::<HashMap>()
        //             .into_iter()
        //             .map(|v| {
        //                 v.into_iter()
        //                     .map(|(k, v)| v.into_str().unwrap())
        //                     .collect()
        //                 // .unwrap()
        //                 // .collect::<Vec<String>>()
        //             })
        //     })
        //     // .unwrap::<Option<>>()
        //     // .map(|v| v.into_iter().map(|(k, v)| v.into_str().unwrap()))
        //     .collect();
        // println!("ignored paths {:#?}", ignored_paths);
        let more_paths = GitGlobalConfig::get_raw_config()
            .get_array("IGNORED_PATHS")
            .unwrap();
        println!("more paths {:#?}", more_paths);
        let further: Vec<_> = more_paths
            .into_iter()
            .map(|v| v.into_table().unwrap())
            .collect();
        println!("further {:#?}", further);
        let mooofe: Vec<String> = further
            .into_iter()
            // .into_iter()
            .map(|v| {
                v.into_iter().map(|(k, v)| v.into_str().unwrap()).collect()
            })
            .collect();
        println!("mooofe {:#?}", mooofe);
    }
}
