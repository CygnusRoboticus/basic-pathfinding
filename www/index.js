import * as wasm from "pathfinding";

function findPath() {
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
    unwalkable_coords: wasm.to_coord_map_js([
      { x: 1, y: 3 },
      { x: 2, y: 3 }
    ]),
    grid_type: "Cardinal",
  };
  const start = { x: 1, y: 2 };
  const end = { x: 3, y: 2 };
  const opts = {};

  const path = wasm.find_path_js(grid, start, end, opts);
  const walkable = wasm.find_walkable_js(grid, [start], { cost_threshold: 2 });
  console.log("path: ", path);
  console.log("walkable: ", walkable);
}
findPath();
