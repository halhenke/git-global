//! The `filter` subcommand: lists all repos known to git-global.

use core::{GitGlobalResult, get_repos};
use errors::Result;
use subcommands::utilities::{print_str_pat};

/// Forces the display of each repo path, without any extra output.
pub fn get_results(pat: &str) -> Result<GitGlobalResult> {
    let repos = get_repos();
    let mut result = GitGlobalResult::new(&repos);
    for repo in repos.iter().filter(|&x| x.path().contains(pat)) {
        // GitGlobalResult.print() already prints out the repo name if it has
        // any messages, so just add an empty string to force display of the
        // repo name.
        result.add_repo_message(repo, print_str_pat(&repo.path(), Some(pat)));
    }
    Ok(result)
}
