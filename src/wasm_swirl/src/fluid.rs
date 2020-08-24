use crate::convert;
use crate::space::{vector::Vector2, Grid, Point};
use crate::time::TimeDelta;

pub struct Diffusion(f64);
convert!(impl From<f64> for newtype Diffusion {});

pub struct Viscosity(f64);
convert!(impl From<f64> for newtype Viscosity {});

pub struct FluidDensity(Grid<f64>);
convert!(impl From<Grid<f64>> for newtype FluidDensity {});

pub struct FluidVelocity(Grid<Vector2>);
convert!(impl From<Grid<Vector2>> for newtype FluidVelocity {});

pub struct Fluid {
  diff: Diffusion,
  viscosity: Viscosity,
  dt: TimeDelta,
  size: usize,
  density_prev: FluidDensity,
  density: FluidDensity,
  velocity: FluidVelocity,
  velocity_prev: FluidVelocity,
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
      density_prev: Grid::from_world_size().into(),
      density: Grid::from_world_size().into(),
      velocity: Grid::from_world_size().into(),
      velocity_prev: Grid::from_world_size().into(),
    }
  }

  pub fn add_dye(&mut self, pt: Point, amount: f64) -> () {
    let density = self.density.0.get_mut(pt);
    *density += amount;
  }

  pub fn add_velocity(&mut self, pt: Point, amount: Vector2) -> () {
    let velocity = self.velocity.0.get_mut(pt);
    velocity.add(amount);
  }
}
