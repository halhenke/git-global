//! The `list` subcommand: lists all repos known to git-global.

extern crate github_gql;
extern crate serde_json;
use self::github_gql as gql;
use self::gql::client::{Github};
use self::gql::query::{Query};
use self::serde_json::{Value};

use std::path::Path;
use std::io::Read;
use std::fs::File;

use core::{GitGlobalResult, get_repos};
use errors::Result;

/// Forces the display of each repo path, without any extra output.
pub fn get_results() -> Result<GitGlobalResult> {
    // NOTE: This path should be canonicalized
    // let mut tok = File::open(".secrets").unwrap();
    let p = Path::new(".secrets")
        .canonicalize()
        .unwrap()
        .into_os_string();
    let mut tok = File::open(p).unwrap();
    let mut json = String::new();
    tok.read_to_string(&mut json);
    let tokVal: Value = serde_json::from_str(&json).unwrap();

    let mut gh = Github::new(&tokVal["github"].as_str().unwrap())
        .unwrap();

    // let q_str = r#"
    //     query {
    //         viewer {
    //             login
    //         }
    //     }
    // "#.replace("\n", "");

    let q_str =
        r#"query {
            repository(owner: \"halhenke\", name: \"stack-mate\") {
                labels(first: 10) {
                    edges {
                        node {
                            name
                        }
                    }
                }
            }
        }"#.split("\n").collect::<Vec<_>>().concat();
    let q = Query::new_raw(q_str);
    let (head, stat, code) = gh.query::<Value>(&q).unwrap();

    println!("head {}", head);
    println!("stat {}", stat);
    println!("code {}", code.unwrap());


    let repos = get_repos();
    let mut result = GitGlobalResult::new(&repos);
    for repo in repos.iter() {
        // GitGlobalResult.print() already prints out the repo name if it has
        // any messages, so just add an empty string to force display of the
        // repo name.
        result.add_repo_message(repo, format!(""));
    }
    Ok(result)
}
