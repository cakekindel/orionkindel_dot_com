use std::cell::RefCell;

use super::coords::{Axis, Coord2, Corner, Edge, EdgeInfo, Rect};
use crate::utils::AsSome;

pub struct Matrix2<T> {
  pub dimensions: Rect,
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

  pub fn get_edge(&self, coords: Coord2) -> EdgeInfo {
    let max_x = self.dimensions.width - 1;
    let max_y = self.dimensions.height - 1;

    let left_edge = if coords.x == 0 {
      Some(Edge::Left)
    } else if coords.x == max_x {
      Some(Edge::Right)
    } else {
      None
    };
    let right_edge = if coords.y == 0 {
      Some(Edge::Top)
    } else if coords.y == max_y {
      Some(Edge::Bottom)
    } else {
      None
    };

    (left_edge, right_edge).into()
  }

  pub fn get_edges<'a>(&'a self) -> impl Iterator<Item = (EdgeInfo, Coord2)> + 'a {
    let max_x = self.dimensions.width - 1;
    let max_y = self.dimensions.height - 1;

    let xs = (0..=max_x).map(|x| (Axis::X, x));
    let ys = (0..=max_y).map(|y| (Axis::Y, y));

    xs.chain(ys)
      // map through all x values and y values
      .map(move |(axis, val)| {
        use std::iter::once;

        // get the "boundary cells" for the current
        // x or y value. think of this as starting with a scanline,
        // and picking the 2 points on the edges
        match axis {
          Axis::X => once((val, 0)).chain(once((val, max_y))),
          Axis::Y => once((0, val)).chain(once((max_x, val))),
        }
        .map(Coord2::from)
        .map(move |coords| (self.get_edge(coords), Coord2::from(coords)))
      })
      // at this point we have an iterator over ALL edge
      // points, excluding corners
      .flatten()
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

  pub fn get_corner(&self, corner: Corner) -> (Coord2, &T) {
    self.get(self.coords_of_corner(corner)).unwrap()
  }

  pub fn set_corner(&self, corner: Corner, val: T) -> () {
    self.set(self.coords_of_corner(corner), val).unwrap();
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

  fn coords_of_corner(&self, corner: Corner) -> Coord2 {
    use Edge::*;

    let x = match corner {
      Corner(Left, _) | Corner(_, Left) => 0,
      _ => self.dimensions.width - 1,
    };

    let y = match corner {
      Corner(Top, _) | Corner(_, Top) => 0,
      _ => self.dimensions.height - 1,
    };

    (x, y).into()
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
