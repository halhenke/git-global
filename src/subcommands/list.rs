//! The `list` subcommand: lists all repos known to git-global.

use repo::errors::Result;
use repo::{get_repos, GitGlobalResult};

/// Forces the display of each repo path, without any extra output.
pub fn get_results() -> Result<GitGlobalResult> {
    let repos = get_repos();
    let mut result = GitGlobalResult::new(&repos);
    for repo in repos.iter() {
        // GitGlobalResult.print() already prints out the repo name if it has
        // any messages, so just add an empty string to force display of the
        // repo name.
        println!("BOO {}", &repo);
        result.add_repo_message(repo, format!(""));
    }
    Ok(result)
}
