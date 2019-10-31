use git2;
use std::fmt;
// use std::iter::FromIterator;
use repo::repo_tag::RepoTag;
use std::path::Path;

/// A git repository, represented by the full path to its base directory.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Repo {
    pub path: String,
    pub tags: Vec<RepoTag>,
}

impl Repo {
    pub fn new(path: String) -> Repo {
        Repo {
            path: path,
            tags: vec![],
        }
    }

    /// Returns the full path to the repo as a `String`.
    pub fn path(&self) -> &str {
        self.path.as_str()
    }

    /// Returns the name of the repo as a `String`.
    pub fn name(&self) -> &str {
        Path::new(&self.path).file_name().unwrap().to_str().unwrap()
        // Path::new(self.path).file_stem()
    }

    /// Returns the `git2::Repository` equivalent of this repo.
    pub fn as_git2_repo(&self) -> git2::Repository {
        git2::Repository::open(&self.path).ok().expect(
            "Could not open {} as a git repo. Perhaps you should run \
             `git global scan` again.",
        )
    }

    /// Returns "short format" status output.
    pub fn get_status_lines(
        &self,
        mut status_opts: git2::StatusOptions,
    ) -> Vec<String> {
        let git2_repo = self.as_git2_repo();
        let statuses = git2_repo
            .statuses(Some(&mut status_opts))
            .expect(&format!("Could not get statuses for {}.", self));
        statuses
            .iter()
            .map(|entry| {
                let path = entry.path().unwrap();
                let status = entry.status();
                let status_for_path = self.get_short_format_status(status);
                format!("{} {}", status_for_path, path)
            })
            .collect()
    }

    /// Translates a file's status flags to their "short format" representation.
    ///
    /// Follows an example in the git2-rs crate's `examples/status.rs`.
    fn get_short_format_status(&self, status: git2::Status) -> String {
        let mut istatus = match status {
            s if s.is_index_new() => 'A',
            s if s.is_index_modified() => 'M',
            s if s.is_index_deleted() => 'D',
            s if s.is_index_renamed() => 'R',
            s if s.is_index_typechange() => 'T',
            _ => ' ',
        };
        let mut wstatus = match status {
            s if s.is_wt_new() => {
                if istatus == ' ' {
                    istatus = '?';
                }
                '?'
            }
            s if s.is_wt_modified() => 'M',
            s if s.is_wt_deleted() => 'D',
            s if s.is_wt_renamed() => 'R',
            s if s.is_wt_typechange() => 'T',
            _ => ' ',
        };
        if status.is_ignored() {
            istatus = '!';
            wstatus = '!';
        }
        if status.is_conflicted() {
            istatus = 'C';
            wstatus = 'C';
        }
        // TODO: handle submodule statuses?
        format!("{}{}", istatus, wstatus)
    }

    // =====================================================
    //  TAGS
    // =====================================================

    pub fn tag(&mut self, tag: &str) -> () {
        self.tags.push(RepoTag::new(tag));
    }

    pub fn has_tag(&mut self, tag: &str) -> bool {
        self.tags.iter().any(|t| t.name == tag)
    }

    pub fn untag(&mut self, tag: &str) -> () {
        let id_match = self.tags.iter().position(|x| x.name == tag);
        if let Some(id) = id_match {
            self.tags.remove(id);
        }
    }

    pub fn get_tags(&self) -> Vec<String> {
        return self
            .tags
            .clone()
            .into_iter()
            .map(|x| String::from(x))
            .collect();
    }
}
// type VecRep = Vec<Repo>;

/// All tags from a Vec of repos
pub fn all_tags<'a>(reps: &Vec<Repo>) -> Vec<RepoTag> {
    let mut v: Vec<RepoTag> = Vec::new();
    // reps.iter()
    // .map(|x| &x.tags)
    // .collect()
    // let mut vi = v.ter();
    for r in reps {
        // (&vi).chain(r.tags.iter());
        // &v.append(r.tags.iter())
        let mut t: Vec<RepoTag> = r.tags.clone();
        &v.append(&mut t);
    }
    // return v.iter().flatten().collect();
    // return vi.collect();
    return v;
}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}
