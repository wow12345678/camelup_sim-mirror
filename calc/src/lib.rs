mod camel_map;
mod color;
mod color_state;
mod configuration;
mod simulation;

// Public re-exports for the library API
pub use camel_map::{CamelMap, EffectCard};
pub use color::Color;
pub use color_state::ColorState;
pub use configuration::{Configuration, ConfigurationBuilder, Dice};
pub use simulation::{Placement, SimulationResult, simulate_rounds};
