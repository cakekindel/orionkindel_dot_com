use std::ops::Add;

use crate::constant::{ITER, N};
use crate::convert;
use crate::space::coords::{
  Axis, Coord2, Corner, Edge, Perpendicular, Rect,
};
use crate::space::matrix::Grid;
use crate::space::vector::Vector2;
use crate::{time::TimeDelta, utils::AsSome};

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

pub struct FluidPoint {
  density: Density,
  velocity: Vector2,
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
      density: Grid::<Density>::from_dimensions(dims),
      density_prev: Grid::<Density>::from_dimensions(dims),
      velocity: Grid::<Vector2>::from_dimensions(dims),
      velocity_prev: Grid::<Vector2>::from_dimensions(dims),
    }
  }

  pub fn add_dye(&mut self, pt: Coord2, amount: f64) -> Option<()> {
    self
      .density
      .get(pt)
      .map(|dens| *dens)
      .map(|density| self.density.set(pt, density + amount.into()))
      .map(|_| ())
  }

  pub fn add_velocity(
    &mut self,
    pt: Coord2,
    amount: Vector2,
  ) -> Option<()> {
    self
      .velocity
      .get(pt)
      .map(|vector| *vector)
      .map(|velocity| {
        self.velocity.set(pt, velocity + amount);
      })
  }

  pub fn diffuse(
    grid: &mut Grid<f64>,
    grid_prev: &Grid<f64>,
    diffusion: f64,
    dt: TimeDelta,
    counteract_axis: Option<Axis>,
  ) -> () {
    let a = &dt as &f64 * diffusion * (N - 2) as f64 * (N - 2) as f64;
    Self::lin_solve(grid, grid_prev, a, 0.0, counteract_axis);
  }

  pub fn lin_solve(
    grid: &mut Grid<f64>,
    grid_prev: &Grid<f64>,
    a: f64,
    c: f64,
    counteract_axis: Option<Axis>,
  ) {
    std::iter::repeat(()).take(ITER).for_each(|_| {
      grid
        .iter_mut()
        .map(|(coords, val_mut)| {
          let neighbor_sum = grid_prev
            .get_adjacent_neighbors(coords)
            .map(|(_, val)| val)
            .fold(0.0, Add::add);

          let neighbor_influence = a * neighbor_sum;

          let prev_val = grid_prev
            .get(coords)
            .expect("floats and floats_prev were different sizes!!");

          let next_val = (prev_val + neighbor_influence) / c;
          (val_mut, next_val)
        })
        .for_each(|(val_mut, next_val)| {
          *val_mut = next_val;
        })
    });

    Self::set_bnd(grid, counteract_axis)
  }

  pub fn set_bnd(
    grid: &mut Grid<f64>,
    counteract_axis: Option<Axis>,
  ) {
    Self::fix_corners(grid);
    Self::fix_edges(grid, counteract_axis);
  }

  // TODO: FixEdges trait?
  pub fn fix_edges(
    grid: &mut Grid<f64>,
    counteract_axis: Option<Axis>,
  ) {
    // This function calls `Grid::edges_iter_mut`,
    // and wants to read the values of the edge points'
    // neighbors.

    // We need to bypass the borrow checker because
    // Rust doesn't _know_ that the neighbor won't be changed
    // as a result of the mutable borrow
    let grid: *mut _ = grid;

    unsafe {
      grid
        .as_mut()
        .unwrap()
        .edges_iter_mut()
        // filter out corners
        .filter_map(|(edge_info, coords, val_mut)| {
          edge_info
            .get_edge()
            .map(|edge| (edge, coords, val_mut))
        })
        .map(|(edge, coords, val_mut)| {
          // we want to read the value of the point next to the
          // current point on the edge
          let neighbor_coords = match edge {
            | Edge::Top => (coords.x, coords.y + 1),
            | Edge::Bottom => (coords.x, coords.y - 1),
            | Edge::Left => (coords.x + 1, coords.y),
            | Edge::Right => (coords.x - 1, coords.y),
          };

          let neighbor = grid
            .as_ref()
            .unwrap()
            .get(neighbor_coords)
            .map(|f| *f)
            .unwrap_or_default();

          let new_val = counteract_axis
            .and_then(|axis| edge.perpendicular(&axis).as_some(()))
            .map(|_should_counteract| -neighbor)
            .unwrap_or(neighbor);

          (new_val, val_mut)
        })
        .for_each(|(new_val, val_mut)| {
          *val_mut = new_val;
        });
    }
  }

  // TODO: FixCorners trait?
  pub fn fix_corners(grid: &mut Grid<f64>) {
    Corner::all().iter().for_each(|corner| {
      let corner = *corner;
      grid
        .get_corner(corner)
        .map(|(coords, _)| grid.get_adjacent_neighbors(coords))
        .map(|neighbors| {
          let neighbor_sum =
            neighbors.map(|(_, val)| val).fold(0.0, Add::add);

          neighbor_sum / 2.0
        })
        .map(|new_val| grid.set_corner(corner, new_val));
    });
  }
}
