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
export enum GridType {
  Cardinal,
  Hex,
  Intercardinal,
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
}
/**
*/
export class SearchOpts {
  free(): void;
  cost_threshold: number;
  end_on_unstoppable: boolean;
}
