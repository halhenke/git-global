//! Core functionality of git-global.
//!
//! Includes the `Repo`, `GitGlobalConfig`, and `GitGlobalResult` structs, and
//! the `get_repos()` function.

use colored::*;
pub use repo::config::GitGlobalConfig;
pub use repo::repo::{Repo, RepoTag};
pub use repo::result::GitGlobalResult;
// use std::fmt;
use std::sync::Arc;

use jwalk;
use walkdir::{DirEntry, WalkDir};

extern crate serde;
extern crate serde_json;

/// Trying to get .gitignore contents
/// - part of a strategy to not recurse into ignored directories
/// Not used at preesnt but perhaps later.
pub fn repo_filter(
    e: &DirEntry,
    uc: GitGlobalConfig,
) -> Result<bool, std::io::Error> {
    // ) -> Result<bool, &'static str> {
    use std::fs;
    if uc.filter(e) {
        return Ok(true);
    }
    if e.file_type().is_dir() {
        for f in e.path().read_dir().expect("read dir failed") {
            let ff = f.expect("unwrap again...");
            if ff.file_type()?.is_file() && ff.file_name() == ".gitignore" {
                let _contents = fs::read_to_string(ff.path());
                return Ok(true);
            }
        }
    }
    return Ok(false);
}

/// Is this the path of a .git directory?
fn is_a_git(entry: &std::fs::DirEntry) -> bool {
    entry.path().is_dir() && entry.file_name() == ".git"
}

/// Is this the path of a git repository?
fn new_is_a_repo(entry: &jwalk::DirEntry) -> bool {
    // debug!("entry is {}", entry.path().to_str().unwrap());
    entry.file_type.as_ref().unwrap().is_dir()
        && entry.path().read_dir().expect("read dir failed").any(|f| {
            let ff = f.expect("works");
            is_a_git(&ff)
        })
}

/// Add repos to the list of repos
fn my_new_repo_check(repos: &mut Vec<Repo>, entry: jwalk::DirEntry) -> () {
    if new_is_a_repo(&entry) {
        debug!("A REPO IS {}", entry.path().to_str().unwrap());
        repos.push(Repo::new(entry.path().to_str().unwrap().to_string()));
    }
}

/// Walks the configured base directory, looking for git repos.
pub fn new_find_repos() -> Vec<Repo> {
    let mut repos: Vec<Repo> = Vec::new();
    let user_config = GitGlobalConfig::new();
    let basedir = &user_config.basedir;
    let mut walker = jwalk::WalkDir::new(basedir)
        .skip_hidden(false)
        // .num_threads(1)
        .process_entries(|v| {
            v.into_iter().for_each(|de| {
                // debug!("In the map ");
                let mut d: &mut jwalk::DirEntry = de.as_mut().unwrap();
                if d.file_type.as_ref().unwrap().is_dir()
                    && d.path().read_dir().unwrap().any(|f| {
                        let ff = f.unwrap();
                        // debug!(".git path is {}", ff.path().display());
                        ff.file_name() == ".git"
                    })
                {
                    debug!("A match! {}", d.path().display());
                    d.content_spec = None;
                    // debug!("d.content_spec {:?}", d.content_spec);
                }
            });
        })
        .into_iter();
    format!(
        "{}, {}",
        "Scanning for git repos under {}; this may take a while...",
        basedir.green()
    );

    // debug!("You went through {} paths", walker.by_ref().count());
    // debug!(
    //     "You set {} content_specs to zero",
    //     walker
    //         .by_ref()
    //         .filter(|d| d.as_ref().unwrap().content_spec.is_none())
    //         .count()
    // );

    for entry in walker {
        match entry {
            Ok(entry) => {
                if entry.file_type.as_ref().unwrap().is_dir()
                    && entry.content_spec.is_none()
                {
                    debug!("A GIT: {}", entry.file_name.to_str().unwrap());
                    my_new_repo_check(&mut repos, entry);
                }
            }
            Err(_) => (),
        }
    }
    repos.sort_by(|a, b| a.path().cmp(&b.path()));
    repos
}

/// Caches repo list to disk, in the XDG cache directory for git-global.
pub fn cache_repos(repos: &Vec<Repo>) {
    let user_config = GitGlobalConfig::new();
    user_config.cache_repos(repos);
}

pub fn get_tagged_repos(tags: &Vec<RepoTag>) -> Vec<Repo> {
    if tags.len() == 0 {
        // println!("NO TAGS");
        return get_repos();
    } else {
        debug!("tags!!!! {}", tags.len());
        return get_repos()
            .into_iter()
            // .cloned()
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
pub fn get_repos() -> Vec<Repo> {
    // debug!("get repos");
    let user_config = GitGlobalConfig::new();
    // debug!("got config");

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&user_config).unwrap();
    // Prints serialized = {"x":1,"y":2}
    // debug!("serialized = {}", serialized);

    if !user_config.has_cache() {
        println!("{}", "You have no cached repos yet...".yellow());
        let repos = new_find_repos();
        cache_repos(&repos);
        repos
    } else {
        println!("{}", "You have a cache!".green());
        user_config.get_cached_repos()
    }
}
