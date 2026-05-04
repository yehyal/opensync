use directories::ProjectDirs;
use std::path::{Path, PathBuf};

fn app_data_dir() -> PathBuf {
    let proj =
        ProjectDirs::from("com", "yourname", "opensync").expect("failed to resolve project dirs");

    let dir = proj.data_dir();
    std::fs::create_dir_all(dir).expect("failed to create data dir");

    dir.to_path_buf()
}

pub fn db_path() -> PathBuf {
    // app_data_dir().join("app.db")
    Path::new("app.db").to_path_buf()
}
