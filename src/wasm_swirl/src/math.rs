pub trait Clamp
where
  Self: PartialOrd + Sized,
{
  fn clam(self, min: Self, max: Self) -> Self {
    if self < min {
      min
    } else if self > max {
      max
    } else {
      self
    }
  }
}

impl<T> Clamp for T where T: PartialOrd {}
