use std::path::PathBuf;

pub fn path_tilde_expand(path: PathBuf) -> PathBuf {
    if let Ok(stripped_path) = path.strip_prefix("~/") {
        let user_dirs = directories::UserDirs::new().unwrap();
        let home = user_dirs.home_dir();
        return home.join(stripped_path);
    }

    if *path == *"~" {
        let user_dirs = directories::UserDirs::new().unwrap();
        let home = user_dirs.home_dir();
        return home.to_path_buf();
    }

    path
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::path_tilde_expand;

    #[test]
    fn tilde_prefix_expands_to_home() {
        let result = path_tilde_expand("~/Documents/notes.txt".into());
        let home = directories::UserDirs::new().unwrap();
        assert_eq!(result, home.home_dir().join("Documents/notes.txt"));
    }

    #[test]
    fn tilde_alone_expands_to_home_dir() {
        let result = path_tilde_expand("~".into());
        let home = directories::UserDirs::new().unwrap();
        assert_eq!(result, home.home_dir().to_path_buf());
    }

    #[test]
    fn absolute_path_without_tilde_is_unchanged() {
        let path = PathBuf::from("/usr/local/bin");
        let result = path_tilde_expand(path.clone());
        assert_eq!(result, path);
    }

    #[test]
    fn relative_path_without_tilde_is_unchanged() {
        let path = PathBuf::from("some/relative/path");
        let result = path_tilde_expand(path.clone());
        assert_eq!(result, path);
    }
}
