//! Core functionality of git-global.
//!
//! Includes the `Repo`, `GitGlobalConfig`, and `GitGlobalResult` structs, and
//! the `get_repos()` function.

use colored::*;
pub use core::config::GitGlobalConfig;
pub use core::repo::{Repo, RepoTag};
pub use core::result::GitGlobalResult;
// use std::fmt;

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
// fn is_a_git<D>(entry: &D) -> bool {
//     entry.file_type().is_dir() && entry.file_name() == ".git"
// fn is_a_git(entry: &DirEntry) -> bool {
//     entry.file_type().is_dir() && entry.file_name() == ".git"
// }

/// Is this the path of a git repository?
fn is_a_repo(entry: &DirEntry) -> bool {
    debug!("entry is {}", entry.path().to_str().unwrap());
    entry.file_type().is_dir()
        && entry.path().read_dir().expect("read dir failed").any(|f| {
            let ff = f.expect("works");
            // ff.file_type().unwrap().is_dir() && ff.file_name() == ".git"
            is_a_git(&ff)
        })
}

/// Add repos to the list of repos
fn my_repo_check(repos: &mut Vec<Repo>, entry: DirEntry) -> () {
    if is_a_repo(&entry) {
        debug!("A REPO IS {}", entry.path().to_str().unwrap());
        repos.push(Repo::new(entry.path().to_str().unwrap().to_string()));
    }
}

/// Does this list of repos contain an ancestor of the current path?
fn repos_contains_ancestor(entry: &DirEntry, repos: &Vec<Repo>) -> bool {
    repos.into_iter().any(|r| entry.path().starts_with(&r.path))
}

/// Should we
/// 1) Do Something with this path
/// 2) Recurse into any contents of this path
fn walk_here(entry: &DirEntry, uc: &GitGlobalConfig) -> bool {
    uc.filter(entry)
}

/// Walks the configured base directory, looking for git repos.
pub fn find_repos() -> Vec<Repo> {
    let mut repos: Vec<Repo> = Vec::new();
    let user_config = GitGlobalConfig::new();
    let basedir = &user_config.basedir;
    let walker = WalkDir::new(basedir).into_iter();
    format!(
        "{}, {}",
        "Scanning for git repos under {}; this may take a while...",
        basedir.green()
    );

    for entry in walker.filter_entry(|e| walk_here(&e, &user_config)) {
        match entry {
            Ok(entry) => {
                if !repos_contains_ancestor(&entry, &repos) {
                    my_repo_check(&mut repos, entry);
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
    debug!("get repos");
    let user_config = GitGlobalConfig::new();
    debug!("got config");

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&user_config).unwrap();
    // Prints serialized = {"x":1,"y":2}
    debug!("serialized = {}", serialized);

    if !user_config.has_cache() {
        println!("{}", "You have no cached repos yet...".yellow());
        let repos = find_repos();
        cache_repos(&repos);
        repos
    } else {
        println!("{}", "You have a cache!".green());
        user_config.get_cached_repos()
    }
}
