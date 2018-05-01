//! The `filter` subcommand: lists all repos known to git-global.

use core::{GitGlobalResult, RepoTag, get_repos, get_tagged_repos};
use clap::{Arg, App, SubCommand, Values};
use errors::Result;
use subcommands::utilities::{print_str_pat};

/// Forces the display of each repo path, without any extra output.
pub fn get_results(pat: &str, tags: Option<Values>) -> Result<GitGlobalResult> {
// pub fn get_results(pat: &str, tags: &Option<Values>) -> Result<GitGlobalResult> {
    // println!("Now 1");
    // let vtags: Vec<_> = tags
    //     .clone()
    //     .unwrap()
    //     // .into_iter()
    //     // .cloned()
    //     // .split(",")
    //     .collect();
    //     // .collect::<Vec<_>>();
    //     // .collect::<Vec<&str>>();
    // println!("Now 2: {:?}", vtags);
    // println!("Now 2: {:?}", tags.unwrap().collect::<Vec<&str>>());


    // let tag_conv: Vec<RepoTag> = vec![];
    let tag_conv: Vec<RepoTag> = tags
            // .next()
            .unwrap()
            // .into_iter()
            // .clone()
            // .collect::<&str>()
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.trim())
            // .cloned()
            // .iter()
            // .map(|x| String::from(x))
            .map(|x| RepoTag::new(x))
            .collect();



    // let tag_conv: Vec<RepoTag> = if let Some(tags_unwrap) = tags {
    // // let tagxxxx = if let Some(tags_unwrap) = tags {
    // // let tag_conv = if let Some(tags_unwrap) = tags {
    //     tags_unwrap
    //         .next()
    //         .unwrap()
    //         // .into_iter()
    //         // .clone()
    //         // .collect::<&str>()
    //         .split(",")
    //         // .cloned()
    //         // .iter()
    //         // .map(|x| String::from(x))
    //         .map(|x| RepoTag::new(x))
    //         .collect()
    // } else {
    //     vec![]
    // };

    // let tag_conv = vec![RepoTag::new("hoot")];

    // let tag_conv: Vec<RepoTag> = tags
    //     .unwrap_or("")
    //     .split(",")
    //     .map(|x| RepoTag::new(x))
    //     .collect();
    // println!("Now 2: {}", tags.unwrap_or("HOLD"));

    // println!("Now 2: {}", tags.expect("damn").cloned().collect());
    // println!("Now 3: {}", tag_conv.len());

    let repos = get_tagged_repos(tag_conv);
    // let repos = get_repos();
    let mut result = GitGlobalResult::new(&repos);
    for repo in repos.iter().filter(|&x| x.path().contains(pat)) {
        // GitGlobalResult.print() already prints out the repo name if it has
        // any messages, so just add an empty string to force display of the
        // repo name.
        result.add_repo_message(repo, print_str_pat(&repo.path(), Some(pat)));
    }
    Ok(result)
}
