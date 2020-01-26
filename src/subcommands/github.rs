//! Cant remember what this does...

extern crate github_rs;
extern crate serde_json;
extern crate strfmt;
use self::github_rs as gql;
use self::gql::client::{Executor, Github};
// use self::gql::client::{Client, Github};
// use self::gql::query::Query;
use self::serde_json::Value;

use self::strfmt::strfmt;
use std::collections::HashMap;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::models::errors::Result;
use crate::models::{config::GitGlobalConfig, result::GitGlobalResult};

fn get_query(owner: &str, name: &str) -> String {
    // fn get_query(owner: &str, name: &str) -> Value {
    let p = Path::new("src/queries/tags.json")
        .canonicalize()
        .unwrap()
        .into_os_string();
    let mut tok = File::open(p).unwrap();
    let mut template = String::new();
    tok.read_to_string(&mut template).expect("Read String fail");
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
    let mut tok = File::open(p)?;
    let mut json = String::new();
    tok.read_to_string(&mut json).expect("Read String fail");
    let tok_val: Value = serde_json::from_str(&json).unwrap();

    let gh = Github::new(&tok_val["github"].as_str().unwrap()).unwrap();

    // let q_str = r#"
    //     query {
    //         viewer {
    //             login
    //         }
    //     }
    // "#.replace("\n", "");
    let _q_str = r#"query {
            repository(owner: \"halhenke\", name: \"stack-mate\") {
                labels(first: 10) {
                    edges {
                        node {
                            name
                        }
                    }
                }
            }
        }"#
    .split("\n")
    .collect::<Vec<_>>()
    .concat();
    // let q = gh.client.get_query(q_str);
    let q = gh.get().user().execute::<Value>();
    // let q = client.get()
    // let q = gh.query(q_str);

    // NOTE: Trying to work it
    // return "Sorry!";

    // let q = Query::new_raw(q_str);
    let (_head, stat, code) = q.unwrap();
    // let (head, stat, code) = gh.query::<Value>(&q).unwrap();

    // println!("head {}", head);
    println!("stat {}", stat);
    println!("code {}", code.unwrap());

    let mut gc = GitGlobalConfig::new();
    let repos = gc.get_repos();
    let mut result = GitGlobalResult::new(&repos);
    // for repo in repos.iter() {
    //     // GitGlobalResult.print() already prints out the repo name if it has
    //     // any messages, so just add an empty string to force display of the
    //     // repo name.
    //     result.add_repo_message(repo, format!(""));
    // }
    Ok(result)
}
