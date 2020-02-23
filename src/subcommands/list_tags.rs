//! The `list` subcommand: lists all repos known to git-global.

// use crate::models::does_this_work;
use crate::models::errors::Result;
use crate::models::{config::GitGlobalConfig, result::GitGlobalResult};
use colored::Colorize;


/// Forces the display of each repo path, without any extra output.
pub fn get_results(with_repos: bool) -> Result<GitGlobalResult> {
    let mut gc = GitGlobalConfig::new();
    // let tags = gc.get_cached_tags().unwrap_or(vec![]);
    let tags = gc.get_cached_tags();

    let repos = gc.get_repos();
    let result = GitGlobalResult::new(&repos);

    tags.into_iter().for_each(|t| {
        println!("{} {}", "Tag:".yellow(), t.name.green().underline());
        if with_repos {
            println!("{}", "Repos:".yellow());
            repos.iter().for_each(|r| {
                // for r in repos {
                if r.tags.iter().any(|t2| t2 == &t) {
                    println!("{}", r.path.green().underline());
                }
            })
        }
    });
    // print!("does this work: {:?}", does_this_work(tags));
    Ok(result)
}
