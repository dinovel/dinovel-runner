#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod spawn_server;

pub mod app_conf;
pub mod conf_handler;

fn main() {
  spawn_server::spawn();

  let context = tauri::generate_context!();

  let app = tauri::Builder::default();
  
  app.run(context)
    .expect("error while running tauri application");
}
