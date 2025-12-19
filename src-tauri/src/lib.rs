use render::Fractal;
use std::sync::Mutex;
use tauri::{Manager, State};

mod render;

#[tauri::command]
fn get_pixels(width: usize, height: usize, fractal_state: State<'_, Mutex<Fractal>>) -> Vec<u8> {
    let fractal = fractal_state.lock().unwrap();

    fractal.get_fractal_pixels(width, height)
}

#[tauri::command]
fn zoom(
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    zoom_mult: f64,
    fractal_state: State<'_, Mutex<Fractal>>,
) {
    let mut fractal = fractal_state.lock().unwrap();

    fractal.zoom(width, height, x, y, zoom_mult);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(Fractal::default()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_pixels, zoom])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
