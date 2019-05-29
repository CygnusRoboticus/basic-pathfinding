import { findPath, findWalkable, toCoordMap, GridType } from "pathfinding";

function path() {
  const grid = {
    tiles: [
      [1, 1, 0, 1, 1],
      [1, 1, 0, 1, 1],
      [1, 1, 0, 1, 1],
      [1, 1, 1, 1, 1],
      [1, 1, 1, 1, 1],
    ],
    walkable_tiles: [1],
    costs: {},
    extra_costs: {},
    unstoppable_coords: {},
    unwalkable_coords: toCoordMap([
      { x: 1, y: 3 },
      { x: 2, y: 3 }
    ]),
    grid_type: "Cardinal",//GridType.Cardinal,
  };

  const start = { x: 1, y: 2 };
  const end = { x: 3, y: 2 };
  const opts = {};

  console.log(grid);
  const path = findPath(grid, start, end, opts);
  const walkable = findWalkable(grid, [start], { cost_threshold: 2 });
  console.log("path: ", path);
  console.log("walkable: ", walkable);
  document.write(JSON.stringify(path));
}
path();
