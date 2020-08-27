pub trait Perpendicular<Cmp = Self> {
  fn perpendicular(&self, cmp: &Cmp) -> bool;
}

#[derive(Copy, Clone, PartialEq)]
pub enum Axis {
  X,
  Y,
}
impl Perpendicular for Axis {
  fn perpendicular(&self, cmp: &Self) -> bool {
    self != cmp
  }
}

impl Perpendicular<Axis> for Edge {
  fn perpendicular(&self, cmp: &Axis) -> bool {
    self.as_axis().perpendicular(cmp)
  }
}

impl Perpendicular for Edge {
  fn perpendicular(&self, cmp: &Self) -> bool {
    self.as_axis().perpendicular(&cmp.as_axis())
  }
}

#[derive(Copy, Clone)]
pub enum EdgeInfo {
  Edge(Edge),
  Corner(Edge, Edge),
  Neither,
}

impl EdgeInfo {
  pub fn get_edge(&self) -> Option<Edge> {
    match self {
      Self::Edge(edge) => Some(*edge),
      _ => None,
    }
  }
}

impl From<(Option<Edge>, Option<Edge>)> for EdgeInfo {
  fn from(edges: (Option<Edge>, Option<Edge>)) -> Self {
    use Edge::*;

    // this is dumb.
    match edges {
      (None, None) => Self::Neither,
      (Some(Top), Some(Left)) | (Some(Left), Some(Top)) => Self::Corner(Top, Left),
      (Some(Top), Some(Right)) | (Some(Right), Some(Top)) => Self::Corner(Top, Right),
      (Some(Bottom), Some(Right)) | (Some(Right), Some(Bottom)) => Self::Corner(Bottom, Right),
      (Some(Bottom), Some(Left)) | (Some(Left), Some(Bottom)) => Self::Corner(Bottom, Left),
      (Some(Top), _) | (_, Some(Top)) => Self::Edge(Top),
      (Some(Bottom), _) | (_, Some(Bottom)) => Self::Edge(Bottom),
      (Some(Left), _) | (_, Some(Left)) => Self::Edge(Left),
      (Some(Right), _) | (_, Some(Right)) => Self::Edge(Right),
    }
  }
}

#[derive(Copy, Clone)]
pub enum Edge {
  Top,
  Right,
  Bottom,
  Left,
}
impl Edge {
  pub fn as_axis(&self) -> Axis {
    use Axis::*;
    use Edge::*;

    match self {
      Top | Bottom => Y,
      Left | Right => X,
    }
  }
}

#[derive(Copy, Clone)]
pub struct Corner(pub Edge, pub Edge);
impl Corner {
  pub fn all() -> [Self; 4] {
    use Edge::*;
    [
      Self(Top, Left),
      Self(Top, Right),
      Self(Bottom, Left),
      Self(Bottom, Right),
    ]
  }
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
