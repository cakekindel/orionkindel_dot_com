use std::cell::RefCell;

use super::coords::{Coord2, Rect};
use crate::utils::AsSome;

pub struct Matrix2<T> {
  dimensions: Rect,
  pub(self) items: RefCell<Vec<T>>,
}

impl<T> Matrix2<T> {
  pub fn from_dimensions(dimensions: Rect) -> Self {
    let allocated_vec = Vec::with_capacity(dimensions.area());
    Self {
      items: RefCell::new(allocated_vec),
      dimensions,
    }
  }

  pub fn get_adjacent_neighbors(&self, coords: Coord2) -> [Option<(Coord2, &T)>; 4] {
    [
      self.get((coords.x - 1, coords.y)),
      self.get((coords.x + 1, coords.y)),
      self.get((coords.x, coords.y - 1)),
      self.get((coords.x, coords.y + 1)),
    ]
  }

  pub fn get_diagonal_neighbors(&self, coords: Coord2) -> [Option<(Coord2, &T)>; 4] {
    [
      self.get((coords.x - 1, coords.y - 1)),
      self.get((coords.x - 1, coords.y + 1)),
      self.get((coords.x + 1, coords.y - 1)),
      self.get((coords.x + 1, coords.y + 1)),
    ]
  }

  pub fn get(&self, coords: impl Into<Coord2>) -> Option<(Coord2, &T)> {
    let coords = coords.into();
    let items = unsafe { self.items.as_ptr().as_ref().unwrap() };

    self
      .index_of(coords)
      .and_then(|ix| items.get(ix))
      .map(|t| (coords, t))
  }

  pub fn set(&self, coords: Coord2, val: T) -> Option<T> {
    let items = unsafe { self.items.as_ptr().as_mut().unwrap() };

    self
      .index_of(coords)
      .map(|ix| std::mem::replace(&mut items[ix], val))
  }

  fn index_of(&self, coords: Coord2) -> Option<usize> {
    self
      .dimensions
      .contains(coords)
      .as_some(coords.x + (coords.y * self.dimensions.height))
  }

  fn coords_of(&self, ix: usize) -> Option<Coord2> {
    let x = ix % self.dimensions.height;
    let y = ix / self.dimensions.height;
    let coords = Coord2 { x, y };

    self.dimensions.contains(coords).as_some(coords)
  }

  pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    Iter {
      matrix: self,
      ix: 0,
    }
  }
}

pub struct Iter<'a, T> {
  matrix: &'a Matrix2<T>,
  ix: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = (Coord2, &'a T);
  fn next(&mut self) -> Option<Self::Item> {
    self
      .matrix
      .coords_of(self.ix)
      .and_then(|coords| self.matrix.get(coords))
      .map(|ret| {
        self.ix += 1;
        ret
      })
  }
}
