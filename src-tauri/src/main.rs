#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rustube::{Id, VideoFetcher};

#[tauri::command]
async fn download_video(url: &str, path: &str) -> Result<String, String> {
    let id = match Id::from_raw(url) {
        Ok(id) => id,
        Err(err) => panic!("{}", err.to_string()),
    };
    let vidfetcher = match VideoFetcher::from_id(id.into_owned()) {
        Ok(descram) => descram,
        Err(err) => panic!("{}", err.to_string()),
    };
    let descrambler = match vidfetcher.fetch().await {
        Ok(fetched) => fetched,
        Err(err) => panic!("{}", err.to_string()),
    };
    let video = match descrambler.descramble() {
        Ok(vid) => vid,
        Err(err) => panic!("{}", err.to_string()),
    };

    let best_stream = video.best_quality().unwrap();

    match best_stream.download_to_dir(path).await {
        Ok(path) => Ok(format!("Downloaded to {:?}", path)),
        Err(err) => panic!("{}", err.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![download_video])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
