pub struct ApplyOutcome {
    pub system: Option<anyhow::Error>,
    pub packages: Option<anyhow::Error>,
    pub dotfiles: Option<anyhow::Error>,
    pub services: Option<anyhow::Error>,
    pub users: Option<anyhow::Error>,
}

impl ApplyOutcome {
    pub fn is_success(&self) -> bool {
        self.system.is_none()
            && self.packages.is_none()
            && self.dotfiles.is_none()
            && self.services.is_none()
            && self.users.is_none()
    }

    pub fn errors(&self) -> impl Iterator<Item = &anyhow::Error> {
        [
            &self.system,
            &self.packages,
            &self.dotfiles,
            &self.services,
            &self.users,
        ]
        .into_iter()
        .filter_map(Option::as_ref)
    }
}

#[cfg(test)]
mod tests {
    use super::ApplyOutcome;

    #[test]
    fn apply_outcome_is_success_when_all_subsystems_succeed() {
        let outcome = ApplyOutcome {
            system: None,
            packages: None,
            dotfiles: None,
            services: None,
            users: None,
        };
        assert!(outcome.is_success());
        assert_eq!(outcome.errors().count(), 0);
    }

    #[test]
    fn apply_outcome_is_not_success_when_any_subsystem_fails() {
        let outcome = ApplyOutcome {
            system: None,
            packages: Some(anyhow::anyhow!("paru failed")),
            dotfiles: None,
            services: Some(anyhow::anyhow!("systemctl failed")),
            users: None,
        };
        assert!(!outcome.is_success());
        assert_eq!(outcome.errors().count(), 2);
    }
}
