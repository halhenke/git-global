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

/// Returns the definitive clap::App instance for git-global.
pub fn get_clap_app<'a, 'b>() -> App<'a, 'b> {
    trace!("get_clap_app");
    App::new("git-global")
        .version(crate_version!())
        .author("Eric Petersen <eric@ericpetersen.io>")
        .about("git subcommand for working with all git repos on a machine")
        // .arg(
        //     Arg::with_name("generate-zsh-completions")
        //         .long("zsh")
        //         .help("generate zsh completions for this command"),
        // )
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
            SubCommand::with_name("actions")
                .about("commands related to actions that can be performed in specific repositories")
                .subcommand(
                    SubCommand::with_name("perform")
                        .about("perform one (or more?) actions for repositories filtered by tag/path")
                        .arg(
                            Arg::with_name("actions")
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
        .subcommand(
            SubCommand::with_name("cache")
                .about("cache related commands")
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
                    )
                )
                .subcommand(
                    SubCommand::with_name("print-cache").about("prints the location of the cache file - not contents")
                )
        )
        .subcommand(
            SubCommand::with_name("config")
                .about("config related commands")
                .subcommand(
                    SubCommand::with_name("print")
                        .about("print the location and value of the settings file")
                )
        )
        .subcommand(
            SubCommand::with_name("prompt")
                .about("demo various interactive prompts")
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
        )
        .subcommand(
            SubCommand::with_name("repos")
                .about("repo related commands")
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
                    SubCommand::with_name("select")
                    .about("interactively select repos so as to perform some action on them")
                    .arg(
                        Arg::with_name("path_filter")
                            .short("p")
                            .long("paths")
                            .takes_value(true)
                            .required(false),
                    ),
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
        )
        .subcommand(
            SubCommand::with_name("tags")
                .about("tag related commands")
                .subcommand(
                SubCommand::with_name("list")
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
                    SubCommand::with_name("add")
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
        )
        .subcommand(
            SubCommand::with_name("status")
                .about("old and new status command implementations")
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
                        )
                )
        )
        .subcommand(
            SubCommand::with_name("completions")
            .about("outputs zsh specific completion commands and writes them to a file - `_git-global-hal` that can then be copied across to your fpath"))
}
