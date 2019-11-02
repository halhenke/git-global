use repo::Repo;
use subprocess::{Exec, Popen};

/// Trying out a nested/weird enum to see how felxible they are
enum ActionType {
    RepoAction {
        GitAction: String,
        NotGitAction: String,
        // Other(String)
    },
    Gumball(
        String,
        i32,
        // Box::<enum Adder>
        // Noon(String)
    ),
    // Nonsense {
    //     obj: String
    // }),
    // GumboAction {
    //     Nando: Thwack(i32)
    // }
    GeneralAction,
}

// enum RepoActionType {
//     GitAction,
//     NotGitAction,
// }

// enum ActionType {
//     RepoAction(RepoActionType),
//     GlobalAction,
// }

type RepoPath = String;
type CommandName = String;
type Command = String;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RepoAction {
    GitAction(RepoPath, CommandName, Command, Vec<String>),
    NonGitAction(CommandName, Command, Vec<String>),
}

pub enum RepoActionError {
    ActionFailed,
}

type ActionResult<T> = Result<T, RepoActionError>;

impl RepoAction {
    pub fn perform_action_for_repo(&self) -> ActionResult<(String)> {
        match self {
            RepoAction::GitAction(path, name, cmd, args) => {
                // let r = {
                //     Exec::shell(format!(
                //         "cd {:?}",
                //         path // (self as RepoAction::GitAction).0
                //     )) | Exec::shell("pwd")
                // }
                let r =
                    Exec::shell(cmd).args(args).cwd(path).capture().unwrap().stdout_str();
                Ok(r)

                // Ok("Yo")
            }
            RepoAction::NonGitAction(name, cmd, args) => {
                let r = Exec::shell(cmd).args(args).capture().unwrap().stdout_str();
                Ok(r)
                // Ok("No".to_owned())
            }
            // GitAction => println!("Yo"),
            // NonGitAction => println!("No"),
        }
        // return format!("Performing action {} for Repo {}", self.0, repo.path);
        // // println!("Performing action {} for Repo :{}", self.0, repo.path);
    }
}

#[cfg(test)]
mod repoaction_tests {
    use super::*;

    #[test]
    pub fn enum_rep() {
        let ga = RepoAction::GitAction(
            "/usr/local".to_owned(),
            "nob".to_owned(),
            "pwd".to_owned(),
            vec![],
        );
        ga.perform_action_for_repo();
        if let RepoAction::GitAction(path, _, _, _) = ga {
            assert_eq!(format!("{:?}", path), "");
        }
    }
}

// impl RepoAction::GitAction {
//     pub fn perform_action_for_repo(&self) {

//     }
// }

/// Represents an Action - intended to be used in a repo
/// - possibly should be split into repo/path independent
/// actions and ones that take place in a specific Repo
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Action(String, Vec<String>);

impl Action {
    pub fn perform_action_for_repo(&self, repo: Repo) -> String {
        return format!("Performing action {} for Repo {}", self.0, repo.path);
        // println!("Performing action {} for Repo :{}", self.0, repo.path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_do() {
        let action = Action("brew".to_owned(), vec!["update".to_owned()]);
        let repo = Repo::new("~/code".to_owned());
        let res = action.perform_action_for_repo(repo);
        assert_eq!(res, "Performing action brew for Repo ~/code");
    }
}
