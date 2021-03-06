//! The `filter` subcommand: lists all repos known to git-global.

use super::utilities::print_str_pat;
use crate::models::errors::Result;
use crate::models::{
    config::GitGlobalConfig, repo_tag::RepoTag, result::GitGlobalResult,
};

/// Forces the display of each repo path, without any extra output.
pub fn get_results(pat: &str, tags: Vec<&str>) -> Result<GitGlobalResult> {
    let tag_conv = &tags
        .iter()
        .flat_map(|x| x.split(","))
        .map(|x| x.trim())
        .map(|x| RepoTag::new(&x))
        .collect();

    let mut gc = GitGlobalConfig::new();
    let repos = gc.get_tagged_repos(tag_conv);
    let mut result = GitGlobalResult::new(&repos);
    for repo in repos.iter().filter(|&x| x.path().contains(pat)) {
        // GitGlobalResult.print() already prints out the repo name if it has
        // any messages, so just add an empty string to force display of the
        // repo name.
        result.add_repo_message(repo, print_str_pat(&repo.path(), Some(pat)));
    }
    Ok(result)
}
