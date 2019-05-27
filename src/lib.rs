mod utils;
mod node;
mod search;
pub mod grid;
pub mod coord;
pub mod pathfinding;

#[macro_use]
extern crate serde_derive;
extern crate log;
extern crate env_logger;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
