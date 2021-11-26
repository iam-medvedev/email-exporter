#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod exporter;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![exporter::run_email_export])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
