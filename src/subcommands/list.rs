//! The `list` subcommand: lists all repos known to git-global.

use crate::repo::errors::Result;
use crate::repo::{get_repos, GitGlobalResult};
use colored::*;
use itertools::Itertools;

/// Forces the display of each repo path, without any extra output.
pub fn get_results(path_filter: Option<&str>) -> Result<GitGlobalResult> {
    // pub fn get_results(path_filter: Option<String>) -> Result<GitGlobalResult> {
    let repos = get_repos();
    let mut result = GitGlobalResult::new(&repos);
    for repo in repos.iter() {
        if let Some(path_filter) = path_filter {
            if !repo.path.contains(path_filter) {
                continue;
            }
            let mut ss = repo
                .path
                .split(path_filter)
                .into_iter()
                .map(|s| s.blue())
                .join(&path_filter.green().to_string());
            // note: this is actually a bit of a hack - our API is supposed to deal with "result-wide" messages or repo-wide messages - but the latter automatically print the (non-colored) repo path also
            result.add_message(ss);
        // result.add_repo_message(repo, ss);
        } else {
            result.add_repo_message(repo, format!(""));
        }
        // GitGlobalResult.print() already prints out the repo name if it has
        // any messages, so just add an empty string to force display of the
        // repo name.
        // println!("BOO {}", &repo);
    }
    Ok(result)
}
