#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod browser;
mod parser;

use browser::BrowserState;
use parser::ParsedRow;
use tauri::Manager;

#[tauri::command]
fn parse_text(payload: String) -> Vec<ParsedRow> {
    parser::parse_lines(&payload)
}

#[tauri::command]
async fn open_clickpost_browser(
    app: tauri::AppHandle,
    state: tauri::State<'_, BrowserState>,
    url: String,
) -> Result<(), String> {
    let profile_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("browser-profile");
    browser::open(&state, &profile_dir, &url).await
}

#[tauri::command]
async fn inject_rows(
    app: tauri::AppHandle,
    state: tauri::State<'_, BrowserState>,
    rows: Vec<ParsedRow>,
) -> Result<Vec<String>, String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let mapping = browser::load_or_init_field_mapping(&config_dir)?;
    browser::inject(&state, &mapping, &rows).await
}

fn main() {
  tauri::Builder::default()
    .manage(browser::new_state())
    .invoke_handler(tauri::generate_handler![
        parse_text,
        open_clickpost_browser,
        inject_rows
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
