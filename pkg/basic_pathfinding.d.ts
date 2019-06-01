/* tslint:disable */
export function findPath(grid: Grid, start: Coord, end: Coord, opts: SearchOpts): Coord[];
export function findWalkable(grid: any, source: any, opts: any): any;
export function toCoordMap(coords: any): any;
/**
*/
export type GridType = 'Cardinal' | 'Hex' | 'Intercardinal';
/**
*/
export interface Coord {
  x: number;
  y: number;
}
/**
*/
export interface Grid {
    tiles: number[][];
    walkable_tiles: number[];
    costs: { [tile: number]: number | undefined };
    extra_costs: { [y: number]: { [x: number]: number | undefined } | undefined };
    unstoppable_coords: { [y: number]: { [x: number]: number | undefined } | undefined };
    unwalkable_coords: { [y: number]: { [x: number]: number | undefined } | undefined };
    grid_type: GridType;
}
/**
*/
export interface SearchOpts {
  cost_threshold?: number;
  end_on_unstoppable?: boolean;
}
