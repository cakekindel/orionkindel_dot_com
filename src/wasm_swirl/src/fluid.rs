use crate::constant::{ITER, N};
use crate::convert;
use crate::space::coords::{Coord2, Rect};
use crate::space::matrix::Matrix2;
use crate::space::vector::Vector2;
use crate::time::TimeDelta;

#[derive(Default, Clone, Copy)]
pub struct Diffusion(f64);
convert!(impl From<f64> for newtype Diffusion {});

#[derive(Default, Clone, Copy)]
pub struct Viscosity(f64);
convert!(impl From<f64> for newtype Viscosity {});

#[derive(Default, Clone, Copy)]
pub struct Density(f64);
convert!(impl From<f64> for newtype Density {});
impl std::ops::Add for Density {
  type Output = Self;
  fn add(self, other: Self) -> Self::Output {
    Self(self.0 + other.0)
  }
}

pub struct Fluid {
  diff: Diffusion,
  viscosity: Viscosity,
  dt: TimeDelta,
  size: usize,
  density: Matrix2<Density>,
  density_prev: Matrix2<Density>,
  velocity: Matrix2<Vector2>,
  velocity_prev: Matrix2<Vector2>,
}

impl Fluid {
  pub fn new<
    TimeDeltaLike: Into<TimeDelta>,
    DiffusionLike: Into<Diffusion>,
    ViscosityLike: Into<Viscosity>,
  >(
    dt: TimeDeltaLike,
    diffusion: DiffusionLike,
    viscosity: ViscosityLike,
  ) -> Self {
    let dims = Rect {
      width: N,
      height: N,
      origin: (0, 0).into(),
    };

    Self {
      dt: dt.into(),
      diff: diffusion.into(),
      viscosity: viscosity.into(),
      size: crate::constant::N,
      density: Matrix2::<Density>::from_dimensions(dims).into(),
      density_prev: Matrix2::<Density>::from_dimensions(dims).into(),
      velocity: Matrix2::<Vector2>::from_dimensions(dims).into(),
      velocity_prev: Matrix2::<Vector2>::from_dimensions(dims).into(),
    }
  }

  pub fn add_dye(&mut self, pt: Coord2, amount: f64) -> Option<()> {
    self
      .density
      .get(pt)
      .map(|(_, density)| self.density.set(pt, *density + Density::from(amount)))
      .map(|_| ())
  }

  pub fn add_velocity(&mut self, pt: Coord2, amount: Vector2) -> Option<()> {
    self.velocity.get(pt).map(|(pt, velocity)| {
      self.velocity.set(pt, *velocity + amount);
    })
  }

  pub fn diffuse(
    b: i32,
    x: Matrix2<f64>,
    x_prev: Matrix2<f64>,
    diffusion: f64,
    dt: TimeDelta,
  ) -> () {
    let a = &dt as &f64 * diffusion * (N - 2) as f64 * (N - 2) as f64;
    Self::lin_solve(b, x, x_prev, a, 0.0);
  }

  pub fn lin_solve(b: i32, floats: Matrix2<f64>, floats_prev: Matrix2<f64>, a: f64, c: f64) {
    use std::iter::repeat;
    use std::ops::Add;
    let c_reciprocal = 1.0 / c;

    repeat(()).take(ITER).for_each(|_| {
      floats
        .iter()
        .map(|(coords, val)| {
          let neighbors = floats.get_adjacent_neighbors(coords);

          // TODO: gather influence from wrapped neighbors if at bounds
          let neighbors_sum = neighbors
            .into_iter()
            .filter_map(|opt| opt.map(|(_, val)| val))
            .fold(0.0, Add::add);

          let neighbor_influence = a * neighbors_sum;

          let prev_val = floats_prev
            .get(coords)
            .expect("floats and floats_prev were different sizes!!")
            .1;

          (coords, prev_val + neighbor_influence)
        })
        .for_each(|(coords, next_val)| {
          floats.set(coords, c_reciprocal * next_val);
        })
    })
  }
}
