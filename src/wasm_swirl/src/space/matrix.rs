use std::collections::HashMap;

use super::coords::{
  Coord2, Corner, EdgeInfo, Edged, Rect,
};

/// # 2-dimensional Grid
/// This grid structure is used to represent
/// a rectangular coordinate space of values of type `T`.
///
/// The coordinate space has an origin of (0, 0)
/// and can be viewed as Q1 of a cartesian grid
///
/// # Example
/// ```
/// # use wasm_swirl::space::matrix::Grid;
/// # use wasm_swirl::space::coords::{Rect};
/// let dimensions = Rect { width: 10, height: 10, origin: (0, 0).into() };
/// let grid = Grid::<i32>::from_dimensions(dimensions);
///
/// // ___________________
/// // |  |  |  |  |  |  | y = grid.dimensions.height - 1 = 9
/// // -------------------
/// // |  |  |  |  |  |  | y = grid.dimensions.height - 2 = 8
/// // -------------------
/// // |  |  |  |  |  |  |
/// // -------------------
/// // |  |  |  |  |  |  |
/// // -------------------
/// // |  |  |  |  |  |  | y = 1
/// // -------------------
/// // |  |  |  |  |  |  | y = 0
/// // -------------------
/// //  ^  ^           ^
/// // x=0 x=1         x = grid.dimensions.width - 1 = 9
/// ```
pub struct Grid<T> {
  pub dimensions: Rect,
  items: HashMap<Coord2, T>,
}

impl<T> Grid<T>
where
  T: Default,
{
  /// Fill the grid with the default value for `T`
  pub fn fill_with_default(mut self) -> Self {
    self.fill(Default::default)
  }
}

impl<'a, T> Grid<T>
where
  T: std::ops::Add<Output = T>
    + std::ops::Mul<f64, Output = T>
    + Default
    + Copy,
{
  /// Get the combined value of a point that sits
  /// between 4 other points. Uses a crude bilinear
  /// interpolation algorithm
  ///
  /// Note that this will return the default for `T`
  /// if given an edge point.
  pub fn interpolate(&'a self, pt: (f64, f64)) -> T {
    //! # Example
    //! ```
    //! # use wasm_swirl::space::matrix::{Grid};
    //! # use wasm_swirl::space::coords::{Rect};
    //! let mut grid: Grid<f64> = Grid::from_dimensions(Rect { width: 2, height: 2, origin: (0, 0).into() });
    //! grid.set((0, 0), 1.0);
    //! grid.set((0, 1), 1.0);
    //! grid.set((1, 0), 0.5);
    //! grid.set((1, 1), 0.5);
    //!
    //! let middle = grid.interpolate((0.5, 0.5));
    //! assert_eq!(middle, 0.75);
    //!
    //! // interesting behavior alert!!
    //! // this will return 0.0, because top is on
    //! // an edge!
    //! let top = grid.interpolate((0.5, 1.0));
    //! assert_eq!(top, 0.0);
    //!
    //! let somewhere_in_the_middle = grid.interpolate((0.6, 0.21));
    //! assert_eq!(somewhere_in_the_middle, 0.8);
    //! ```

    let floored: Coord2 =
      (pt.0.floor() as usize, pt.1.floor() as usize).into();

    let x_left = floored.x;
    let y_bot = floored.y;
    let x_right = floored.x + 1;
    let y_top = floored.y + 1;

    let left_weight = pt.0 - x_left as f64;
    let right_weight = 1.0 - left_weight;
    let bot_weight = pt.1 - y_bot as f64;
    let top_weight = 1.0 - bot_weight;

    let bottom_left_weight = self
      .get((x_left, y_bot))
      .map(|val| *val * left_weight * bot_weight)
      .unwrap_or_default();

    let top_left_weight = self
      .get((x_left, y_top))
      .map(|val| *val * left_weight * top_weight)
      .unwrap_or_default();

    let bottom_right_weight = self
      .get((x_right, y_bot))
      .map(|val| *val * right_weight * bot_weight)
      .unwrap_or_default();

    let top_right_weight = self
      .get((x_right, y_top))
      .map(|val| *val * right_weight * top_weight)
      .unwrap_or_default();

    bottom_left_weight
      + top_left_weight
      + bottom_right_weight
      + top_right_weight
  }
}
impl<T> Grid<T>
where
  T: Copy,
{
  /// Fill the grid with a default value
  pub fn fill_with(mut self, val: T) -> Self {
    self.dimensions.coords_iter().for_each(|coords| {
      self.items.insert(coords, val);
    });
    self
  }
}

impl<T> Grid<T> {
  /// Create a grid of specified dimensions
  pub fn from_dimensions(dims: Rect) -> Self {
    Self {
      items: HashMap::with_capacity(dims.area()),
      dimensions: dims,
    }
  }

  /// Fill the grid with the result of a function
  ///
  /// ## Related Methods
  /// For `T` that implements `Copy`, you can fill the
  /// grid by copying a particular value with `fill_with`
  ///
  /// For `T` that implements `Default`, you can fill the
  /// grid with the default value for `T` with `fill_with_default`
  pub fn fill<F>(mut self, f: F) -> Self
  where
    F: Fn() -> T,
  {
    self.dimensions.coords_iter().for_each(|coords| {
      self.items.insert(coords, f());
    });
    self
  }

  /// Returns an iterator over the corner and edge points of the grid.
  /// The tuple being iterated over contains:
  /// - whether the point is on an edge or corner (2 edges)
  /// - the coordinates of the point
  /// - the value of the point
  pub fn edges_iter<'a>(
    &'a self,
  ) -> impl Iterator<Item = (EdgeInfo, Coord2, &T)> + 'a {
    //! # Example
    //! ```
    //! # use wasm_swirl::space::matrix::Grid;
    //! # use wasm_swirl::space::coords::{Rect, EdgeInfo::*, Edge::*};
    //! let dims = Rect { width: 3, height: 3, origin: (0,0).into() };
    //! let mut grid = Grid::<i32>::from_dimensions(dims);
    //! grid.fill_with_default();
    //!
    //! let mut edges: Vec<_> = grid
    //!   .edges_iter()
    //!   .map(|(edge, coords, val)| (edge, coords.to_pair(), val))
    //!   .collect();
    //!
    //! edges.sort_by(|a, b| {
    //!   // sort by corners, then by edge, then by coordinate
    //!   # if a.0 == b.0 { a.1.cmp(&b.1) }
    //!   # else { a.0.cmp(&b.0) }
    //! });
    //!
    //! assert_eq!(
    //!   edges
    //!     .iter()
    //!     .map(|(a, b, c)| (*a, *b, **c))
    //!     .collect::<Vec<_>>(),
    //!   vec![
    //!     (Corner(Top, Left), (0, 2), 0),
    //!     (Corner(Top, Right), (2, 2), 0),
    //!     (Corner(Bottom, Left), (0, 0), 0),
    //!     (Corner(Bottom, Right), (2, 0), 0),
    //!     (Edge(Top), (1, 2), 0),
    //!     (Edge(Bottom), (1, 0), 0),
    //!     (Edge(Left), (0, 1), 0),
    //!     (Edge(Right), (2, 1), 0),
    //!   ]
    //! );
    //!
    //! assert_eq!(edges.len(), 8);
    //! ```

    self.dimensions.edge_coord_iter().filter_map(
      move |(edge_info, coords)| {
        self.get(coords).map(|val| (edge_info, coords, val))
      },
    )
  }

  /// Returns an iterator over mutable references to corner and edge points of the grid
  /// The tuple being iterated over contains:
  /// - whether the point is on an edge or corner (2 edges)
  /// - the coordinates of the point
  /// - the value of the point
  pub fn edges_iter_mut<'a>(
    &'a mut self,
  ) -> impl Iterator<Item = (EdgeInfo, Coord2, &'a mut T)> + 'a
  {
    //! # Example
    //! ```
    //! # use wasm_swirl::space::matrix::Grid;
    //! # use wasm_swirl::space::coords::{Rect, EdgeInfo::*, Edge::*};
    //! let dims = Rect { width: 3, height: 3, origin: (0,0).into() };
    //! let mut grid = Grid::<i32>::from_dimensions(dims);
    //! grid.fill_with_default();
    //!
    //! let mut edges: Vec<_> = grid
    //!   .edges_iter_mut()
    //!   .map(|(edge, coords, val)| (edge, coords.to_pair(), val))
    //!   .collect();
    //!
    //! edges.sort_by(|a, b| {
    //!   // sort by corners, then by edge, then by coordinate
    //!   # if a.0 == b.0 { a.1.cmp(&b.1) }
    //!   # else { a.0.cmp(&b.0) }
    //! });
    //!
    //! assert_eq!(
    //!   edges
    //!     .iter()
    //!     .map(|(a, b, c)| (*a, *b, **c))
    //!     .collect::<Vec<_>>(),
    //!   vec![
    //!     (Corner(Top, Left), (0, 2), 0),
    //!     (Corner(Top, Right), (2, 2), 0),
    //!     (Corner(Bottom, Left), (0, 0), 0),
    //!     (Corner(Bottom, Right), (2, 0), 0),
    //!     (Edge(Top), (1, 2), 0),
    //!     (Edge(Bottom), (1, 0), 0),
    //!     (Edge(Left), (0, 1), 0),
    //!     (Edge(Right), (2, 1), 0),
    //!   ]
    //! );
    //!
    //! *edges[0].2 = 12;
    //!
    //! assert_eq!(
    //!   edges
    //!     .iter()
    //!     .map(|(a, b, c)| (*a, *b, **c))
    //!     .collect::<Vec<_>>(),
    //!   vec![
    //!     (Corner(Top, Left), (0, 2), 12),
    //!     // ...
    //!     # (Corner(Top, Right), (2, 2), 0),
    //!     # (Corner(Bottom, Left), (0, 0), 0),
    //!     # (Corner(Bottom, Right), (2, 0), 0),
    //!     # (Edge(Top), (1, 2), 0),
    //!     # (Edge(Bottom), (1, 0), 0),
    //!     # (Edge(Left), (0, 1), 0),
    //!     # (Edge(Right), (2, 1), 0),
    //!   ]
    //! );
    //!
    //! assert_eq!(edges.len(), 8);
    //! ```

    // `edge_coord_iter` should not ever yield the same
    // coords more than once, which makes issuing mutable references
    // to items in the map via this ptr OK
    let map_ptr: *mut _ = &mut self.items;

    self.dimensions.edge_coord_iter().filter_map(
      move |(edge_info, coords)| {
        let map = unsafe { map_ptr.as_mut().unwrap() };
        map
          .get_mut(&coords)
          .map(|val| (edge_info, coords, val))
      },
    )
  }


  /// Get the points touching a point at given coordinates
  pub fn get_adjacent_neighbors(
    &self,
    coords: impl Into<Coord2>,
  ) -> impl Iterator<Item = (Coord2, &T)> {
    //! # Example
    //! ```
    //! # use wasm_swirl::space::matrix::Grid;
    //! # use wasm_swirl::space::coords::{Rect};
    //! let dims = Rect { width: 3, height: 3, origin: (0,0).into() };
    //! let mut grid = Grid::<i32>::from_dimensions(dims);
    //!
    //! grid.set((0, 1), 0);
    //! grid.set((2, 1), 1);
    //! grid.set((1, 0), 2);
    //! grid.set((1, 2), 3);
    //!
    //! // -------------
    //! // |   | 2 |   |
    //! // -------------
    //! // | 0 | x | 1 |
    //! // -------------
    //! // |   | 3 |   |
    //! // -------------
    //!
    //! let x = (1, 1);
    //! let mut neighbors: Vec<i32> = grid.get_adjacent_neighbors(x)
    //!   .map(|(_, i): (_, &i32)| *i)
    //!   .collect();
    //!
    //! neighbors.sort();
    //!
    //! let expected: Vec<i32> = vec![0, 1, 2, 3];
    //! assert_eq!(neighbors, expected);
    //! ```
    use std::iter::once;

    let coords = coords.into();

    once((coords.x.checked_sub(1).unwrap_or(0), coords.y))
      .chain(once((coords.x + 1, coords.y)))
      .chain(once((coords.x, coords.y.checked_sub(1).unwrap_or(0))))
      .chain(once((coords.x, coords.y + 1)))
      .map(move |coords| {
        self.get(coords).map(|val| (coords.into(), val))
      })
      .filter_map(|opt| opt)
  }

  /// Get the point at a given corner of the grid
  pub fn get_corner(
    &self,
    corner: Corner,
  ) -> Option<(Coord2, &T)> {
    //! # Example
    //! ```
    //! # use wasm_swirl::space::matrix::Grid;
    //! # use wasm_swirl::space::coords::{Rect, Corner, Edge::*};
    //! let dims = Rect { width: 3, height: 3, origin: (0,0).into() };
    //! let mut grid = Grid::<i32>::from_dimensions(dims);
    //!
    //! grid.set_corner(Corner(Bottom, Left), 12);
    //!
    //! assert!(matches!(grid.get_corner(Corner(Bottom, Left)), Some((_, 12))));
    //! ```

    let coords = self.coords_of_corner(corner);

    self.get(coords).map(|val| (coords, val))
  }

  /// Update the value at a given corner, and returns an Option
  /// containing the old value, if there was one.
  pub fn set_corner(
    &mut self,
    corner: Corner,
    val: T,
  ) -> Option<T> {
    //! # Example
    //! ```
    //! # use wasm_swirl::space::matrix::Grid;
    //! # use wasm_swirl::space::coords::{Rect, Corner, Edge::*};
    //! let dims = Rect { width: 3, height: 3, origin: (0,0).into() };
    //! let mut grid = Grid::<i32>::from_dimensions(dims);
    //!
    //! grid.set_corner(Corner(Bottom, Left), 12);
    //!
    //! assert!(matches!(grid.get_corner(Corner(Bottom, Left)), Some((_, 12))));
    //! ```

    self.set(self.coords_of_corner(corner), val)
  }

  /// Get the value of a point at the given coordinates
  pub fn get(
    &self,
    coords: impl Into<Coord2>,
  ) -> Option<&T> {
    //! # Example
    //! ```
    //! # use wasm_swirl::space::matrix::Grid;
    //! # use wasm_swirl::space::coords::{Rect, Corner, Edge::*};
    //! let dims = Rect { width: 3, height: 3, origin: (0,0).into() };
    //! let grid = Grid::<i32>::from_dimensions(dims);
    //!
    //! assert!(matches!(grid.get((0, 0)), None));
    //! ```

    let coords = coords.into();

    self.items.get(&coords)
  }

  /// Get a mutable reference of the value at the given coordinates
  pub fn get_mut<'a>(
    &'a mut self,
    coords: impl Into<Coord2>,
  ) -> Option<&'a mut T> {
    //! # Example
    //! ```
    //! # use wasm_swirl::space::matrix::Grid;
    //! # use wasm_swirl::space::coords::{Rect, Corner, Edge::*};
    //! let dims = Rect { width: 3, height: 3, origin: (0,0).into() };
    //! let grid = Grid::<i32>::from_dimensions(dims);
    //!
    //! assert!(matches!(grid.get((0, 0)), None));
    //! ```

    let coords = coords.into();

    self.items.get_mut(&coords)
  }

  /// Update the value and coordinates of a point at the given coordinates
  pub fn set(
    &mut self,
    coords: impl Into<Coord2>,
    new_val: impl Into<T>,
  ) -> Option<T> {
    //! # Example
    //! ```
    //! # use wasm_swirl::space::matrix::Grid;
    //! # use wasm_swirl::space::coords::{Rect, Corner, Edge::*};
    //! let dims = Rect { width: 3, height: 3, origin: (0,0).into() };
    //! let grid = Grid::<i32>::from_dimensions(dims);
    //!
    //! assert!(matches!(grid.get((0, 0)), None));
    //! ```

    self.items.insert(coords.into(), new_val.into())
  }

  /// Get an iterator over the points in the grid,
  /// yielding a tuple with the coordinates and value
  /// at the point.
  ///
  /// Notably, this will not yield points whose values
  /// have not been set.
  pub fn iter<'a>(
    &'a self,
  ) -> impl Iterator<Item = (Coord2, &T)> + 'a {
    self.items.iter().map(|(coord_ref, t)| (*coord_ref, t))
  }

  /// Get an iterator over the points in the grid,
  /// yielding a tuple with the coordinates and
  /// a mutable reference to the value at the point.
  ///
  /// Notably, this will not yield points whose values
  /// have not been set.
  pub fn iter_mut<'a>(
    &'a mut self,
  ) -> impl Iterator<Item = (Coord2, &mut T)> + 'a {
    self
      .items
      .iter_mut()
      .map(|(coord_ref, t)| (*coord_ref, t))
  }
}

impl<T> Edged for Grid<T> {
  fn edge_at_coords(
    &self,
    coords: impl Into<Coord2>,
  ) -> EdgeInfo {
    self.dimensions.edge_at_coords(coords)
  }

  fn coords_of_corner(
    &self,
    corner: impl Into<Corner>,
  ) -> Coord2 {
    self.dimensions.coords_of_corner(corner)
  }
}
