#![feature(vec_remove_item)]
#![feature(async_closure)]
#![feature(termination_trait_lib)]
#![feature(test)]
#![feature(const_fn)]
// #![feature(custom_attribute)]

//! Keep track of all your git repositories.
//!
//! This crate houses the binary and library for the git-global subcommand, a
//! way to find, query statuses, and gain other insights about all the git repos
//! on your machine. The binary can be installed with cargo: `cargo install
//! git-global`.
//!
//! # Command-line Usage
//!
//! ```bash
//! $ git global [status]  # show `git status -s` for all your git repos
//! $ git global info      # show information about git-global itself
//! $ git global list      # show all git repos git-global knows about
//! $ git global scan      # search your filesystem for git repos and update cache
//! ```
//!
//! # Public Interface
//!
//! The [`Repo`] struct is a git repository that is identified by the full path
//! to its base directory (i.e., not its `.git` directory).
//!
//! The [`GitGlobalConfig`] struct holds a user's git-global configuration
//! information, which merges some default values with values in the `[global]`
//! section of the user's global `.gitconfig` file.
//!
//! A [`GitGlobalResult`] result contains messages added by a subcommand, either
//! about the overall process or about a specific repo, as well as a list of
//! repos. All subcommands expose a `get_results()` function that returns a
//! `GitGlobalResult`.
//!
//! The [`get_repos()`] function returns the list of known repos, performing a
//! scan if necessary.
//!
//! All git-global subcommands are implemented in the [`subcommands`] module.
//!
//! Most mportant Data Structures are defined in the [`repo`](repo/index.html) module
//!

//! [`GitGlobalConfig`]: repo/config/struct.GitGlobalConfig.html
//! [`GitGlobalResult`]: repo/result/struct.GitGlobalResult.html
//! [`Repo`]: repo/repo/struct.Repo.html
//! [`LightTable`]: repo/light_table/struct.LightTable.html
//! [`get_repos()`]: fn.get_repos.html
//! [`subcommands`]: subcommands/index.html
//! [`repo`]: repo/index.html
//! [`cli`]: repo/index.html
//! [`queries`]: queries/index.html

// Unstable feature (test)
extern crate test;

extern crate app_dirs;
extern crate chrono;
extern crate colored;
extern crate config;
extern crate dirs;
extern crate git2;
extern crate itertools;
extern crate mut_static;
extern crate pipeline;
extern crate serde_json;
extern crate tui;
extern crate walkdir;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate json;
#[macro_use]
extern crate serde_derive;
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate icecream;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
extern crate precision;
#[cfg(test)]
extern crate rand;

// Have to use macros before other stuff
#[macro_use]
pub mod macros;

extern crate take_mut;

extern crate crossbeam_channel;
extern crate cursive;
extern crate jwalk;
#[macro_use]
extern crate ring_queue;
extern crate anyhow;
extern crate subprocess;
#[macro_use]
extern crate thiserror;

pub mod cli;
pub mod models;
pub mod subcommands;
