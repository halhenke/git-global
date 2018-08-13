//! The `list` subcommand: lists all repos known to git-global.

extern crate github_gql;
extern crate serde_json;
extern crate strfmt;
use self::github_gql as gql;
use self::gql::client::{Github};
use self::gql::query::{Query};
use self::serde_json::{Value};

use self::strfmt::strfmt;
use std::collections::HashMap;

use std::path::Path;
use std::io::Read;
use std::fs::File;

use core::{GitGlobalResult, get_repos};
use core::errors::Result;

fn get_query(owner: &str, name: &str) -> String {
// fn get_query(owner: &str, name: &str) -> Value {
    let p = Path::new("src/queries/tags.json")
        .canonicalize()
        .unwrap()
        .into_os_string();
    let mut tok = File::open(p).unwrap();
    let mut template = String::new();
    tok.read_to_string(&mut template);
    // let mut json = String::new();
    // NOTE: How do I use a non string literal as a formatter?
    // - you need to use a templating thing
    let mut vars = HashMap::new();
    vars.insert("owner".to_string(), owner);
    vars.insert("name".to_string(), name);
    return strfmt(&template, &vars).unwrap();

    // return json;
    // let tok_val: Value = serde_json::from_str(&json).unwrap();
    // return tok_val;
}

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
    let tok_val: Value = serde_json::from_str(&json).unwrap();

    let mut gh = Github::new(&tok_val["github"].as_str().unwrap())
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
