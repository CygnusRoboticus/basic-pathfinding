extern crate basic_pathfinding;
use basic_pathfinding::coord::Coord;
use basic_pathfinding::grid::{Grid, GridType};
use basic_pathfinding::pathfinding::{find_path, SearchOpts};

macro_rules! hashmap {
  ($( $key: expr => $val: expr ),*) => {{
    let mut map = ::std::collections::HashMap::new();
    $( map.insert($key, $val); )*
    map
  }}
}

#[test]
fn traverses_walkable_tiles() {
  let grid = Grid {
    tiles: vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
    ],
    walkable_tiles: vec![1],
    ..Grid::default()
  };
  let start = Coord::new(1, 2);
  let end = Coord::new(3, 2);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert_eq!(
    path.unwrap(),
    vec![
      Coord::new(1, 3),
      Coord::new(2, 3),
      Coord::new(3, 3),
      Coord::new(3, 2),
    ]
  );
}

#[test]
fn path_avoids_unwalkable_coords() {
  let grid = Grid {
    tiles: vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
    ],
    walkable_tiles: vec![1],
    unwalkable_coords: hashmap![3 => hashmap![
      2 => true, 3 => true
    ]],
    ..Grid::default()
  };
  let start = Coord::new(1, 2);
  let end = Coord::new(3, 2);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert_eq!(
    path.unwrap(),
    vec![
      Coord::new(1, 3),
      Coord::new(1, 4),
      Coord::new(2, 4),
      Coord::new(3, 4),
      Coord::new(4, 4),
      Coord::new(4, 3),
      Coord::new(4, 2),
      Coord::new(3, 2)
    ]
  );
}

#[test]
fn early_returns() {
  let grid = Grid {
    tiles: vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
    ],
    walkable_tiles: vec![1],
    ..Grid::default()
  };
  let start = Coord::new(1, 2);
  let end = Coord::new(1, 2);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert_eq!(path.unwrap(), vec![]);
}

#[test]
fn none_when_cannot_find_path() {
  let grid = Grid {
    tiles: vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
    ],
    walkable_tiles: vec![1],
    ..Grid::default()
  };
  let start = Coord::new(0, 2);
  let end = Coord::new(4, 2);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert!(path.is_none());
}

#[test]
fn none_when_not_walkable() {
  let grid = Grid {
    tiles: vec![
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
    ],
    walkable_tiles: vec![1],
    unwalkable_coords: hashmap![2 => hashmap![4 => true]],
    ..Grid::default()
  };
  let start = Coord::new(0, 2);
  let end = Coord::new(4, 2);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert!(path.is_none());
}

#[test]
fn none_when_target_unstoppable() {
  let grid = Grid {
    tiles: vec![
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
    ],
    walkable_tiles: vec![1],
    unstoppable_coords: hashmap![
      2 => hashmap![4 => true]
    ],
    ..Grid::default()
  };
  let start = Coord::new(0, 2);
  let end = Coord::new(4, 2);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert!(path.is_none());
}

#[test]
fn accepts_opt_to_end_on_unstoppable() {
  let grid = Grid {
    tiles: vec![
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
    ],
    walkable_tiles: vec![1],
    unstoppable_coords: hashmap![2 => hashmap![2 => true]],
    ..Grid::default()
  };

  let start = Coord::new(0, 2);
  let end = Coord::new(4, 2);
  let opts = SearchOpts {
    end_on_unstoppable: true,
    ..SearchOpts::default()
  };
  let path = find_path(&grid, start, end, Some(opts));

  assert_eq!(
    path.unwrap(),
    vec![
      Coord::new(1, 2),
      Coord::new(2, 2),
      Coord::new(3, 2),
      Coord::new(4, 2)
    ]
  );
}

#[test]
fn prefers_straight_paths() {
  let grid = Grid {
    tiles: vec![
      vec![0, 0, 0, 0, 0],
      vec![0, 0, 0, 0, 0],
      vec![0, 0, 0, 0, 0],
      vec![0, 0, 0, 0, 0],
      vec![0, 0, 0, 0, 0],
    ],
    walkable_tiles: vec![0],
    ..Grid::default()
  };
  let start = Coord::new(0, 2);
  let end = Coord::new(4, 2);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert_eq!(
    path.unwrap(),
    vec![
      Coord::new(1, 2),
      Coord::new(2, 2),
      Coord::new(3, 2),
      Coord::new(4, 2),
    ]
  );
}

#[test]
fn respects_costs() {
  let grid = Grid {
    tiles: vec![
      vec![0, 2, 2, 2, 0],
      vec![0, 2, 2, 2, 0],
      vec![0, 2, 2, 2, 0],
      vec![0, 1, 1, 1, 0],
      vec![0, 1, 1, 1, 0],
    ],
    walkable_tiles: vec![0, 1, 2],
    costs: hashmap![2 => 4],
    ..Grid::default()
  };
  let start = Coord::new(0, 2);
  let end = Coord::new(4, 2);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert_eq!(
    path.unwrap(),
    [
      Coord::new(0, 3),
      Coord::new(1, 3),
      Coord::new(2, 3),
      Coord::new(3, 3),
      Coord::new(4, 3),
      Coord::new(4, 2),
    ]
  );
}

#[test]
fn respects_extra_costs() {
  let grid = Grid {
    tiles: vec![
      vec![0, 2, 2, 2, 0],
      vec![0, 2, 2, 2, 0],
      vec![0, 2, 2, 2, 0],
      vec![0, 1, 1, 1, 0],
      vec![0, 1, 1, 1, 0],
    ],
    walkable_tiles: vec![0, 1],
    extra_costs: hashmap![
      3 => hashmap![1 => 4],
      4 => hashmap![3 => 4]
    ],
    ..Grid::default()
  };
  let start = Coord::new(0, 2);
  let end = Coord::new(4, 2);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert_eq!(
    path.unwrap(),
    vec![
      Coord::new(0, 3),
      Coord::new(0, 4),
      Coord::new(1, 4),
      Coord::new(2, 4),
      Coord::new(2, 3),
      Coord::new(3, 3),
      Coord::new(4, 3),
      Coord::new(4, 2),
    ]
  );
}

#[test]
fn path_cancels_early_with_cost_threshold() {
  let grid = Grid {
    tiles: vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
    ],
    walkable_tiles: vec![1],
    ..Grid::default()
  };
  let start = Coord::new(1, 2);
  let end = Coord::new(3, 2);
  let mut opts = SearchOpts {
    cost_threshold: Some(3),
    ..SearchOpts::default()
  };
  let path = find_path(&grid, start, end, Some(opts));

  assert!(path.is_none());

  opts = SearchOpts {
    cost_threshold: Some(4),
    ..SearchOpts::default()
  };
  let path = find_path(&grid, start, end, Some(opts));

  assert_eq!(
    path.unwrap(),
    vec![
      Coord::new(1, 3),
      Coord::new(2, 3),
      Coord::new(3, 3),
      Coord::new(3, 2),
    ]
  );
}

#[test]
fn path_finds_closest_with_cost_threshold() {
  let grid = Grid {
    tiles: vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
    ],
    walkable_tiles: vec![1],
    ..Grid::default()
  };
  let start = Coord::new(1, 2);
  let end = Coord::new(3, 2);
  let opts = SearchOpts {
    cost_threshold: Some(2),
    path_closest: true,
    ..SearchOpts::default()
  };
  let path = find_path(&grid, start, end, Some(opts));

  assert_eq!(path.unwrap(), vec![Coord::new(1, 3), Coord::new(2, 3)]);
}

#[test]
fn path_navigates_hex_grids() {
  let grid = Grid {
    tiles: vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 0, 1, 0, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 1, 1, 1],
    ],
    walkable_tiles: vec![1],
    grid_type: GridType::Hex,
    ..Grid::default()
  };
  let start = Coord::new(1, 1);
  let end = Coord::new(2, 2);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert_eq!(
    path.unwrap(),
    vec![
      Coord::new(0, 2),
      Coord::new(0, 3),
      Coord::new(1, 3),
      Coord::new(2, 2),
    ]
  );
}

#[test]
fn path_navigates_intercardinal_grids() {
  let grid = Grid {
    tiles: vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 0, 1, 0, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 1, 1, 1],
    ],
    walkable_tiles: vec![1],
    grid_type: GridType::Intercardinal,
    ..Grid::default()
  };
  let start = Coord::new(1, 1);
  let end = Coord::new(3, 3);
  let opts = None;
  let path = find_path(&grid, start, end, opts);

  assert_eq!(path.unwrap(), vec![Coord::new(2, 2), Coord::new(3, 3),]);
}
