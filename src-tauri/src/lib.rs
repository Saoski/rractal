use render::Fractal;
use std::sync::Mutex;
use tauri::{Manager, State};

mod render;

#[tauri::command]
fn get_pixels(fractal_state: State<'_, Mutex<Fractal>>) -> Vec<u8> {
    let fractal = fractal_state.lock().unwrap();

    fractal.get_fractal_pixels()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(Fractal::default()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_pixels])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
