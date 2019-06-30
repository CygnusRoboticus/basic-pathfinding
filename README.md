# 🗺️ `basic-pathfinding`

Pathfinding is a simple package for performing 2D [A-star](https://en.wikipedia.org/wiki/A*_search_algorithm) pathfinding in square- and hex-based tile grids.

## 🚴 Rust Usage

```rust
let grid = Grid {
  tiles:
    vec![
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 0, 1, 1],
      vec![1, 1, 1, 1, 1],
      vec![1, 1, 1, 1, 1]
    ],
  walkable_tiles: vec![1],
  grid_type: GridType::Cardinal,
  ..Default::default()
);
let start = Coord::new(1, 2);
let end = Coord::new(3, 2);
let path = find_path(&grid, start, end, None);

path == Some(vec![
  Coord { x: 1, y: 2 },
  Coord { x: 1, y: 3 },
  Coord { x: 2, y: 3 },
  Coord { x: 3, y: 3 },
  Coord { x: 3, y: 2 },
]);
```

### 🛠️ Building

```
cargo build
```

### 🔬 Testing

```
cargo test
```
