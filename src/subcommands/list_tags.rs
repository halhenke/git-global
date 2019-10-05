//! The `list` subcommand: lists all repos known to git-global.

// use core::does_this_work;
use core::errors::Result;
use core::{get_repos, GitGlobalConfig, GitGlobalResult};

/// Forces the display of each repo path, without any extra output.
pub fn get_results() -> Result<GitGlobalResult> {
    let repos = get_repos();
    let result = GitGlobalResult::new(&repos);

    let user_config = GitGlobalConfig::new();
    // user_config.print_tags();
    let tags = user_config.read_tags().unwrap_or(vec![]);
    tags.into_iter().for_each(|t| println!("Your tag is {}", t));
    // print!("does this work: {:?}", does_this_work(tags));
    Ok(result)
}
