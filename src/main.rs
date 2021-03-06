#![feature(async_closure)]
#![feature(try_blocks)]
#![feature(backtrace)]
//! Entry point for the binary.

extern crate git_global;
extern crate pretty_env_logger;
// #[macro_use] extern crate log;

use anyhow::Result;
use git_global::cli::interpretation::run_from_command_line;
use std::process::exit;

// extern crate cursive;
// use cursive::logger;

/// Runs git-global from the command line, exiting with its return value.
// fn main() -> Result<(), std::io::Error> {
//     pretty_env_logger::init();
//     // logger::init();
//     println!("I am in rust land\n\n");

//     // let rt = tokio::runtime::Runtime::new().expect("tokio fail");
// let mut rt = tokio::runtime::Builder::new()
//     // .basic_scheduler()
//     .threaded_scheduler()
//     .enable_io()
//     .build()?;
// println!("Runtime is built\n\n");
// rt.block_on(async {
//     tokio::spawn(
//     // let rts = rt.spawn(
//         git_global::run_from_command_line()
//             // .await
//             // .expect("tokio fail"),
//     );
//     println!("Spawn is spawned\n\n");
// });
// Ok(())
// exit(0);
// exit(git_global::run_from_command_line())
// exit(git_global::cli::run_from_command_line())
// }

fn main() -> Result<()> {
    pretty_env_logger::init();
    run_from_command_line()
    // exit(0)
}
