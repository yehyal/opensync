use directories::ProjectDirs;
use std::path::PathBuf;

fn app_data_dir() -> PathBuf {
    let proj =
        ProjectDirs::from("com", "yehya", "opensync").expect("failed to resolve project dirs");

    let dir = proj.data_dir();
    std::fs::create_dir_all(dir).expect("failed to create data dir");

    dir.to_path_buf()
}

pub fn db_path() -> PathBuf {
    if cfg!(debug_assertions) {
        std::env::current_dir()
            .expect("failed to resolve current directory")
            .join("app.db")
    } else {
        app_data_dir().join("app.db")
    }
}
