use std::marker::PhantomData;

use num::Complex;

use crate::fractal::*;
use crate::palette::Rgb;
use crate::pixel::PixelAlgo;
use crate::Fractal;

/// Scales a coordinate in the window into the given range of numbers
fn scale_coordinate(coord: f64, window_dim: f64, min: f64, max: f64) -> f64 {
    coord / window_dim * (max - min) + min
}

pub struct SimpleEscapeFractal {
    center_x: f64,
    center_y: f64, // Center of frame
    zoom: f64,
    pixel_algo: PixelAlgo<f64>,
}

impl Default for SimpleEscapeFractal {
    fn default() -> Self {
        Self {
            center_x: (X_MAX + X_MIN) / 2.0,
            center_y: (Y_MAX + Y_MIN) / 2.0,
            zoom: 1.0,
            pixel_algo: PixelAlgo::Escape {
                max_iter: MAX_ITER,
                num_type: PhantomData,
            },
        }
    }
}

impl Fractal for SimpleEscapeFractal {
    fn zoom(&mut self, width: usize, height: usize, px: usize, py: usize, zoom_mult: f64) {
        self.zoom *= zoom_mult;

        let delta_x = ((X_MAX - X_MIN) / 2.0) / self.zoom;
        let delta_y = ((Y_MAX - Y_MIN) / 2.0) / self.zoom;

        self.center_x = scale_coordinate(
            px as f64,
            (width as u32).into(),
            self.center_x - delta_x,
            self.center_x + delta_x,
        );

        self.center_y = scale_coordinate(
            py as f64,
            (height as u32).into(),
            self.center_y - delta_y,
            self.center_y + delta_y,
        );
    }

    fn get_fractal_pixels(&self, width: usize, height: usize) -> Vec<u8> {
        // According to this documentation (https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial/Pixel_manipulation_with_canvas),
        // The ImageData object uses RGBA, so each pixel needs four bytes
        let mut pixels = Vec::with_capacity(width * height * 4_usize);

        // calculate window dimension values (distance from center to edge of screen in the complex plane)
        let delta_x = ((X_MAX - X_MIN) / 2.0) / self.zoom;
        let delta_y = ((Y_MAX - Y_MIN) / 2.0) / self.zoom;

        for i in 0..height {
            for j in 0..width {
                let x0 = scale_coordinate(
                    j as f64,
                    (width as u32).into(),
                    self.center_x - delta_x,
                    self.center_x + delta_x,
                );

                let y0 = scale_coordinate(
                    i as f64,
                    (height as u32).into(),
                    self.center_y - delta_y,
                    self.center_y + delta_y,
                );

                let iter_count = self.pixel_algo.compute_pixel(x0, y0);
                pixels.extend(iter_to_rgb(iter_count).into_rgba_bytes());
            }
        }
        pixels
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

fn iter_to_rgb(iter_count: u32) -> Rgb {
    let val: u8 = (iter_count as f64 / MAX_ITER as f64 * 256.0) as u8;

    Rgb::from_greyscale(val)
}
