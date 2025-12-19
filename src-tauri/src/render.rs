use crate::render::palette::Rgb;

const MAX_ITER: u32 = 500;
const X_MIN: f64 = -2.0;
const X_MAX: f64 = 0.47;
const Y_MIN: f64 = -1.12;
const Y_MAX: f64 = -Y_MIN;

mod palette;

/// Scales a coordinate in the window into the given range of numbers
fn scale_coordinate(coord: f64, window_dim: f64, min: f64, max: f64) -> f64 {
    coord / window_dim * (max - min) + min
}

pub struct Fractal {
    center_x: f64,
    center_y: f64, // Center of frame
    zoom: f64,
}

impl Default for Fractal {
    fn default() -> Self {
        Self {
            center_x: (X_MAX + X_MIN) / 2.0,
            center_y: (Y_MAX + Y_MIN) / 2.0,
            zoom: 1.0,
        }
    }
}

impl Fractal {
    pub fn zoom(&mut self, width: usize, height: usize, x: usize, y: usize, zoom_mult: f64) {}

    pub fn get_fractal_pixels(&self, width: usize, height: usize) -> Vec<u8> {
        // According to this documentation (https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API/Tutorial/Pixel_manipulation_with_canvas),
        // The ImageData object uses RGBA, so each pixel needs four bytes
        let mut pixels = Vec::with_capacity(width * height * 4_usize);

        // calculate window dimension values

        let delta_x = ((X_MAX - X_MIN) / 2.0) / self.zoom;
        let delta_y = ((Y_MAX - Y_MIN) / 2.0) / self.zoom;

        for i in 0..height {
            for j in 0..width {
                let pixel =
                    self.unoptimized_get_pixel(j as f64, i as f64, width, height, delta_x, delta_y);
                pixels.extend(pixel.to_rgba_bytes());
            }
        }

        pixels
    }

    fn unoptimized_get_pixel(
        &self,
        px: f64,
        py: f64,
        width: usize,
        height: usize,
        delta_x: f64,
        delta_y: f64,
    ) -> Rgb {
        let x0 = scale_coordinate(
            px,
            (width as u32).into(),
            self.center_x - delta_x,
            self.center_x + delta_x,
        );

        let y0 = scale_coordinate(
            py,
            (height as u32).into(),
            self.center_y - delta_y,
            self.center_y + delta_y,
        );

        let mut x = 0.0;
        let mut y = 0.0;
        let mut i = 0;
        while x * x + y * y <= 4.0 && i < MAX_ITER {
            let temp = x * x - y * y + x0;
            y = 2.0 * x * y + y0;
            x = temp;
            i += 1;
        }

        let val: u8 = (i as f64 / MAX_ITER as f64 * 256.0) as u8;

        Rgb::from_greyscale(val)
    }
}
