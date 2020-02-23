//! The command line interface for git-global.

use crate::models::{
    errors::{GitGlobalError, Result},
    result::GitGlobalResult,
};
use crate::subcommands;
use anyhow::{Context, Error};
use clap::{App, Arg, ArgMatches, Shell, SubCommand};
// use futures::{future, io};
use std::io::{stderr, Write};

// use dirs::home_dir;
// use config::{Config, ConfigError, File};

// pub fn make_config() {
//     let mut c = Config::new();
//     let mut home_config = home_dir().unwrap();
//     home_config.push(".git_global.ini");
//     c.merge(File::with_name(home_config.to_str().unwrap()))
//         .unwrap();
// }

/// Returns the definitive clap::App instance for git-global.
pub fn get_clap_app<'a, 'b>() -> App<'a, 'b> {
    trace!("get_clap_app");
    App::new("git-global")
        .version(crate_version!())
        .author("Eric Petersen <eric@ericpetersen.io>")
        .about("git subcommand for working with all git repos on a machine")
        .arg(
            Arg::with_name("generate-zsh-completions")
                .long("zsh")
                .help("generate zsh completions for this command"),
        )
        .arg(
            Arg::with_name("json")
                .long("json")
                .help("Output results in JSON."),
        )
        .subcommand(
            SubCommand::with_name("info")
                .about("show meta-information about git-global")
                .arg(Arg::with_name("raw").required(false).takes_value(false)),
        )
        .subcommand(
            SubCommand::with_name("github")
                .about("Logging into github GraphQL API"),
        )
        .subcommand(
            SubCommand::with_name("action")
                .about("commands related to actions that can be performed in specific repositories")
                .subcommand(
                    SubCommand::with_name("perform")
                        .about("perform one (or more?) actions for repositories filtered by tag/path")
                        .arg(
                            Arg::with_name("action")
                                .help("perform this action")
                                .takes_value(true)
                                .required(true))
                        .arg(
                            Arg::with_name("tags")
                                .help("on repos with these tags")
                                .long("tags")
                                .short("t")
                                .takes_value(true))
                                // .required(true)
                        .arg(
                            Arg::with_name("path")
                                .help("on repos which match this path")
                                .multiple(true)
                                .long("path")
                                .short("p")
                                .takes_value(true))
                                // .required(true)
                        )
                .subcommand(
                    SubCommand::with_name("list")
                        .about("list available actions")
                )
        )
        .subcommand(SubCommand::with_name("clean").about("Clear the cache")
            .subcommand(
                SubCommand::with_name("tags")
                    .help("Remove all tags")
            )
            .subcommand(
                SubCommand::with_name("all")
                    .help("Remove all cached repos and tags")
            )
            .subcommand(
                SubCommand::with_name("remove")
                    .help("Remove the cache file")
        ))
        .subcommand(
            SubCommand::with_name("prompt-tui")
                .about("demo the TUI Terminal UI library - pretty useless")
        )
        .subcommand(
            SubCommand::with_name("prompt-cursive")
                .about("demo the Cursive Terminal UI library - also generally does nothing"),
        )
        .subcommand(
            SubCommand::with_name("prompt-iced")
                .about("demo the iced UI library - see how it works out"),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("lists all git repos on your machine [the default]")
                .arg(
                    Arg::with_name("path_filter")
                        .short("p")
                        .long("paths")
                        .takes_value(true)
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("list-tags")
                .about("lists all tags on your machine [the default]")
                .arg(
                    Arg::with_name("with_repos")
                        .short("r")
                        .long("repos")
                        .takes_value(false)
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("add-tags")
                .about("add tags and save them to cache - not sure how/why..."),
        )
        .subcommand(
            SubCommand::with_name("tag-projects")
                .about("edit the association between tags and projects")
                .arg(
                    Arg::with_name("path_filter")
                        .short("p")
                        .long("paths")
                        .takes_value(true)
                        .required(false),
                )
                .arg(
                    Arg::with_name("tag_filter")
                        .short("t")
                        .long("tags")
                        .takes_value(true)
                        .required(false),
                )
        )
        // NOTE: This seems superfluous with list
        .subcommand(
            SubCommand::with_name("filter")
                .about(
                    "lists all git repos on your machine filtered by a pattern",
                )
                .arg(Arg::with_name("pattern").required(true))
                .arg(Arg::with_name("tags").short("t").takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("scan").about("update cache of git repos on your machine"),
        )
        .subcommand(
            SubCommand::with_name("print-cache").about("prints the location of the cache file - not contents")
        )
        .subcommand(
            SubCommand::with_name("status")
                .about("shows status of all git repos (original MSPC implementation)")
                .arg(
                    Arg::with_name("path_filter")
                        .short("p")
                        .long("paths")
                        .takes_value(true)
                        .required(false),
                )
                .arg(
                    Arg::with_name("modified")
                        .short("m")
                        .long("modified-only")
                        .required(false),
                )
                .arg(
                    Arg::with_name("ignore_untracked")
                        .short("i")
                        .long("ignore-untracked")
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("new-status")
                .about("shows status of all git repos (newer implementation with crossbeam)")
                .arg(
                    Arg::with_name("path_filter")
                        .short("p")
                        .long("paths")
                        .takes_value(true)
                        .required(false),
                )
                .arg(
                    Arg::with_name("modified")
                        .short("m")
                        .long("modified-only")
                        .required(false),
                )
                .arg(
                    Arg::with_name("ignore_untracked")
                        .short("i")
                        .long("ignore-untracked")
                        .required(false),
                ),
        )
        .subcommand(SubCommand::with_name("completions").about("outputs zsh specific completion commands and writes them to a file - `_git-global-hal` that can then be copied across to your fpath"))
}

/// Runs the appropriate git-global subcommand based on command line arguments.
///
/// As the effective binary entry point for `git-global`, prints results to
/// `STDOUT` and returns an exit code.
// pub fn run_from_command_line() -> impl futures::Future<Output = i32> {
pub async fn run_from_command_line_scoped() -> Result<()> {
    trace!("run_from_command_line__scoped");
    debug!("I am in async land\n\n");
    let (modified, path_filter, ignore_untracked) = {
        let clap_app = get_clap_app();
        let matches: ArgMatches<'static> = clap_app.get_matches();
        let modified;
        let path_filter;
        let ignore_untracked;
        if let Some("new-status") = matches.subcommand_name() {
            debug!("We matched!!");
            let (m, pf, iu) = {
                let modified =
                    matches.subcommand_matches("new-status").is_some()
                        && matches.is_present("modified");
                let path_filter = get_path_filter(&matches, "new-status");
                let ignore_untracked =
                    matches.subcommand_matches("new-status").is_some()
                        && matches.is_present("ignore_untracked");
                (modified, path_filter, ignore_untracked)
            };
            modified = m;
            path_filter = pf;
            ignore_untracked = iu;
        } else {
            modified = false;
            path_filter = None;
            ignore_untracked = true;
        }
        (modified, path_filter, ignore_untracked)
    };

    debug!("Another debug message!!");
    let l = subcommands::new_status::get_results(
        modified,
        ignore_untracked,
        path_filter,
        None,
    )
    .await
    .expect("");
    show_results(l, false);

    return Ok(());
}

/// Original-ish version of function
// pub fn run_from_command_line__nested() -> i32 {
pub fn run_from_command_line_nested() -> Result<()> {
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
        // Some("list") => subcommands::list::get_results(),
        Some("list") => {
            let path_filter = matches
                .subcommand_matches("list")
                .unwrap()
                .value_of("path_filter");
            // let path_filter = get_path_filter(&matches, "list");
            subcommands::list::get_results(path_filter)
        }
        Some("list-tags") => {
            let with_repos = matches
                .subcommand_matches("list-tags")
                .unwrap()
                .is_present("with_repos");
            subcommands::list_tags::get_results(with_repos)
        }
        Some("action") => match matches
            .subcommand_matches("action")
            .unwrap()
            .subcommand_name()
        {
            Some("perform") => {
                let tags = get_subcommand_value(
                    matches.subcommand_matches("action").unwrap(),
                    "perform",
                    "tags",
                );
                let path = get_subcommand_value(
                    matches.subcommand_matches("action").unwrap(),
                    "perform",
                    "path",
                );
                let action = get_subcommand_values(
                    matches.subcommand_matches("action").unwrap(),
                    "perform",
                    "action",
                );
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
        },
        Some("add-tags") => subcommands::add_tags::go(),
        Some("filter") => {
            let sub_com =
                matches.subcommand_matches("filter").expect("filter panic");
            let pat =
                sub_com.value_of("pattern").expect("a pattern is expected");
            if let Some(tags) = sub_com.values_of("tags") {
                subcommands::filter::get_results(pat, tags.collect())
            } else {
                subcommands::filter::get_results(pat, vec![])
            }
            // subcommands::filter::get_results(pat, tags)
        }
        Some("clean") => {
            trace!("clean matched: {}", matches.subcommand_name().unwrap());
            let sub_com =
                matches.subcommand_matches("clean").expect("clean panic");
            match sub_com.subcommand_name() {
                Some(n) => subcommands::clean::cache_clear(n),
                None => Err(GitGlobalError::MissingSubcommand(
                    vec!["tags", "all"].into_iter().map(String::from).collect(),
                ))
                .context("Not sure this should happen due to clap"),
            }
        }
        Some("scan") => subcommands::scan::get_results(),
        Some("print-cache") => {
            let gc = crate::models::config::GitGlobalConfig::new();
            gc.print_cache()
        }
        Some("prompt-tui") => subcommands::prompt_tui::go(),
        Some("prompt-cursive") => subcommands::prompt_cursive::go(),
        Some("prompt-iced") => subcommands::prompt_iced::go(),
        Some("tag-projects") => {
            let pf = get_path_filter(&matches, "tag-projects");
            subcommands::tag_projects::go(pf)
        }
        Some("status") => get_sync_status(matches),
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
        Some("new-status") => {
            let mut rt = tokio::runtime::Builder::new()
                // .basic_scheduler()
                .threaded_scheduler()
                .enable_io()
                .build()?;
            debug!("Runtime is built\n\n");
            let _modified = matches.subcommand_matches("new-status").is_some()
                && matches.is_present("modified");
            let _path_filter = get_path_filter(&matches, "new-status");
            let _ignore_untracked =
                matches.subcommand_matches("new-status").is_some()
                    && matches.is_present("ignore_untracked");
            let r = rt.block_on(async move {
                // tokio::spawn(async move {
                //     subcommands::sync_status::get_results(
                //         modified,
                //         ignore_untracked,
                //         path_filter,
                //     )
                // });
                tokio::spawn(async move {
                    // let r = get_new_status(&matches).await;
                    get_new_status(&matches).await
                    // print_results(Ok(r), use_json)
                })
                .await
                .expect("new-status spawn faled to join")
                // Err(GitGlobalError::BadSubcommand("oolah".to_owned()))

                // Ok(GitGlobalResult::new(vec![]))
                //     let l = get_new_status(&matches).await.expect("await fail");
                //     show_results(l, use_json);
                //     Err(GitGlobalError::BadSubcommand(cmd.to_string()))
                // // return Ok(());
                // } else {
                //     Err(GitGlobalError::BadSubcommand(cmd.to_string()))
                // }
            });
            Ok(r)
        }
        Some(_) => get_sync_status(matches),
        None => get_sync_status(matches),
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

fn get_sync_status(matches: clap::ArgMatches) -> Result<GitGlobalResult> {
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
