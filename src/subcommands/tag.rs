//! The `tag` subcommand: lists all repos known to git-global.

use crate::models::errors::Result;
use crate::models::{get_tagged_repos, GitGlobalResult, RepoTag};
// use subcommands::utilities::print_str_pat;

/// Forces the display of each repo path, without any extra output.
pub fn get_results(tags: Vec<&str>) -> Result<GitGlobalResult> {
    let tag_conv = &tags
        .iter()
        .flat_map(|x| x.split(","))
        .map(|x| x.trim())
        .map(|x| RepoTag::new(&x))
        .collect();

    let repos = get_tagged_repos(tag_conv);
    let mut result = GitGlobalResult::new(&repos);
    for repo in repos.iter() {
        // for repo in repos.iter().filter(|&x| x.path().contains(pat)) {
        // GitGlobalResult.print() already prints out the repo name if it has
        // any messages, so just add an empty string to force display of the
        // repo name.
        result.add_repo_message(repo, format!("Tagged repo {}", repo));
    }
    Ok(result)
}
