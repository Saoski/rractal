mod fractal;
mod palette;
mod pixel;
mod unoptimized_escape;

use fractal::Fractal;
use std::sync::Mutex;
use tauri::{Manager, State};
use unoptimized_escape::SimpleEscapeFractal;

#[tauri::command]
fn get_pixels(width: usize, height: usize, fractal_state: State<'_, FractalState>) -> Vec<u8> {
    let fractal = fractal_state.fractal.lock().unwrap();

    fractal.get_fractal_pixels(width, height)
}

#[tauri::command]
fn zoom(
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    zoom_mult: f64,
    fractal_state: State<'_, FractalState>,
) {
    let mut fractal = fractal_state.fractal.lock().unwrap();

    fractal.zoom(width, height, x, y, zoom_mult);
}

#[tauri::command]
fn reset_zoom(fractal_state: State<'_, FractalState>) {
    let mut fractal = fractal_state.fractal.lock().unwrap();

    fractal.reset();
}

struct FractalState {
    fractal: Mutex<Box<dyn Fractal + Send>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(FractalState {
                fractal: Mutex::new(Box::new(SimpleEscapeFractal::default())),
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_pixels, zoom, reset_zoom])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
