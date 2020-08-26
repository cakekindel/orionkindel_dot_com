use crate::constant::N;
use crate::convert;

pub mod math {
  use crate::utils::AsSome;
  use std::cell::RefCell;

  pub enum NeighborStrategy {
    Adjacent,
    IncludeDiag,
  }

  #[derive(Copy, Clone)]
  pub struct Coord2 {
    pub x: usize,
    pub y: usize,
  }
  impl Coord2 {
    pub fn is_neighbor(&self, other: Coord2, strategy: NeighborStrategy) -> bool {
      let x_adjacent = self.x >= other.x - 1 && self.x <= other.x + 1;
      let y_adjacent = self.y >= other.y - 1 && self.y <= other.y + 1;

      match strategy {
        NeighborStrategy::Adjacent => {
          x_adjacent && (self.y == other.y) || y_adjacent && (self.x == other.x)
        }
        NeighborStrategy::IncludeDiag => x_adjacent && y_adjacent,
      }
    }
  }
  impl From<(usize, usize)> for Coord2 {
    fn from((x, y): (usize, usize)) -> Self {
      Coord2 { x, y }
    }
  }

  #[derive(Clone, Copy)]
  pub struct Rect {
    height: usize,
    width: usize,
    origin: Coord2,
  }

  impl Rect {
    pub fn new(origin: impl Into<Coord2>, width: usize, height: usize) -> Self {
      Rect {
        origin: origin.into(),
        width,
        height,
      }
    }

    pub fn contains(&self, coords: Coord2) -> bool {
      let x_bound_lower = self.origin.x;
      let x_bound_upper = self.origin.x + self.width;
      let x_good = coords.x >= x_bound_lower && coords.x <= x_bound_upper;

      let y_bound_lower = self.origin.y;
      let y_bound_upper = self.origin.y + self.height;
      let y_good = coords.y >= y_bound_lower && coords.y <= y_bound_upper;

      x_good && y_good
    }

    pub fn area(&self) -> usize {
      self.height * self.width
    }
  }
  pub struct Matrix2<T> {
    dimensions: Rect,
    pub(self) items: RefCell<Vec<T>>,
  }

  impl<T> Matrix2<T> where T: Default {
    pub fn set(&self, coords: Coord2, val: T) -> Option<T> {
      let items = unsafe {
        self.items.as_ptr().as_mut().unwrap()
      };

      self.get_ix(coords)
        .map(|ix| {
          std::mem::replace(&mut items[ix], val)
        })
    }
  }

  impl<T> Matrix2<T> {
    pub fn from_size(dimensions: Rect) -> Self {
      Self {
        items: RefCell::new(Vec::with_capacity(dimensions.area())),
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
    pub fn get_neighbors(
      &self,
      coords: Coord2,
      strategy: NeighborStrategy,
    ) -> [Option<(Coord2, &T)>; 8] {
      let adjacent = self.get_adjacent_neighbors(coords);
      match strategy {
        NeighborStrategy::Adjacent => [
          None,
          None,
          None,
          None,
          adjacent[0],
          adjacent[1],
          adjacent[2],
          adjacent[3],
        ],
        NeighborStrategy::IncludeDiag => {
          let diags = self.get_diagonal_neighbors(coords);
          [
            diags[0],
            diags[0],
            diags[0],
            diags[0],
            adjacent[0],
            adjacent[1],
            adjacent[2],
            adjacent[3],
          ]
        }
      }
    }

    pub fn get(&self, coords: impl Into<Coord2>) -> Option<(Coord2, &T)> {
      let coords = coords.into();
      let items = unsafe {
        self.items.as_ptr().as_ref().unwrap()
      };

      self
        .get_ix(coords)
        .and_then(|ix| items.get(ix))
        .map(|t| (coords, t))
    }

    pub fn get_mut<'a>(&'a mut self, coords: Coord2) -> Option<(Coord2, &'a mut T)> {
      let coords = coords.into();
      match self
        .get_ix(coords) {
          Some(ix) => self.items.get_mut().get_mut(ix).map(|val| (coords, val)),
          None => None,
        }
    }

    fn get_ix(&self, coords: Coord2) -> Option<usize> {
      self
        .dimensions
        .contains(coords)
        .as_some(coords.x + (coords.y * self.dimensions.height))
    }

    fn get_coords(&self, ix: usize) -> Option<Coord2> {
      Self::get_coords_with_rect(self.dimensions, ix)
    }

    fn get_coords_with_rect(rect: Rect, ix: usize) -> Option<Coord2> {
      let x = ix % rect.height;
      let y = ix / rect.height;
      let coords = Coord2 { x, y };

      rect.contains(coords).as_some(coords)
    }

    pub fn iter<'a>(&'a self) -> Matrix2Iter<'a, T> {
      Matrix2Iter {
        matrix: self,
        ix: 0,
      }
    }

    fn enumerate_ixs<Val>(&self, (ix, val): (usize, Val)) -> (Coord2, Val) {
      (self.get_coords(ix).expect("index OOB"), val)
    }

    pub fn iter_mut<'a>(&'a mut self) -> Matrix2IterMut<'a, T, impl Iterator<Item = (Coord2, &'a mut T)>> {
      let dims = self.dimensions;

      Matrix2IterMut {
        iter: self
          .items
          .get_mut()
          .iter_mut()
          .enumerate()
          .map(move |(ix, val)| (Self::get_coords_with_rect(dims, ix).unwrap(), val))
      }
    }
  }

  pub struct Matrix2Iter<'a, T> {
    matrix: &'a Matrix2<T>,
    ix: usize,
  }

  impl<'a, T> Iterator for Matrix2Iter<'a, T> {
    type Item = (Coord2, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
      self
        .matrix
        .get_coords(self.ix)
        .and_then(|coords| self.matrix.get(coords))
        .map(|ret| {
          self.ix += 1;
          ret
        })
    }
  }

  pub struct Matrix2IterMut<'a, T: 'a, I: Iterator<Item = (Coord2, &'a mut T)>> {
    iter: I
  }

  impl<'a, T: 'a, I: Iterator<Item = (Coord2, &'a mut T)>> Iterator for Matrix2IterMut<'a, T, I> {
    type Item = (Coord2, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
      self.iter.next()
    }
  }

  #[derive(Clone, Copy, PartialEq, Default)]
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
}

pub struct Grid<T> {
  points: math::Matrix2<T>,
}

impl<T: Default> Grid<T> {
  pub fn of_universe_dimensions() -> Self {
    let size = math::Rect::new((0, 0), N, N);

    Self::from_dimensions(size)
  }

  pub fn from_dimensions(dimensions: math::Rect) -> Self {
    Self {
      points: math::Matrix2::from_size(dimensions),
    }
  }

  pub fn get_neighbors<'a>(
    &'a self,
    coords: math::Coord2,
    strategy: math::NeighborStrategy,
  ) -> [Option<(math::Coord2, &'a T)>; 8] {
    self.points.get_neighbors(coords, strategy)
  }

  pub fn get_mut(&mut self, coords: math::Coord2) -> Option<(math::Coord2, &mut T)> {
    self.points.get_mut(coords)
  }

  pub fn get(&self, coords: math::Coord2) -> Option<(math::Coord2, &T)> {
    self.points.get(coords)
  }

  pub fn set(&self, coords: math::Coord2, val: T) -> Option<T> {
    self.points.set(coords, val)
  }

  pub fn iter(&self) -> math::Matrix2Iter<T> {
    self.points.iter()
  }
}
