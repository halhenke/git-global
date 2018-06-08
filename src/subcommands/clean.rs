//! The `clear` subcommand: clears the cache

use core::{GitGlobalResult, GitGlobalConfig, get_repos};
use errors::Result;

/// Forces the display of each repo path, without any extra output.
pub fn cache_clear() -> Result<GitGlobalResult> {
// pub fn cache_clear() -> Result<()> {
    let config = GitGlobalConfig::new();
    config.destroy_cache()?;
    println!("Cache destroyed");
    Ok(GitGlobalResult::new(&vec![]))
}
