//! The `list` subcommand: lists all repos known to git-global.

use core::{GitGlobalConfig, GitGlobalResult, get_repos};
use errors::Result;

/// Forces the display of each repo path, without any extra output.
pub fn get_results() -> Result<GitGlobalResult> {
    let repos = get_repos();
    let mut result = GitGlobalResult::new(&repos);

    let user_config = GitGlobalConfig::new();
    user_config.print_tags();

    Ok(result)
}
