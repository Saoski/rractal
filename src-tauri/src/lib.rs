mod fractal;
mod fractals;
mod palette;
mod pixel;

use fractal::{Algorithm, Fractal};
use fractals::Fractalf64;
use serde::Serialize;
use tauri_plugin_log::log;
use std::sync::Mutex;
use strum::VariantNames;
use tauri::{Manager, State};

#[tauri::command]
fn get_pixels(width: usize, height: usize, fractal_state: State<'_, FractalState>) -> Vec<u8> {
    let fractal = fractal_state.fractal.lock().unwrap();
    log::info!("Drawing fractal!");
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
    log::info!("Changed zoom")
}

#[tauri::command]
fn reset_zoom(fractal_state: State<'_, FractalState>) {
    let mut fractal = fractal_state.fractal.lock().unwrap();

    fractal.reset();
    log::info!("Zoom has been reset")
}

#[tauri::command]
fn choose_algo(algo: Algorithm, fractal_state: State<'_, FractalState>) {
    let mut fractal = fractal_state.fractal.lock().unwrap();

    fractal.choose_algo(algo);
    log::info!("Selected fractal algorithm: {}", algo)
}

#[tauri::command]
fn get_algos() -> impl Serialize {
    Algorithm::VARIANTS
}

struct FractalState {
    fractal: Mutex<Box<dyn Fractal + Send>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .setup(|app| {
            app.manage(FractalState {
                fractal: Mutex::new(Box::new(Fractalf64::default())),
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_pixels,
            zoom,
            reset_zoom,
            get_algos,
            choose_algo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
