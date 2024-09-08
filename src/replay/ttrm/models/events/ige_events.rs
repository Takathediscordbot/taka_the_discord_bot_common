use serde::{Serialize, Deserialize};
use crate::replay::utils::string_or_t::StringOrObject;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IgeData {
    id: i32,
    frame: i32,
    #[serde(rename = "type")]
    _type: String,
    data: IgeDataDataType
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum IgeDataDataType {
    Interaction {
        data: IgeDataDataDataType,
        sender: Option<String>,
        sender_id: Option<String>,
        sent_frame: Option<i32>,
        cid: Option<i32>
    },
    #[serde(rename = "interaction_confirm")]
    InteractionConfirm {
        data: IgeDataDataDataType,
        sender: Option<String>,
        sender_id: Option<String>,
        sent_frame: Option<i32>,
        cid: Option<i32>
    },
    Target {
        targets: Vec<String>,
        frame: u64
    },
    #[serde(rename = "allow_targeting")]

    AllowTargeting {
        value: bool,
        frame: i32
    },
    Attack {
        lines: i32,
        column: i32,
        sender: String,
        sent_frame: i32
    },
    Kev {
        fire: f64,
        victim: StringOrObject<Victim>,
        killer: Killer
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Victim {
    gameid: String,
    name: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Killer {
    gameid: Option<String>,
    name: Option<String>,
    #[serde(rename = "type")]
    _type: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum IgeDataDataDataType {
    Garbage {
        iid: Option<i32>,
        amt: i32,
        ackiid: Option<i32>,
        x: i32,
        y: i32,
        column: i32
    },
    Targeted {
        value: bool
    }
}