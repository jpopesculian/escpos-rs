extern crate serde;

use serde::{Deserialize, Serialize};

/// Alignment for text printing
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Justification {
    Left,
    Center,
    Right,
}
