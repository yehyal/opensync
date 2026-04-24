use std::{fs::File, io::Error};

use reqwest::{StatusCode, blocking::Body};

use crate::{cli::SyncArgs, util::archive};

pub fn run(args: SyncArgs) -> anyhow::Result<()> {
    let path = args.path;
    let tar_stream = archive::create(&path)?;
    let client = reqwest::blocking::Client::new();
    let res = client
        .post("http://localhost:3000/api/v1/upload")
        .body(Body::new(tar_stream))
        .send()?;

    if res.status() == StatusCode::OK {
        println!("UPLOAD COMPLETE");
    } else {
        println!("UPLOAD FAILED");
    }

    Ok(())
}
