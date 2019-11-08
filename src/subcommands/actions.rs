//! The `action` subcommand:
//!     - `perform` - performs the given action/s in named repo/s or repo/s filtered by name/tag
//!     - `list` - list all currently available actions

// use repo::does_this_work;
use repo::errors::Result;
use repo::Repo;
use repo::{get_repos, GitGlobalConfig, GitGlobalResult};

/// Forces the display of each repo path, without any extra output.
pub fn list() -> Result<GitGlobalResult> {
    let gc = GitGlobalConfig::new();
    gc.actions
        .into_iter()
        .for_each(|a| println!("Action:\t{}", a));
    // let result: Vec<GitGlobalResult> =
    let repos = vec![];
    let result = GitGlobalResult::new(&repos as &Vec<Repo>);
    Ok(result)
}

pub fn perform() -> Result<GitGlobalResult> {
    unimplemented!();
}
