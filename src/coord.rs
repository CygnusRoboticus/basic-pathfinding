use std::cmp::Ordering;

#[derive(Default, Eq, Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Coord {
  pub x: i32,
  pub y: i32,
}

impl Coord {
  pub fn new(x: i32, y: i32) -> Coord {
    Coord { x: x, y: y }
  }

  pub fn matches(&self, x: i32, y: i32) -> bool {
    self.x == x && self.y == y
  }

  pub fn distance(&self, other: &Self) -> i32 {
    (self.x - other.x).abs() + (self.y - other.y).abs()
  }

  pub fn to_flat_repr(coords: Vec<Coord>) -> Vec<i32> {
    let mut flat_result = vec![];
    for coord in coords {
      flat_result.push(coord.x);
      flat_result.push(coord.y);
    }
    flat_result
  }
}

impl Ord for Coord {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.x.cmp(&other.x) {
      Ordering::Equal => self.y.cmp(&other.y),
      ord => ord,
    }
  }
}

impl PartialOrd for Coord {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for Coord {
  fn eq(&self, other: &Self) -> bool {
    (self.x == other.x) & (self.y == other.y)
  }
}
