use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TargetsData {
    id: String,
    frame: i32,
    #[serde(rename = "type")]
    _type: String,
    data: Vec<String>
}
