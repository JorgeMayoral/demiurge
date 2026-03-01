use std::fmt::Display;

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::config::{User, Users};

#[derive(Debug, Clone)]
pub struct UsersChanges(Vec<UserChanges>);

impl UsersChanges {
    pub fn new(new_users_config: &Users, applied_users_config: &Users) -> Self {
        let users_changes = new_users_config
            .users()
            .iter()
            .map(|user| {
                let applied_user = applied_users_config
                    .users()
                    .iter()
                    .find(|applied_user| applied_user.name() == user.name())
                    .map(ToOwned::to_owned)
                    .unwrap_or_default();
                UserChanges::new(user, &applied_user)
            })
            .collect::<Vec<UserChanges>>();
        Self(users_changes)
    }

    pub fn apply(&self) -> Result<()> {
        for change in &self.0 {
            change.apply()?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct UserChanges {
    pub add_groups: User,
    pub remove_groups: User,
}

impl UserChanges {
    pub fn new(new_user_config: &User, applied_user_config: &User) -> Self {
        let groups_to_add: Vec<String> = new_user_config
            .groups()
            .iter()
            .filter(|group| !applied_user_config.groups().contains(group))
            .map(ToOwned::to_owned)
            .collect();
        let groups_to_remove: Vec<String> = applied_user_config
            .groups()
            .iter()
            .filter(|group| !new_user_config.groups().contains(group))
            .map(ToOwned::to_owned)
            .collect();

        Self {
            add_groups: User::new(new_user_config.name(), groups_to_add),
            remove_groups: User::new(new_user_config.name(), groups_to_remove),
        }
    }

    pub fn apply(&self) -> Result<()> {
        if !self.add_groups.groups().is_empty() {
            self.add_groups.add_groups()?;
        }

        if !self.remove_groups.groups().is_empty() {
            self.remove_groups.remove_groups()?;
        }

        Ok(())
    }
}

impl Display for UsersChanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = "Users".blue().bold().to_string();
        let users_text = self
            .0
            .iter()
            .map(|user| format!("{user}"))
            .collect::<Vec<String>>()
            .join("\n");

        let text = format!("{title}\n{users_text}");
        write!(f, "{text}")
    }
}

impl Display for UserChanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = self.add_groups.name().bright_cyan().bold().to_string();
        let groups_to_add = self
            .add_groups
            .groups()
            .iter()
            .map(|group| {
                let symbol = "+".green().to_string();
                format!("[{symbol}] {group}")
            })
            .collect::<Vec<String>>();
        let groups_to_add_text = if groups_to_add.is_empty() {
            "No groups to add".yellow().to_string()
        } else {
            groups_to_add.join("\n")
        };

        let groups_to_remove = self
            .remove_groups
            .groups()
            .iter()
            .map(|group| {
                let symbol = "-".red().to_string();
                format!("[{symbol}] {group}")
            })
            .collect::<Vec<String>>();
        let groups_to_remove_text = if groups_to_remove.is_empty() {
            "No groups to remove".yellow().to_string()
        } else {
            groups_to_remove.join("\n")
        };

        let text = format!("{title}\n{groups_to_add_text}\n\n{groups_to_remove_text}");
        write!(f, "{text}")
    }
}

#[cfg(test)]
mod tests {
    use super::{UserChanges, UsersChanges};
    use crate::config::{User, Users};

    fn make_user(name: &str, groups: &[&str]) -> User {
        User::new(
            name.to_owned(),
            groups.iter().map(|g| (*g).to_owned()).collect(),
        )
    }

    fn make_users(json: &str) -> Users {
        serde_json::from_str(json).unwrap()
    }

    #[test]
    fn new_group_goes_to_add_set() {
        let new_user = make_user("alice", &["wheel", "docker"]);
        let applied_user = make_user("alice", &["wheel"]);
        let changes = UserChanges::new(&new_user, &applied_user);
        assert!(changes.add_groups.groups().contains(&"docker".to_owned()));
        assert!(changes.remove_groups.groups().is_empty());
    }

    #[test]
    fn removed_group_goes_to_remove_set() {
        let new_user = make_user("alice", &["wheel"]);
        let applied_user = make_user("alice", &["wheel", "docker"]);
        let changes = UserChanges::new(&new_user, &applied_user);
        assert!(
            changes
                .remove_groups
                .groups()
                .contains(&"docker".to_owned())
        );
        assert!(changes.add_groups.groups().is_empty());
    }

    #[test]
    fn unchanged_groups_produce_empty_sets() {
        let user = make_user("alice", &["wheel", "docker"]);
        let changes = UserChanges::new(&user, &user);
        assert!(changes.add_groups.groups().is_empty());
        assert!(changes.remove_groups.groups().is_empty());
    }

    #[test]
    fn new_user_puts_all_groups_in_add_set() {
        let new_users = make_users(r#"[{"name": "alice", "groups": ["wheel", "docker"]}]"#);
        let applied_users = make_users(r#"[]"#);
        let changes = UsersChanges::new(&new_users, &applied_users);
        assert_eq!(changes.0.len(), 1);
        let user_changes = &changes.0[0];
        assert!(
            user_changes
                .add_groups
                .groups()
                .contains(&"wheel".to_owned())
        );
        assert!(
            user_changes
                .add_groups
                .groups()
                .contains(&"docker".to_owned())
        );
        assert!(user_changes.remove_groups.groups().is_empty());
    }
}
