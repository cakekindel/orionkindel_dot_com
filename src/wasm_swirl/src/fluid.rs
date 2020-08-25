use crate::constant::{ITER, N};
use crate::convert;
use crate::space::{vector::Vector2, Grid, Point};
use crate::time::TimeDelta;

pub struct Diffusion(f64);
convert!(impl From<f64> for newtype Diffusion {});

pub struct Viscosity(f64);
convert!(impl From<f64> for newtype Viscosity {});

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
      density: Grid::from_world_size().into(),
      density_prev: Grid::from_world_size().into(),
      velocity: Grid::from_world_size().into(),
      velocity_prev: Grid::from_world_size().into(),
    }
  }

  pub fn add_dye(&mut self, pt: Point, amount: f64) -> () {
    let density = self.density.get_mut(pt);
    *density += Density::from(amount);
  }

  pub fn add_velocity(&mut self, pt: Point, amount: Vector2) -> () {
    let velocity = self.velocity.get_mut(pt);
    velocity.add(amount);
  }

  pub fn diffuse(b: i32, x: Grid<f64>, x_prev: Grid<f64>, diffusion: f64, dt: TimeDelta) -> () {
    let a = &dt as &f64 * diffusion * (N - 2) as f64 * (N - 2) as f64;
    Self::lin_solve(b, x, x_prev, 0.0, 0.0);
  }

  pub fn lin_solve(b: i32, mut floats: Grid<f64>, floats_prev: Grid<f64>, a: f64, c: f64) {
    use std::iter::repeat;
    let c_reciprocal = 1.0 / c;

    fn get_sum_of_neighbors<T: std::ops::Add>(pt: Point, grid: Grid<T>) -> T {
      grid.get_neighbors(pt);
    }

    repeat(()).take(ITER).for_each(|_| {
      floats.iter().for_each(|y| {
        let pt = Point { x, y };
        let sum_of_neighbors = floats.get(x);
        let neighbor_influence = a * sum_of_neighbors;
        let x_at_pt = floats.get_mut(pt);
        let prev_x = floats_prev.get_mut(pt);
        let next_x = prev_x + neighbor_influence;
        *x_at_pt = c_reciprocal * next_x;
      })
    })
  }
}
