// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use mc_query::status;
use crate::status::StatusResponse;

#[tauri::command]
async fn api() -> Result<StatusResponse, Error> {
    let data = status("mc.hypixel.net", 25565).await;
    data
}
