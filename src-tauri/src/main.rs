#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

use std::sync::{Mutex, Arc};
use mut_static::MutStatic;

mod spawn_server;

pub mod app_conf;
pub mod conf_handler;
pub mod port_scanner;
pub mod tauri_args;

lazy_static! {
  static ref SERVER_ADDRESS: MutStatic<String> = {
    MutStatic::from("".to_string())
  };
}

#[tauri::command]
fn server_address() -> String {
  SERVER_ADDRESS.read().unwrap().to_string()
}

fn main() {
  let context = tauri::generate_context!();
  let app_args = tauri_args::read_args(&context);
  let server = Arc::new(Mutex::new(spawn_server::spawn(app_args.env.as_str())));

  SERVER_ADDRESS.write()
    .unwrap()
    .push_str(server.lock().unwrap().to_string().as_str());

  tauri::Builder::default()
    .on_window_event(move |event| match event.event() {
      tauri::WindowEvent::Destroyed => {
        server.lock().unwrap().kill();
      }
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![
      server_address
    ])
    .run(context)
    .expect("error while running tauri application");
}
