use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::cli::SyncArgs;

pub fn run(args: SyncArgs) -> anyhow::Result<()> {
    let mut res = reqwest::blocking::get("http://localhost:3000/api/v1/test")?;

    let config_path = Path::new(&args.path).join("test");
    fs::create_dir_all(&config_path)?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let file_name = format!("output_file_{}", timestamp);
    let output_path = config_path.join(file_name);

    println!("Writing to {}", output_path.display());
    let mut file = File::create(output_path)?;

    let mut buffer = [0u8; 8192];
    let mut downloaded: u64 = 0;

    let file_size = res.content_length();
    loop {
        let bytes_read = res.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
        downloaded += bytes_read as u64;
        if let Some(total) = file_size {
            let progress = downloaded as f64 / total as f64 * 100.0;
            print!("\rProgress: {:.2}%", progress)
        } else {
            print!("\rDowloaded {} bytes", downloaded);
        }
        std::io::stdout().flush().ok();
    }
    println!("Download Complete");
    Ok(())
}
