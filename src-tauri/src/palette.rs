pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    /// Create an rgb value based on a given int in the range of [0, 255]
    pub fn from_greyscale(val: u8) -> Self {
        Self {
            r: val,
            g: val,
            b: val,
        }
    }

    /// Returns a vec of three bytes containing the rgb values and an alpha value of 255
    pub fn into_rgba_bytes(self) -> Vec<u8> {
        vec![self.r, self.g, self.b, 255]
    }

    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        let r;
        let g;
        let b;
        if s == 0.0 {
            r = l;
            g = l;
            b = l;
        } else {
            let q = if l < 0.5 {
                l * (1.0 + s)
            } else {
                l + s - l * s
            };
            let p = 2.0 * l - q;
            r = Self::from_hue(p, q, h + 1.0 / 3.0);
            g = Self::from_hue(p, q, h);
            b = Self::from_hue(p, q, h - 1.0 / 3.0);
        }

        Self {
            r: (r * 255.0).floor() as u8,
            g: (g * 255.0).floor() as u8,
            b: (b * 255.0).floor() as u8,
        }
    }

    /// Adapted from https://stackoverflow.com/questions/2353211/hsl-to-rgb-color-conversion
    fn from_hue(p: f32, q: f32, mut t: f32) -> f32 {
        if t < 0.0 {
            t += 1.0
        };
        if t > 1.0 {
            t -= 1.0
        };
        if t < 1.0 / 6.0 {
            return p + (q - p) * 6.0 * t;
        };
        if t < 1.0 / 2.0 {
            return q;
        };
        if t < 2.0 / 3.0 {
            return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
        };

        p
    }
}
