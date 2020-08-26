use crate::constant::{ITER, N};
use crate::convert;
use crate::space::math;
use crate::space::Grid;
use crate::time::TimeDelta;
use math::{Coord2, Vector2};

#[derive(Default, Clone, Copy)]
pub struct Diffusion(f64);
convert!(impl From<f64> for newtype Diffusion {});

#[derive(Default, Clone, Copy)]
pub struct Viscosity(f64);
convert!(impl From<f64> for newtype Viscosity {});

#[derive(Default, Clone, Copy)]
pub struct Density(f64);
convert!(impl From<f64> for newtype Density {});
impl std::ops::AddAssign for Density {
  fn add_assign(&mut self, other: Self) -> () {
    self.0 += other.0;
  }
}

pub struct Fluid {
  diff: Diffusion,
  viscosity: Viscosity,
  dt: TimeDelta,
  size: usize,
  density: Grid<Density>,
  density_prev: Grid<Density>,
  velocity: Grid<Vector2>,
  velocity_prev: Grid<Vector2>,
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
    Self {
      dt: dt.into(),
      diff: diffusion.into(),
      viscosity: viscosity.into(),
      size: crate::constant::N,
      density: Grid::<Density>::of_universe_dimensions().into(),
      density_prev: Grid::<Density>::of_universe_dimensions().into(),
      velocity: Grid::<Vector2>::of_universe_dimensions().into(),
      velocity_prev: Grid::<Vector2>::of_universe_dimensions().into(),
    }
  }

  pub fn add_dye(&mut self, pt: Coord2, amount: f64) -> Option<()> {
    self
      .density
      .get_mut(pt)
      .map(|(_, density)| *density += Density::from(amount))
  }

  pub fn add_velocity(&mut self, pt: Coord2, amount: Vector2) -> Option<()> {
    self.velocity.get_mut(pt).map(|(_, velocity)| {
      velocity.add(amount);
      ()
    })
  }

  pub fn diffuse(b: i32, x: Grid<f64>, x_prev: Grid<f64>, diffusion: f64, dt: TimeDelta) -> () {
    let a = &dt as &f64 * diffusion * (N - 2) as f64 * (N - 2) as f64;
    Self::lin_solve(b, x, x_prev, a, 0.0);
  }

  pub fn lin_solve(b: i32, floats: Grid<f64>, floats_prev: Grid<f64>, a: f64, c: f64) {
    use crate::space::math::NeighborStrategy;
    use std::iter::repeat;
    let c_reciprocal = 1.0 / c;

    repeat(()).take(ITER).for_each(|_| {
      floats.iter()
      .map(|(coords, val)| {
        let neighbors = floats.get_neighbors(coords, NeighborStrategy::Adjacent);
        // TODO: gather influence from wrapped neighbors if at bounds
        let sum_of_neighbors = neighbors
          .into_iter()
          .filter_map(|opt| opt.map(|(_, val)| val))
          .fold(0.0, std::ops::Add::add);
        let neighbor_influence = a * sum_of_neighbors;
        let prev_val = floats_prev
          .get(coords)
          .map(|(_, val)| val)
          .expect("floats and floats_prev were different sizes!!");
        (coords, prev_val + neighbor_influence)
      }).for_each(|(coords, next_val)| {
          floats.set(coords, c_reciprocal * next_val);
      })
    })
  }
}
