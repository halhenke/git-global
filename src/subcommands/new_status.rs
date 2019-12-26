//! The `status` subcommand: shows `git status -s` for all known repos.

extern crate colored;
use self::colored::*;
// use std::io::{stderr, Write};

use crossbeam_channel::{bounded, unbounded};
use std::sync::{Arc, Mutex};
use std::thread;
// NOTE: TOKIO 2019 - Replacing Crossbeam
use tokio;
// use tokio::sync::mpsc;
use tokio::sync::broadcast;

use git2;

use crate::models::errors::Result;
use crate::models::{
    errors::GitGlobalError, repo::Repo, result::GitGlobalResult,
    utils::get_repos,
};

/// Gathers `git status -s` for all known repos.
/// This is a reimplementation of `status` command using `crossbeam
pub async fn get_results(
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

    let (s, mut r) = broadcast::channel(100);
    // let (mut s, mut r) = mpsc::channel(100);
    // let (s, r) = bounded(10);
    // let (s, r) = unbounded();

    // TODO: limit number of threads, perhaps with mpsc::sync_channel(n)?

    // SEND MESSAGES LOOP
    for repo in repos {
        let s = s.clone();
        let repo = Arc::new(repo);
        tokio::spawn(async move {
            // thread::spawn(move || {
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
            debug!("path is {} and lines are {:#?}", path, lines);
            s.send((path.clone(), lines))
                .expect(&format!("Send failed at {}", path));
            // s.send((path, lines)).unwrap();
        });
    }
    type ArMuGgr = Arc<Mutex<GitGlobalResult>>;
    let pf = Arc::new(path_filter);
    let result: Arc<Mutex<GitGlobalResult>> = Arc::new(Mutex::new(result));

    let thread_count = 1;
    debug!(
        "Thread Count is {}, n_repos is {}, and n_repos / thread_count is {}",
        thread_count,
        n_repos,
        n_repos / thread_count
    );

    for _ in 0..thread_count {
        debug!("Once for each THREAD");
        let mut r_loop = s.subscribe();
        // let r = r.clone();
        let pf = pf.clone();
        let result = result.clone();

        // RECEIVE MESSAGES LOOP
        let j = tokio::spawn(async move {
            // let j = thread::spawn(move || {
            for _ in 0..((n_repos) / thread_count) {
                let out = r_loop.recv().await.unwrap();
                // let out = r.recv().unwrap();
                let (path, lines): (String, Vec<String>) = out;

                if let Some(pf) = &(*pf) {
                    if !path.contains(pf) {
                        continue;
                    }
                }
                let mut result = result.lock().unwrap();
                let repo = Repo::new(path.to_string());
                let ss = format!(
                    "{} {}",
                    "Status for".blue(),
                    repo.path().green().underline()
                );
                if lines.is_empty() {
                    if !only_modified {
                        (*result)
                            .add_repo_message(&repo, ss.dimmed().to_string());
                    }
                } else {
                    (*result).add_repo_message(&repo, ss.to_string());
                }
                for line in lines {
                    (*result).add_repo_message(&repo, line);
                }
            }
            return result;
        });
        let ac: Arc<Mutex<GitGlobalResult>> =
            j.await.expect("Arc unwrap failure!");
    }
    Ok(Arc::try_unwrap(result)
        .expect("preCommand failed")
        .into_inner()
        .expect("Mutex unwrap failure!"))
}
