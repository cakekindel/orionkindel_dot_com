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
    if mag == 0.0 { self } else { self / mag }
  }

  pub fn max_mag(self, max: f64) -> Self {
    if self.get_mag() > max {
      self.set_mag(max)
    } else {
      self
    }
  }

  pub fn get_mag(self) -> f64 {
    let (x, y) = (self.x, self.y);
    let (sqx, sqy) = (x * x, y * y);
    let sq_sum = sqx + sqy;
    sq_sum.sqrt()
  }

  pub fn set_mag(self, mag: f64) -> Self {
    self.normalize() * mag
  }
}

impl std::ops::Add for Vector2 {
  type Output = Self;
  fn add(self, other: Self) -> Self::Output {
    Vector2 {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl From<(f64, f64)> for Vector2 {
  fn from((x, y): (f64, f64)) -> Self {
    Self {
      x,
      y,
    }
  }
}

impl std::ops::Mul<f64> for Vector2 {
  type Output = Self;
  fn mul(self, scalar: f64) -> Self::Output {
    Vector2 {
      x: self.x * scalar,
      y: self.y * scalar,
    }
  }
}

impl std::ops::Div<f64> for Vector2 {
  type Output = Self;
  fn div(self, scalar: f64) -> Self::Output {
    Vector2 {
      x: self.x / scalar,
      y: self.y / scalar,
    }
  }
}
