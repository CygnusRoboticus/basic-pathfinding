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

// find_walkable/3
#[test]
fn only_traverses_walkable_tiles() {
  let grid = Grid::new(
    vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![2, 2, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
    ],
    vec![1],
    hashmap![],
    hashmap![],
    hashmap![],
    hashmap![],
    GridType::Cardinal,
  );
  let source = vec![Coord::new(1, 2)];
  let opts = None;
  let path = find_walkable(&grid, source, opts);

  assert_eq!(path, vec![
    Coord::new(0, 0),
    Coord::new(0, 1),
    Coord::new(0, 2),
    Coord::new(1, 0),
    Coord::new(1, 1),
    Coord::new(1, 2),
  ]);
}

#[test]
fn searches_from_multiple_sources() {
  let grid = Grid::new(
    vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![2, 2, 2, 2, 2],
      vec![1, 1, 1, 1, 1],
    ],
    vec![1],
    hashmap![],
    hashmap![],
    hashmap![],
    hashmap![],
    GridType::Cardinal,
  );
  let source = vec![
    Coord::new(1, 2),
    Coord::new(4, 2)
  ];
  let opts = None;
  let path = find_walkable(&grid, source, opts);

  assert_eq!(path, vec![
    Coord::new(0, 0),
    Coord::new(0, 1),
    Coord::new(0, 2),
    Coord::new(1, 0),
    Coord::new(1, 1),
    Coord::new(1, 2),
    Coord::new(3, 0),
    Coord::new(3, 1),
    Coord::new(3, 2),
    Coord::new(4, 0),
    Coord::new(4, 1),
    Coord::new(4, 2),
  ]);
}

#[test]
fn avoids_unwalkable_coords() {
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
    hashmap![
      3 => hashmap![0 => true, 1 => true]
    ],
    GridType::Cardinal,
  );

  let source = vec![Coord::new(1, 2)];
  let opts = None;
  let path = find_walkable(&grid, source, opts);

  assert_eq!(path, vec![
    Coord::new(0, 0),
    Coord::new(0, 1),
    Coord::new(0, 2),
    Coord::new(1, 0),
    Coord::new(1, 1),
    Coord::new(1, 2),
  ]);
}

#[test]
fn avoids_unstoppable_coords() {
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
    hashmap![3 => hashmap![
      0 => true, 1 => true
    ]],
    hashmap![],
    GridType::Cardinal,
  );

  let source = vec![Coord::new(1, 2)];
  let opts = None;
  let path = find_walkable(&grid, source, opts);

  assert_eq!(path, vec![
    Coord::new(0, 0),
    Coord::new(0, 1),
    Coord::new(0, 2),
    Coord::new(0, 3),
    Coord::new(0, 4),
    Coord::new(1, 0),
    Coord::new(1, 1),
    Coord::new(1, 2),
    Coord::new(1, 3),
    Coord::new(1, 4),
  ]);
}

#[test]
fn cancels_early_with_cost_threshold() {
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

  let source = vec![Coord::new(1, 2)];
  let opts = SearchOpts { cost_threshold: Some(1) };
  let path = find_walkable(&grid, source, Some(opts));

  assert_eq!(path, vec![
    Coord::new(0, 2),
    Coord::new(1, 1),
    Coord::new(1, 2),
    Coord::new(1, 3),
  ]);

  let source = vec![Coord::new(1, 2)];
  let opts = SearchOpts { cost_threshold: Some(4) };
  let path = find_walkable(&grid, source, Some(opts));

  assert_eq!(path, vec![
    Coord::new(0, 0),
    Coord::new(0, 1),
    Coord::new(0, 2),
    Coord::new(0, 3),
    Coord::new(0, 4),
    Coord::new(1, 0),
    Coord::new(1, 1),
    Coord::new(1, 2),
    Coord::new(1, 3),
    Coord::new(1, 4),
    Coord::new(2, 3),
    Coord::new(2, 4),
    Coord::new(3, 2),
    Coord::new(3, 3),
    Coord::new(3, 4),
    Coord::new(4, 3),
  ]);
}

#[test]
fn reports_start_only_when_cost_zero() {
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
    hashmap![],
    hashmap![],
    GridType::Cardinal,
  );

  let source = vec![Coord::new(1, 2)];
  let opts = SearchOpts { cost_threshold: Some(0) };
  let path = find_walkable(&grid, source, Some(opts));

  assert_eq!(path, vec![
    Coord::new(1, 2),
  ]);
}

#[test]
fn doesnt_include_unwalkable_start() {
  let grid = Grid::new(
    vec![
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
    ],
    vec![],
    hashmap![],
    hashmap![],
    hashmap![],
    hashmap![],
    GridType::Cardinal,
  );

  let source = vec![Coord::new(1, 2)];
  let opts = SearchOpts { cost_threshold: Some(4) };
  let path = find_walkable(&grid, source, Some(opts));

  assert_eq!(path, vec![]);
}

#[test]
fn navigates_hex_grids() {
  let grid = Grid::new(
    vec![
      vec![1, 0, 1, 0, 1],
      vec![0, 1, 0, 0, 1],
      vec![1, 0, 1, 0, 1],
      vec![0, 1, 0, 0, 1],
      vec![1, 1, 0, 1, 1]
    ],
    vec![1],
    hashmap![],
    hashmap![],
    hashmap![],
    hashmap![],
    GridType::Hex,
  );

  let source = vec![Coord::new(1, 1)];
  let opts = None;
  let path = find_walkable(&grid, source, opts);

  assert_eq!(path, vec![
    Coord::new(0, 2),
    Coord::new(1, 1),
    Coord::new(2, 0),
  ]);
}

#[test]
fn navigates_intercardinal_grids() {
  let grid = Grid::new(
    vec![
      vec![1, 0, 0, 0, 0],
      vec![0, 1, 0, 0, 0],
      vec![0, 1, 0, 0, 0],
      vec![1, 0, 0, 0, 0],
      vec![0, 1, 0, 0, 0],
    ],
    vec![1],
    hashmap![],
    hashmap![],
    hashmap![],
    hashmap![],
    GridType::Intercardinal
  );

  let source = vec![Coord::new(1, 1)];
  let opts = None;
  let path = find_walkable(&grid, source, opts);

  assert_eq!(path, vec![
    Coord::new(0, 0),
    Coord::new(0, 3),
    Coord::new(1, 1),
    Coord::new(1, 2),
    Coord::new(1, 4),
  ]);
}
