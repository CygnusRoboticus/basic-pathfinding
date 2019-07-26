use std::collections::HashMap;

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum GridType {
  Cardinal,
  Hex,
  Intercardinal,
}

impl Default for GridType {
  fn default() -> GridType {
    GridType::Cardinal
  }
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Grid {
  pub tiles: Vec<Vec<i32>>,
  pub walkable_tiles: Vec<i32>,
  #[serde(default = "HashMap::new")]
  pub costs: HashMap<i32, i32>,
  #[serde(default = "HashMap::new")]
  pub extra_costs: HashMap<i32, HashMap<i32, i32>>,
  #[serde(default = "HashMap::new")]
  pub unstoppable_coords: HashMap<i32, HashMap<i32, bool>>,
  #[serde(default = "HashMap::new")]
  pub unwalkable_coords: HashMap<i32, HashMap<i32, bool>>,
  #[serde(default = "default_grid_type")]
  pub grid_type: GridType,
}

fn default_grid_type() -> GridType {
  GridType::Cardinal
}

impl Grid {
  pub fn is_cardinal(&self) -> bool {
    match self.grid_type {
      GridType::Cardinal => true,
      _ => false,
    }
  }

  pub fn is_hex(&self) -> bool {
    match self.grid_type {
      GridType::Hex => true,
      _ => false,
    }
  }

  pub fn is_intercardinal(&self) -> bool {
    match self.grid_type {
      GridType::Intercardinal => true,
      _ => false,
    }
  }

  pub fn in_grid(&self, x: i32, y: i32) -> bool {
    match self {
      _grid if (x < 0) | (y < 0) => false,
      grid if (y as usize) < grid.tiles.len() => (x as usize) < grid.tiles[y as usize].len(),
      _ => false,
    }
  }

  pub fn is_coord_stoppable(&self, x: i32, y: i32) -> bool {
    if get_nested_bool(&self.unstoppable_coords, x, y) {
      false
    } else {
      self.is_coord_walkable(x, y)
    }
  }

  pub fn is_coord_walkable(&self, x: i32, y: i32) -> bool {
    if get_nested_bool(&self.unwalkable_coords, x, y) {
      false
    } else {
      let tile = self.tiles[y as usize][x as usize];
      self.walkable_tiles.contains(&tile)
    }
  }

  pub fn get_coord_cost(&self, x: i32, y: i32) -> i32 {
    match self.get_extra_cost(x, y) {
      Some(extra) => extra,
      _ => {
        let tile = self.tiles[y as usize][x as usize];
        match self.costs.get(&tile) {
          None => 1,
          Some(cost) => *cost,
        }
      }
    }
  }

  fn get_extra_cost(&self, x: i32, y: i32) -> Option<i32> {
    match self.extra_costs.get(&y) {
      Some(inner_hash) => inner_hash.get(&x).cloned(),
      _ => None,
    }
  }
}

fn get_nested_bool(map: &HashMap<i32, HashMap<i32, bool>>, x: i32, y: i32) -> bool {
  match map.get(&y) {
    Some(nested) => match nested.get(&x) {
      Some(_) => true,
      _ => false,
    },
    _ => false,
  }
}
