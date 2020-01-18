//! The `clear` subcommand: clears the cache

use crate::models::errors::Result;
use crate::models::{config::GitGlobalConfig, result::GitGlobalResult};

/// Forces the display of each repo path, without any extra output.
pub fn cache_clear(clear_cmd: &str) -> Result<GitGlobalResult> {
    trace!("cache_clear");
    // pub fn cache_clear(clear_cmd: Option<&str>) -> Result<GitGlobalResult> {
    // match     {}
    if clear_cmd == "remove" {
        let config = GitGlobalConfig::new();
        config.remove_cache_file()?;
        println!("Cache destroyed");
        Ok(GitGlobalResult::new(&vec![]))
    } else if clear_cmd == "all" {
        let config = GitGlobalConfig::new();
        config.clear_cache()?;
        println!("Cache destroyed");
        Ok(GitGlobalResult::new(&vec![]))
    } else if clear_cmd == "tags" {
        let config = GitGlobalConfig::new();
        config.clear_cache()?;
        println!("Tags removed");
        Ok(GitGlobalResult::new(&vec![]))
    } else {
        unreachable!();
        // Err(GitGlobalError::MissingSubcommand(
        //     vec!["all", "tags"].into_iter().map(String::from).collect(),
        // ))
    }
}
