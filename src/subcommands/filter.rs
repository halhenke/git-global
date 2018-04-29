//! The `filter` subcommand: lists all repos known to git-global.

extern crate colored;
use self::colored::*;
extern crate itertools;
use self::itertools::Itertools;
use core::{GitGlobalResult, get_repos};
use errors::Result;

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


fn print_str_pat<'a>(string: &'a str, pat: Option<&str>) -> String {
    if let Some(pat) = pat {
        let split: String = string
            .split(pat)
            .map(|x| x.blue())
            .intersperse(pat.green())
            .join("")
            .to_string();
        return split;
    }
    else {
        // io::stdout().write(string.as_ptr());
        // println!("{}", string.blue());
        return string.blue().to_string();
    }
}
