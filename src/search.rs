use std::collections::BinaryHeap;
use std::collections::HashMap;

use crate::coord::Coord;
use crate::grid::Grid;
use crate::node::Node;

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchOpts {
  pub cost_threshold: Option<i32>,
  pub end_on_unstoppable: bool,
  pub path_to_threshold: bool,
  pub path_adjacent: bool,
}

pub struct Search {
  pub start: Coord,
  pub end: Vec<Coord>,
  pub dest: Option<Coord>,
  pub heap: BinaryHeap<Node>,
  pub cache: HashMap<i32, HashMap<i32, Node>>,
  pub opts: SearchOpts,
}

impl Search {
  pub fn new(start: Coord, end: Vec<Coord>, dest: Option<Coord>, opts: SearchOpts) -> Search {
    Search {
      start: start,
      dest: dest,
      end: end,
      heap: BinaryHeap::new(),
      cache: HashMap::new(),
      opts: opts,
    }
  }

  pub fn reached_destination(&self) -> bool {
    match self.peek() {
      Some(curr) => self
        .end
        .iter()
        .find(|c| c.matches(curr.x, curr.y))
        .is_some(),
      _ => false,
    }
  }

  pub fn push(&mut self, node: Node) {
    self.heap.push(node.clone());
    self.cache(node);
  }

  pub fn cache(&mut self, node: Node) {
    match self.cache.remove(&node.y) {
      None => {
        let mut inner_hash = HashMap::new();
        inner_hash.insert(node.x, node);
        self.cache.insert(node.y, inner_hash);
      }
      Some(mut inner_hash) => {
        inner_hash.insert(node.x, node);
        self.cache.insert(node.y, inner_hash);
      }
    }
  }

  pub fn peek(&self) -> Option<&Node> {
    match self.heap.peek() {
      None => None,
      Some(node) => self.get_node(node.x, node.y),
    }
  }

  pub fn pop(&mut self) -> Option<Node> {
    self.heap.pop()
  }

  pub fn size(&self) -> usize {
    self.heap.len()
  }

  pub fn update(&mut self, node: Node) {
    self.cache(node);
  }

  pub fn get_node(&self, x: i32, y: i32) -> Option<&Node> {
    match self.cache.get(&y) {
      None => None,
      Some(inner_hash) => inner_hash.get(&x),
    }
  }

  pub fn is_pathing(&self) -> bool {
    self.dest.is_some()
  }

  pub fn coordinate_to_node(&self, parent: Option<&Node>, x: i32, y: i32, cost: i32) -> Node {
    match self.get_node(x, y) {
      Some(&node) => node,
      None => {
        let distance = match !self.is_pathing() {
          true => 1,
          false => {
            let dest = self.dest.unwrap();
            get_distance(x, y, dest.x, dest.y)
          }
        };

        Node::new(
          parent,
          x,
          y,
          match parent {
            None => cost,
            Some(parent) => parent.cost + cost,
          },
          distance,
        )
      }
    }
  }

  pub fn check_adjacent_node(&mut self, grid: &Grid, source_node: &Node, x: i32, y: i32) {
    let adjacent_x = source_node.x + x;
    let adjacent_y = source_node.y + y;
    let adjacent_cost = grid.get_coord_cost(adjacent_x, adjacent_y);

    if grid.is_coord_walkable(adjacent_x, adjacent_y)
      & can_afford(
        source_node,
        adjacent_cost,
        match self.opts.path_to_threshold {
          true => None,
          _ => self.opts.cost_threshold,
        },
      )
    {
      let mut adjacent_node =
        self.coordinate_to_node(Some(source_node), adjacent_x, adjacent_y, adjacent_cost);

      if !adjacent_node.visited {
        self.push(adjacent_node);
      } else if (source_node.cost + adjacent_cost) < adjacent_node.cost {
        adjacent_node.cost = source_node.cost + adjacent_cost;
        adjacent_node.parent = Some(Coord::new(source_node.x, source_node.y));
        self.update(adjacent_node);
      }
    }
  }

  pub fn traversed_nodes(&self) -> Vec<&Node> {
    let nodes = &mut vec![];
    for list in self.cache.values() {
      nodes.extend(list.values());
    }
    nodes.to_vec()
  }
}

fn can_afford(node: &Node, cost: i32, cost_threshold: Option<i32>) -> bool {
  match cost_threshold {
    None => true,
    Some(cost_threshold) => node.cost + cost <= cost_threshold,
  }
}

fn get_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
  let dx = (x1 - x2).abs();
  let dy = (y1 - y2).abs();
  dx + dy
}
