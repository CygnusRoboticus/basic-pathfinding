use crate::coord::Coord;
use crate::grid::Grid;
use crate::search::Search;
pub use crate::search::SearchOpts;
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReachableResult {
  pub stoppable: Vec<Coord>,
  pub walkable: Vec<Coord>,
}

pub fn find_path(grid: &Grid, start: Coord, end: Coord, opts: SearchOpts) -> Option<Vec<Coord>> {
  let end_on_unstoppable = opts.end_on_unstoppable;
  let end_coords = match opts.path_adjacent {
    true => {
      let mut coords = vec![];
      for coord in grid.get_adjacent(&end) {
        if end_on_unstoppable | grid.is_coord_stoppable(coord.x, coord.y) {
          coords.push(coord);
        }
      }
      coords
    }
    false => vec![end],
  };

  if end_coords.contains(&start) {
    Some(vec![])
  } else if end_coords
    .iter()
    .find(|c| grid.is_coord_stoppable(c.x, c.y))
    .is_none()
    & !end_on_unstoppable
  {
    None
  } else {
    let mut search = Search::new(start, end_coords.clone(), Some(end), opts);
    let start_node = search.coordinate_to_node(None, start.x, start.y, 0);
    search.push(start_node);

    calculate(&mut search, &grid);

    match search.pop() {
      None => None,
      Some(node) => Some(node.format_path(&search)),
    }
  }
}

pub fn find_reachable(grid: &Grid, source: Vec<Coord>, opts: SearchOpts) -> ReachableResult {
  let mut search = Search::new(*source.first().unwrap(), vec![], None, opts);

  for coord in source {
    let node = search.coordinate_to_node(None, coord.x, coord.y, 0);
    search.push(node);
  }

  calculate(&mut search, &grid);

  let mut stoppable = vec![];
  let mut walkable = vec![];
  for node in search.traversed_nodes() {
    if grid.is_coord_stoppable(node.x, node.y) {
      stoppable.push(Coord::new(node.x, node.y));
    } else if grid.is_coord_walkable(node.x, node.y) {
      walkable.push(Coord::new(node.x, node.y));
    }
  }
  stoppable.sort();
  walkable.sort();

  ReachableResult {
    stoppable: stoppable,
    walkable: walkable,
  }
}

pub fn to_coord_map(
  mut hash: HashMap<i32, HashMap<i32, bool>>,
  coords: Vec<Coord>,
) -> HashMap<i32, HashMap<i32, bool>> {
  for Coord { x, y } in coords {
    match hash.get_mut(&y) {
      None => {
        let mut inner_hash = HashMap::new();
        inner_hash.insert(x, true);
        hash.insert(y, inner_hash);
      }
      Some(inner_hash) => {
        inner_hash.insert(x, true);
      }
    };
  }
  hash.to_owned()
}

fn calculate(search: &mut Search, grid: &Grid) {
    while search.size() != 0 && !search.reached_destination() {
        let mut node = search.pop().unwrap();
        node.visited = true;
        search.cache(node);

        // cardinal
        if grid.in_grid(node.x, node.y - 1) {
            search.check_adjacent_node(grid, &node, 0, -1)
        }
        // hex & intercardinal
        if !grid.is_cardinal() & grid.in_grid(node.x + 1, node.y - 1) {
            search.check_adjacent_node(grid, &node, 1, -1)
        }
        // cardinal
        if grid.in_grid(node.x + 1, node.y) {
            search.check_adjacent_node(grid, &node, 1, 0)
        }
        // intercardinal
        if grid.is_intercardinal() & grid.in_grid(node.x + 1, node.y + 1) {
            search.check_adjacent_node(grid, &node, 1, 1)
        }
        // cardinal
        if grid.in_grid(node.x, node.y + 1) {
            search.check_adjacent_node(grid, &node, 0, 1)
        }
        // hex & intercardinal
        if !grid.is_cardinal() & grid.in_grid(node.x - 1, node.y + 1) {
            search.check_adjacent_node(grid, &node, -1, 1)
        }
        // cardinal
        if grid.in_grid(node.x - 1, node.y) {
            search.check_adjacent_node(grid, &node, -1, 0)
        }
        // intercardinal
        if grid.is_intercardinal() & grid.in_grid(node.x - 1, node.y - 1) {
            search.check_adjacent_node(grid, &node, -1, -1)
        }
    }
}
