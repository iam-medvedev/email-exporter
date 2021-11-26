#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod exporter;
use tauri::{Builder, Menu, MenuItem};

fn main() {
  let menu = Menu::new()
    .add_native_item(MenuItem::Quit)
    .add_native_item(MenuItem::Copy)
    .add_native_item(MenuItem::Cut)
    .add_native_item(MenuItem::Paste);

  Builder::default()
    .menu(menu)
    .invoke_handler(tauri::generate_handler![exporter::run_email_export])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
