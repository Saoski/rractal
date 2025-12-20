use std::marker::PhantomData;

pub trait ComputePixel<T> {
    fn compute_pixel(max_iter: u32, x0: T, y0: T) -> u32;
}

struct UnoptimizedEscape;

impl ComputePixel<f64> for UnoptimizedEscape {
    fn compute_pixel(max_iter: u32, x0: f64, y0: f64) -> u32 {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut i = 0;

        while x * x + y * y <= 4.0 && i < max_iter {
            let temp = x * x - y * y + x0;
            y = x * y * 2.0 + y0;
            x = temp;
            i += 1;
        }

        i
    }
}

pub enum PixelAlgo<T> {
    Escape {
        max_iter: u32,
        num_type: PhantomData<T>,
    },
}

impl PixelAlgo<f64> {
    pub fn compute_pixel(&self, x0: f64, y0: f64) -> u32 {
        match self {
            PixelAlgo::Escape { max_iter, .. } => {
                UnoptimizedEscape::compute_pixel(*max_iter, x0, y0)
            }
        }
    }
}
