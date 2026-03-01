use std::{io::Write, path::Path};

use anyhow::{Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::config::CURRENT_USERS_CONFIG_FILE_NAME;

#[derive(Debug, Clone, Deserialize, Serialize, Default, JsonSchema)]
pub struct Users(Vec<User>);

impl Users {
    pub fn users(&self) -> Vec<User> {
        self.0.clone()
    }

    pub fn read_applied_config(data_path: &Path) -> Option<Self> {
        let data = std::fs::read(data_path.join(CURRENT_USERS_CONFIG_FILE_NAME)).ok()?;
        let applied_config_data = bitcode::deserialize(&data).ok()?;
        Some(applied_config_data)
    }

    pub fn save_applied_config(self, data_path: &Path) -> Result<()> {
        let mut current_config_file =
            std::fs::File::create(data_path.join(CURRENT_USERS_CONFIG_FILE_NAME))
                .context("create users config file")?;
        let current_config_data = bitcode::serialize(&self).context("serialize users config")?;
        current_config_file
            .write_all(&current_config_data)
            .context("write users config file")?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, JsonSchema)]
pub struct User {
    name: String,
    groups: Vec<String>,
}

impl User {
    pub fn new(name: String, groups: Vec<String>) -> Self {
        Self { name, groups }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn groups(&self) -> Vec<String> {
        self.groups.clone()
    }

    pub fn add_groups(&self) -> Result<()> {
        let user = self.name();
        let groups = self.groups();
        Self::create_groups_if_needed(&groups).context("create missing groups")?;
        let groups_string = groups.join(",");
        duct::cmd!(
            "sudo",
            "usermod",
            "--append",
            "--groups",
            &groups_string,
            &user
        )
        .run()
        .with_context(|| format!("add user {user} to groups {groups_string}"))?;
        Ok(())
    }

    fn create_groups_if_needed(groups: &[String]) -> Result<()> {
        let existing_groups_output = duct::cmd!("getent", "group")
            .read()
            .context("read system groups")?;
        let existing_groups = existing_groups_output
            .lines()
            .filter_map(|line| line.split(':').next())
            .map(ToString::to_string)
            .collect::<Vec<String>>();
        for group in groups {
            if !existing_groups.contains(group) {
                duct::cmd!("sudo", "groupadd", group)
                    .run()
                    .with_context(|| format!("create group {group}"))?;
            }
        }
        Ok(())
    }

    pub fn remove_groups(&self) -> Result<()> {
        let user = self.name();
        let groups = self.groups();
        let groups_string = groups.join(",");
        duct::cmd!(
            "sudo",
            "usermod",
            "--remove",
            "--groups",
            &groups_string,
            &user
        )
        .run()
        .with_context(|| format!("remove user {user} from groups {groups_string}"))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{User, Users};

    #[test]
    fn users_persistence_round_trip() {
        let dir = tempfile::TempDir::new().unwrap();
        let users: Users = serde_json::from_str(
            r#"[{"name": "alice", "groups": ["wheel", "docker"]}, {"name": "bob", "groups": []}]"#,
        )
        .unwrap();
        users.save_applied_config(dir.path()).unwrap();
        let loaded = Users::read_applied_config(dir.path()).unwrap();
        assert_eq!(loaded.users().len(), 2);
        let alice = loaded
            .users()
            .into_iter()
            .find(|u| u.name() == "alice")
            .unwrap();
        assert!(alice.groups().contains(&"wheel".to_owned()));
        assert!(alice.groups().contains(&"docker".to_owned()));
    }

    #[test]
    fn read_applied_config_returns_none_when_missing() {
        let dir = tempfile::TempDir::new().unwrap();
        assert!(Users::read_applied_config(dir.path()).is_none());
    }

    #[test]
    fn user_new_stores_name_and_groups() {
        let user = User::new(
            "alice".to_owned(),
            vec!["wheel".to_owned(), "docker".to_owned()],
        );
        assert_eq!(user.name(), "alice");
        assert_eq!(user.groups(), vec!["wheel".to_owned(), "docker".to_owned()]);
    }
}
