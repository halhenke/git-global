//! The command line interface for git-global.

// use crate::models::errors::Result;
use crate::models::{
    errors::{GitGlobalError, Result},
    light_table::LightTable,
    result::GitGlobalResult,
};

use clap::{App, Arg, ArgMatches, Shell, SubCommand};
use std::io::{stderr, Write};
// use tokio::
use futures::executor::LocalPool;
use futures::{future, io};
use std::future::Future;

use crate::subcommands;

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
            SubCommand::with_name("bullshit")
                .about("Just mucking around with stuff"),
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
            .arg(
                Arg::with_name("tags")
                    .help("Remove all tags")
                    .takes_value(false)
                    .required(false)
            )
            .arg(
                Arg::with_name("all")
                    .help("Remove all cached repos and tags")
                    .takes_value(false)
                    .required(false)
        ))
        .subcommand(
            SubCommand::with_name("prompt")
                .about("demo the TUI Terminal UI library")
        )
        .subcommand(
            SubCommand::with_name("prompt-cursive")
                .about("demo the Cursive Terminal UI library"),
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
                .about("lists all tags on your machine [the default]"),
        )
        .subcommand(
            SubCommand::with_name("add-tags")
                .about("add tags on your machine [the default]"),
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
        .subcommand(
            SubCommand::with_name("tag")
                .about("tag a single git repo")
                .arg(
                    Arg::with_name("tag_arg").required(true).takes_value(true),
                ),
        )
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
pub async fn run_from_command_line() -> Result<()> {
    trace!("run_from_command_line__scoped");
    println!("I am in async land\n\n");
    let (modified, path_filter, ignore_untracked) = {
        let clap_app = get_clap_app();
        let matches: ArgMatches<'static> = clap_app.get_matches();
        let use_json = matches.is_present("json");
        let modified;
        let path_filter;
        let ignore_untracked;
        if let Some("new-status") = matches.subcommand_name() {
            println!("We matched!!");
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

    // if matches.is_present("generate-zsh-completions") {
    //     get_clap_app().gen_completions_to(
    //         "_git-global-hal",
    //         Shell::Zsh,
    //         &mut io::stdout(),
    //     );
    //     return 0;
    // }

    // NOTE: THIS may have been causing program to crash:
    // let mut exec = LocalPool::new();
    println!("Another debug message!!");
    let l = subcommands::new_status::get_results(
        modified,
        ignore_untracked,
        path_filter,
    )
    .await
    .expect("");
    show_results(l, false);

    return Ok(());

    // if let Some("new-status") = matches.subcommand_name() {
    //     println!("We matched!!");
    //     let (modified, path_filter, ignore_untracked) = {
    //         let modified = matches.subcommand_matches("new-status").is_some()
    //             && matches.is_present("modified");
    //         let path_filter = get_path_filter(&matches, "new-status");
    //         let ignore_untracked =
    //             matches.subcommand_matches("new-status").is_some()
    //                 && matches.is_present("ignore_untracked");
    //         (modified, path_filter, ignore_untracked)
    //     };
    //     drop(matches);
    //     let l = subcommands::new_status::get_results(
    //         modified,
    //         ignore_untracked,
    //         path_filter,
    //     )
    //     .await
    //     .expect("");
    //     // let l = { get_new_status(&matches) }.await;
    //     println!("We stat!!");
    //     // .expect("await fail");
    //     // let l = get_new_status(&matches).await.expect("await fail");
    //     // show_results(l, use_json);
    //     return Ok(());
    // } else {
    //     Err(GitGlobalError::FromIOError(
    //         "something happened...".to_owned(),
    //     ))
    // }

    // let results = match matches.subcommand_name() {
    //     Some("bullshit") => subcommands::bullshit::get_results(),
    //     Some("info") => {
    //         let raw_info = matches
    //             .subcommand_matches("info")
    //             .unwrap()
    //             .is_present("raw");
    //         subcommands::info::get_results(raw_info)
    //     }
    //     // Some("list") => subcommands::list::get_results(),
    //     Some("list") => {
    //         let path_filter = matches
    //             .subcommand_matches("list")
    //             .unwrap()
    //             .value_of("path_filter");
    //         // let path_filter = get_path_filter(&matches, "list");
    //         subcommands::list::get_results(path_filter)
    //     }
    //     Some("list-tags") => subcommands::list_tags::get_results(),
    //     Some("action") => match matches
    //         .subcommand_matches("action")
    //         .unwrap()
    //         .subcommand_name()
    //     {
    //         Some("perform") => {
    //             let tags = get_subcommand_value(
    //                 matches.subcommand_matches("action").unwrap(),
    //                 "perform",
    //                 "tags",
    //             );
    //             let path = get_subcommand_value(
    //                 matches.subcommand_matches("action").unwrap(),
    //                 "perform",
    //                 "path",
    //             );
    //             let action = get_subcommand_values(
    //                 matches.subcommand_matches("action").unwrap(),
    //                 "perform",
    //                 "action",
    //             );
    //             subcommands::actions::perform(tags, path, action)
    //         }
    //         Some("list") => subcommands::actions::list(),
    //         _ => Err(GitGlobalError::BadSubcommand(String::from(
    //             "bad action subcommand",
    //         ))),
    //     },
    //     Some("add-tags") => subcommands::add_tags::go(),
    //     Some("filter") => {
    //         let sub_com =
    //             matches.subcommand_matches("filter").expect("filter panic");
    //         let pat =
    //             sub_com.value_of("pattern").expect("a pattern is expected");
    //         let tags = sub_com.values_of("tags").unwrap().collect();
    //         subcommands::filter::get_results(pat, tags)
    //     }
    //     Some("clean") => {
    //         let sub_com =
    //             matches.subcommand_matches("clean").expect("clean panic");
    //         match sub_com.subcommand_name() {
    //             Some(n) => subcommands::clean::cache_clear(n),
    //             None => Err(GitGlobalError::MissingSubcommand(
    //                 vec!["tags", "all"].into_iter().map(String::from).collect(),
    //             )),
    //         }
    //     }
    //     Some("scan") => subcommands::scan::get_results(),
    //     Some("prompt") => subcommands::prompt::go(),
    //     Some("prompt-cursive") => subcommands::prompt_cursive::go(),
    //     Some("tag") => {
    //         let sub_com =
    //             matches.subcommand_matches("tag").expect("filter panic");
    //         let tag = sub_com.values_of("tag_arg").unwrap().collect();
    //         subcommands::tag::get_results(tag)
    //     }
    //     Some("tag-projects") => {
    //         let pf = get_path_filter(&matches, "tag-projects");
    //         subcommands::tag_projects::go(pf)
    //     }
    //     Some("status") => get_sync_status(matches),
    //     // Some("new-status") => get_new_status(matches),
    //     Some("completions") => {
    //         let mut file = std::fs::File::create("_git-global-hal")
    //             .expect("Could not write completions file");
    //         get_clap_app().gen_completions_to(
    //             "git-global-hal",
    //             Shell::Zsh,
    //             &mut file, // &mut io::stdout(),
    //         );
    //         Ok(GitGlobalResult::new(&Vec::new()))
    //     }
    //     // Some(cmd) => Err(GitGlobalError::BadSubcommand(cmd.to_string())),
    //     Some(cmd) => {
    //         if (cmd == "new-status") {
    //             let l = get_new_status(&matches).await.expect("await fail");
    //             show_results(l, use_json);
    //             Err(GitGlobalError::BadSubcommand(cmd.to_string()))
    //         // return Ok(());
    //         } else {
    //             Err(GitGlobalError::BadSubcommand(cmd.to_string()))
    //         }
    //     }
    //     None => get_sync_status(matches),
    // };
    // match results {
    //     Ok(res) => {
    //         show_results(res, use_json);
    //         Ok(())
    //     }
    //     Err(err) => {
    //         show_error(err, use_json);
    //         Err(GitGlobalError::FromIOError(
    //             "something happened...".to_owned(),
    //         ))
    //     }
    // }
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
    println!("values is {}", result);
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
async fn show_results(results: GitGlobalResult, use_json: bool) -> i32 {
    trace!("show_results");
    let r = results;
    // let r: GitGlobalResult = results.await;
    println!("We showed!!");

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
fn show_error(error: GitGlobalError, use_json: bool) -> i32 {
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
