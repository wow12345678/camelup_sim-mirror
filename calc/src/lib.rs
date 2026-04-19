#![feature(optimize_attribute)]
mod camel_map;
mod camel_stack;
mod color;
mod color_state;
mod configuration;
mod simulation;

// Public re-exports for the library API
pub use camel_map::{CamelMap, EffectCardType};
pub use color::Color;
pub use color_state::ColorState;
pub use configuration::{Configuration, ConfigurationBuilder, Dice};
pub use simulation::{simulate_round, simulate_rounds, SimulationResult};
