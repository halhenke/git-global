//! The `status` subcommand: shows `git status -s` for all known repos.

extern crate colored;
use self::colored::*;
// use std::io::{stderr, Write};
use std::sync::{mpsc, Arc};
use std::thread;

use git2;

use crate::models::errors::Result;
use crate::models::{repo::Repo, result::GitGlobalResult, utils::get_repos};

/// Gathers `git status -s` for all known repos.
pub fn get_results(
    only_modified: bool,
    ignore_untracked: bool,
    path_filter: Option<String>,
) -> Result<GitGlobalResult> {
    trace!("get_results");
    let include_untracked = true;
    // let include_untracked = config.show_untracked;
    let repos = get_repos();
    let n_repos = repos.len();
    let mut result = GitGlobalResult::new(&repos);
    result.pad_repo_output();
    // TODO: limit number of threads, perhaps with mpsc::sync_channel(n)?
    let (tx, rx): (
        std::sync::mpsc::Sender<(String, Vec<String>)>,
        std::sync::mpsc::Receiver<(_, _)>,
    ) = mpsc::channel();
    // let (tx, rx) = mpsc::channel();
    for repo in repos {
        let tx = tx.clone();
        let repo = Arc::new(repo);
        thread::spawn(move || {
            let path = repo.path().to_string();
            let mut status_opts = git2::StatusOptions::new();
            status_opts
                .show(git2::StatusShow::IndexAndWorkdir)
                .include_untracked(include_untracked)
                .include_ignored(false);
            let mut lines = repo.get_status_lines(status_opts);

            if ignore_untracked {
                lines = lines
                    .into_iter()
                    .filter(|l| !l.starts_with("??"))
                    .collect();
            }
            tx.send((path, lines)).unwrap();
        });
    }
    for _ in 0..n_repos {
        let (path, lines) = rx.recv().unwrap();

        if let Some(path_filter) = &path_filter {
            if !path.contains(path_filter) {
                continue;
            }
        }

        let repo = Repo::new(path.to_string());

        let ss = format!(
            "{} {}",
            "Status for".blue(),
            repo.path().green().underline()
        );
        if lines.is_empty() {
            if !only_modified {
                result.add_repo_message(&repo, ss.dimmed().to_string());
            }
        } else {
            result.add_repo_message(&repo, ss.to_string());
        }
        for line in lines {
            result.add_repo_message(&repo, line);
        }
    }
    Ok(result)
}
