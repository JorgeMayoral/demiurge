use std::path::PathBuf;

use anyhow::Result;

pub fn path_tilde_expand(path: PathBuf) -> Result<PathBuf> {
    if let Ok(stripped_path) = path.strip_prefix("~/") {
        let user_dirs = directories::UserDirs::new()
            .ok_or(anyhow::anyhow!("Could not get user directories."))?;
        let home = user_dirs.home_dir();
        return Ok(home.join(stripped_path));
    }

    if *path == *"~" {
        let user_dirs = directories::UserDirs::new()
            .ok_or(anyhow::anyhow!("Could not get user directories."))?;
        let home = user_dirs.home_dir();
        return Ok(home.to_path_buf());
    }

    Ok(path)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::path_tilde_expand;

    #[test]
    fn tilde_prefix_expands_to_home() {
        let result = path_tilde_expand("~/Documents/notes.txt".into())
            .expect("path is valid and home dir exists");
        let home =
            directories::UserDirs::new().expect("test environment has a valid home directory");
        assert_eq!(result, home.home_dir().join("Documents/notes.txt"));
    }

    #[test]
    fn tilde_alone_expands_to_home_dir() {
        let result = path_tilde_expand("~".into()).expect("path is valid and home dir exists");
        let home =
            directories::UserDirs::new().expect("test environment has a valid home directory");
        assert_eq!(result, home.home_dir().to_path_buf());
    }

    #[test]
    fn absolute_path_without_tilde_is_unchanged() {
        let path = PathBuf::from("/usr/local/bin");
        let result = path_tilde_expand(path.clone()).expect("path is valid and home dir exists");
        assert_eq!(result, path);
    }

    #[test]
    fn relative_path_without_tilde_is_unchanged() {
        let path = PathBuf::from("some/relative/path");
        let result = path_tilde_expand(path.clone()).expect("path is valid and home dir exists");
        assert_eq!(result, path);
    }
}
