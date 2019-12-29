//! The `list` subcommand: lists all repos known to git-global.

// use crate::models::does_this_work;
use crate::models::errors::Result;
use crate::models::{config::GitGlobalConfig, result::GitGlobalResult};

/// Forces the display of each repo path, without any extra output.
pub fn get_results() -> Result<GitGlobalResult> {
    let mut gc = GitGlobalConfig::new();
    // let tags = gc.get_cached_tags().unwrap_or(vec![]);
    let tags = gc.get_cached_tags();

    let repos = gc.get_repos();
    let result = GitGlobalResult::new(&repos);

    tags.into_iter().for_each(|t| println!("Your tag is {}", t));
    // print!("does this work: {:?}", does_this_work(tags));
    Ok(result)
}
