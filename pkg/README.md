# 🗺️ `rust-pathfinding`

Pathfinding is a simple package for performing 2D [A-star](https://en.wikipedia.org/wiki/A*_search_algorithm) pathfinding in square- and hex-based tile grids.

## Basic Usage

```typescript
import { find_path_js } from 'pathfinding';

const grid: Grid = {
  tiles: [
    [1, 1, 0, 1, 1],
    [1, 1, 0, 1, 1],
    [1, 1, 0, 1, 1],
    [1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1]
  ],
  walkable_tiles: [1]
};

const path = await find_path_js(grid, 1, 2, 3, 2);
// [
//   { x: 1, y: 2 },
//   { x: 1, y: 3 },
//   { x: 2, y: 3 },
//   { x: 3, y: 3 },
//   { x: 3, y: 2 }
// ]
```


## 🚴 Usage

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```
