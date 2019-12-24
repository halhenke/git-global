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
// use futures::executor::LocalPool;
use git2;
use std::fs::{remove_file, File};
use std::io::{Error, ErrorKind};
use std::io::{Read, Result, Write};
use std::path::{Path, PathBuf};
use walkdir::DirEntry;

use crate::models::{
    action::Action,
    new_find_repos_executed,
    // repo::{Repo, RepoTag},
    repo::Updatable,
    result::GitGlobalResult,
    utils::new_find_repos,
    Repo,
    RepoTag,
};

const APP: AppInfo = AppInfo {
    name: "git-global",
    author: "hal",
};
const CACHE_FILE: &'static str = "repos.txt";
const TAG_CACHE_FILE: &'static str = "tags.txt";
const SETTING_BASEDIR: &'static str = "global.basedir";
const SETTING_IGNORED: &'static str = "global.ignore";
const SETTINGS_DEFAULT_TAGS: &'static str = "global.default-tags";
const SETTINGS_DEFAULT_GIT_ACTIONS: &'static str = "global.default-git-actions";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrentState {
    pub tags: Vec<RepoTag>,
    pub repos: Vec<Repo>,
    pub actions: Vec<Repo>,
}

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
    pub actions: Vec<Action>,
    pub cache_file: PathBuf,
    pub tags_file: PathBuf,
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
        let tags_file = match get_app_dir(AppDataType::UserCache, &APP, "cache")
        {
            Ok(mut dir) => {
                dir.push(TAG_CACHE_FILE);
                dir
            }
            Err(_) => panic!("TODO: work without XDG"),
        };

        // NOTE: Handle this earlier
        // if basedir == "" {
        //     unimplemented!();
        // }
        let mut ggc = GitGlobalConfig {
            basedir: basedir,
            basedirs: basedirs,
            current: CurrentState::new(),
            repos: vec![],
            tags: vec![],
            default_tags,
            actions: default_actions,
            ignored_patterns: patterns,
            cache_file: cache_file,
            tags_file,
        };
        // TODO: get rid of this
        // ggc.tags = ggc.read_tags().unwrap_or(vec![]);
        ggc
    }

    /// Initialise With Repos
    pub fn new_with_repos() -> Self {
        let mut gc = GitGlobalConfig::new();
        gc.get_repos();
        gc
    }

    /// A version that prepopulates some set of tags and/or actions
    pub fn new_with_defaults(tags: Vec<RepoTag>, actions: Vec<Action>) -> Self {
        let mut gcc = Self::new();
        gcc.tags = tags;
        gcc.actions = actions;
        gcc
    }

    /// Add tags to the [`GitGlobalConfig`] object - Chainable
    pub fn with_tags(&mut self, tags: Vec<RepoTag>) -> &mut Self {
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

    pub fn make_empty_cache(&self) -> Result<()> {
        let mut f = File::create(&self.cache_file)?;
        let rt: RepoTagCache = RepoTagCache::new(&Vec::new(), &Vec::new());
        let serialized = serde_json::to_string(&rt)?;
        f.write_all(serialized.as_bytes())
    }

    /// Remove the cache file
    pub fn destroy_cache(&self) -> Result<()> {
        remove_file(self.cache_file.as_path())
    }

    #[allow(dead_code)]
    /// Do we have any repos in the cache?
    fn empty_cache(&self) -> bool {
        self.get_cached_repos().len() == 0
    }

    /// Reads the cache and returns Vec of Tags
    pub fn read_tags(&self) -> Result<Vec<RepoTag>> {
        if !self.has_cache() {
            // Try to create the cache directory if the cache *file* doesn't
            // exist; app_dir() handles an existing directory just fine.
            match app_dir(AppDataType::UserCache, &APP, "cache") {
                Ok(_) => {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "Cache Directory exists but no Cache file",
                    ));
                }
                Err(e) => {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "No Cache Directory exists",
                    ));
                }
            }
        }
        let mut f =
            File::open(&self.cache_file).expect("Could not create cache file.");
        let reader = &mut Vec::new();
        f.read_to_end(reader).expect("Couldnt read.");

        let _temp: RepoTagCache =
            serde_json::from_slice(reader).expect("Could not deserialize");

        let _tags: &Vec<RepoTag> = &_temp.tags;
        let tags = _tags.to_vec();
        debug!("Tags are {:?}", &tags);
        Ok(tags)
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
        // hmmmm...
    }

    /// Dont replace all repos
    pub fn update_repos_and_tags(
        &mut self,
        repos: Vec<Repo>,
        tags: Vec<RepoTag>,
    ) {
        // self.current.tags = tags;
        // self.current.repos = repos;
        self.efficient_repos_update(repos);
    }

    /// Returns all known git repos, populating the cache first, if necessary.
    /// TODO: Shouldnt this be a method on GitGlobalConfig?
    /// TODO? Surely this should update the `repos` field?
    pub async fn get_repos(&mut self) -> Vec<Repo> {
        if self.has_cache() {
            println!("{}", "You have no cached repos yet...".yellow());
            let repos = new_find_repos();
            // let repos = new_find_repos_executed();
            // asyn
            // crate::models::utils::async_find_repos_and_nothing();
            // let repos = new_find_repos();
            // self.repos = repos.await;
            // self.repos = vec![];
            self.repos = repos;
            self.cache_repos(&self.repos);
            self.repos.clone()
        } else {
            println!("{}", "You have a cache!".green());
            self.repos = self.get_cached_repos();
            self.repos.clone()
        }
    }

    /// Returns the list of repos found in the cache file.
    pub fn get_cached_repos(&self) -> Vec<Repo> {
        debug!("GET CACHED REPOS - 0");

        let mut repos = Vec::new();
        if self.has_cache() {
            let mut f = File::open(&self.cache_file)
                .expect("Could not open cache file.");

            let reader = &mut Vec::new();
            f.read_to_end(reader).expect("Couldnt read ");
            debug!(
                "reader is {}",
                String::from_utf8(reader.clone()).expect("more")
            );

            let _temp: RepoTagCache =
                serde_json::from_slice(reader).expect("Could not deserialize");

            let _repos: &Vec<Repo> = &_temp.repos;
            repos = _repos.to_vec();
        }
        repos
    }

    pub fn clear_tags(&self) -> Result<&str> {
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

    /// Reads cached Repos from disk and returns them as a `GitGlobalResult`
    pub fn get_cached_results(&self) -> GitGlobalResult {
        GitGlobalResult::new(&self.get_cached_repos())
    }
}

trait Cached {
    fn cache_repos(&self, repos: &Vec<Repo>);
    fn get_cache_repos(&self) -> &Vec<Repo>;
    fn empty_cache(&self) -> bool;
}
