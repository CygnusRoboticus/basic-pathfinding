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
    walkableTiles: number[];
    costs: { [tile: number]: number | undefined };
    extraCosts: { [y: number]: { [x: number]: number | undefined } | undefined };
    unstoppableCoords: { [y: number]: { [x: number]: number | undefined } | undefined };
    unwalkableCoords: { [y: number]: { [x: number]: number | undefined } | undefined };
    gridType: GridType;
}
/**
*/
export interface SearchOpts {
  costThreshold?: number;
  endOnUnstoppable?: boolean;
}
