#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use image::{io::Reader as ImageReader, DynamicImage};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn processImage(path: &str) {
    let img = loadImage(path);
}

fn loadImage(path: &str) -> Result<DynamicImage, image::ImageError> {
    let img = ImageReader::open(path)?.decode()?;
    Ok(img)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
