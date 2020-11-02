use space::coords::Rect;
use wasm_bindgen::prelude::*;

pub mod constant;
pub mod fluid;
pub mod math;
pub mod space;
pub mod time;
pub mod utils;

static mut UNIVERSE: Option<Universe> = None;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &JsValue);
}

#[wasm_bindgen]
pub struct FluidPoint {
  pub x: u32,
  pub y: u32,
  pub density: f64,
}

#[wasm_bindgen]
pub fn tick() -> js_sys::Array {
  console_error_panic_hook::set_once();

  let universe = unsafe {
    UNIVERSE.as_mut().unwrap_or_else(|| {
      UNIVERSE = Some(Universe::new());
      log(&"created".into());

      log(&format!("size: {}", UNIVERSE.as_ref().unwrap().fluid.density.dimensions.area()).into());

      UNIVERSE.as_mut().unwrap()
    })
  };

  log(&"ticking".into());

  universe.fluid.tick();

  let density_arr = js_sys::Array::new();

  log(&format!("generating array to render...").into());
  universe.fluid
    .density
    .iter()
    .for_each(|(coord, dens)| {
      let fluid_point = js_sys::Array::new();
      fluid_point.push(&(coord.x as u32).into());
      fluid_point.push(&(coord.y as u32).into());
      fluid_point.push(&dens.0.into());
      density_arr.push(&fluid_point.into());
    });
  log(&format!("done").into());

    density_arr
  }

#[wasm_bindgen]
pub fn mouse_move(viewport_width: usize, viewport_height: usize, mouse_x: usize, mouse_y: usize) {
  let universe = unsafe { UNIVERSE.as_mut() };

  match universe {
    Some(universe) => {
      let Rect {height, width, ..} = universe.fluid.density.dimensions;
      let fluid_to_viewport_ratio = ( width as f64/viewport_width as f64 ,  height as f64/viewport_height as f64 );

      let x = mouse_x as f64 * fluid_to_viewport_ratio.0;
      let y = mouse_y as f64 * fluid_to_viewport_ratio.1;
      log(&format!("{}, {}", x, y).into());

      universe.fluid.add_dye((x as usize, y as usize), 1.0);
    },
    None => (),
  }
}

pub struct Universe {
  pub fluid: fluid::Fluid,
}

impl Universe {
  pub fn new() -> Self {
    Self {
      fluid: fluid::Fluid::new(0.1, 0.0, 0.0),
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
