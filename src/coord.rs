use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Eq, Clone, Copy)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
  pub fn new(x: i32, y: i32) -> Coord {
    Coord { x: x, y: y }
  }
}

pub fn equals(a: Option<Coord>, b: Option<Coord>) -> bool {
  match (a, b) {
    (Some(Coord{x: x1, y: y1}), Some(Coord{x: x2, y: y2})) if (x1 == x2) & (y1 == y2) => true,
    _ => false,
  }
}

impl PartialEq for Coord {
  fn eq(&self, other: &Self) -> bool {
    (self.x == other.x) & (self.y == other.y)
  }
}
