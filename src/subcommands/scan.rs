//! The `scan` subcommand: scans the filesystem for git repos.
//!
//! By default, the user's home directory is walked, but this starting point can
//! be configured in `~/.gitconfig`:
//!
//! ```bash
//! $ git config --global global.basedir /some/path
//! ```
//!
//! The `scan` subcommand caches the list of git repos paths it finds, and can
//! be rerun at any time to refresh the list.

use crate::repo::errors::Result;
use crate::repo::{cache_repos, new_find_repos, GitGlobalResult};

/// Caches the results of `find_repos()` and says how many were found.
pub fn get_results() -> Result<GitGlobalResult> {
    let repos = new_find_repos();
    cache_repos(&repos);
    let mut result = GitGlobalResult::new(&repos);
    result.add_message(format!(
        "Found {} repos. Use `git global list` to show them.",
        repos.len()
    ));
    Ok(result)
}
