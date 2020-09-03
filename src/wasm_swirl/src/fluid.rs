use std::ops::{Add, Div, Mul};

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

  pub fn tick(&mut self) {
    self.diffuse_velocity();
    self.project_velocity();

    self.advect_velocity();
    self.project_velocity();

    self.diffuse_density();
    self.advect_density();
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

  pub fn diffuse_velocity(&mut self) {
    Self::diffuse(
      self.diff,
      self.dt,
      &mut self.velocity,
      &mut self.velocity_prev,
    )
  }

  pub fn diffuse_density(&mut self) {
    Self::diffuse(
      self.diff,
      self.dt,
      &mut self.density,
      &mut self.density_prev,
    )
  }

  pub fn diffuse<T>(
    diff: Diffusion,
    dt: TimeDelta,
    grid: &mut Grid<T>,
    grid_prev: &Grid<T>,
  ) -> ()
  where
    T: Default
      + Copy
      + ValueAtEdge
      + Add<T, Output = T>
      + Mul<f64, Output = T>
      + Div<f64, Output = T>,
  {
    let a = &dt as &f64
      * diff.0
      * (N - 2) as f64
      * (N - 2) as f64;

    Self::lin_solve(grid, grid_prev, a, 1.0 + (4.0 * a));
  }

  pub fn lin_solve<'a, T>(
    grid: &mut Grid<T>,
    grid_prev: &Grid<T>,
    a: f64,
    c: f64,
  ) where
    T: Default
      + Copy
      + ValueAtEdge
      + Add<T, Output = T>
      + Mul<f64, Output = T>
      + Div<f64, Output = T>,
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

  pub fn set_bnd<T>(grid: &mut Grid<T>)
  where
    T: Default
      + Copy
      + ValueAtEdge
      + Add<T, Output = T>
      + Div<f64, Output = T>,
  {
    Self::fix_corners(grid);
    Self::fix_edges(grid);
  }

  // TODO: FixEdges trait?
  pub fn fix_edges<T: Default + ValueAtEdge + Copy>(
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
    T: Default
      + Copy
      + Add<T, Output = T>
      + Div<f64, Output = T>,
  {
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

  pub fn project_velocity(&mut self) {
    Self::project(
      &mut self.velocity,
      &mut self.velocity_prev,
    )
  }
  pub fn project(
    vel: &mut Grid<Vector2>,
    other: &mut Grid<Vector2>,
  ) {
    use std::iter::repeat;

    other
      .dimensions
      .coords_iter()
      .map(|coord| (coord, coord.neighbors()))
      .map(|(coord, neighbors)| {
        (coord, neighbors.map(|coord| vel.get(coord)))
      })
      .map(|(coord, neighbors)| {
        (
          coord,
          neighbors.map(|vel| {
            vel.map(|vel| *vel).unwrap_or_default()
          }),
        )
      })
      .map(|(coord, neighbor_vals)| {
        let new_val = -(neighbor_vals.left.x
          - neighbor_vals.right.x
          + neighbor_vals.top.y
          - neighbor_vals.bottom.y)
          / (N as f64 * 2.0);
        (coord, new_val)
      })
      .for_each(|(coord, computed_val)| {
        let new_val = Vector2 {
          x: 0.0,
          y: computed_val,
        };

        other.set(coord, new_val);
      });

    Self::set_bnd(other);

    let other_ptr: *mut _ = other;
    let other_ref = unsafe { other_ptr.as_ref().unwrap() };
    let other_mut = unsafe { other_ptr.as_mut().unwrap() };

    repeat(()).take(ITER).for_each(|_| {
      vel
        .dimensions
        .coords_iter()
        .filter_map(|coords| {
          coords
            .neighbors()
            .map(|coords| other_ref.get(coords).copied())
            .opt()
            .map(|neighbors| (coords, neighbors))
        })
        .map(|(c, ns)| (c, ns.map(|v| v.x)))
        .map(|(c, neighbors)| (c, neighbors.sum()))
        .map(|(c, neighbor_sum)| {
          (
            c,
            other.get(c).map(|t| t.y).unwrap_or_default()
              + neighbor_sum,
          )
        })
        .map(|(c, new_x)| (c, new_x / 4.0))
        .for_each(|(coords, new_x)| {
          let y = other_ref
            .get(coords)
            .map(|v| f64::from(v.y))
            .unwrap_or_default();
          other_mut.set(coords, (new_x, y));
        })
    });

    Self::set_bnd(other);

    let (width, height) =
      (other.dimensions.width, other.dimensions.height);
    let (width, height) = (width as f64, height as f64);

    other
      .dimensions
      .coords_iter()
      .map(|coord| (coord, coord.neighbors()))
      .filter_map(|(coord, neighbors)| {
        neighbors
          .map(|coord| other.get(coord))
          .opt()
          .map(|neighbors| (coord, neighbors))
      })
      .map(|(coord, neighbors)| {
        (coord, neighbors.map(|v| v.x))
      })
      .map(|(coord, neighbors)| {
        (
          coord,
          neighbors.right - neighbors.left,
          neighbors.top - neighbors.bottom,
        )
      })
      .map(|(coord, x_diff, y_diff)| {
        (coord, x_diff * width / 2.0, y_diff * height / 2.0)
      })
      .for_each(|(coord, x_sub, y_sub)| {
        let velocity =
          vel.get(coord).map(|v| *v).unwrap_or_default();
        vel.set(
          coord,
          (velocity.x - x_sub, velocity.y - y_sub),
        );
      });
    Self::set_bnd(vel);
  }

  pub fn advect_velocity(&mut self) {
    Self::advect(
      self.dt,
      &mut self.velocity,
      &self.velocity_prev,
      &self.velocity_prev,
    )
  }

  pub fn advect_density(&mut self) {
    Self::advect(
      self.dt,
      &mut self.density,
      &self.density_prev,
      &self.velocity_prev,
    )
  }

  pub fn advect<T>(
    dt: TimeDelta,
    grid: &mut Grid<T>,
    grid_prev: &Grid<T>,
    vel_prev: &Grid<Vector2>,
  ) where
    T: std::ops::Add<Output = T>
      + std::ops::Mul<f64, Output = T>
      + Default
      + Copy,
  {
    use crate::math::Clamp;
    // Rather than advecting quantities by computing where a particle moves over the current time step,
    // we trace the trajectory of the particle from each grid cell back in time to its former position,
    // and we copy the quantities at that position to the starting grid cell. [NVIDIA GPU Gems, Ch. 38 #Advection]

    // Get the distance in the grid to travel based on
    // the amount of time since last run
    let time_step = dt.0 * N as f64;

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
      | Left | Right => Self {
        x: -neighbor.x,
        y: neighbor.y,
      },
      | Top | Bottom => Self {
        x: neighbor.x,
        y: -neighbor.y,
      },
    }
  }
}

impl ValueAtEdge for Density {
  fn value_at_edge(edge: Edge, neighbor: Self) -> Self {
    neighbor
  }
}
