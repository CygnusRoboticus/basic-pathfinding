use std::hash::Hash;
use std::cmp::Eq;
use std::collections::HashMap;
use rustler::{Env, Term, NifResult, Decoder, Encoder, MapIterator};

use crate::grid::{Grid, GridType};
use crate::coord::Coord;
use crate::search::SearchOpts;
use crate::pathfinding::{find_path, find_walkable, to_coord_map};

mod atoms {
  rustler_atoms! {
    atom ok;
    atom tiles;
    atom walkable_tiles;
    atom costs;
    atom extra_costs;
    atom unstoppable_coords;
    atom unwalkable_coords;
    atom grid_type;

    //grid types
    atom cardinal;
    atom hex;
    atom intercardinal;
    //atom error;
    //atom __true__ = "true";
    //atom __false__ = "false";
  }
}

impl<'a> Decoder<'a> for GridType {
  fn decode(term: Term<'a>) -> NifResult<GridType> {
    let env = term.get_env();
    match (
      atoms::cardinal().to_term(env),
      atoms::cardinal().to_term(env),
      atoms::cardinal().to_term(env),
    ) {
      (c, _, _) if term.eq(&c) => Ok(GridType::Cardinal),
      (_, h, _) if term.eq(&h) => Ok(GridType::Hex),
      (_, _, i) if term.eq(&i) => Ok(GridType::Intercardinal),
      _ => Ok(GridType::Cardinal),
    }
  }
}

impl<'a> Decoder<'a> for Grid {
  fn decode(term: Term<'a>) -> NifResult<Grid> {
    let env = term.get_env();
    let tiles = term.map_get(atoms::tiles().to_term(env))?;
    let walkable_tiles = term.map_get(atoms::walkable_tiles().to_term(env))?;
    let costs = term.map_get(atoms::costs().to_term(env))?;
    let extra_costs = term.map_get(atoms::extra_costs().to_term(env))?;
    let unstoppable_coords = term.map_get(atoms::unstoppable_coords().to_term(env))?;
    let unwalkable_coords = term.map_get(atoms::unwalkable_coords().to_term(env))?;
    let grid_type = term.map_get(atoms::grid_type().to_term(env))?;

    let grid = Grid::new(
      tiles.decode()?,
      walkable_tiles.decode()?,
      decode_hash(costs)?,
      decode_nested_hash(extra_costs)?,
      decode_nested_hash(unstoppable_coords)?,
      decode_nested_hash(unwalkable_coords)?,
      grid_type.decode()?,
    );
    Ok(grid)
  }
}

pub fn find_path_nif<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
  let grid: Grid = args[0].decode()?;
  let start: Coord = args[1].decode()?;
  let end: Coord = args[2].decode()?;
  let opts: SearchOpts = args[3].decode()?;

  let result = find_path(&grid, start, end, Some(opts));
  Ok((atoms::ok(), result).encode(env))
}

pub fn find_walkable_nif<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
  let grid: Grid = args[0].decode()?;
  let source: Vec<Coord> = args[1].decode()?;
  let opts: Option<SearchOpts> = args[2].decode()?;

  let result = find_walkable(&grid, source, opts);
  Ok((atoms::ok(), result).encode(env))
}

pub fn to_coord_map_nif<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
  let coords: Vec<Coord> = args[0].decode()?;
  let hash = to_coord_map(coords);

  let mut keys = vec![];
  let mut values = vec![];
  for (key, value) in hash.iter() {
    keys.push(key.encode(env));
    let nested_hash: Term<'a> = encode_hash(env, value)?;
    values.push(nested_hash);
  }
  let result = Term::map_from_arrays(env, &keys, &values)?;
  Ok((atoms::ok(), result).encode(env))
}

fn decode_hash<'a, T: Eq + Hash + Decoder<'a>, U: Eq + Hash + Decoder<'a>>(term: Term<'a>) -> NifResult<HashMap<T, U>> {
  let mut hash = HashMap::new();
  let iter = MapIterator::new(term).unwrap();
  for (k, v) in iter {
    let key: T = k.decode()?;
    let value: U = v.decode()?;
    hash.insert(key, value);
  }
  Ok(hash)
}

fn decode_nested_hash<'a, T: Eq + Hash + Decoder<'a>, U: Eq + Hash + Decoder<'a>, V: Eq + Hash + Decoder<'a>>(term: Term<'a>) -> NifResult<HashMap<T, HashMap<U, V>>> {
  let mut hash = HashMap::new();
  let iter = MapIterator::new(term).unwrap();
  for (k, v) in iter {
    let key: T = k.decode()?;
    let value: HashMap<U, V> = decode_hash(v)?;
    hash.insert(key, value);
  }
  Ok(hash)
}

fn encode_hash<'a, T: Encoder, U: Encoder>(env: Env<'a>, hash: &HashMap<T, U>) -> NifResult<Term<'a>> {
  let mut keys = vec![];
  let mut values = vec![];
  for (key, value) in hash.iter() {
    keys.push(key.encode(env));
    values.push(value.encode(env));
  }
  Term::map_from_arrays(env, &keys, &values)
}
