/*!
    Defines the [`GitGlobalResult`] struct
    At the moment this data structure contains
        - a list of Repos
        - a list of Global Messages
        - a list of messages per [`Repo`]
    Its designed to be the result of any particular command - not sure it fits anymore - or at least we might want a different data structure for other stuff
*/
use crate::models::{repo::Repo, repo_tag::RepoTag};
use std::collections::HashMap;

/// The result of a git-global subcommand.
///
/// Contains overall messages, per-repo messages, and a list of repos.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GitGlobalResult {
    messages: Vec<String>,
    pub repos: Vec<Repo>,
    repo_messages: HashMap<Repo, Vec<String>>,
    flag_pad_repo_output: bool,
}

impl GitGlobalResult {
    pub fn new(repos: &Vec<Repo>) -> GitGlobalResult {
        let mut repo_messages: HashMap<Repo, Vec<String>> = HashMap::new();
        for repo in repos {
            repo_messages.insert(repo.clone(), Vec::new());
        }
        GitGlobalResult {
            messages: Vec::new(),
            repos: repos.clone(),
            repo_messages: repo_messages,
            flag_pad_repo_output: false,
        }
    }

    pub fn blank() -> GitGlobalResult {
        GitGlobalResult {
            messages: Vec::new(),
            repos: Vec::new(),
            repo_messages: HashMap::new(),
            flag_pad_repo_output: false,
        }
    }

    /// Declares desire to separate output when showing per-repo messages.
    ///
    /// Sets flag that indicates a blank line should be inserted between
    /// messages for each repo when showing results output.
    pub fn pad_repo_output(&mut self) {
        self.flag_pad_repo_output = true;
    }

    /// Adds a message that applies to the overall operation.
    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
    }

    /// Adds a message that applies to a particular repo.
    pub fn add_repo_message(&mut self, repo: &Repo, data_line: String) {
        match self.repo_messages.get_mut(&repo) {
            Some(item) => item.push(data_line),
            None => (),
        }
    }

    /// Writes all result messages to STDOUT, as text.
    pub fn print(&self) {
        for msg in self.messages.iter() {
            println!("{}", msg);
        }
        for repo in self.repos.iter() {
            let messages = self.repo_messages.get(&repo).unwrap();
            if messages.len() > 0 {
                println!("{}", repo);
                for line in messages.iter().filter(|l| *l != "") {
                    println!("{}", line);
                }
                if self.flag_pad_repo_output {
                    println!();
                }
            }
        }
    }

    /// Writes all result messages to STDOUT, as JSON.
    pub fn print_json(&self) {
        let mut json = object! {
            "error" => false,
            "messages" => array![],
            "repo_messages" => object!{}
        };
        for msg in self.messages.iter() {
            json["results"]["messages"]
                .push(msg.to_string())
                .expect("Failing pushing message to JSON messages array.");
        }
        for (repo, messages) in self.repo_messages.iter() {
            json["repo_messages"][&repo.path] = array![];
            if messages.len() > 0 {
                for line in messages.iter().filter(|l| *l != "") {
                    json["repo_messages"][&repo.path]
                        .push(line.to_string())
                        .expect(
                            "Failed pushing line to JSON repo-messages array.",
                        );
                }
            }
        }
        println!("{:#}", json);
    }

    /// prob a bad idea but want to be able to merge results for purposes of threads and results
    pub fn merge_ggr(&mut self, mut ggr: GitGlobalResult) {
        // GitGlobalResult::new(repos: &Vec<Repo>)
        self.messages.append(&mut ggr.messages);
        self.repos.append(&mut ggr.repos);
        self.repo_messages.extend(ggr.repo_messages);
        // *self
        // GitGlobalResult {
        //     messages: self.messages,
        //     repos: self.repos,
        //     repo_messages: self.repo_messages,
        //     flag_pad_repo_output: false,
        // }
    }

    /**
               When we need to get all the tags from the current tags
    */
    pub fn all_tags(&self) -> Vec<&RepoTag> {
        // self.repos
        //     .iter()
        //     .map(|r| &r.tags)
        //     .for_each(|p| {
        //         println!("{:?}", p);
        //     });
        self.repos
            // .into_iter()
            // .map(|r| r.tags.as_ref())
            // .flatten()
            // .collect::<Vec<&RepoTag>>()
            .iter()
            .map(|r| &r.tags)
            .flatten()
            .collect::<Vec<&RepoTag>>()
    }

    /// Return the list of repos filtered by those that include one of a set of tags
    pub fn filter_repos_by_tags(&self, tags: Vec<RepoTag>) -> Vec<Repo> {
        self.repos
            .clone()
            .into_iter()
            .filter(|r| r.tags.iter().any(|rt| tags.contains(rt)))
            .collect()
    }
}
