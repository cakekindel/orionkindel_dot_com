use std::ops::{Add, Mul, Div};

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
impl Add for Density {
  type Output = Self;
  fn add(self, other: Self) -> Self::Output {
    Self(self.0 + other.0)
  }
}
impl Mul<f64> for Density {
  type Output = Self;
  fn mul(self, other: f64) -> Self::Output {
    Self(self.0 * other)
  }
}
impl Div<f64> for Density {
  type Output = Self;
  fn div(self, other: f64) -> Self::Output {
    Self(self.0 / other)
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

  pub fn tick(&self) {
    self.diffuse(&mut self.velocity, &self.velocity_prev);
    // Self::project();
    self.advect(&mut self.velocity, &self.velocity_prev, &self.velocity_prev);
    //Self::project(Vx, Vy, Vz, Vx0, Vy0, 4, N);
    self.diffuse(&mut self.density, &self.density_prev);
    self.advect(&mut self.density, &self.density_prev, &self.velocity_prev);
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

  pub fn diffuse<T>(
    &self,
    grid: &mut Grid<T>,
    grid_prev: &Grid<T>,
  ) -> ()
  where T: Default + Copy + ValueAtEdge + Add<T, Output = T> + Mul<f64, Output = T> + Div<f64, Output = T>
  {
    let a = &self.dt as &f64
      * self.diff.0
      * (N - 2) as f64
      * (N - 2) as f64;

    Self::lin_solve(
      grid,
      grid_prev,
      a,
      1.0 + (4.0 * a),
    );
  }

  pub fn lin_solve<'a, T>(
    grid: &mut Grid<T>,
    grid_prev: &Grid<T>,
    a: f64,
    c: f64,
  )
  where T: Default + Copy + ValueAtEdge + Add<T, Output = T> + Mul<f64, Output = T> + Div<f64, Output = T>
{
    std::iter::repeat(()).take(ITER).for_each(|_| {
      grid
        .iter_mut()
        .map(|(coords, val_mut)| {
          let neighbor_sum: T = grid_prev
            .get_adjacent_neighbors(coords)
            .map(|(_, val)| *val)
            .fold(Default::default(), Add::add);

          let neighbor_influence = neighbor_sum * a;

          let prev_val = grid_prev.get(coords).expect(
            "floats and floats_prev were different sizes!!",
          );

          let next_val =
            (*prev_val + neighbor_influence) / c;
          (val_mut, next_val)
        })
        .for_each(|(val_mut, next_val)| {
          *val_mut = next_val;
        })
    });

    Self::set_bnd(grid)
  }

  pub fn set_bnd<T>(
    grid: &mut Grid<T>,
  )
  where T: Default + Copy + ValueAtEdge + Add<T, Output = T> + Div<f64, Output = T>
  {
    Self::fix_corners(grid);
    Self::fix_edges(grid);
  }

  // TODO: FixEdges trait?
  pub fn fix_edges<T: Default + ValueAtEdge>(
    grid: &mut Grid<T>,
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

        let new_val = T::value_at_edge(edge, neighbor);

        (new_val, val_mut)
      })
      .for_each(|(new_val, val_mut)| {
        *val_mut = new_val;
      });
  }

  pub fn fix_corners<T>(grid: &mut Grid<T>)
  where
    T: Default + Copy + Add<T, Output = T> + Div<f64, Output = T> {
    Corner::all().iter().for_each(|corner| {
      let corner = *corner;
      grid
        .get_corner(corner)
        .map(|(coords, _)| {
          grid.get_adjacent_neighbors(coords)
        })
        .map(|neighbors| {
          let neighbor_sum: T = neighbors
            .map(|(_, val)| *val)
            .fold(Default::default(), Add::add);

          neighbor_sum / 2.0
        })
        .map(|new_val| grid.set_corner(corner, new_val));
    });
  }

  pub fn project<T>(&self, vel: &Grid<Vector2>, grid_a: &mut Grid<T>, grid_b: &mut Grid<T>)
  where T: Default + Copy + ValueAtEdge + Add<T, Output = T> + Div<f64, Output = T> {
    use std::iter::repeat;

    grid_a.dimensions.coords_iter()
      .map(|coord| (coord, coord.neighbors()))
      .map(|(coord, neighbors)| (coord, neighbors.map(|coord| vel.get(coord))))
      .map(|(coord, neighbor_velocities)| {
        let new_velocity = -(neighbor_velocities.left - neighbor_velocities.right + neighbor_velocities.top - neighbor_velocities.bottom) / (N * 2);
        (coord, new_velocity)
      })
      .for_each(|(coord, new_velocity)| {
        grid_b.set(coord, new_velocity);
        grid_a.set(coord, Default::default());
      });

    Self::set_bnd(grid_a);
    Self::set_bnd(grid_b);

    repeat(()).take(ITER).for_each(|_| {
      grid_a.dimensions
        .coords_iter()
        .map(|coords| (coords, coords.neighbors().map(|coords| grid_a.get(coords))))
        .map(|(coords, neighbors)| (coords, neighbors.sum()))
        .map(|(coords, neighbor_sum)| (coords, grid_b.get(coords).map(|t| *t).unwrap_or_default() + neighbor_sum))
        .map(|(coords, new_val)| (coords, new_val / 4.0))
        .for_each(|(coords, new_val)| {grid_a.set(coords, new_val);})
    });

    Self::set_bnd(grid_a);

    let (width, height) = (grid_a.dimensions.width, grid_a.dimensions.height);

      grid_a.dimensions
        .coords_iter()
        .map(|coord| (coord, coord.neighbors()))
        .map(|(coord, neighbors)| (coord, neighbors.map(|coord| grid_a.get(coord))))
        .map(|(coord, neighbors)| (coord, neighbors.right - neighbors.left, neighbors.top - neighbors.bottom))
        .map(|(coord, x_diff, y_diff)| (coord, x_diff * width / 2, y_diff * height / 2))
        .for_each(|(coord, x_sub, y_sub)| {
          let velocity = vel.get(coord).map(|v| *v).unwrap_or_default();
          vel.set(coord, (velocity.x - x_sub, velocity.y - y_sub));
        });
    Self::set_bnd(&mut vel);
  }

  pub fn advect<T>(
    &mut self,
    grid: &mut Grid<T>,
    grid_prev: &Grid<T>,
    vel_prev: &Grid<Vector2>,
  )
where
  T: std::ops::Add<Output = T>
    + std::ops::Mul<f64, Output = T>
    + Default
    + Copy
    {    use crate::math::Clamp;
    // Rather than advecting quantities by computing where a particle moves over the current time step,
    // we trace the trajectory of the particle from each grid cell back in time to its former position,
    // and we copy the quantities at that position to the starting grid cell. [NVIDIA GPU Gems, Ch. 38 #Advection]

    // Get the distance in the grid to travel based on
    // the amount of time since last run
    let time_step = self.dt.0 * N as f64;

    // iterate over the coords in the center, excluding
    // edges and corners
    let max_x = grid.dimensions.width - 2;
    let max_y = grid.dimensions.height - 2;

    for x in 1..=max_x {
      for y in 1..=max_y {
        let vel: Vector2 = *vel_prev.get((x, y)).unwrap();

        let distance_travelled_in_time_step: Vector2 =
          (vel.x * time_step, vel.y * time_step).into();

        let trace_back: Coord2<f64> = (
          (x as f64) - distance_travelled_in_time_step.x,
          (y as f64) - distance_travelled_in_time_step.y,
        )
          .into();

        // clamp to 0.5 away from edges
        let trace_back: Coord2<f64> = (
          trace_back.x.clam(0.5, (max_x as f64) - 0.5),
          trace_back.y.clam(0.5, (max_y as f64) - 0.5),
        )
          .into();

        let new_val = grid_prev.interpolate(trace_back);

        grid.set((x, y), new_val);
      }
    }
  }
}

pub trait ValueAtEdge {
  fn value_at_edge(edge: Edge, neighbor: Self) -> Self;
}

impl ValueAtEdge for Vector2 {
  fn value_at_edge(edge: Edge, neighbor: Self) -> Self {
    use Edge::*;
    match edge {
      Left | Right => Self {
        x: -neighbor.x,
        y: neighbor.y,
      },
      Top | Bottom => Self {
        x: neighbor.x,
        y: -neighbor.y,
      }
    }
  }
}

impl ValueAtEdge for Density {
  fn value_at_edge(edge: Edge, neighbor: Self) -> Self {
    neighbor
  }
}
