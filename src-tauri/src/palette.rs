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
}
