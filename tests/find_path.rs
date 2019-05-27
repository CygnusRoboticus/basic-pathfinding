//! Test suite for the Web and headless browsers.

// #![cfg(target_arch = "wasm32")]

// extern crate wasm_bindgen_test;
// use wasm_bindgen_test::*;

// wasm_bindgen_test_configure!(run_in_browser);

use std::collections::HashMap;

extern crate pathfinding;
use pathfinding::grid::{Grid, GridType};
use pathfinding::coord::Coord;
use pathfinding::pathfinding::{find_path, find_walkable, SearchOpts};

macro_rules! hashmap {
  ($( $key: expr => $val: expr ),*) => {{
    let mut map = ::std::collections::HashMap::new();
    $( map.insert($key, $val); )*
    map
  }}
}

// find_path/4
#[test]
fn traverses_walkable_tiles() {
  let grid = Grid::new(
    vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1]
    ],
    vec![1],
    hashmap![],
    hashmap![],
    hashmap![],
    hashmap![],
    GridType::Cardinal,
  );
  let start = Coord::new(1, 2);
  let end = Coord::new(3, 2);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert_eq!(path.unwrap(), vec![
    Coord::new(1, 2),
    Coord::new(1, 3),
    Coord::new(2, 3),
    Coord::new(3, 3),
    Coord::new(3, 2),
  ]);
}

#[test]
fn path_avoids_unwalkable_coords() {
  let grid = Grid::new(
    vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1]
    ],
    vec![1],
    hashmap![],
    hashmap![],
    hashmap![],
    hashmap![3 => hashmap![
      2 => true, 3 => true
    ]],
    GridType::Cardinal,
  );
  let start = Coord::new(1, 2);
  let end = Coord::new(3, 2);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert_eq!(path.unwrap(), vec![
    Coord::new(1, 2),
    Coord::new(1, 3),
    Coord::new(1, 4),
    Coord::new(2, 4),
    Coord::new(3, 4),
    Coord::new(4, 4),
    Coord::new(4, 3),
    Coord::new(4, 2),
    Coord::new(3, 2)
  ]);
}

#[test]
fn early_returns() {
  let grid = Grid::new(
    vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
    ],
    vec![1],
    hashmap![],
    hashmap![],
    hashmap![],
    hashmap![],
    GridType::Cardinal,
  );
  let start = Coord::new(1, 2);
  let end = Coord::new(1, 2);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert_eq!(path.unwrap(), vec![]);
}

#[test]
fn none_when_cannot_find_path() {
  let grid = Grid::new(
    vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
    ],
    vec![1],
    hashmap![],
    hashmap![],
    hashmap![],
    hashmap![],
    GridType::Cardinal
  );
  let start = Coord::new(0, 2);
  let end = Coord::new(4, 2);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert!(path.is_none());
}

#[test]
fn none_when_not_walkable() {
  let grid = Grid::new(
    vec![
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
    ],
    vec![1],
    hashmap![],
    hashmap![],
    hashmap![],
    hashmap![2 => hashmap![4 => true]],
    GridType::Cardinal,
  );
  let start = Coord::new(0, 2);
  let end = Coord::new(4, 2);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert!(path.is_none());
}

#[test]
fn none_when_target_unstoppable() {
  let grid = Grid::new(
    vec![
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1]
    ],
    vec![1],
    hashmap![],
    hashmap![],
    hashmap![
      2 => hashmap![4 => true]
    ],
    hashmap![],
    GridType::Cardinal,
  );
  let start = Coord::new(0, 2);
  let end = Coord::new(4, 2);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert!(path.is_none());
}

#[test]
fn prefers_straight_paths() {
  let grid = Grid::new(
    vec![
      vec![0, 0, 0, 0, 0],
      vec![0, 0, 0, 0, 0],
      vec![0, 0, 0, 0, 0],
      vec![0, 0, 0, 0, 0],
      vec![0, 0, 0, 0, 0],
    ],
    vec![0],
    hashmap![],
    hashmap![],
    hashmap![],
    hashmap![],
    GridType::Cardinal,
  );
  let start = Coord::new(0, 2);
  let end = Coord::new(4, 2);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert_eq!(path.unwrap(), vec![
    Coord::new(0, 2),
    Coord::new(1, 2),
    Coord::new(2, 2),
    Coord::new(3, 2),
    Coord::new(4, 2),
  ]);
}

#[test]
fn respects_costs() {
  let grid = Grid::new(
    vec![
      vec![0, 2, 2, 2, 0],
      vec![0, 2, 2, 2, 0],
      vec![0, 2, 2, 2, 0],
      vec![0, 1, 1, 1, 0],
      vec![0, 1, 1, 1, 0],
    ],
    vec![0, 1, 2],
    hashmap![2 => 4],
    hashmap![],
    hashmap![],
    hashmap![],
    GridType::Cardinal,
  );
  let start = Coord::new(0, 2);
  let end = Coord::new(4, 2);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert_eq!(path.unwrap(), [
    Coord::new(0, 2),
    Coord::new(0, 3),
    Coord::new(1, 3),
    Coord::new(2, 3),
    Coord::new(3, 3),
    Coord::new(4, 3),
    Coord::new(4, 2),
  ]);
}

#[test]
fn respects_extra_costs() {
  let grid = Grid::new(
    vec![
      vec![0, 2, 2, 2, 0],
      vec![0, 2, 2, 2, 0],
      vec![0, 2, 2, 2, 0],
      vec![0, 1, 1, 1, 0],
      vec![0, 1, 1, 1, 0],
    ],
    vec![0, 1],
    hashmap![],
    hashmap![
      3 => hashmap![1 => 4],
      4 => hashmap![3 => 4]
    ],
    hashmap![],
    hashmap![],
    GridType::Cardinal,
  );
  let start = Coord::new(0, 2);
  let end = Coord::new(4, 2);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert_eq!(path.unwrap(), vec![
    Coord::new(0, 2),
    Coord::new(0, 3),
    Coord::new(0, 4),
    Coord::new(1, 4),
    Coord::new(2, 4),
    Coord::new(2, 3),
    Coord::new(3, 3),
    Coord::new(4, 3),
    Coord::new(4, 2),
  ]);
}

#[test]
fn path_cancels_early_with_cost_threshold() {
  let grid = Grid::new(
    vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
    ],
    vec![1],
    hashmap![],
    hashmap![],
    hashmap![],
    hashmap![],
    GridType::Cardinal,
  );
  let start = Coord::new(1, 2);
  let end = Coord::new(3, 2);
  let mut opts = SearchOpts { cost_threshold: Some(3) };
  let path = find_path(&grid, start, end, Some(opts));

  assert!(path.is_none());

  opts = SearchOpts { cost_threshold: Some(4) };
  let path = find_path(&grid, start, end, Some(opts));

  assert_eq!(path.unwrap(), vec![
    Coord::new(1, 2),
    Coord::new(1, 3),
    Coord::new(2, 3),
    Coord::new(3, 3),
    Coord::new(3, 2),
  ]);
}

#[test]
fn path_navigates_hex_grids() {
  let grid = Grid::new(
    vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 0, 1, 0, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 1, 1, 1],
    ],
    vec![1],
    hashmap![],
    hashmap![],
    hashmap![],
    hashmap![],
    GridType::Hex,
  );
  let start = Coord::new(1, 1);
  let end = Coord::new(2, 2);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert_eq!(path.unwrap(), vec![
    Coord::new(1, 1),
    Coord::new(0, 2),
    Coord::new(0, 3),
    Coord::new(1, 3),
    Coord::new(2, 2),
  ]);
}

#[test]
fn path_navigates_intercardinal_grids() {
  let grid = Grid::new(
    vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 0, 1, 0, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 1, 1, 1],
    ],
    vec![1],
    hashmap![],
    hashmap![],
    hashmap![],
    hashmap![],
    GridType::Intercardinal,
  );
  let start = Coord::new(1, 1);
  let end = Coord::new(3, 3);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert_eq!(path.unwrap(), vec![
    Coord::new(1, 1),
    Coord::new(2, 2),
    Coord::new(3, 3),
  ]);
}
