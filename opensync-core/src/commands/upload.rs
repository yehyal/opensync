use std::{fs::File, io::Error};

use reqwest::StatusCode;

use crate::cli::SyncArgs;

pub fn run(args: SyncArgs) -> anyhow::Result<()> {
    let path = args.path;
    let file = File::open(path)?;
    let client = reqwest::blocking::Client::new();
    let res = client
        .post("http://localhost:3000/api/v1/upload")
        .body(file)
        .send()?;

    if res.status() == StatusCode::OK {
        println!("UPLOAD COMPLETE");
        Ok(())
    } else {
        println!("UPLOAD FAILED");
        Ok(())
    }
}
