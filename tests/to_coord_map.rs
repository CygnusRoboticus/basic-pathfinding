extern crate basic_pathfinding;
use basic_pathfinding::pathfinding::to_coord_map;
use basic_pathfinding::coord::Coord;

macro_rules! hashmap {
  ($( $key: expr => $val: expr ),*) => {{
    let mut map = ::std::collections::HashMap::new();
    $( map.insert($key, $val); )*
    map
  }}
}

#[test]
fn converts_to_coord_map() {
  let map = to_coord_map(vec![
    Coord::new(1, 2),
    Coord::new(2, 2),
    Coord::new(1, 3),
    Coord::new(2, 3),
  ]);

  assert_eq!(map, hashmap![
    2 => hashmap![
      1 => true,
      2 => true
    ],
    3 => hashmap![
      1 => true,
      2 => true
    ]
  ]);
}
