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
    let mut nodes = vec![];
    nodes.push(self);

    let mut parent = &self.parent;
    while parent.is_some() {
      match parent {
        None => (),
        Some(p) => {
          let node = search.get_node(p.x, p.y).unwrap();
          parent = &node.parent;
          nodes.push(node);
        }
      }
    }

    nodes.pop();
    nodes.reverse();

    let threshold = search.opts.cost_threshold;
    let fully_path = threshold.is_some() & search.opts.path_to_threshold;
    let threshold = match threshold {
      None => 0,
      Some(t) => t,
    };

    let mut path = vec![];
    for node in nodes.iter() {
      if !fully_path | (fully_path & (threshold >= node.cost)) {
        path.push(Coord::new(node.x, node.y));
      }
    }

    path.to_vec()
  }
}

impl Ord for Node {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.guess_total_cost().cmp(&other.guess_total_cost()) {
      Ordering::Less => Ordering::Greater,
      Ordering::Equal => Ordering::Equal,
      Ordering::Greater => Ordering::Less,
    }
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
