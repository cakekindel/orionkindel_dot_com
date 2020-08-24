use crate::constant::N;
use crate::convert;

pub mod vector {
  use crate::convert;
  pub struct Vector2 {}
  pub struct Velocity(Vector2);
  convert!(impl From<Vector2> for newtype Velocity {});
}

pub struct Point { x: usize, y: usize }

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
    self.points
      .get_mut((pt.x + pt.y) * self.size)
  }
  pub fn get_mut(&mut self, pt: Point) -> &mut T {
    self.try_get_mut(pt).unwrap()
  }

  pub fn try_get(&self, pt: Point) -> Option<&T> {
    self.points
      .get((pt.x + pt.y) * self.size)
  }
  pub fn get(&self, pt: Point) -> &T {
    self.try_get(pt).unwrap()
  }
}
