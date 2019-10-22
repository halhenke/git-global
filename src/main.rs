//! Entry point for the binary.

extern crate git_global;
extern crate pretty_env_logger;
// #[macro_use] extern crate log;

use std::process::exit;
// extern crate cursive;
// use cursive::logger;

/// Runs git-global from the command line, exiting with its return value.
fn main() {
    pretty_env_logger::init();
    // logger::init();
    exit(git_global::run_from_command_line())
    // exit(git_global::cli::run_from_command_line())
}
