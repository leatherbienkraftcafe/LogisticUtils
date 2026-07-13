#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod parser;

use parser::ParsedRow;

#[tauri::command]
fn parse_text(payload: String) -> Vec<ParsedRow> {
    parser::parse_lines(&payload)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![parse_text])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
