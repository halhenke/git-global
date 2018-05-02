//! The command line interface for git-global.

use std::io::{Write, stderr};

use clap::{Arg, App, SubCommand, Values};

use core::GitGlobalResult;
use errors::GitGlobalError;
use subcommands;

/// Returns the definitive clap::App instance for git-global.
fn get_clap_app<'a, 'b>() -> App<'a, 'b> {
    App::new("git-global")
        .version(crate_version!())
        .author("Eric Petersen <eric@ericpetersen.io>")
        .about("git subcommand for working with all git repos on a machine")
        .arg(Arg::with_name("json")
            .long("json")
            .help("Output results in JSON."))
        .subcommand(SubCommand::with_name("info")
            .about("show meta-information about git-global"))
        .subcommand(SubCommand::with_name("list")
            .about("lists all git repos on your machine [the default]"))
        .subcommand(SubCommand::with_name("tag")
            .about("tag a single git repo")
                .arg(Arg::with_name("tag_arg")
                .required(true)
                .takes_value(true)))
        .subcommand(SubCommand::with_name("filter")
            .about("lists all git repos on your machine filtered by a pattern")
            .arg(Arg::with_name("pattern")
                .required(true))
            .arg(Arg::with_name("tags")
                .short("t")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("scan")
            .about("update cache of git repos on your machine"))
        .subcommand(SubCommand::with_name("status")
            .about("shows status of all git repos"))
}

/// Runs the appropriate git-global subcommand based on command line arguments.
///
/// As the effective binary entry point for `git-global`, prints results to
/// `STDOUT` and returns an exit code.
pub fn run_from_command_line() -> i32 {
    let clap_app = get_clap_app();
    let matches = clap_app.get_matches();
    let use_json = matches.is_present("json");
    let results = match matches.subcommand_name() {
        Some("info") => subcommands::info::get_results(),
        Some("list") => subcommands::list::get_results(),
        Some("filter") => {
            let sub_com = matches
                .subcommand_matches("filter").expect("filter panic");
            let pat = sub_com
                .value_of("pattern")
                .expect("a pattern is expected");
            /// To do this iterator stuff we need to have
            ///  - the iterator be mutable because `by_ref` takes a mutable self
            ///  - use `by_ref`
            ///  - use reference/borrow
            // let mut t1 = sub_com.values_of("tags").unwrap();
            // let t2: &Vec<&str> = &t1
            //     .by_ref()
            //     .flat_map(|x| x.split(","))
            //     .collect();
            // let t3: &Vec<&str> = &t1
            //     .by_ref()
            //     .flat_map(|x| x.split(","))
            //     .collect();

            let tags = sub_com.values_of("tags").unwrap().collect();
            subcommands::filter::get_results(pat, tags)
        },
        Some("scan") => subcommands::scan::get_results(),
        Some("tag") => {
            let sub_com = matches
                .subcommand_matches("tag")
                .expect("filter panic");
            let tag = sub_com
                .values_of("tag_arg")
                .unwrap()
                .collect();
            subcommands::tag::get_results(tag)
        },
        Some("status") => subcommands::status::get_results(),
        Some(cmd) => Err(GitGlobalError::BadSubcommand(cmd.to_string())),
        None => subcommands::status::get_results(),
    };
    match results {
        Ok(res) => show_results(res, use_json),
        Err(err) => show_error(err, use_json),
    }
}

/// Writes results to STDOUT, as either text or JSON, and returns `0`.
fn show_results(results: GitGlobalResult, use_json: bool) -> i32 {
    if use_json {
        results.print_json();
    } else {
        results.print();
    }
    0
}

/// Writes errors to STDERR, as either text or JSON, and returns `1`.
fn show_error(error: GitGlobalError, use_json: bool) -> i32 {
    if use_json {
        let json = object!{
            "error" => true,
            "message" => format!("{}", error)
        };
        writeln!(&mut stderr(), "{:#}", json).expect("failed write to STDERR");
    } else {
        writeln!(&mut stderr(), "{}", error).expect("failed write to STDERR");
    }
    1
}
