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
use serde_json;

#[tauri::command]
async fn api() -> String {
    let data = status("survival.limeskey.com", 25565).await;
    println!("{:?}", data);
    match data {
        Ok(data) => serde_json::to_string(&data).unwrap(),
        Err(_) => panic!("hi"),

    }
}
