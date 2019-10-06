//! The command line interface for git-global.

use clap::{App, Arg, Shell, SubCommand};
use std::io;
use std::io::{stderr, Write};

use repo::errors;
use repo::GitGlobalError;
use repo::GitGlobalResult;
use subcommands;

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
        .subcommand(SubCommand::with_name("clean").about("Clear the cache"))
        .subcommand(
            SubCommand::with_name("prompt")
                .about("demo the TUI Terminal UI library"),
        )
        .subcommand(
            SubCommand::with_name("prompt-cursive")
                .about("demo the Cursive Terminal UI library"),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("lists all git repos on your machine [the default]"),
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
                .about("edit the association between tags and projects"),
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
            SubCommand::with_name("scan")
                .about("update cache of git repos on your machine"),
        )
        .subcommand(
            SubCommand::with_name("status")
                .about("shows status of all git repos")
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
        .subcommand(SubCommand::with_name("completions"))
}

/// Runs the appropriate git-global subcommand based on command line arguments.
///
/// As the effective binary entry point for `git-global`, prints results to
/// `STDOUT` and returns an exit code.
pub fn run_from_command_line() -> i32 {
    let clap_app = get_clap_app();
    let matches = clap_app.get_matches();
    let use_json = matches.is_present("json");

    // if matches.is_present("generate-zsh-completions") {
    //     get_clap_app().gen_completions_to(
    //         "_git-global-hal",
    //         Shell::Zsh,
    //         &mut io::stdout(),
    //     );
    //     return 0;
    // }

    let results = match matches.subcommand_name() {
        Some("bullshit") => subcommands::bullshit::get_results(),
        Some("info") => {
            let raw_info = matches
                .subcommand_matches("info")
                .unwrap()
                .is_present("raw");
            // .value_of("raw");
            // .expect("raw panic");
            subcommands::info::get_results(raw_info)
        }
        Some("list") => subcommands::list::get_results(),
        Some("list-tags") => subcommands::list_tags::get_results(),
        Some("add-tags") => subcommands::add_tags::go(),
        Some("filter") => {
            let sub_com =
                matches.subcommand_matches("filter").expect("filter panic");
            let pat =
                sub_com.value_of("pattern").expect("a pattern is expected");
            let tags = sub_com.values_of("tags").unwrap().collect();
            subcommands::filter::get_results(pat, tags)
        }
        Some("clean") => subcommands::clean::cache_clear(),
        Some("scan") => subcommands::scan::get_results(),
        Some("prompt") => subcommands::prompt::go(),
        Some("prompt-cursive") => subcommands::prompt_cursive::go(),
        Some("tag") => {
            let sub_com =
                matches.subcommand_matches("tag").expect("filter panic");
            let tag = sub_com.values_of("tag_arg").unwrap().collect();
            subcommands::tag::get_results(tag)
        }
        // Some("tag-projects") => subcommands::tag_projects::go(),
        // Some("tag-projects") => subcommands::tag_projects_two::go(),
        Some("tag-projects") => subcommands::tag_projects_nova::go(),
        // Some("tag-projects") => subcommands::tag_projects_simple::go(),
        // Some("tag-projects") => subcommands::tag_projects_lazy::go(),
        Some("status") => get_status(matches),
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
        Some(cmd) => Err(GitGlobalError::BadSubcommand(cmd.to_string())),
        None => get_status(matches),
    };
    match results {
        Ok(res) => show_results(res, use_json),
        Err(err) => show_error(err, use_json),
    }
}

fn get_status(matches: clap::ArgMatches) -> errors::Result<GitGlobalResult> {
    let modified = matches
        .subcommand_matches("status")
        .unwrap()
        .is_present("modified");
    let path_filter = matches
        .subcommand_matches("status")
        .unwrap()
        .value_of("path_filter");
    let ignore_untracked = matches
        .subcommand_matches("status")
        .unwrap()
        .is_present("ignore_untracked");
    subcommands::status::get_results(modified, ignore_untracked, path_filter)
}

/// Writes results to STDOUT, as either text or JSON, and returns `0`.
fn show_results(results: GitGlobalResult, use_json: bool) -> i32 {
    // println!("SHOW RESULTS");
    if use_json {
        // println!("SHOW RESULTS - json");
        results.print_json();
    } else {
        // println!("SHOW RESULTS - no json");
        results.print();
    }
    0
}

/// Writes errors to STDERR, as either text or JSON, and returns `1`.
fn show_error(error: GitGlobalError, use_json: bool) -> i32 {
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
