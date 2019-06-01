import React from 'react';
import ReactDOM from 'react-dom';
import { findPath, findWalkable } from "basic-pathfinding";

function random(max) {
  return Math.floor(Math.random() * Math.floor(max)) + 1;
}

class PathingExample extends React.Component {
  grid = {
    tiles: Array.from({ length: 20 }).map(
      () => Array.from({ length: 20 }).map(() => random(5))
    ),
    walkable_tiles: [1, 2, 3, 4],
    costs: {
      2: 2,
      3: 3,
      4: 4
    },
    extra_costs: {},
    unstoppable_coords: {},
    unwalkable_coords: {},
    grid_type: 'Cardinal',
  };

  get path() {
    return findPath(this.grid, this.state.start, this.state.end, this.opts);
  }

  get walkable() {
    return findWalkable(this.grid, [this.state.start], this.opts);
  }

  get opts() {
    return {
      cost_threshold: this.state.movement
    }
  }

  constructor() {
    super();
    this.state = {
      movement: 5,
      start: { x: 0, y: 0 },
      end: { x: 0, y: 0 },
    };
  }

  render() {
    const path = this.path;
    const walkable = this.walkable;
    return (
      <div className="flex">
        <div>
          {this.grid.tiles.map((row, y) => {
            return <div key={y}>
              {row.map((tile, x) => {
                return (
                  <button
                    key={x}
                    className={
                      path && path.length && path.find((coord) => coord.x === x && coord.y === y) ? 'active' : '' +
                      walkable && walkable.length && walkable.find((coord) => coord.x === x && coord.y === y) ? 'walkable' : '' +
                      this.state.start && this.state.start.x === x && this.state.start.y === y ? 'start' : '' +
                      this.state.end && this.state.end.x === x && this.state.end.y === y ? 'end' : '' +
                      `t${tile}`
                    }
                    onClick={() => this.setState({ start: { x, y }})}
                    onMouseOver={() => this.setState({ end: { x, y }})}
                  >
                    {tile}
                  </button>
                )
              })}
            </div>
          })}
          </div>
          <div>
            <label>Movement</label>
            <div>
              <input
                type="number"
                min="1"
                value={this.state.movement}
                onChange={(e) => this.setState({ movement: parseInt(e.target.value) })}
              />
            </div>
          </div>
      </div>
    );
  }
}

ReactDOM.render(
  <PathingExample />,
  document.getElementById("container")
)
