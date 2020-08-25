use crate::constant::N;
use crate::convert;

pub mod math {
  pub struct Coord2 {
    pub x: usize,
    pub y: usize,
  }
  pub struct Rect<T> {
    height: T,
    width: T,
  }
  impl Rect<usize> {
    pub fn contains(&self, coords: Coord2) -> Option<()> {
      if coords.x <= self.height && coords.y <= self.width {
        Some(())
      } else {
        None
      }
    }
  }
  impl<T> Rect<T>
  where
    T: std::ops::Mul,
  {
    pub fn area(&self) -> <T as std::ops::Mul>::Output {
      self.height * self.width
    }
  }
  pub struct Matrix2<T> {
    dimensions: Rect<usize>,
    items: Vec<T>,
  }
  impl<T> Matrix2<T> {
    pub fn from_size(dimensions: Rect<usize>) -> Self {
      Self {
        items: Vec::with_capacity(dimensions.area()),
        dimensions,
      }
    }

    pub fn get(&self, coords: Coord2) -> Option<&T> {
      self.get_ix(coords).and_then(|ix| self.items.get(ix))
    }

    pub fn get_mut(&mut self, coords: Coord2) -> Option<&mut T> {
      self.get_ix(coords).and_then(|ix| self.items.get_mut(ix))
    }

    fn get_ix(&mut self, coords: Coord2) -> Option<usize> {
      self
        .dimensions
        .contains(coords)
        .map(|_| coords.x + (coords.y * self.dimensions.height))
    }

    fn get_coords(&mut self, ix: usize) -> Option<Coord2> {
      let x = ix % self.dimensions.height;
      let y = ix / self.dimensions.height;
      let coords = Coord2 { x, y };

      self.dimensions.contains(coords).map(|_| coords)
    }

    pub fn iter<'a>(&'a self) -> Matrix2Iter<'a, T> {
      Matrix2Iter {
        matrix: self,
        ix: 0,
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
        .and_then(|coords| self.matrix.get(coords).map(|val| (coords, val)))
        .map(|ret| {
          self.ix += 1;
          ret
        })
    }
  }

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
}

pub struct Grid<T> {
  points: math::Matrix2<T>,
}

impl<T> Grid<T> {
  pub fn of_universe_dimensions() -> Self {
    let size = math::Rect {
      width: N,
      height: N,
    };

    Self::from_dimensions(size)
  }

  pub fn from_dimensions(dimensions: math::Rect<usize>) -> Self {
    Self {
      points: math::Matrix2::from_size(dimensions),
    }
  }

  pub fn get_mut(&mut self, coords: math::Coord2) -> Option<&mut T> {
    self.points.get_mut(coords)
  }

  pub fn get(&self, coords: math::Coord2) -> Option<&T> {
    self.points.get(coords)
  }

  pub fn iter(&self) -> math::Matrix2Iter<T> {
    self.points.iter()
  }
}
