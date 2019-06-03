mod utils;
mod node;
mod search;
pub mod grid;
pub mod coord;
pub mod pathfinding;

#[macro_use] extern crate serde_derive;
extern crate log;
extern crate env_logger;

mod js;
// mod rustler_nifs;
// #[macro_use] extern crate rustler;
// #[macro_use] extern crate rustler_codegen;
// extern crate lazy_static;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// rustler_export_nifs! {
//     "Elixir.BasicPathfinding",
//     [
//         ("find_path", 4, rustler_nifs::find_path_nif),
//         ("find_walkable", 4, rustler_nifs::find_walkable_nif),
//         ("to_coord_map", 4, rustler_nifs::to_coord_map_nif),
//     ],
//     None
// }
