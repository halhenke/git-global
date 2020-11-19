//! The command line interface for git-global.

use crate::models::{
    config::GitGlobalConfig,
    errors::{GitGlobalError, Result},
    result::GitGlobalResult,
};
use crate::{cli::declaration::*, subcommands};
use anyhow::{Context, Error};
use clap::{App, Arg, ArgMatches, Shell, SubCommand};
use colored::Colorize;
use std::io::{stderr, Write};

/// Original-ish version of function
// pub fn run_from_command_line__nested() -> i32 {
pub fn run_from_command_line() -> Result<()> {
    trace!("run_from_command_line__nested");
    let clap_app = get_clap_app();
    let matches: ArgMatches<'static> = clap_app.get_matches();
    let use_json = matches.is_present("json");

    let results = match matches.subcommand_name() {
        Some("github") => subcommands::github::get_results(),
        Some("info") => {
            let raw_info = matches
                .subcommand_matches("info")
                .unwrap()
                .is_present("raw");
            subcommands::info::get_results(raw_info)
        }
        Some("config") => {
            let matches = matches.subcommand_matches("config").unwrap();
            match matches.subcommand_name() {
                Some("print") => {
                    // let gc = crate::models::config::GitGlobalConfig::new();
                    let settings_path = GitGlobalConfig::get_settings_path();
                    println!("\n⏢⏢⏢⏢⏢⏢⏢⏢⏢⏢⏢⏢");
                    println!(
                        "{}: {}",
                        "Settings located at".yellow(),
                        settings_path.red().underline()
                    );
                    println!("⏢⏢⏢⏢⏢⏢⏢⏢⏢⏢⏢⏢\n");
                    let settings = GitGlobalConfig::get_parsed_config()?; //.unwrap();
                    println!("{}", settings);
                    Ok(GitGlobalResult::blank())
                }
                // TODO Fix these fucking errors
                Some(sc) => Err(GitGlobalError::BadSubcommand(sc.to_owned()))
                    .context("Not sure this should happen due to clap"),
                None => Err(GitGlobalError::MissingSubcommand(vec![
                    "config".to_owned()
                ]))
                .context("Not sure this should happen due to clap"),
            }
        }
        Some("repos") => {
            let matches = matches.subcommand_matches("repos").unwrap();
            match matches.subcommand_name() {
                Some("list") => {
                    let path_filter = matches
                        .subcommand_matches("list")
                        .unwrap()
                        .value_of("path_filter");
                    // let path_filter = get_path_filter(&matches, "list");
                    subcommands::list::get_results(path_filter)
                }
                Some("select") => {
                    let path_filter = get_path_filter(&matches, "select");
                    subcommands::repos::select::go(path_filter)
                }
                Some("filter") => {
                    let sub_com = matches
                        .subcommand_matches("filter")
                        .expect("filter panic");
                    let pat = sub_com
                        .value_of("pattern")
                        .expect("a pattern is expected");
                    if let Some(tags) = sub_com.values_of("tags") {
                        subcommands::filter::get_results(pat, tags.collect())
                    } else {
                        subcommands::filter::get_results(pat, vec![])
                    }
                    // subcommands::filter::get_results(pat, tags)
                }
                Some("scan") => subcommands::scan::get_results(),
                // TODO Fix these fucking errors
                Some(sc) => Err(GitGlobalError::BadSubcommand(sc.to_owned()))
                    .context("Not sure this should happen due to clap"),
                None => Err(GitGlobalError::MissingSubcommand(vec![
                    "repos".to_owned()
                ]))
                .context("Not sure this should happen due to clap"),
            }
        }
        Some("tags") => {
            let matches = matches.subcommand_matches("tags").unwrap();
            match matches.subcommand_name() {
                Some("list") => {
                    let with_repos = matches
                        .subcommand_matches("list")
                        .unwrap()
                        .is_present("with_repos");
                    subcommands::list_tags::get_results(with_repos)
                }
                Some("add") => subcommands::add_tags::go(),
                Some("tag-projects") => {
                    let pf = get_path_filter(&matches, "tag-projects");
                    subcommands::tag_projects::go(pf)
                }
                // TODO Fix these fucking errors
                Some(sc) => Err(GitGlobalError::BadSubcommand(sc.to_owned()))
                    .context("Not sure this should happen due to clap"),
                None => Err(GitGlobalError::MissingSubcommand(vec![
                    "tags".to_owned()
                ]))
                .context("Not sure this should happen due to clap"),
            }
        }
        Some("actions") => {
            let matches = matches.subcommand_matches("actions").unwrap();
            match matches.subcommand_name() {
                Some("perform") => {
                    let tags = get_subcommand_value(matches, "perform", "tags");
                    let path = get_subcommand_value(matches, "perform", "path");
                    let action =
                        get_subcommand_values(matches, "perform", "action");
                    subcommands::actions::perform(tags, path, action)
                }
                Some("list") => subcommands::actions::list(),
                _ => Err(GitGlobalError::BadSubcommand(String::from(
                    "bad action subcommand",
                )))
                .context("tried to use list"),
                // _ => Err(GitGlobalError::BadSubcommand(String::from(
                //     "bad action subcommand",
                // ))),
            }
        }
        Some("cache") => {
            let matches = matches.subcommand_matches("cache").unwrap();
            match matches.subcommand_name() {
                Some("clean") => {
                    trace!(
                        "clean matched: {}",
                        matches.subcommand_name().unwrap()
                    );
                    let sub_com = matches
                        .subcommand_matches("clean")
                        .expect("clean panic");
                    match sub_com.subcommand_name() {
                        Some(n) => subcommands::clean::cache_clear(n),
                        None => Err(GitGlobalError::MissingSubcommand(
                            vec!["tags", "all"]
                                .into_iter()
                                .map(String::from)
                                .collect(),
                        ))
                        .context("Not sure this should happen due to clap"),
                    }
                }
                Some("print") => {
                    let gc = crate::models::config::GitGlobalConfig::new();
                    gc.print_cache()
                }
                // TODO Fix these fucking errors
                Some(sc) => Err(GitGlobalError::BadSubcommand(sc.to_owned()))
                    .context("Not sure this should happen due to clap"),
                None => Err(GitGlobalError::MissingSubcommand(vec![
                    "cache".to_owned()
                ]))
                .context("Not sure this should happen due to clap"),
            }
        }
        Some("prompt") => {
            let matches = matches.subcommand_matches("prompt").unwrap();
            match matches.subcommand_name() {
                Some("prompt-tui") => subcommands::prompt_tui::go(),
                Some("prompt-cursive") => subcommands::prompt_cursive::go(),
                Some("prompt-iced") => subcommands::prompt_iced::go(),
                // TODO Fix these fucking errors
                Some(sc) => Err(GitGlobalError::BadSubcommand(sc.to_owned()))
                    .context("Not sure this should happen due to clap"),
                None => Err(GitGlobalError::MissingSubcommand(vec![
                    "prompt".to_owned()
                ]))
                .context("Not sure this should happen due to clap"),
            }
        }
        Some("status") => {
            let matches = matches.subcommand_matches("status").unwrap();
            match matches.subcommand_name() {
                Some("status") => get_sync_status(matches),
                Some("new-status") => {
                    let mut rt = tokio::runtime::Builder::new_multi_thread()
                        // .basic_scheduler()
                        // .threaded_scheduler()
                        .worker_threads(4)
                        .enable_io()
                        .build()?;
                    debug!("Runtime is built\n\n");
                    let _modified =
                        matches.subcommand_matches("new-status").is_some()
                            && matches.is_present("modified");
                    let _path_filter = get_path_filter(&matches, "new-status");
                    let _ignore_untracked =
                        matches.subcommand_matches("new-status").is_some()
                            && matches.is_present("ignore_untracked");
                    let m = matches.clone();
                    let r = rt.block_on(async move {
                        tokio::spawn(async move { get_new_status(&m).await })
                            .await
                            .expect("new-status spawn faled to join")
                    });
                    Ok(r)
                }
                // TODO Fix these fucking errors
                Some(sc) => Err(GitGlobalError::BadSubcommand(sc.to_owned()))
                    .context("Not sure this should happen due to clap"),
                None => Err(GitGlobalError::MissingSubcommand(vec![
                    "status".to_owned()
                ]))
                .context("Not sure this should happen due to clap"),
            }
        }
        // Some("new-status") => get_new_status(matches),
        Some("completions") => {
            let mut file = std::fs::File::create("_git-global-hal")
                .expect("Could not write completions file");
            get_clap_app().gen_completions_to(
                "git-global-hal",
                Shell::Zsh,
                &mut file, // &mut io::stdout(),
            );
            Ok(GitGlobalResult::new(&Vec::new()))
        }
        // Some(cmd) => Err(GitGlobalError::BadSubcommand(cmd.to_string())),
        Some(_) => get_sync_status(&matches),
        None => get_sync_status(&matches),
    };
    // debug!("Results are {:#?}", results);
    print_results(results, use_json)
}

pub fn print_results(
    results: Result<GitGlobalResult>,
    use_json: bool,
) -> Result<()> {
    match results {
        Ok(res) => {
            debug!("Made it here - show_results");
            show_results(res, use_json);
            Ok(())
        }
        Err(err) => {
            debug!("Made it here - show_error");
            show_error(err, use_json);
            Err(GitGlobalError::FromIOError(
                "something happened...".to_owned(),
            ))
            .context("I catch error stupidly - this should be a map function")
        }
    }
}

fn get_sync_status(matches: &clap::ArgMatches) -> Result<GitGlobalResult> {
    // fn get_sync_status(matches: clap::ArgMatches) -> errors::Result<GitGlobalResult> {
    trace!("get_sync_status");
    let modified = matches.subcommand_matches("status").is_some()
        && matches.is_present("modified");
    let path_filter = get_path_filter(&matches, "status");
    let ignore_untracked = matches.subcommand_matches("status").is_some()
        && matches.is_present("ignore_untracked");
    subcommands::sync_status::get_results(
        modified,
        ignore_untracked,
        path_filter,
    )
}

async fn get_new_status(
    matches: &clap::ArgMatches<'_>,
    // ) -> Result<GitGlobalResult> {
) -> GitGlobalResult {
    trace!("get_new_status");
    let modified = matches.subcommand_matches("new-status").is_some()
        && matches.is_present("modified");
    let path_filter = get_path_filter(&matches, "new-status");
    let ignore_untracked = matches.subcommand_matches("new-status").is_some()
        && matches.is_present("ignore_untracked");
    subcommands::new_status::get_results(
        modified,
        ignore_untracked,
        path_filter,
        None,
    )
    .await
    .expect("")
    // .expect("Future failed")
}

fn get_path_filter(matches: &ArgMatches, subcommand: &str) -> Option<String> {
    get_subcommand_value(matches, subcommand, "path_filter")
}

/// Get the [`Arg`] value for a given [`SubCommand`]
fn get_subcommand_value(
    matches: &ArgMatches,
    subcommand: &str,
    value: &str,
) -> Option<String> {
    // matches
    //     .subcommand_matches(subcommand)
    //     .unwrap()
    //     .value_of(value)
    //     .map(|s| String::from(s))
    match matches.subcommand_matches(subcommand) {
        Some(s) => s.value_of(value).map(|s| String::from(s)),
        None => None,
    }
}

/// Get all [`Arg`] values for a given [`SubCommand`]
fn get_subcommand_values(
    matches: &ArgMatches,
    subcommand: &str,
    value: &str,
) -> Option<String> {
    let result = matches
        .subcommand_matches(subcommand)
        .unwrap()
        .values_of(value)
        .unwrap()
        .fold(String::new(), |a, b| [a, b.to_owned()].concat());
    debug!("values is {}", result);
    Some(result)

    // .map(|s| String::from(s))
}

/// Writes results to STDOUT, as either text or JSON, and returns `0`.
// async fn show_results(results: impl Future, use_json: bool) -> i32
// async fn show_results<T>(
//     // async fn show_results<T, T: Output = GitGlobalResult>(
//     results: T,
//     use_json: bool,
// ) -> i32
// where
//     T: Future<Output = GitGlobalResult>,
//     // T::Output: GitGlobalResult,
// {
// async fn show_results(results: GitGlobalResult, use_json: bool) -> i32 {
fn show_results(results: GitGlobalResult, use_json: bool) -> i32 {
    trace!("show_results");
    let r = results;
    // let r: GitGlobalResult = results.await;
    debug!("We showed!!");

    if use_json {
        r.print_json();
    // results.print_json();
    } else {
        r.print();
        // results.print();
    }
    0
}

/// Writes errors to STDERR, as either text or JSON, and returns `1`.
fn show_error(error: Error, use_json: bool) -> i32 {
    trace!("show_error");
    if use_json {
        let json = object! {
            "error" => true,
            "message" => format!("{}", error)
        };
        writeln!(&mut stderr(), "{:#}", json).expect("failed write to STDERR");
    } else {
        writeln!(&mut stderr(), "{}", error).expect("failed write to STDERR");
    }
    1
}
