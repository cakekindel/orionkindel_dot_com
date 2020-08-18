use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}
impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn from_angle(angle: f64) -> Self {
        Vector::new(angle.cos(), angle.sin())
    }

    pub fn normalize(self) -> Self {
        let mag = self.get_mag();
        if mag == 0.0 {
            self
        } else {
            self.div(mag)
        }
    }

    pub fn max_mag(self, max: f64) -> Self {
        if self.get_mag() > max {
            self.set_mag(max)
        } else {
            self
        }
    }

    pub fn add(mut self, other: Self) -> Self {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self
    }

    pub fn mult(mut self, mag: f64) -> Self {
        self.x = self.x * mag;
        self.y = self.y * mag;
        self
    }

    pub fn div(self, mag: f64) -> Self {
        self.mult(1.0 / mag)
    }

    pub fn get_mag(self) -> f64 {
        let (x, y) = (self.x, self.y);
        let (sqx, sqy) = (x * x, y * y);
        let sq_sum = sqx + sqy;
        sq_sum.sqrt()
    }

    pub fn set_mag(self, mag: f64) -> Self {
        self.normalize().mult(mag)
    }
}
