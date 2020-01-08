//! Error handling for git-global.

// use std::error::Error;
use std::fmt;
use std::io;

use thiserror::Error;

/// An error.
#[derive(Error, Debug)]
pub enum GitGlobalError {
    BadSubcommand(String),
    // MissingSubcommand(String),
    MissingSubcommand(Vec<String>),
    FromIOError(String),
    Generic,
    Io {
        #[from]
        source: io::Error,
        // backtrace: Backtrace,
    },
}

/// Our `Result` alias with `GitGlobalError` as the error type.
// pub type Result<T> = result::Result<T, GitGlobalError>;
use anyhow::Result as AHResult;
pub type Result<T> = AHResult<T, GitGlobalError>;

impl fmt::Display for GitGlobalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // use GitGlobalError::*;
        match self {
            GitGlobalError::BadSubcommand(ref cmd) => {
                write!(f, "Unknown subcommand, {}.", cmd)
            }
            GitGlobalError::MissingSubcommand(possibles) => write!(
                f,
                "Was expecting one of the following subcommands: {:?}",
                possibles
            ),
            _generic => write!(f, "An error occured :(."),
        }
    }
}

// impl Error for GitGlobalError {
//     fn description(&self) -> &str {
//         // use GitGlobalError::*;
//         match self {
//             GitGlobalError::BadSubcommand(_) => "unknown subcommand",
//             _generic => "an error occurred :(",
//         }
//     }
// }

// impl From<io::Error> for GitGlobalError {
//     #[allow(unused_variables)]
//     fn from(err: io::Error) -> GitGlobalError {
//         GitGlobalError::FromIOError(format!("{}", err))
//     }
// }
