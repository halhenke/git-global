//! The `status` subcommand: shows `git status -s` for all known repos.

extern crate colored;
use self::colored::*;
// use std::io::{stderr, Write};
use crossbeam_channel::{bounded, unbounded};
use std::sync::{Arc, Mutex};
use std::thread;

use git2;

use repo::errors::Result;
use repo::{get_repos, GitGlobalError, GitGlobalResult, Repo};

/// Gathers `git status -s` for all known repos.
pub fn get_results(
    only_modified: bool,
    ignore_untracked: bool,
    // path_filter: Option<&str>,
    path_filter: Option<String>,
    // path_filter: Option<&'static str>,
) -> Result<GitGlobalResult> {
    let include_untracked = true;
    // let include_untracked = config.show_untracked;
    let repos = get_repos();
    let n_repos = repos.len();
    let mut result = GitGlobalResult::new(&repos);
    result.pad_repo_output();
    // let mut result =
    //     Arc::new(Mutex::new(GitGlobalResult::new(&repos).pad_repo_output()));

    // let (s, r) = bounded(2);
    let (s, r) = unbounded();

    // TODO: limit number of threads, perhaps with mpsc::sync_channel(n)?
    // let (tx, rx): (
    //     std::sync::mpsc::Sender<(String, Vec<String>)>,
    //     std::sync::mpsc::Receiver<(_, _)>,
    // ) = mpsc::channel();
    // let (tx, rx) = mpsc::channel();

    for repo in repos {
        let s = s.clone();
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
            s.send((path, lines)).unwrap();
        });
    }
    // r.iter().for_each(|(path, lines)| {
    // });
    type ArMuGgr = Arc<Mutex<GitGlobalResult>>;
    let mut resvec: Vec<ArMuGgr> = vec![];
    // let mut resvec: Vec<GitGlobalResult> = vec![];
    let pf = Arc::new(path_filter);
    let result: Arc<Mutex<GitGlobalResult>> = Arc::new(Mutex::new(result));

    for _ in 0..2 {
        // let r = Arc::clone(&r);
        // let pf = Arc::clone(&pf);
        // let result = Arc::clone(&result);
        let r = r.clone();
        let pf = pf.clone();
        let result = result.clone();

        // let pf: &'static str = path_filter.unwrap().clone();
        // println!("i is {}", i);
        let j = thread::spawn(move || {
            for _ in 0..(n_repos / 2) {
                let out = r.recv().unwrap();
                let (path, lines): (String, Vec<String>) = out;
                // let (path: String, lines: Vec<String>) = r.recv().unwrap();

                // debug!("We are in {}\n", path);

                if let Some(pf) = &(*pf) {
                    if !path.contains(pf) {
                        continue;
                        // return;
                    }
                }
                // let mut result = result.lock().unwrap();
                let mut result = result.lock().unwrap();
                // let result: &mut GitGlobalResult =
                //     Arc::get_mut(&mut result).unwrap();
                // let pre_res = result.get_mut();
                // let result: &mut GitGlobalResult = (result.get_mut()).unwrap();

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
        // let ac: Arc<Mutex<GitGlobalResult>> =
        //     j.join().expect("Arc unwrap failure!");
        let ac: Arc<Mutex<GitGlobalResult>> =
            j.join().expect("Arc unwrap failure!");
        // let acb: GitGlobalResult = Arc::try_unwrap(ac)
        //     .expect("preCommand failed")
        //     .into_inner()
        //     .expect("Mutex unwrap failure!");
        // resvec.push(ac);
        // resvec.push(acb);
        // resvec.push(j.join().unwrap().into_inner().unwrap().clone());
        // vec![]
    }

    // let ac: Arc<Mutex<GitGlobalResult>> =
    //     j.join().expect("Arc unwrap failure!");
    // let acb: GitGlobalResult = Arc::try_unwrap(resvec.remove(0))
    //     .expect("preCommand failed")
    //     .into_inner()
    //     .expect("Mutex unwrap failure!");

    // Err(GitGlobalError::BadSubcommand("whoops".to_string()))
    Ok(Arc::try_unwrap(result)
        .expect("preCommand failed")
        .into_inner()
        .expect("Mutex unwrap failure!"))
    // Ok(resvec.remove(0))
    // Ok((*result).into_inner().unwrap())
}
