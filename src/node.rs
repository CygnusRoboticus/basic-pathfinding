use std::cmp::Ordering;

use crate::coord::Coord;
use crate::search::Search;

#[derive(Eq, Clone, Copy)]
pub struct Node {
    pub parent: Option<Coord>,
    pub x: i32,
    pub y: i32,
    pub cost: i32,
    pub distance: i32,
    pub visited: bool,
}

impl Node {
  pub fn new(parent: Option<&Node>, x: i32, y: i32, cost: i32, distance: i32) -> Node {
    Node {
      parent: match parent {
        Some(parent) => Some(Coord::new(parent.x, parent.y)),
        None => None,
      },
      x: x,
      y: y,
      cost: cost,
      distance: distance,
      visited: false,
    }
  }

  pub fn guess_total_cost(&self) -> i32 {
    self.cost + self.distance
  }

  pub fn format_path(&self, search: &Search) -> Vec<Coord> {
    let path = &mut Vec::<Coord>::new();
    path.push(Coord::new(self.x, self.y));

    let mut parent = &self.parent;
    let mut node;
    while parent.is_some() {
      match parent {
        None => (),
        Some(p) => {
          node = search.get_node(&p.x, &p.y).unwrap();
          path.push(Coord::new(p.x, p.y));
          parent = &node.parent;
        },
      }
    }
    path.reverse();
    path.to_vec()
  }
}

impl Ord for Node {
  fn cmp(&self, other: &Self) -> Ordering {
    self.guess_total_cost().cmp(&other.guess_total_cost())
  }
}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for Node {
  fn eq(&self, other: &Self) -> bool {
    (self.x == other.x) & (self.y == other.y)
  }
}
