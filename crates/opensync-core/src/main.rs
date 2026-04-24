use crate::cli::{Cli, Commands};
use arboard::Clipboard;
use clap::{Command, Parser};
use clipboard_watcher::{Body, ClipboardEventListener};
use futures::StreamExt;
use log::Level;

mod cli;
mod commands;
mod util;

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    // let args = Cli::parse();

    // run(args)
    // let mut event_listener = ClipboardEventListener::builder().spawn().unwrap();

    // // Specifies the buffer size
    // let mut stream = event_listener.new_stream(32);

    // env_logger::init();

    // while let Some(result) = stream.next().await {
    //     // You can enable logging with RUST_LOG for more detailed inspection.
    //     // Otherwise, the activity will be logged as follows
    //     if !log::log_enabled!(Level::Debug) {
    //         match result {
    //             Ok(content) => {
    //                 match content.as_ref() {
    //                     Body::PlainText(v) => println!("Received string:\n{v}"),
    //                     Body::RawImage(image) => {
    //                         println!("Received raw image");
    //                         if let Some(path) = &image.path {
    //                             println!("Image Path: {}", path.display());
    //                         }
    //                     }
    //                     Body::PngImage {
    //                         path,
    //                         bytes: _bytes,
    //                     } => {
    //                         println!("Received png image");
    //                         if let Some(path) = &path {
    //                             println!("Image Path: {}", path.display());
    //                         }
    //                     }
    //                     Body::FileList(files) => println!("Received files: {files:#?}"),
    //                     Body::Html(html) => println!("Received html: \n{html}"),
    //                     Body::Custom { .. } => {}
    //                 };
    //             }
    //             Err(e) => eprintln!("Got an error: {e}"),
    //         }
    //     }
    // }
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text("Hello from Rust!")?;
    Ok(())
}
pub fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Sync(args) => crate::commands::sync::run(args),
        Commands::Upload(args) => crate::commands::upload::run(args),
    }
}
