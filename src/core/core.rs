//! Core functionality of git-global.
//!
//! Includes the `Repo`, `GitGlobalConfig`, and `GitGlobalResult` structs, and
//! the `get_repos()` function.

use colored::*;
pub use core::config::GitGlobalConfig;
pub use core::repo::{Repo, RepoTag};
pub use core::result::GitGlobalResult;
use std::fmt;

use walkdir::{DirEntry, WalkDir};

extern crate serde;
extern crate serde_json;

/// Trying to get .gitignore contents...
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
                let contents = fs::read_to_string(ff.path());
                return Ok(true);
            }
        }
    }
    return Ok(false);
}

fn repo_check(repos: &mut Vec<Repo>, entry: DirEntry) -> () {
    // println!("We are checking {} to see if it has a repo...", entry.path().to_str().expect("MISSING"));
    if entry.file_type().is_dir() && entry.file_name() == ".git" {
        let parent_path =
            entry.path().parent().expect("Could not determine parent.");
        match parent_path.to_str() {
            Some(path) => {
                repos.push(Repo::new(path.to_string()));
            }
            None => (),
        }
        // Lets not recurse into git directories...
        // next;
    }
}

fn is_a_repo(entry: &DirEntry) -> bool {
    // println!("entry is {}", entry.path().to_str().unwrap());
    entry.file_type().is_dir()
        && entry.path().read_dir().expect("read dir failed").any(|f| {
            let ff = f.expect("works");
            ff.file_type().unwrap().is_dir() && ff.file_name() == ".git"
        })
}

fn is_a_git(entry: &DirEntry) -> bool {
    entry.file_type().is_dir() && entry.file_name() == ".git"
}

// fn parent_is_a_repo(entry: &DirEntry) -> bool {
//     // println!("entry is {}", entry.path().to_str().unwrap());
//     // parent = entry.path()
//     entry.file_type().is_dir()
//         && entry.path().read_dir().expect("read dir failed").any(|f| {
//             let ff = f.expect("works");
//             ff.file_type().unwrap().is_dir() && ff.file_name() == ".git"
//         })
// }

fn my_repo_check(repos: &mut Vec<Repo>, entry: DirEntry) -> () {
    // println!("We are checking {} to see if it has a repo...", entry.path().to_str().expect("MISSING"));
    if is_a_repo(&entry) {
        // if entry.file_type().is_dir() {
        //     if entry.path().read_dir().expect("read dir failed").any(|f| {
        //         let ff = f.expect("works");
        //         ff.file_type().unwrap().is_dir() && ff.file_name() == ".git"
        //     }) {
        println!("A REPO IS {}", entry.path().to_str().unwrap());
        repos.push(Repo::new(entry.path().to_str().unwrap().to_string()));
        // }

        // for f in entry.path().read_dir().expect("read dir failed") {
        //     let ff = f.expect("unwrap again...");
        //     if ff.file_type().unwrap().is_dir() && ff.file_name() == ".git" {
        //         repos.push(Repo::new(
        //             entry.path().to_str().unwrap().to_string(),
        //         ));
        //         break;
        //         // let contents = fs::read_to_string(ff.path());
        //         // return Ok(true);
        //     }
        // }
        // let it = entry.path().read_dir().expect("read dir failed").collect();
        // if it.contains()

        // }
        // entry.file_name() == ".git" {
        //     let parent_path =
        //         entry.path().parent().expect("Could not determine parent.");
        //     match parent_path.to_str() {
        //         Some(path) => {
        //             repos.push(Repo::new(path.to_string()));
        //         }
        //         None => (),
        //     }
        // Lets not recurse into git directories...
        // next;
    }
}

fn repos_contains_ancestor(
    entry: &DirEntry,
    // uc: &GitGlobalConfig,
    repos: &Vec<Repo>,
) -> bool {
    repos.into_iter().any(|r| entry.path().starts_with(&r.path))
}

fn walk_here(
    entry: &DirEntry,
    uc: &GitGlobalConfig,
    // repos: &Vec<Repo>,
) -> bool {
    uc.filter(entry)
    // && is_a_repo(entry)
    // && repos
    //     .into_iter()
    //     .any(|r| r.path == entry.path().to_str().unwrap())
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
    // for entry in walker {
    //     if let Ok(e) = entry {
    //         if user_config.filter(&e) && is_a_repo(&e) {
    //             repos.push(Repo::new(e.path().to_str().unwrap().to_string()));
    //         }
    //     }
    // }
    for entry in walker.filter_entry(|e| walk_here(&e, &user_config))
    // walker.filter_entry(|e| user_config.filter(e) && !is_a_repo(&e))
    {
        match entry {
            Ok(entry) => {
                // if !((&repos).into_iter().any(|r| {
                //     println!(
                //         "rpath {}, entry_parent {}",
                //         r.path,
                //         entry.path().parent().unwrap().to_str().unwrap()
                //     );
                //     return r.path
                //         == entry.path().parent().unwrap().to_str().unwrap();
                // })) {
                // if !(entry.path().parent().unwrap().read_dir().unwrap().any(
                //     |ff| {
                //         // let
                //         let fff = ff.unwrap();
                //         return (&fff).file_type().unwrap().is_dir()
                //             && (&fff).file_name() == ".git";
                //     },
                // )) {
                if !repos_contains_ancestor(&entry, &repos) {
                    my_repo_check(&mut repos, entry);
                }
                // my_repo_check(&mut repos, entry);
                // repo_check(&mut repos, entry);
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
