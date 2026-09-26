use crossbeam_channel::Sender;
use serde::Deserialize;
use strum_macros::{Display, EnumString};

pub const MAX_ITER: u32 = 1000;
pub const X_MIN: f64 = -2.0;
pub const X_MAX: f64 = 0.47;
pub const Y_MIN: f64 = -1.12;
pub const Y_MAX: f64 = -Y_MIN;

pub enum DispatchType {
    Sequential,
    Rayon,
    // StdThreads,
}

#[derive(EnumString, strum_macros::VariantNames, Deserialize, Display, Copy, Clone)]
pub enum Algorithm {
    #[strum(serialize = "Optimized Escape")]
    OptimizedEscape,
    #[strum(serialize = "Unoptimized Escape")]
    UnoptimizedEscape,
}

pub trait Fractal {
    fn zoom(&mut self, width: usize, height: usize, px: usize, py: usize, zoom_mult: f64);

    fn get_fractal_pixels(&self, width: usize, height: usize, tx: Sender<()>) -> Vec<u8>;

    /// Returns the fractal to its initial zoom and position
    fn reset(&mut self);

    fn choose_algo(&mut self, algo: Algorithm);
}
