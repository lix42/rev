//! Loads and validates config.toml from ~/.config/rev/config.toml.

mod defaults;
mod load;
pub mod types;
mod validate;

pub use load::load_config;
pub use types::*;
