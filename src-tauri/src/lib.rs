mod fractal;
mod fractals;
mod palette;
mod pixel;

use crossbeam_channel::unbounded;
use fractal::{Algorithm, Fractal};
use fractals::Fractalf64;
use serde::Serialize;
use std::{sync::Mutex, thread};
use strum::VariantNames;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_log::log;

#[tauri::command]
fn get_pixels(
    width: usize,
    height: usize,
    fractal_state: State<'_, FractalState>,
    app: AppHandle,
) -> Vec<u8> {
    let fractal = fractal_state.fractal.lock().unwrap();
    // Channel doesn't need to send actual data, just the event that a pixel was calculated
    let (tx, rx) = unbounded::<()>();
    let progress_listener_handle = thread::spawn(move || {
        let pixel_count = width * height;
        let mut pixels_computed = 0;
        while pixels_computed < pixel_count {
            rx.recv()
                .expect("Pixel progress listener recv should not fail");
            pixels_computed += 1;
            // Broadcast a percentage progress update
            if pixels_computed % 20000 == 0 {
                let percent_progress = (pixels_computed * 100 / pixel_count) as u32;
                app.emit("fractal-progress", percent_progress)
                    .unwrap();
            }
        }
        // Send final 100% progress event
        app.emit("fractal-progress", 100u32).unwrap();
    });
    log::info!("Drawing fractal!");
    let computed_pixels = fractal.get_fractal_pixels(width, height, tx);
    progress_listener_handle
        .join()
        .expect("Progress listener thread should not fail to join");
    computed_pixels
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
