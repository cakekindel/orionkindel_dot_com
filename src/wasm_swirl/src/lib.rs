use wasm_bindgen::*;

pub mod constant;
pub mod fluid;
pub mod space;
pub mod time;
pub mod utils;

static mut UNIVERSE: Option<Universe> = None;

pub fn setup() -> () {
  unsafe {
    UNIVERSE = Some(Universe::new());
  }
}

pub struct Universe {
  pub fluids: Vec<fluid::Fluid>,
}

impl Universe {
  pub fn new() -> Self {
    Self {
      fluids: vec![fluid::Fluid::new(0.1, 0.0, 0.0)],
    }
  }
}

#[macro_export]
macro_rules! convert {
  (impl From<$source:ty> for newtype $dest:tt {}) => {
    impl From<$source> for $dest {
      fn from(src: $source) -> Self {
        $dest(src)
      }
    }
  };

  (impl From<$source:ty> for $dest:ty => $closure:expr) => {
    impl From<$source> for $dest {
      fn from(src: $source) -> Self {
        $closure(src)
      }
    }
  };
  (impl Deref<$dest:ty> for newtype $src:tt {}) => {
    impl std::ops::Deref for $src {
      type Target = $dest;
      fn deref(&self) -> &Self::Target {
        &self.0
      }
    }
  };
  (impl<'_> From<$source:ident> for $dest:ident => $closure:expr) => {
    impl<'a> From<$source<'a>> for $dest<'a> {
      fn from(src: $source<'a>) -> $dest<'a> {
        $closure(src)
      }
    }
  };
  (impl<'a> From<$source:ty> for $dest:ty => $closure:expr) => {
    impl<'a> From<$source<'a>> for $dest<'a> {
      fn from(src: $source<'a>) -> $dest<'a> {
        $closure(src)
      }
    }
  };
  (impl<'a> From<$source:ty> for $dest:ty => $closure:expr) => {
    impl<'a> From<$source> for $dest {
      fn from(src: $source) -> $dest {
        $closure(src)
      }
    }
  };
  (impl From<impl $trait_:ident<$source:ty>> for $dest:ty => $closure:expr) => {
    impl<T> From<T> for $dest
    where
      T: $trait_<$source>,
    {
      fn from(src: T) -> Self {
        $closure(src)
      }
    }
  };
  (impl<'_> From<impl $trait_:ident<$source:ident>>
 for $dest:ident => |$param:ident| $body:expr) => {
    impl<'a, T> From<T> for $dest<'a>
    where
      T: $trait_<$source<'a>>,
    {
      fn from($param: T) -> Self {
        $body
      }
    }
  };
}
