use crate::models::{
    action::{Action, ActionError},
    repo::Repo,
};
use std::vec::Vec;

type Danger = String;
type Warn = String;

/// A struct that tracks global configuration related to Actions
/// - particularly which might be considered dangerous or worthy of a prompt for the user before doing them - e.g. `rm`, `mv etc
struct ActionSet {
    actions: Vec<Action>,
    danger_list: Vec<Danger>,
    warn_list: Vec<Warn>,
}

impl ActionSet {
    /// Check if a particular action is "dangerous"
    /// atm this is a pretty dumb string match
    pub fn is_dangerous(&self, action: Action) -> bool {
        self.danger_list
            .iter()
            .any(|d| action.get_command().contains(d))
    }
}
