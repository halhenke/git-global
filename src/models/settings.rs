use crate::models::{
    action::Action, repo::Repo, repo::Updatable, repo_tag::RepoTag,
    result::GitGlobalResult, utils::new_find_repos,
};
use anyhow::Context;
// use anyhow::{Context, Result};
// use config::{Environment, File as CFile, Source, Value};
use colored::*;
use std::{
    collections::hash_map::HashMap,
    fmt::{Display, Error, Formatter},
    result::Result,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsRaw {
    // BEGINNING: String,
    // OUT: i32,
    basedir: Option<String>,
    basedirs: Option<Vec<String>>,
    // ignored_paths: Option<Vec<HashMap<String, String>>>,
    ignored_paths: Option<Vec<String>>,
    ignored_patterns: Option<Vec<String>>,
    path_shortcuts: Option<HashMap<String, String>>,
    // path_shortcuts: HashMap<String, String>,
    ignored_repos: Option<Vec<Repo>>,
    default_repos: Option<Vec<Repo>>,
    default_tags: Option<Vec<RepoTag>>,
    actions: Option<Vec<Action>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    /// NOTE: Basedir shouldnt be defaulted to an empty string as we need at least one place to scan? This might change if we wish to override at CLI
    pub basedir: Option<String>,
    pub basedirs: Vec<String>,
    // pub ignored_paths: Vec<HashMap<String, String>>,
    pub ignored_paths: Vec<String>,
    pub ignored_patterns: Vec<String>,
    pub path_shortcuts: HashMap<String, String>,
    // path_shortcuts: HashMap<String, String>,
    pub ignored_repos: Vec<Repo>,
    pub default_repos: Vec<Repo>,
    pub default_tags: Vec<RepoTag>,
    pub actions: Vec<Action>,
}

impl From<SettingsRaw> for Settings {
    fn from(sr: SettingsRaw) -> Settings {
        Settings {
            basedir: sr.basedir,
            basedirs: sr.basedirs.unwrap_or(vec![]),
            // ignored_paths: sr.ignored_paths.unwrap_or(vec![]),
            ignored_paths: sr.ignored_paths.unwrap_or(vec![]),
            ignored_patterns: sr.ignored_patterns.unwrap_or(vec![]),
            path_shortcuts: sr.path_shortcuts.unwrap_or(HashMap::new()),
            // path_shortcuts: HashMap<String, String>,
            ignored_repos: sr.ignored_repos.unwrap_or(vec![]),
            default_repos: sr.default_repos.unwrap_or(vec![]),
            default_tags: sr.default_tags.unwrap_or(vec![]),
            actions: sr.actions.unwrap_or(vec![]),
        }
    }
}
// use anyhow::Error;

fn write_vec_str<T>(f: &mut Formatter, setting: &str, set_vec: &Vec<T>)
where
    T: ToString,
{
    let tmp = format!("{}:\n", setting).green().underline();
    f.write_fmt(format_args!("{}", tmp));
    // f.write_str(&"basedirs:".green().underline());
    set_vec.iter().for_each(|bd| {
        f.write_fmt(format_args!("\t{}\n", bd.to_string().yellow()))
            .unwrap()
    });
}

impl Display for Settings {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_fmt(format_args!("{}", "Settings:\n".green().underline()));
        f.write_fmt(format_args!(
            "{}:\n\t{}\n",
            "basedir".green().underline(),
            self.basedir
                .as_ref()
                .unwrap_or(&"Nothing Provided".to_owned())
                .yellow()
        ));
        write_vec_str(f, "basedirs", &self.basedirs);
        write_vec_str(f, "ignored_paths", &self.ignored_paths);
        write_vec_str(f, "ignored_patterns", &self.ignored_patterns);
        f.write_fmt(format_args!(
            "{}",
            "path_shortcuts:\n".green().underline()
        ));
        self.path_shortcuts.iter().for_each(|(k, v)| {
            f.write_fmt(format_args!(
                "\t{}\t{}\n",
                k.to_string().yellow().underline(),
                v.to_string().yellow()
            ))
            .unwrap();
        });

        // pub path_shortcuts: HashMap<String, String>,
        // // path_shortcuts: HashMap<String, String>,
        write_vec_str(f, "ignored_repos", &self.ignored_repos);
        write_vec_str(f, "default_repos", &self.default_repos);
        write_vec_str(f, "default_tags", &self.default_tags);
        // pub actions: Vec<Action>,
        Ok(())
    }
}
