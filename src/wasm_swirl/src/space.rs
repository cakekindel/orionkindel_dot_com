use crate::constant::N;
use crate::convert;

pub mod vector {
  use crate::convert;
  #[derive(Clone, Copy, PartialEq)]
  pub struct Vector2 {
    pub x: f64,
    pub y: f64,
  }
  impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
      Self { x, y }
    }

    pub fn from_angle(angle: f64) -> Self {
      Self::new(angle.cos(), angle.sin())
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

  pub struct Velocity(Vector2);
  convert!(impl From<Vector2> for newtype Velocity {});
}

pub struct Point {
  x: usize,
  y: usize,
}

pub struct Grid<T> {
  size: usize,
  points: Vec<T>,
}

impl<T> Grid<T> {
  pub fn from_world_size() -> Self {
    Self::from_size(N * N)
  }
  pub fn from_size(size: usize) -> Self {
    Self {
      points: Vec::with_capacity(size),
      size,
    }
  }
  pub fn try_get_mut(&mut self, pt: Point) -> Option<&mut T> {
    self.points.get_mut((pt.x + pt.y) * self.size)
  }
  pub fn get_mut(&mut self, pt: Point) -> &mut T {
    self.try_get_mut(pt).unwrap()
  }

  pub fn try_get(&self, pt: Point) -> Option<&T> {
    self.points.get((pt.x + pt.y) * self.size)
  }
  pub fn get(&self, pt: Point) -> &T {
    self.try_get(pt).unwrap()
  }
}
