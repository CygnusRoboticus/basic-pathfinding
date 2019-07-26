extern crate basic_pathfinding;
use basic_pathfinding::coord::Coord;
use basic_pathfinding::grid::{Grid, GridType};
use basic_pathfinding::pathfinding::{find_reachable, SearchOpts};

macro_rules! hashmap {
  ($( $key: expr => $val: expr ),*) => {{
    let mut map = ::std::collections::HashMap::new();
    $( map.insert($key, $val); )*
    map
  }}
}

#[test]
fn only_traverses_walkable_tiles() {
  let grid = Grid {
    tiles: vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![2, 2, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
    ],
    walkable_tiles: vec![1],
    ..Grid::default()
  };
  let source = vec![Coord::new(1, 2)];
  let opts = SearchOpts::default();
  let result = find_reachable(&grid, source, opts);

  assert_eq!(
    result.stoppable,
    vec![
      Coord::new(0, 0),
      Coord::new(0, 1),
      Coord::new(0, 2),
      Coord::new(1, 0),
      Coord::new(1, 1),
      Coord::new(1, 2),
    ]
  );
}

#[test]
fn searches_from_multiple_sources() {
  let grid = Grid {
    tiles: vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![2, 2, 2, 2, 2],
      vec![1, 1, 1, 1, 1],
    ],
    walkable_tiles: vec![1],
    ..Grid::default()
  };
  let source = vec![Coord::new(1, 2), Coord::new(4, 2)];
  let opts = SearchOpts::default();
  let result = find_reachable(&grid, source, opts);

  println!("asdf {:?}", result);

  assert_eq!(
    result.stoppable,
    vec![
      Coord { x: 0, y: 0 },
      Coord { x: 0, y: 1 },
      Coord { x: 0, y: 2 },
      Coord { x: 1, y: 0 },
      Coord { x: 1, y: 1 },
      Coord { x: 1, y: 2 },
      Coord { x: 3, y: 0 },
      Coord { x: 3, y: 1 },
      Coord { x: 3, y: 2 },
      Coord { x: 4, y: 0 },
      Coord { x: 4, y: 1 },
      Coord { x: 4, y: 2 }
    ]
  );
}

#[test]
fn avoids_unwalkable_coords() {
  let grid = Grid {
    tiles: vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
    ],
    walkable_tiles: vec![1],
    unwalkable_coords: hashmap![
      3 => hashmap![0 => true, 1 => true]
    ],
    ..Grid::default()
  };

  let source = vec![Coord::new(1, 2)];
  let opts = SearchOpts::default();
  let result = find_reachable(&grid, source, opts);

  assert_eq!(
    result.stoppable,
    vec![
      Coord::new(0, 0),
      Coord::new(0, 1),
      Coord::new(0, 2),
      Coord::new(1, 0),
      Coord::new(1, 1),
      Coord::new(1, 2),
    ]
  );
}

#[test]
fn avoids_unstoppable_coords() {
  let grid = Grid {
    tiles: vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
    ],
    walkable_tiles: vec![1],
    unstoppable_coords: hashmap![3 => hashmap![
      0 => true, 1 => true
    ]],
    ..Grid::default()
  };

  let source = vec![Coord::new(1, 2)];
  let opts = SearchOpts::default();
  let result = find_reachable(&grid, source, opts);

  assert_eq!(result.walkable, vec![Coord::new(0, 3), Coord::new(1, 3)]);
  assert_eq!(
    result.stoppable,
    vec![
      Coord::new(0, 0),
      Coord::new(0, 1),
      Coord::new(0, 2),
      Coord::new(0, 4),
      Coord::new(1, 0),
      Coord::new(1, 1),
      Coord::new(1, 2),
      Coord::new(1, 4),
    ]
  );
}

#[test]
fn cancels_early_with_cost_threshold() {
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

  let source = vec![Coord::new(1, 2)];
  let opts = SearchOpts {
    cost_threshold: Some(1),
    ..SearchOpts::default()
  };
  let result = find_reachable(&grid, source, opts);

  assert_eq!(
    result.stoppable,
    vec![
      Coord::new(0, 2),
      Coord::new(1, 1),
      Coord::new(1, 2),
      Coord::new(1, 3),
    ]
  );

  let source = vec![Coord::new(1, 2)];
  let opts = SearchOpts {
    cost_threshold: Some(4),
    ..SearchOpts::default()
  };
  let result = find_reachable(&grid, source, opts);

  assert_eq!(
    result.stoppable,
    vec![
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
    ]
  );
}

#[test]
fn reports_start_only_when_cost_zero() {
  let grid = Grid {
    tiles: vec![
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
    ],
    walkable_tiles: vec![1],
    ..Grid::default()
  };

  let source = vec![Coord::new(1, 2)];
  let opts = SearchOpts {
    cost_threshold: Some(0),
    ..SearchOpts::default()
  };
  let result = find_reachable(&grid, source, opts);

  assert_eq!(result.stoppable, vec![Coord::new(1, 2),]);
}

#[test]
fn doesnt_include_unwalkable_start() {
  let grid = Grid {
    tiles: vec![
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1],
    ],
    walkable_tiles: vec![],
    ..Grid::default()
  };

  let source = vec![Coord::new(1, 2)];
  let opts = SearchOpts {
    cost_threshold: Some(4),
    ..SearchOpts::default()
  };
  let result = find_reachable(&grid, source, opts);

  assert_eq!(result.stoppable, vec![]);
}

#[test]
fn navigates_hex_grids() {
  let grid = Grid {
    tiles: vec![
      vec![1, 0, 1, 0, 1],
      vec![0, 1, 0, 0, 1],
      vec![1, 0, 1, 0, 1],
      vec![0, 1, 0, 0, 1],
      vec![1, 1, 0, 1, 1],
    ],
    walkable_tiles: vec![1],
    grid_type: GridType::Hex,
    ..Grid::default()
  };

  let source = vec![Coord::new(1, 1)];
  let opts = SearchOpts::default();
  let result = find_reachable(&grid, source, opts);

  assert_eq!(
    result.stoppable,
    vec![Coord::new(0, 2), Coord::new(1, 1), Coord::new(2, 0),]
  );
}

#[test]
fn navigates_intercardinal_grids() {
  let grid = Grid {
    tiles: vec![
      vec![1, 0, 0, 0, 0],
      vec![0, 1, 0, 0, 0],
      vec![0, 1, 0, 0, 0],
      vec![1, 0, 0, 0, 0],
      vec![0, 1, 0, 0, 0],
    ],
    walkable_tiles: vec![1],
    grid_type: GridType::Intercardinal,
    ..Grid::default()
  };

  let source = vec![Coord::new(1, 1)];
  let opts = SearchOpts::default();
  let result = find_reachable(&grid, source, opts);

  assert_eq!(
    result.stoppable,
    vec![
      Coord::new(0, 0),
      Coord::new(0, 3),
      Coord::new(1, 1),
      Coord::new(1, 2),
      Coord::new(1, 4),
    ]
  );
}
