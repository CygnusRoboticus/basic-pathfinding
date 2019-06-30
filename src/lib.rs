pub mod coord;
pub mod grid;
mod node;
pub mod pathfinding;
mod search;

#[macro_use]
extern crate serde_derive;
extern crate env_logger;
extern crate log;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
