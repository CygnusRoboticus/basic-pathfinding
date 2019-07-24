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

pub fn find_path(
  grid: &Grid,
  start: Coord,
  end: Coord,
  opts: Option<SearchOpts>,
) -> Option<Vec<Coord>> {
  let end_on_unstoppable = match &opts {
    None => false,
    Some(opts) => match opts.end_on_unstoppable {
      None => false,
      Some(end_on_unstoppable) => end_on_unstoppable,
    },
  };

  if Coord::equals(Some(start), Some(end)) {
    Some(vec![])
  } else if !grid.is_coord_stoppable(&end.x, &end.y) & !end_on_unstoppable {
    None
  } else {
    let mut search = Search::new(start, Some(end), opts);
    let start_node = search.coordinate_to_node(None, &start.x, &start.y, &0);
    search.push(start_node);

    calculate(&mut search, &grid);

    match search.pop() {
      None => None,
      Some(node) => Some(node.format_path(&search)),
    }
  }
}

pub fn find_reachable(
  grid: &Grid,
  source: Vec<Coord>,
  opts: Option<SearchOpts>,
) -> ReachableResult {
  let mut search = Search::new(*source.first().unwrap(), None, opts);

  for coord in source {
    let node = search.coordinate_to_node(None, &coord.x, &coord.y, &0);
    search.push(node);
  }

  calculate(&mut search, &grid);

  let mut stoppable = vec![];
  let mut walkable = vec![];
  for node in search.traversed_nodes() {
    if grid.is_coord_stoppable(&node.x, &node.y) {
      stoppable.push(Coord::new(node.x, node.y));
    } else if grid.is_coord_walkable(&node.x, &node.y) {
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

pub fn to_coord_map(coords: Vec<Coord>) -> HashMap<i32, HashMap<i32, bool>> {
  let hash = &mut HashMap::new();
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
  match search.size() {
    0 => (),
    _ => {
      if search.reached_destination() {
        ()
      } else {
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

        calculate(search, grid)
      }
    }
  }
}
