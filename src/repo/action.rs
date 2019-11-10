use repo::Repo;
use std::fmt::{Display, Error};
use subprocess::{Exec, Popen};

/// Trying out a nested/weird enum to see how felxible they are
enum ActionType {
    RepoAction {
        GitAction: String,
        NotGitAction: String,
    },
    Gumball(
        String,
        i32,
        // Box::<enum Adder>
        // Noon(String)
    ),
    // Nonsense {
    //     nested: {
    //         no: bool
    //     }
    // },
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

/// Represents an Action - intended to be used in a repo
/// - possibly should be split into repo/path independent
/// actions and ones that take place in a specific Repo
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Action {
    // GitAction(RepoPath, CommandName, Command, Vec<String>),
    PathAction(RepoPath, CommandName, Command, Vec<String>),
    NeedsAPathAction(CommandName, Command, Vec<String>),
    NonPathAction(CommandName, Command, Vec<String>),
}

#[derive(Clone, Debug)]
pub enum ActionError {
    ActionFailed,
    NeedAPath(String),
    NotANeedAPath(String),
}

type ActionResult<T> = Result<T, ActionError>;

impl Display for Action {
    // fn fmt(&self, &mut std::fmt::Formatter<'_>) -> String {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), Error> {
        // return String::from("Booyah");
        // *f = String::from("Booyah");
        // f.write_str(self.);
        match self {
            // Action::GitAction(path, name, _, _) => {},
            Action::PathAction(path, name, _, _) => {
                f.write_str(&format!("Action `{}` for path {}", name, path));
            }
            Action::NeedsAPathAction(name, _, _) => {
                f.write_str(&format!(
                    "Action `{}` needs to be associated with a path before execution",
                    name
                ));
            }
            Action::NonPathAction(name, _, _) => {
                f.write_str(&format!(
                    "Action `{}` not associated with a path",
                    name
                ));
            }
        }
        Ok(())
    }
}

impl Action {
    /// Check if
    /// - this is a [`NeedsAPathAction`] variant
    /// - the command name matches
    /// If yes, bind the path and return
    /// Method calls [`name_match`] and [`path_bind`]
    pub fn check_needs_path_match_name(
        &self,
        action: &str,
        path: String,
    ) -> Option<Self> {
        if self.name_match(action).is_none() {
            return None;
        }
        // if let Ok(bound_action) = self.path_bind(path) {
        //     return Some(bound_action);
        // }
        match self.path_bind(path) {
            Ok(bound_action) => Some(bound_action),
            Err(_) => None,
        }
    }

    /// return Action name
    pub fn get_name(&self) -> &str {
        match self {
            Action::PathAction(_, name, ..) => name,
            Action::NeedsAPathAction(name, ..)
            | Action::NonPathAction(name, ..) => name,
        }
    }

    /// return Action command
    pub fn get_command(&self) -> &str {
        match self {
            Action::PathAction(_, _, cmd, ..) => cmd,
            Action::NeedsAPathAction(_, cmd, ..)
            | Action::NonPathAction(_, cmd, ..) => cmd,
        }
    }

    /// Indicates whether the action needs to be provided with a path in which to be executed - i.e. is it a [`Action::NeedsAPathAction`]
    pub fn needs_path(&self) -> bool {
        match self {
            Action::NeedsAPathAction(..) => true,
            _ => false,
        }
    }

    /// Check if a given string matches the name of the action
    pub fn name_match(&self, act: &str) -> Option<Self> {
        match self {
            Action::PathAction(path, name, cmd, args) => {
                if name == act {
                    return Some(self.clone());
                }
            }
            Action::NeedsAPathAction(name, _, _) => {
                if name == &act {
                    return Some(self.clone());
                }
            }
            Action::NonPathAction(name, _, _) => {
                if name == &act {
                    return Some(self.clone());
                }
            }
        }
        return None;
        // return Some(self.clone());
    }

    /// Turn a [`NeedsAPathAction`] variant into a [`PathAction`]
    pub fn path_bind(&self, path: String) -> Result<Self, ActionError> {
        match self {
            Action::NeedsAPathAction(name, cmd, args) => {
                Ok(Action::PathAction(
                    path,
                    name.to_owned(),
                    cmd.to_owned(),
                    args.to_owned(),
                ))
            }
            // _ => Err(ActionError::NotANeedAPath(format!(
            // Action::NonPathAction(name, cmd, args) => {
            //     Err(ActionError::NotANeedAPath(format!(
            //         "Action {} is not a Action::NeedsAPathAction",
            //         "self.name"
            //     )))
            // }
            Action::NonPathAction(name, cmd, args)
            | Action::PathAction(_, name, cmd, args) => {
                Err(ActionError::NotANeedAPath(format!(
                    "Action {} is not a Action::NeedsAPathAction",
                    name
                )))
            }
        }
    }

    /// If we have a [`Action::PathAction`] or a [`Action::NonPathAction`] perform the action and return output as a string.
    /// If we have a [`Action::NeedsAPathAction`] then return an Error
    pub fn perform_action_for_repo(&self) -> ActionResult<(String)> {
        match self {
            Action::PathAction(path, name, cmd, args) => {
                let r = Exec::shell(cmd)
                    .args(args)
                    .cwd(path)
                    .capture()
                    .unwrap()
                    .stdout_str();
                // println!("Here is some r: {}", r);
                Ok(r)
            }
            Action::NonPathAction(name, cmd, args) => {
                let r =
                    Exec::shell(cmd).args(args).capture().unwrap().stdout_str();
                Ok(r)
            }
            Action::NeedsAPathAction(name, _, _) => {
                Err(ActionError::NeedAPath(
                    format!(
                        "The command {} needs a path to be performed in",
                        name
                    )
                    .to_owned(),
                ))
            } // Action::NonGitAction(name, cmd, args) => {
              //     let r = Exec::shell(cmd).args(args).capture().unwrap().stdout_str();
              //     Ok(r)
              //     // Ok("No".to_owned())
              // }
        }
        // return format!("Performing action {} for Repo {}", self.0, repo.path);
        // // println!("Performing action {} for Repo :{}", self.0, repo.path);
    }
}

#[cfg(test)]
mod action_tests {
    use super::*;

    #[test]
    pub fn enum_rep() {
        let ga = Action::PathAction(
            // let ga = Action::GitAction(
            "/usr/local".to_owned(),
            "nob".to_owned(),
            "pwd".to_owned(),
            vec![],
        );
        ga.perform_action_for_repo();
        if let Action::PathAction(path, _, _, _) = ga {
            // if let Action::GitAction(path, _, _, _) = ga {
            assert_eq!(format!("{:?}", path), "");
        }
    }
}

// impl RepoAction::GitAction {
//     pub fn perform_action_for_repo(&self) {

//     }
// }

// /// Represents an Action - intended to be used in a repo
// /// - possibly should be split into repo/path independent
// /// actions and ones that take place in a specific Repo
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct Action(String, Vec<String>);

// impl Action {
//     pub fn perform_action_for_repo(&self, repo: Repo) -> String {
//         return format!("Performing action {} for Repo {}", self.0, repo.path);
//         // println!("Performing action {} for Repo :{}", self.0, repo.path);
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn action_do() {
//         let action = Action("brew".to_owned(), vec!["update".to_owned()]);
//         let repo = Repo::new("~/code".to_owned());
//         let res = action.perform_action_for_repo(repo);
//         assert_eq!(res, "Performing action brew for Repo ~/code");
//     }
// }
