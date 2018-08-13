//! The `filter` subcommand: lists all repos known to git-global.

use core::{GitGlobalResult, RepoTag, get_tagged_repos};
use clap::{Arg, App, SubCommand, Values};
use core::errors::Result;
use subcommands::utilities::{print_str_pat};

/// Forces the display of each repo path, without any extra output.
pub fn get_results(pat: &str, tags: Vec<&str>) -> Result<GitGlobalResult> {

    let tag_conv = &tags.iter()
        .flat_map(|x| x.split(","))
        .map(|x| x.trim())
        .map(|x| RepoTag::new(&x))
        .collect();

    let repos = get_tagged_repos(tag_conv);
    let mut result = GitGlobalResult::new(&repos);
    for repo in repos.iter().filter(|&x| x.path().contains(pat)) {
        // GitGlobalResult.print() already prints out the repo name if it has
        // any messages, so just add an empty string to force display of the
        // repo name.
        result.add_repo_message(repo, print_str_pat(&repo.path(), Some(pat)));
    }
    Ok(result)
}
