use wasm_bindgen::prelude::*;
use crate::utils::set_panic_hook;

use crate::grid::Grid;
use crate::coord::Coord;
use crate::search::SearchOpts;
use crate::pathfinding::{find_path, find_walkable, to_coord_map};

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &JsValue);
}

#[wasm_bindgen(js_name = "findPath")]
#[allow(dead_code)]
pub fn find_path_js(grid: &JsValue, start: &JsValue, end: &JsValue, opts: &JsValue) -> JsValue {
  set_panic_hook();

  let grid: Grid = grid.into_serde().unwrap();
  let start: Coord = start.into_serde().unwrap();
  let end: Coord = end.into_serde().unwrap();
  let opts: SearchOpts = opts.into_serde().unwrap();

  let result = find_path(&grid, start, end, Some(opts));
  JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen(js_name = "findWalkable")]
#[allow(dead_code)]
pub fn find_walkable_js(grid: &JsValue, source: &JsValue, opts: &JsValue) -> JsValue {
  set_panic_hook();

  let grid: Grid = grid.into_serde().unwrap();
  let source: Vec<Coord> = source.into_serde().unwrap();
  let opts: Option<SearchOpts> = Some(opts.into_serde().unwrap());

  let result = find_walkable(&grid, source, opts);
  JsValue::from_serde(&result).unwrap()
}

#[wasm_bindgen(js_name = "toCoordMap")]
#[allow(dead_code)]
pub fn to_coord_map_js(coords: &JsValue) -> JsValue {
  set_panic_hook();

  let coords: Vec<Coord> = coords.into_serde().unwrap();
  let hash = to_coord_map(coords);
  JsValue::from_serde(&hash).unwrap()
}
