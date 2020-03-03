use std::fmt::{Display, Error};
use subprocess::Exec;

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
type Args = Vec<String>;

/// Represents an Action - intended to be used in a repo
/// - possibly should be split into repo/path independent
/// actions and ones that take place in a specific Repo
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Action {
    // GitAction(RepoPath, CommandName, Command, Vec<String>),
    PathAction {
        path: RepoPath,
        cmd: Commander, // name: CommandName,
                        // command: Command,
                        // args: Args
    },
    NeedsAPathAction {
        cmd: Commander, // name: CommandName,
                        // command: Command,
                        // args: Args
    },
    NonPathAction {
        cmd: Commander, // name: CommandName,
                        // command: Command,
                        // args: Args
    }, // PathAction(RepoPath, CommandName, Command, Args),
       // NeedsAPathAction(CommandName, Command, Args),
       // NonPathAction(CommandName, Command, Args)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Commander {
    name: CommandName,
    command: Command,
    args: Args,
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
            // Action::GitAction(path, name, command, _) => {},
            Action::PathAction { path, cmd } => {
                // Action::PathAction(path, name, command, _) => {
                f.write_str(&format!(
                    "Action::PathAction: `{}`\npath: {}\ncommand: {}\nargs: {:#?}",
                    cmd.name, path, cmd.command, cmd.args
                ))
            }
            // Action::NeedsAPathAction(name, command, _) => {
            Action::NeedsAPathAction { cmd } => f.write_str(&format!(
                "Action::NeedsAPathAction: `{}`\ncommand: {}\nargs: {:#?}",
                cmd.name, cmd.command, cmd.args
            )),
            Action::NonPathAction { cmd } => {
                // Action::NonPathAction(name, command, _) => {
                f.write_str(&format!(
                    "Action::NonPathAction: `{}`\ncommand: {}\nargs: {:#?}",
                    cmd.name, cmd.command, cmd.args
                ))
            }
        }
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
            Action::PathAction { cmd, .. }
            | Action::NeedsAPathAction { cmd, .. }
            | Action::NonPathAction { cmd, .. } => &cmd.name,
        }
    }

    /// return Action command
    pub fn get_command(&self) -> &str {
        match self {
            Action::PathAction { cmd, .. }
            | Action::NeedsAPathAction { cmd, .. }
            | Action::NonPathAction { cmd, .. } => &cmd.command,
        }
    }

    /// Indicates whether the action needs to be provided with a path in which to be executed - i.e. is it a [`Action::NeedsAPathAction`]
    pub fn needs_path(&self) -> bool {
        match self {
            Action::NeedsAPathAction { .. } => true,
            _ => false,
        }
    }

    /// Check if a given string matches the name of the action
    pub fn name_match(&self, act: &str) -> Option<Self> {
        match self {
            Action::PathAction { cmd, .. } => {
                if cmd.name == act {
                    return Some(self.clone());
                }
            }
            Action::NeedsAPathAction { cmd, .. } => {
                if cmd.name == act {
                    return Some(self.clone());
                }
            }
            Action::NonPathAction { cmd, .. } => {
                if cmd.name == act {
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
            Action::NeedsAPathAction { cmd } => Ok(Action::PathAction {
                path,
                cmd: cmd.to_owned(),
            }),
            // _ => Err(ActionError::NotANeedAPath(format!(
            // Action::NonPathAction(name, cmd, args) => {
            //     Err(ActionError::NotANeedAPath(format!(
            //         "Action {} is not a Action::NeedsAPathAction",
            //         "self.name"
            //     )))
            // }
            Action::NonPathAction { cmd } | Action::PathAction { cmd, .. } => {
                Err(ActionError::NotANeedAPath(format!(
                    "Action {} is not a Action::NeedsAPathAction",
                    cmd.name
                )))
            }
        }
    }

    /// If we have a [`Action::PathAction`] or a [`Action::NonPathAction`] perform the action and return output as a string.
    /// If we have a [`Action::NeedsAPathAction`] then return an Error
    pub fn perform_action_for_repo(&self) -> ActionResult<String> {
        match self {
            Action::PathAction { path, cmd } => {
                let r = Exec::shell(&cmd.command)
                    .args(&cmd.args)
                    .cwd(path)
                    .capture()
                    .unwrap()
                    .stdout_str();
                // println!("Here is some r: {}", r);
                Ok(r)
            }
            Action::NonPathAction { cmd } => {
                let r = Exec::shell(&cmd.command)
                    .args(&cmd.args)
                    .capture()
                    .unwrap()
                    .stdout_str();
                Ok(r)
            }
            Action::NeedsAPathAction { cmd } => Err(ActionError::NeedAPath(
                format!(
                    "The command {} needs a path to be performed in",
                    cmd.name
                )
                .to_owned(),
            )), // Action::NonGitAction(name, cmd, args) => {
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

    /// Not sure what this is supposed to test anymore
    #[test]
    pub fn enum_rep() {
        let ga = Action::PathAction {
            // let ga = Action::GitAction(
            path: "/usr/local".to_owned(),
            cmd: Commander {
                name: "nob".to_owned(),
                command: "pwd".to_owned(),
                args: vec![],
            },
        };
        ga.perform_action_for_repo();
        if let Action::PathAction { path, .. } = ga {
            // if let Action::GitAction(path, _, _, _) = ga {
            assert_eq!(path, "/usr/local");
            // assert_eq!(format!("{:?}", path), "/usr/local");
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::repo::Repo;
    use toml;

    #[test]
    fn action_do() {
        // NonPathAction(CommandName, Command, Vec<String>)
        let action = Action::NonPathAction {
            cmd: Commander {
                name: "brew".to_owned(),
                command: "brew update".to_owned(),
                args: vec!["update".to_owned()],
            },
        };
        // let action = Action("brew".to_owned(), vec!["update".to_owned()]);
        let repo = Repo::new("~/code".to_owned());
        let res = action.perform_action_for_repo();
        assert_eq!(res.unwrap(), "Performing action brew for Repo ~/code");
    }

    #[test]
    fn action_serialize() {
        let path = "~/code".to_owned();
        let name = "list code".to_owned();
        let command = "ls".to_owned();
        let args = vec!["ls -la".to_owned()];

        let action = Action::PathAction {
            path,
            cmd: Commander {
                name,
                command,
                args,
            },
        };
        let toml = serde_json::to_string(&action).unwrap();
        // let toml = toml::to_string(&action).unwrap();
        println!("{}", toml);
    }
}
