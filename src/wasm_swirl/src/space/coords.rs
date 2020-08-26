pub enum NeighborStrategy {
  Adjacent,
  IncludeDiag,
}

#[derive(Copy, Clone)]
pub struct Coord2 {
  pub x: usize,
  pub y: usize,
}

impl From<(usize, usize)> for Coord2 {
  fn from((x, y): (usize, usize)) -> Self {
    Coord2 { x, y }
  }
}

#[derive(Clone, Copy)]
pub struct Rect {
  pub height: usize,
  pub width: usize,
  pub origin: Coord2,
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
