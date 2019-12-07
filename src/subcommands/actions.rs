//! The `action` subcommand:
//!     - `perform` - performs the given action/s in named repo/s or repo/s filtered by name/tag
//!     - `list` - list all currently available actions

// use crate::models::does_this_work;
use crate::models::errors::Result;
use crate::models::{get_repos, GitGlobalConfig, GitGlobalResult};
use crate::models::{Action, ActionError, Filterable, Repo, RepoTag};
use colored::Colorize;

/// Forces the display of each repo path, without any extra output.
pub fn list() -> Result<GitGlobalResult> {
    let gc = GitGlobalConfig::new();
    gc.actions
        .into_iter()
        .for_each(|a| println!("Action:\t{}", a));
    // let result: Vec<GitGlobalResult> =
    let repos = vec![];
    let result = GitGlobalResult::new(&repos as &Vec<Repo>);
    Ok(result)
}

pub fn perform(
    tags: Option<String>,
    path: Option<String>,
    action: Option<String>,
) -> Result<GitGlobalResult> {
    let gc = GitGlobalConfig::new();
    let mut repos = gc.get_cached_repos();
    if let Some(tags) = tags {
        let tags: Vec<RepoTag> = tags.split(",").map(RepoTag::new).collect();
        repos = repos.filter_tags(tags);
    }
    if let Some(path) = path {
        repos = repos.filter_paths(path);
    }
    for r in &repos {
        println!("Doing {}", r.path);
    }
    // if gc.actions.
    let action_name = action.unwrap();
    match &gc
        .actions
        .iter()
        .find(|a| a.name_match(&action_name).is_some())
    {
        Some(act) => {
            if act.needs_path() {
                println!("{}", "⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️".yellow());
                for r in &repos {
                    match act.path_bind(r.path.clone()) {
                        Ok(a) => {
                            let result = a.perform_action_for_repo();
                            let ss = format!(
                                "{} {} {} {}",
                                "Result of action".blue(),
                                a.get_name().green().underline(),
                                "for".blue(),
                                r.path().green().underline()
                            );
                            println!("{}", ss);
                            for r in result.unwrap().split("\n") {
                                println!("{}", r.green());
                            }
                            println!("{}", "**************".yellow());
                        }
                        Err(ActionError::NotANeedAPath(ae)) => {
                            println!("Struck out: {}", ae);
                        }
                        _ => unreachable!(),
                    }
                }
                println!("{}", "⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️⭐️".yellow());
            }
            return Ok(GitGlobalResult::new(&repos));

            // if let Ok(r) = act.perform_action_for_repo() {
            //     println!("{}", r);
            // }
            // return Ok(GitGlobalResult::new(&repos));
        }
        None => {
            println!("No actions matched {}", action_name);
            gc.actions
                .iter()
                .for_each(|a| println!("Actual action: {}", a))
        }
    }

    // for a in gc.actions {
    //     // if let Some(act) = a.check_needs_path_match_name(&action_name, path) {
    //     if let Some(act) = a.name_match(&action_name) {
    //         let result = act.perform_action_for_repo();
    //         // println!("RUN\n {}", result.unwrap());
    //         println!("{}", result.unwrap().green());
    //         // for r in result.unwrap().split("\n") {
    //         //     println!("{}", r.green());
    //         // }
    //         return Ok(GitGlobalResult::new(&repos));
    //     }
    // }
    println!("No actions matched {}", action_name);
    // for a in gc.actions {
    //     println!("Actual action: {}", a);
    // }
    Ok(GitGlobalResult::new(&repos))
    // unimplemented!();
}
