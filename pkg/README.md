# 🗺️ `basic-pathfinding`

Pathfinding is a simple package for performing 2D [A-star](https://en.wikipedia.org/wiki/A*_search_algorithm) pathfinding in square- and hex-based tile grids.

## Java/TypeScript Usage

```typescript
const { findPath } = await import('pathfinding');

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

const path = findPath(grid, { x: 1, y: 2 }, { x: 3, y: 2 });
// [
//   { x: 1, y: 2 },
//   { x: 1, y: 3 },
//   { x: 2, y: 3 },
//   { x: 3, y: 3 },
//   { x: 3, y: 2 }
// ]
```


## 🚴 Rust Usage

### 🛠️ Building

```
wasm-pack build
```

### 🔬 Testing

```
cargo test
wasm-pack test --headless --firefox
```
