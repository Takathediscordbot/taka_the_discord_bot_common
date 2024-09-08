use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct KeyDownData {
    key: String,
    subframe: f64
}
