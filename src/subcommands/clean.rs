//! The `clear` subcommand: clears the cache

use repo::errors::Result;
use repo::{get_repos, GitGlobalConfig, GitGlobalResult};

/// Forces the display of each repo path, without any extra output.
pub fn cache_clear() -> Result<GitGlobalResult> {
    // pub fn cache_clear() -> Result<()> {
    let config = GitGlobalConfig::new();
    config.destroy_cache()?;
    println!("Cache destroyed");
    Ok(GitGlobalResult::new(&vec![]))
}
