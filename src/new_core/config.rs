extern crate colored;
use std::env;
use std::path::{PathBuf, Path};
use std::io::{BufReader, Read, Write, Result};
use std::fs::{File, remove_file};
use app_dirs::{AppInfo, AppDataType, app_dir, get_app_dir};
use walkdir::{DirEntry};

extern crate serde_json;

pub use new_core::repo::{Repo, RepoTag};
use git2;


const APP: AppInfo = AppInfo { name: "git-global", author: "peap" };
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

impl GitGlobalConfig {
    // pub fn new() -> Result<GitGlobalConfig, Error> {
    pub fn new() -> GitGlobalConfig {
        let home_dir = env::home_dir()
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
        let cache_file = match get_app_dir(AppDataType::UserCache, &APP, "cache") {
            Ok(mut dir) => {
                dir.push(CACHE_FILE);
                dir
            }
            Err(_) => panic!("TODO: work without XDG"),
        };
        let tags_file = match get_app_dir(AppDataType::UserCache, &APP, "cache") {
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
        GitGlobalConfig {
            basedir: basedir,
            basedirs: basedirs,
            tags: vec![],
            ignored_patterns: patterns,
            cache_file: cache_file,
            tags_file,
        }
    }

    /// Returns `true` if this directory entry should be included in scans.
    pub fn filter(&self, entry: &DirEntry) -> bool {
        let entry_path = entry.path().to_str().expect("DirEntry without path.");
        // self.ignored_patterns
        //     // .into_iter()
        //     .iter()
        //     .for_each(|x| println!("Ignored patters is: {}", x));

        (self.ignored_patterns.len() == 1 && self.ignored_patterns[0] == "") || !self.ignored_patterns
            .iter()
            .any(|pat| entry_path.contains(pat))
        // println!("The patterns are empty == {}", self.ignored_patterns.len());
        // println!("This path {} is a {}", entry_path, a);
        // a
    }

    pub fn add_tags(&mut self, tags: Vec<String>) -> () {
    // pub fn add_tags(&self, tags: Vec<String>) -> Vec<RepoTag> {
    // pub fn add_tags(&self, tags: &mut Vec<String>) ->() {
        let new_repos = &mut tags
            .into_iter()
            .map(|t| t.into())
            // .map(|t| RepoTag::from(t))
            .collect();
        self.tags
            .append(
                new_repos
            );
        self.tags
            .dedup_by(|a, b|
                a.name.as_str().eq_ignore_ascii_case(b.name.as_str())
            );
    }

    fn tags(&self) -> &Vec<RepoTag> {
        &self.tags
    }

    pub fn tag_names(&self) -> Vec<&str> {
    // pub fn tag_names(&self) -> &Vec<&str> {
        self.tags
            .iter()
            .map(|g| g.name.as_str())
            .collect()
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

    /// Returns boolean indicating if the cache file exists.
    pub fn has_cache(&self) -> bool {
        self.cache_file.as_path().exists()
    }

    /// Remove the cache file
    pub fn destroy_cache(&self) -> Result<()> {
        remove_file(self.cache_file.as_path())
    }

    /// Do we have any repos in the cache?
    fn empty_cache(&self) -> bool {
        self.get_cached_repos().len() == 0
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
        let mut f = File::create(&self.cache_file).expect("Could not create cache file.");

        // let serialized = serde_json::to_string(&repos).unwrap();
        let serialized = serde_json::to_string(&(&repos, &self.tags)).unwrap();

        f.write_all(serialized.as_bytes()).expect("Problem writing cache file");
    }

    /// Returns the list of repos found in the cache file.
    pub fn get_cached_repos(&self) -> Vec<Repo> {
        let mut repos = Vec::new();
        if self.cache_file.as_path().exists() {
            let mut f = File::open(&self.cache_file).expect("Could not open cache file.");

            // let serialized = serde_json::to_string(&repos).unwrap();

            // let reader = &mut BufReader::new(f);
            let reader = &mut Vec::new();
            f.read_to_end(reader).unwrap();
            // f.read_to_end().unwrap();

            // println!("{:?}", reader);

            type RepoTagTuple = (Vec<Repo>, Vec<RepoTag>);

            // repos = serde_json::from_slice(reader).unwrap();

            let serialized: RepoTagTuple = serde_json::from_slice(reader).unwrap();
            repos = serialized.0;
            // let serialized = serde_json::to_string(&(&repos, &self.tags)).unwrap();
            // println!("{}", test_ser);
            // let (fake_repos: Vec<Repo> , fake_tags: Vec<RepoTag>) = serde_json::from_str(&test_ser).unwrap();
            // let fake_repos: RepoTagTuple = serde_json::from_str(&test_ser).unwrap();
            // println!("REPOS!!!!: {:?}", fake_repos.0);
            // println!("TAGS!!!!: {:?}", fake_repos.1);

            // println!("{:?}", repos);

            // repos = serde_json::from_reader(reader).unwrap();

        }
        repos
    }
}
