use repo::Repo;

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
