pub trait AsSome {
  fn as_some<T>(&self, t: T) -> Option<T>;
}

impl AsSome for bool {
  fn as_some<T>(&self, t: T) -> Option<T> {
    match self {
      | true => Some(t),
      | false => None,
    }
  }
}
