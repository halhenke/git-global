//! The `list` subcommand: lists all repos known to git-global.

use core::{GitGlobalConfig, GitGlobalResult, get_repos};
use core::errors::Result;
use core::does_this_work;

/// Forces the display of each repo path, without any extra output.
pub fn get_results() -> Result<GitGlobalResult> {
    let repos = get_repos();
    let mut result = GitGlobalResult::new(&repos);

    let user_config = GitGlobalConfig::new();
    // user_config.print_tags();
    let tags = user_config.read_tags();
    tags.into_iter()
        .for_each(|t| println!("Your tag is {}",t ));
    // print!("does this work: {:?}", does_this_work(tags));
    Ok(result)
}
