use std::{fs::File, io::Cursor, path::PathBuf};

use tar::Builder;

pub fn create(path: &PathBuf) -> anyhow::Result<Cursor<Vec<u8>>> {
    let mut buffer = Vec::new();
    let mut tar = Builder::new(&mut buffer);

    if path.is_dir() {
        tar.append_dir_all(".", path)?;
    } else {
        let file_name = path.file_name().unwrap();
        let mut file = File::open(path)?;
        tar.append_file(file_name, &mut file)?;
    }

    tar.finish()?;
    drop(tar);
    Ok(Cursor::new(buffer))
}
