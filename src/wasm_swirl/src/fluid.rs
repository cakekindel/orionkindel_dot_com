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
  /// Scalar that affects how quickly the fluid will diffuse
  /// and affect the density of neighboring points
  diff: Diffusion,

  /// Not sure yet
  viscosity: Viscosity,

  // TODO: allow this to vary based on frametime
  /// Scalar that regulates the amount of change we perform
  /// in a tick
  dt: TimeDelta,

  // TODO: can this be replaced with Grid.dimensions.width, ...height, and ...area()?
  /// Total size of the grids
  size: usize,

  /// Current density of fluids
  density: Grid<Density>,

  /// Density of fluids from the previous iteration
  density_prev: Grid<Density>,

  /// Current Velocity of fluids
  velocity: Grid<Vector2>,

  /// Velocity of fluids from the previous iteration
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

  pub fn add_dye(
    &mut self,
    pt: Coord2,
    amount: f64,
  ) -> Option<()> {
    self
      .density
      .get(pt)
      .map(|dens| *dens)
      .map(|density| {
        self.density.set(pt, density + amount.into())
      })
      .map(|_| ())
  }

  pub fn add_velocity(
    &mut self,
    pt: Coord2,
    amount: Vector2,
  ) -> Option<()> {
    self.velocity.get(pt).map(|vector| *vector).map(
      |velocity| {
        self.velocity.set(pt, velocity + amount);
      },
    )
  }

  pub fn diffuse(
    grid: &mut Grid<f64>,
    grid_prev: &Grid<f64>,
    diffusion: f64,
    dt: TimeDelta,
    counteract_axis: Option<Axis>,
  ) -> () {
    let a = &dt as &f64
      * diffusion
      * (N - 2) as f64
      * (N - 2) as f64;
    Self::lin_solve(
      grid,
      grid_prev,
      a,
      1.0 + (4.0 * a),
      counteract_axis,
    );
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

          let prev_val = grid_prev.get(coords).expect(
            "floats and floats_prev were different sizes!!",
          );

          let next_val =
            (prev_val + neighbor_influence) / c;
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
    let grid_ptr: *mut _ = grid;

    // We need to bypass the borrow checker because
    // we need to mutably borrow it (to iterate) and
    // immutably borrow it (to read neighbors).

    // The chief risk would be a use-after-free or race
    // condition, but since this loop doesn't free anything
    // and we are in a single-threaded environment, this
    // should be safe
    let grid_mut = unsafe { grid_ptr.as_mut().unwrap() };
    let grid = unsafe { grid_ptr.as_ref().unwrap() };

    grid_mut
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
          .get(neighbor_coords)
          .map(|f| *f)
          .unwrap_or_default();

        let new_val = counteract_axis
          .and_then(|axis| {
            edge.perpendicular(&axis).as_some(())
          })
          .map(|_should_counteract| -neighbor)
          .unwrap_or(neighbor);

        (new_val, val_mut)
      })
      .for_each(|(new_val, val_mut)| {
        *val_mut = new_val;
      });
  }

  // TODO: FixCorners trait?
  pub fn fix_corners(grid: &mut Grid<f64>) {
    Corner::all().iter().for_each(|corner| {
      let corner = *corner;
      grid
        .get_corner(corner)
        .map(|(coords, _)| {
          grid.get_adjacent_neighbors(coords)
        })
        .map(|neighbors| {
          let neighbor_sum = neighbors
            .map(|(_, val)| val)
            .fold(0.0, Add::add);

          neighbor_sum / 2.0
        })
        .map(|new_val| grid.set_corner(corner, new_val));
    });
  }

  pub fn advect(
    n: usize,
    b: i32,
    d: &mut Grid<f64>,
    d0: &mut Grid<f64>,
    vel_x: &mut Grid<f64>,
    vel_y: &mut Grid<f64>,
    dt: f64,
  ) {
    use crate::math::Clamp;
    // Rather than advecting quantities by computing where a particle moves over the current time step,
    // we trace the trajectory of the particle from each grid cell back in time to its former position,
    // and we copy the quantities at that position to the starting grid cell. [NVIDIA GPU Gems, Ch. 38 #Advection]

    let max_x = d.dimensions.width as f64 - 1.0;
    let max_y = d.dimensions.height as f64 - 1.0;

    let mut i0: usize;
    let mut j0: usize;
    let mut i1: usize;
    let mut j1: usize;

    let mut x_time_back: f64;
    let mut y_time_back: f64;
    let mut s0: f64;
    let mut t0: f64;
    let mut s1: f64;
    let mut t1: f64;

    let time_step = dt * N as f64;
    for x in 1..=N {
      for y in 1..N {
        let vel = (
          *vel_x.get((x, y)).unwrap(),
          *vel_y.get((x, y)).unwrap(),
        );

        let distance_travelled_in_time_step: Vector2 =
          (vel.0 * time_step, vel.1 * time_step).into();

        let trace_back: Coord2<f64> = (
          (x as f64) - distance_travelled_in_time_step.x,
          (y as f64) - distance_travelled_in_time_step.y,
        )
          .into();

        // clamp to 0.5 away from edges because
        // `interpolate` fails on edges
        let trace_back: Coord2<f64> = (
          trace_back.x.clam(0.5, max_x - 0.5),
          trace_back.y.clam(0.5, max_y - 0.5),
        )
          .into();

        let new_val = d0.interpolate(trace_back);

        d.set((x, y), new_val);
      }
    }
  }
}
