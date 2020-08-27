use std::ops::Add;

use crate::constant::{ITER, N};
use crate::convert;
use crate::space::coords::{Axis, Coord2, Corner, Edge, Perpendicular, Rect};
use crate::space::matrix::Matrix2;
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
      density: Matrix2::<Density>::from_dimensions(dims),
      density_prev: Matrix2::<Density>::from_dimensions(dims),
      velocity: Matrix2::<Vector2>::from_dimensions(dims),
      velocity_prev: Matrix2::<Vector2>::from_dimensions(dims),
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
    x: &Matrix2<f64>,
    x_prev: &Matrix2<f64>,
    diffusion: f64,
    dt: TimeDelta,
    counteract_axis: Option<Axis>,
  ) -> () {
    let a = &dt as &f64 * diffusion * (N - 2) as f64 * (N - 2) as f64;
    Self::lin_solve(x, x_prev, a, 0.0, counteract_axis);
  }

  pub fn lin_solve(
    floats: &Matrix2<f64>,
    floats_prev: &Matrix2<f64>,
    a: f64,
    c: f64,
    counteract_axis: Option<Axis>,
  ) {
    std::iter::repeat(()).take(ITER).for_each(|_| {
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

          let next_val = (prev_val + neighbor_influence) / c;
          (coords, next_val)
        })
        .for_each(|(coords, next_val)| {
          floats.set(coords, next_val);
        })
    });

    Self::set_bnd(floats, counteract_axis)
  }

  pub fn set_bnd(floats: &Matrix2<f64>, counteract_axis: Option<Axis>) {
    Self::fix_corners(floats);
    Self::fix_edges(floats, counteract_axis);
  }

  // TODO: FixEdges trait?
  pub fn fix_edges(floats: &Matrix2<f64>, counteract_axis: Option<Axis>) {
    floats
      .get_edges()
      // filter out corners
      .filter_map(|(edge_info, coords)| edge_info.get_edge().map(|edge| (edge, coords)))
      // grab the point next to it; in the following crude
      // drawing, if we're looking at point `a`, we want the
      // coords of point `b`.
      /*
         ______
        |      |
        |ab    |
        |      |
         ------
      */
      .map(|(edge, coords)| {
        let neighbor_coords = match edge {
          Edge::Top => (coords.x, coords.y + 1),
          Edge::Bottom => (coords.x, coords.y - 1),
          Edge::Left => (coords.x + 1, coords.y),
          Edge::Right => (coords.x - 1, coords.y),
        };

        (edge, Coord2::from(neighbor_coords), coords)
      })
      // read the neighbor's value
      .map(|(edge, neighbor_coords, coords)| {
        (edge, floats.get(neighbor_coords).unwrap().1, coords)
      })
      // get the next value for the boundary cell
      .map(|(edge, neighbor, coords)| {
        let new_val =
        counteract_axis
          .and_then(|axis| edge.perpendicular(&axis).as_some(()))
          .map(|_should_counteract| -neighbor)
          .unwrap_or(*neighbor);
        (new_val, coords)
      })
      // update the boundary cell
      .for_each(|(new_val, coords)| {
        floats.set(coords, new_val);
      })
  }

  // TODO: FixCorners trait?
  pub fn fix_corners(floats: &Matrix2<f64>) {
    Corner::all().iter().for_each(|corner| {
      let corner = *corner;
      Some(floats.get_corner(corner))
        .map(|(coords, _)| floats.get_adjacent_neighbors(coords))
        .map(|neighbors| {
          let neighbor_sum = neighbors
            .iter()
            .filter_map(|opt| opt.map(|(_, val)| val))
            .fold(0.0, Add::add);

          neighbor_sum / 2.0
        })
        .map(|new_val| floats.set_corner(corner, new_val))
        .unwrap()
    });
  }
}
