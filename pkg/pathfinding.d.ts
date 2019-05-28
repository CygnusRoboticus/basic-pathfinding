/* tslint:disable */
/**
*/
export function find_path_js(grid: Grid, start: Coord, end: Coord, opts: SearchOpts): Coords[] | null;
/**
*/
export function find_walkable_js(grid: Grid, source: Coord[], opts: SearchOpts): Coord[];
/**
*/
export function to_coord_map_js(coords: Coord[]): any;
/**
*/
export enum GridType {
  Cardinal = "Cardinal",
  Hex = "Hex",
  Intercardinal = "Intercardinal",
}
/**
*/
export class Coord {
  free(): void;
  x: number;
  y: number;
}
/**
*/
export class Grid {
  free(): void;
  tiles: number[][];
  walkable_tiles: number[];
  costs: { [tile: number]: number | undefined; };
  extra_costs: { [y: number]: { [x: number]: number | undefined } | undefined; };
  unstoppable_coords: { [y: number]: { [x: number]: boolean | undefined } | undefined; };
  unwalkable_coords: { [y: number]: { [x: number]: boolean | undefined } | undefined; };
  grid_type: GridType;
}
/**
*/
export class SearchOpts {
  free(): void;
  cost_threshold?: number;
  end_on_unstoppable?: boolean;
}
