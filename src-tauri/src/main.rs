// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if let Err(e) = project_anu_lib::run() {
        eprintln!("Error al iniciar la aplicación: {}", e);
        std::process::exit(1);
    }
}