// use colored::*;
// use std::env;
use app_dirs::{app_dir, get_app_dir, AppDataType, AppInfo};
use git2;
use std::fs::{remove_file, File};
use std::io::{Error, ErrorKind};
use std::io::{Read, Result, Write};
use std::path::{Path, PathBuf};
// use std::result::Result as BResult;
use walkdir::DirEntry;

use repo::{
    repo::{Repo, RepoTag},
    result::GitGlobalResult,
};

const APP: AppInfo = AppInfo {
    name: "git-global",
    author: "hal",
};
const CACHE_FILE: &'static str = "repos.txt";
const TAG_CACHE_FILE: &'static str = "tags.txt";
const SETTING_BASEDIR: &'static str = "global.basedir";
const SETTING_IGNORED: &'static str = "global.ignore";

/// A container for git-global configuration options.

#[derive(Serialize, Deserialize, Debug)]
pub struct GitGlobalConfig {
    pub basedir: String,
    // pub basedirs: String,
    pub basedirs: Vec<String>,
    pub ignored_patterns: Vec<String>,
    pub tags: Vec<RepoTag>,
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
    // pub fn new() -> Result<GitGlobalConfig, Error> {
    pub fn new() -> GitGlobalConfig {
        let home_dir = dirs::home_dir()
            .expect("Could not determine home directory.")
            .to_str()
            .expect("Could not convert home directory path to string.")
            .to_string();
        let (basedir, basedirs, patterns) = match git2::Config::open_default() {
            Ok(config) => {
                (config.get_string(SETTING_BASEDIR).unwrap_or(home_dir.clone()),
                //  vec![config.get_string(SETTING_BASEDIR).unwrap_or(home_dir.clone())],
                 config.get_string(SETTING_BASEDIR)
                    .unwrap_or(home_dir.clone())
                    .split(",")
                    // .by_ref()
                    .map(|p| p.trim().to_string())
                    // .cloned()
                    .collect(),
                 config.get_string(SETTING_IGNORED)
                     .unwrap_or(String::new())
                     .split(",")
                     .map(|p| p.trim().to_string())
                     .collect())
            }
            Err(_) => {
                println!("Hey - you need to setup your git config so I can find stuff");
                panic!("ARRRGH");
            }
            // Err(_) => (home_dir.clone(), vec![home_dir.clone()], Vec::new()),
            // Err(_) => (home_dir, vec![&home_dir], Vec::new()),
        };
        // assert!(Path::exists(Path::new(&basedir)), "Your provided basedir: {} does not exist", basedir);
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
            tags: vec![],
            // tags: vec![],
            ignored_patterns: patterns,
            cache_file: cache_file,
            tags_file,
        };
        ggc.tags = ggc.read_tags().unwrap_or(vec![]);
        ggc
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

    pub fn replace_tags(&mut self, tags: Vec<String>) -> () {
        let new_tags = tags.into_iter().map(|t| t.into()).collect();
        self.tags = new_tags;
    }

    #[allow(dead_code)]
    fn tags(&self) -> &Vec<RepoTag> {
        &self.tags
    }

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
        let mut f = File::create(&self.cache_file).unwrap();
        let rt: RepoTagCache = RepoTagCache::new(&Vec::new(), &Vec::new());
        let serialized = serde_json::to_string(&rt).unwrap();
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

    pub fn read_tags(&self) -> Result<Vec<RepoTag>> {
        if !self.cache_file.as_path().exists() {
            // Try to create the cache directory if the cache *file* doesn't
            // exist; app_dir() handles an existing directory just fine.
            match app_dir(AppDataType::UserCache, &APP, "cache") {
                Ok(_) => {
                    // panic!("Oh SHIT!");
                    // self.make_empty_cache();
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "Cache Directory exists but no Cache file",
                    ));
                    // return Err("Cache Directory exisits but no Cache file")
                }
                // Ok(_) => (),
                Err(e) => {
                    // panic!("OH SNAP!");
                    // return Err("No Cache Directory found");
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "No Cache Directory exists",
                    ));
                    // return Ok(vec!());
                    // return Err(Error::new(ErrorKind::Other, "No Cache Directory found"));
                    // return Error::new(ErrorKind::Other, "No Cache Directory found");
                } // Err(e) => panic!("Could not create cache directory: {}", e),
            }
        }
        let mut f =
            File::open(&self.cache_file).expect("Could not create cache file.");
        let reader = &mut Vec::new();
        f.read_to_end(reader).expect("Couldnt read.");

        let _temp: RepoTagCache =
            serde_json::from_slice(reader).expect("Could not deserialize");

        let _tags: &Vec<RepoTag> = &_temp.tags;
        // let _repos: &Vec<Repo> = serialized.0;
        let tags = _tags.to_vec();
        debug!("Tags are {:?}", &tags);
        Ok(tags)
    }

    pub fn write_tags(&self) {
        debug!("WRITING TAGS: called");

        if !self.cache_file.as_path().exists() {
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
        if !self.cache_file.as_path().exists() {
            // Try to create the cache directory if the cache *file* doesn't
            // exist; app_dir() handles an existing directory just fine.
            match app_dir(AppDataType::UserCache, &APP, "cache") {
                Ok(_) => (),
                Err(e) => panic!("Could not create cache directory: {}", e),
            }
        }
        let mut f = File::create(&self.cache_file)
            .expect("Could not create cache file.");

        // type RepoTagTuple<'a> = (&'a Vec<Repo>, &'a Vec<RepoTag>);

        // let _thing: RepoTagTuple = (&repos, &self.tags);

        let rt: RepoTagCache = RepoTagCache::new(repos, &self.tags);
        let serialized = serde_json::to_string(&rt).unwrap();

        debug!("CACHING REPOS: SERIALIZED:\n{}", &serialized);

        f.write_all(serialized.as_bytes())
            .expect("Problem writing cache file");
    }

    /// Returns the list of repos found in the cache file.
    pub fn get_cached_repos(&self) -> Vec<Repo> {
        debug!("GET CACHED REPOS - 0");

        let mut repos = Vec::new();
        if self.cache_file.as_path().exists() {
            let mut f = File::open(&self.cache_file)
                .expect("Could not open cache file.");

            // let serialized = serde_json::to_string(&repos).unwrap();

            // let reader = &mut BufReader::new(f);
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

    pub fn get_cached_results(&self) -> GitGlobalResult {
        GitGlobalResult::new(&self.get_cached_repos())
    }
}

pub fn save_repos_and_tags(repos: Vec<Repo>, tags: Vec<RepoTag>) {
    let mut gc: GitGlobalConfig = GitGlobalConfig::new();
    // gc.repos = repos;
    gc.tags = tags;
    gc.cache_repos(&repos);
    // hmmmm...
}

trait Cached {
    fn cache_repos(&self, repos: &Vec<Repo>);
    fn get_cache_repos(&self) -> &Vec<Repo>;
    fn empty_cache(&self) -> bool;
}