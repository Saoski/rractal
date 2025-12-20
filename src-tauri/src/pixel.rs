use std::marker::PhantomData;

use num::{Complex, FromPrimitive, Num, Zero};

pub trait ComputePixel<T> {
    fn compute_pixel(max_iter: u32, num: Complex<T>) -> u32;
}

struct Escape;

impl<T> ComputePixel<T> for Escape
where
    T: Num + Clone + PartialOrd + FromPrimitive,
{
    fn compute_pixel(max_iter: u32, c: Complex<T>) -> u32 {
        let mut z: Complex<T> = Complex::zero();
        let mut i = 0;

        let four = T::from_u8(4).unwrap();
        while z.norm_sqr() <= four && i < max_iter {
            z = z.powu(2) + &c;
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

impl<T> PixelAlgo<T>
where
    T: Num + Clone + PartialOrd + FromPrimitive,
{
    pub fn compute_pixel(&self, c: Complex<T>) -> u32 {
        match self {
            PixelAlgo::Escape { max_iter, .. } => Escape::compute_pixel(*max_iter, c),
        }
    }
}
