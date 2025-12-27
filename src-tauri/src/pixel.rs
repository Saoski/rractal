use std::marker::PhantomData;

pub trait ComputePixelf64 {
    fn compute_pixel(max_iter: u32, x0: f64, y0: f64) -> u32;
}

struct UnoptimizedEscape;
struct OptimizedEscape;

impl ComputePixelf64 for UnoptimizedEscape {
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

impl ComputePixelf64 for OptimizedEscape {
    fn compute_pixel(max_iter: u32, x0: f64, y0: f64) -> u32 {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut x2 = 0.0;
        let mut y2 = 0.0;
        let mut i = 0;

        while x2 + y2 <= 4.0 && i < max_iter {
            x2 = x * x;
            y2 = y * y;
            y = (x + x) * y + y0;
            x = x2 - y2 + x0;
            i += 1;
        }

        i
    }
}

pub enum PixelAlgo<T> {
    UnoptimizedEscape {
        max_iter: u32,
        num_type: PhantomData<T>,
    },
    OptimizedEscape {
        max_iter: u32,
        num_type: PhantomData<T>,
    },
}

impl PixelAlgo<f64> {
    pub fn compute_pixel(&self, x0: f64, y0: f64) -> u32 {
        match self {
            PixelAlgo::UnoptimizedEscape { max_iter, .. } => {
                UnoptimizedEscape::compute_pixel(*max_iter, x0, y0)
            }
            PixelAlgo::OptimizedEscape { max_iter, .. } => {
                OptimizedEscape::compute_pixel(*max_iter, x0, y0)
            }
        }
    }
}
