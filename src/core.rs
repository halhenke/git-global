//! Core functionality of git-global.
//!
//! Includes the `Repo`, `GitGlobalConfig`, and `GitGlobalResult` structs, and
//! the `get_repos()` function.


use std::fmt;
pub use new_core::repo::{Repo, RepoTag};
pub use new_core::result::{GitGlobalResult};
pub use new_core::config::{GitGlobalConfig};

use walkdir::{WalkDir};

extern crate serde;
extern crate serde_json;


/// Walks the configured base directory, looking for git repos.
pub fn find_repos() -> Vec<Repo> {
    let mut repos = Vec::new();
    let user_config = GitGlobalConfig::new();
    let basedir = &user_config.basedir;
    let walker = WalkDir::new(basedir).into_iter();
    println!("Scanning for git repos under {}; this may take a while...", basedir);
    for entry in walker.filter_entry(|e| user_config.filter(e)) {
        // println!("Its go time!!!!");
        match entry {
            Ok(entry) => {
                // println!("We are checking {} to see if it has a repo...", entry.path().to_str().expect("MISSING"));
                if entry.file_type().is_dir() && entry.file_name() == ".git" {
                    let parent_path = entry.path().parent().expect("Could not determine parent.");
                    match parent_path.to_str() {
                        Some(path) => {
                            repos.push(Repo::new(path.to_string()));
                        }
                        None => (),
                    }
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
        println!("tags!!!! {}", tags.len());
        return get_repos()
            .into_iter()
            // .cloned()
            .filter(|x|
                tags
                    .iter()
                    .filter(|y| x.tags
                        .iter()
                        .any(|t| &t == y))
                        .count() > 0
            ).collect();
    }
}

/// Returns all known git repos, populating the cache first, if necessary.
pub fn get_repos() -> Vec<Repo> {
    println!("get repos");
    let user_config = GitGlobalConfig::new();
    println!("getgot config");


    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&user_config).unwrap();
    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    if !user_config.has_cache() {
        println!("You have no cached repos yet...");
        let repos = find_repos();
        cache_repos(&repos);
        repos
    } else {
        println!("You have a cache!");
        user_config.get_cached_repos()
    }
}
