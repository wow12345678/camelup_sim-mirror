mod color;
mod color_state;
mod camel_map;
mod configuration;
mod simulation;

// Public re-exports for the library API
pub use color::Color;
pub use color_state::ColorState;
pub use camel_map::CamelMap;
pub use configuration::{Configuration, ConfigurationBuilder, Dice};
pub use simulation::{simulate_rounds, SimulationResult, aggragate_placements, main, Placement};
