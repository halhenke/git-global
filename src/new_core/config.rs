extern crate colored;
use std::env;
use std::path::{PathBuf, Path};
use std::io::{BufRead, BufReader, Write, Result};
use std::fs::{File, remove_file};
use app_dirs::{AppInfo, AppDataType, app_dir, get_app_dir};
use walkdir::{DirEntry};

pub use new_core::repo::{Repo, RepoTag};
use git2;


const APP: AppInfo = AppInfo { name: "git-global", author: "peap" };
const CACHE_FILE: &'static str = "repos.txt";
const SETTING_BASEDIR: &'static str = "global.basedir";
const SETTING_IGNORED: &'static str = "global.ignore";



/// A container for git-global configuration options.
pub struct GitGlobalConfig {
    pub basedir: String,
    // pub basedirs: String,
    pub basedirs: Vec<String>,
    pub ignored_patterns: Vec<String>,
    pub tags: Vec<RepoTag>,
    pub cache_file: PathBuf,
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

    fn print_tags(&self) {
        println!("Tags:");
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
        for repo in repos.iter() {
            match writeln!(f, "{}", repo.path()) {
                Ok(_) => (),
                Err(e) => panic!("Problem writing cache file: {}", e),
            }
        }
    }

    /// Returns the list of repos found in the cache file.
    pub fn get_cached_repos(&self) -> Vec<Repo> {
        let mut repos = Vec::new();
        if self.cache_file.as_path().exists() {
            let f = File::open(&self.cache_file).expect("Could not open cache file.");
            let reader = BufReader::new(f);
            for line in reader.lines() {
                match line {
                    Ok(repo_path) => repos.push(Repo::new(repo_path)),
                    Err(_) => (),  // TODO: handle errors
                }
            }
        }
        repos
    }
}
