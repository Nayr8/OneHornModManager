use serde::{Deserialize, Serialize};

mod logging;
mod errors;
mod fs;

pub use logging::*;
pub use errors::*;
pub use fs::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Mod {
    pub name: String,
    pub description: String,
}