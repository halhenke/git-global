#![feature(async_closure)]
//! Entry point for the binary.

extern crate git_global;
extern crate pretty_env_logger;
// #[macro_use] extern crate log;

use std::process::exit;
use tokio;
// extern crate cursive;
// use cursive::logger;

/// Runs git-global from the command line, exiting with its return value.
// fn main() {
fn main() -> Result<(), std::io::Error> {
    pretty_env_logger::init();
    // logger::init();
    let rt = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_io()
        .build()?;
    rt.spawn(
        git_global::run_from_command_line()
            // .await
            // .expect("tokio fail"),
    );
    // rt.enter(async move|| git_global::run_from_command_line().await.expect("tokio fail"));
    Ok(())
    // exit(0);
    // exit(git_global::run_from_command_line())
    // exit(git_global::cli::run_from_command_line())
}
