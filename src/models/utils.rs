//! Core functionality of git-global.
//!
//! Includes the `Repo`, `GitGlobalConfig`, and `GitGlobalResult` structs, and
//! the `get_repos()` function.

pub use crate::models::config::GitGlobalConfig;
pub use crate::models::result::GitGlobalResult;
pub use crate::models::{repo::Repo, repo_tag::RepoTag};
use colored::*;

use futures::future;

// use std::fmt;

use jwalk;
use walkdir::DirEntry;

extern crate serde;
extern crate serde_json;

/// Trying to get .gitignore contents
/// - part of a strategy to not recurse into ignored directories
/// Not used at preesnt but perhaps later.
fn repo_filter(
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
/// TODO: Shouldnt this be a method on GitGlobalConfig?
async fn new_find_repos_async() -> future::Ready<Result<Vec<Repo>, ()>> {
    trace!("new_find_repos_async");
    let mut repos: Vec<Repo> = Vec::new();
    let user_config = GitGlobalConfig::new();
    let basedir = &user_config.basedir;
    let walker = jwalk::WalkDir::new(basedir)
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
    future::ok::<Vec<Repo>, ()>(repos)
    // repos
}

/// Walks the configured base directory, looking for git repos.
/// TODO: Shouldnt this be a method on GitGlobalConfig?
pub fn new_find_repos() -> Vec<Repo> {
    trace!("new_find_repos");
    let mut repos: Vec<Repo> = Vec::new();
    let user_config = GitGlobalConfig::new();
    let basedir = &user_config.basedir;
    let walker = jwalk::WalkDir::new(basedir)
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
