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
