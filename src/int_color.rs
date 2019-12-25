use nannou::prelude::*;
pub struct IntColor {
    r: i32,
    g: i32,
    b: i32,
}

impl IntColor {
    pub fn new(r: i32, g: i32, b: i32) -> IntColor {
        IntColor {
            r,
            g,
            b
        }
    }

    pub fn as_srgb(&self) -> Srgba {
        srgba((self.r as f32) / 256.0, (self.g as f32) / 256.0, (self.b as f32) / 256.0, 1.0)
    }

    pub fn as_srgba(&self, alpha: f32) -> Srgba {
        srgba((self.r as f32) / 256.0, (self.g as f32) / 256.0, (self.b as f32) / 256.0, alpha)
    }
}