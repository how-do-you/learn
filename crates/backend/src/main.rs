#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod commands;
pub struct MyState(String);
pub struct MyStateTwo(String);
fn main() {
    tauri::Builder::default()
        .manage(MyState("some state value 1".into()))
        .manage(MyStateTwo("some state value 2".into()))
        .invoke_handler(tauri::generate_handler![commands::my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
