// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tokio::main]
async fn main() {
    api().await;
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use mc_query::status;
use serde::Serialize;
use crate::status::StatusResponse;

#[tauri::command]
async fn api() -> StatusResponse {
    let data: Result<StatusResponse, std::io::Error> = status("survival.limeskey.com", 25565).await;
    println!("{:?}", data);
    match data {
        Ok(data) => data,
        Err(_) => {panic!("hi");}
    }
}
