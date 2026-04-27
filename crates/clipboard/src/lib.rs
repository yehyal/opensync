use arboard::Clipboard;
use clipboard_watcher::{Body, ClipboardEventListener};
use futures::StreamExt;
use reqwest::Client;
use serde::Serialize;
use std::{sync::Arc, time::Duration};
use sync::SuppresionCache;

pub fn add(text: &str) -> Result<(), arboard::Error> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(text).ok();
    Ok(())
}

pub async fn watch(cache: Arc<SuppresionCache>) {
    let client = Client::new();
    let mut restart_backoff = Duration::from_millis(200);

    loop {
        let mut event_listener = match ClipboardEventListener::builder().spawn() {
            Ok(listener) => {
                restart_backoff = Duration::from_millis(200);
                listener
            }
            Err(e) => {
                eprintln!(
                    "clipboard watcher: failed to spawn listener ({e:?}); retrying in {:?}",
                    restart_backoff
                );
                std::thread::sleep(restart_backoff);
                restart_backoff = (restart_backoff * 2).min(Duration::from_secs(5));
                continue;
            }
        };

        // Specifies the buffer size
        let mut stream = event_listener.new_stream(32);

        while let Some(result) = stream.next().await {
            match result {
                Ok(content) => {
                    if let Err(e) = handle_clipboard_event(content.as_ref(), &client, &cache).await
                    {
                        eprintln!("clipboard watcher: handler error: {e}");
                    }
                }
                Err(e) => eprintln!("clipboard watcher: stream error: {e}"),
            }
        }

        // If the stream ends, restart the listener instead of exiting the task/thread.
        eprintln!("clipboard watcher: stream ended; restarting listener");
    }
}

async fn handle_clipboard_event(
    content: &Body,
    client: &Client,
    cache: &Arc<SuppresionCache>,
) -> Result<(), reqwest::Error> {
    match content {
        Body::PlainText(v) => {
            println!("Received string:\n{v}");
            let hash = sync::hash(v);
            println!("Hashed value: {hash}");
            if cache.contains_hash(&hash) {
                return Ok(());
            }
            send_clipboard_event(client, v.to_string()).await?;
        }
        Body::RawImage(image) => {
            println!("Received raw image");
            if let Some(path) = &image.path {
                println!("Image Path: {}", path.display());
            }
        }
        Body::PngImage {
            path,
            bytes: _bytes,
        } => {
            println!("Received png image");
            if let Some(path) = &path {
                println!("Image Path: {}", path.display());
            }
        }
        Body::FileList(files) => println!("Received files: {files:#?}"),
        Body::Html(html) => println!("Received html: \n{html}"),
        Body::Custom { .. } => {}
    };
    Ok(())
}

async fn send_clipboard_event(client: &Client, content: String) -> Result<(), reqwest::Error> {
    client
        // .post("http://localhost:3000/event")
        .post("https://hsiu-sociologistic-aliya.ngrok-free.dev/event")
        .json(&ClipboardEvent { text: content })
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

#[derive(Serialize)]
struct ClipboardEvent {
    pub text: String,
}
