/* tslint:disable */
/**
* @param {any} grid 
* @param {any} start 
* @param {any} end 
* @param {any} opts 
* @returns {any} 
*/
export function findPath(grid: any, start: any, end: any, opts: any): any;
/**
* @param {any} grid 
* @param {any} source 
* @param {any} opts 
* @returns {any} 
*/
export function findWalkable(grid: any, source: any, opts: any): any;
/**
* @param {any} coords 
* @returns {any} 
*/
export function toCoordMap(coords: any): any;
/**
*/
export type GridType = "Cardinal" | "Hex" | "Intercardinal";
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
