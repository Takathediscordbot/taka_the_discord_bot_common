use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::replay::ttrm::models::user::UserHandling;
use crate::replay::utils::object_or_array_container::ObjectOrArrayContainer;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Reason {
    Winner,
    Garbagesmash,
    Topout,
    Disconnect
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EndData {
    pub reason: Reason,
    pub export: EndDataExport,

}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EndDataExport {
    pub successful: bool,
    pub gameoverreason: Option<String>,
    pub replay: Option<EndDataReplay>,
    pub source: Option<EndDataSource>,
    pub options: EndDataOptions,
    pub stats: EndDataStats,
    pub targets: Vec<String>,
    pub fire: f64,
    pub game: EndDataGame,
    pub killer: Option<EndDataKiller>,
    pub aggregatestats: EndGameAggregateStats,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EndDataGame {
    pub board: Vec<Vec<Option<String>>>,
    pub bag: Vec<String>,
    pub hold: EndDataGameHold,
    pub g: Option<f64>,
    pub controlling: EndDataGameControlling,
    pub handling: UserHandling,
    pub playing: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EndDataKiller {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EndGameAggregateStats {
    pub pps: f64,
    pub apm: f64,
    pub vsscore: f64,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EndDataReplay{}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EndDataSource{}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
 pub struct EndDataOptions {
    pub version: i32,
    pub seed_random: bool,
    pub seed: i32,
    pub g: f64,
    pub stock: i32,
    pub countdown: bool,
    pub countdown_count: i32,
    pub countdown_interval: i32,
    pub precountdown: i32,
    pub prestart: i32,
    pub mission: Option<String>,
    pub mission_type: String,
    pub zoominto: String,
    pub slot_counter1: Option<String>,
    pub slot_counter2: Option<String>,
    pub slot_counter3: Option<String>,
    pub slot_counter4: Option<String>,
    pub slot_counter5: Option<String>,
    pub slot_bar1: Option<String>,
    pub display_fire: bool,
    pub display_username: bool,
    pub hasgarbage: bool,
    pub neverstopbgm: bool,
    pub display_next: bool,
    pub display_hold: bool,
    pub gmargin: f64,
    pub gincrease: f64,
    pub garbagemultiplier: f64,
    pub garbagemargin: f64,
    pub garbageincrease: f64,
    pub garbagecap: f64,
    pub garbagecapincrease: f64,
    pub garbagecapmax: f64,
    pub garbageholesize: Option<f64>,
    pub garbageblocking: Option<String>,
    pub presets: Option<String>,
    pub bagtype: String,
    pub spinbonuses: String,
    pub combotable: Option<String>,
    pub kickset: String,
    pub nextcount: i32,
    pub allow_harddrop: bool,
    pub display_shadow: bool,
    pub locktime: i32,
    pub garbagespeed: i32,
    pub forfeit_time: i32,
    pub are: i32,
    pub lineclear_are: i32,
    pub infinitemovement: bool,
    pub lockresets: i32,
    pub allow180: bool,
    pub objective: Value,
    pub room_handling: bool,
    pub room_handling_arr: f64,
    pub room_handling_das: f64,
    pub room_handling_sdf: f64,
    pub manual_allowed: bool,
    pub b2bchaining: bool,
    pub allclears: Option<bool>,
    pub clutch: bool,
    pub nolockout: Option<bool>,
    pub passthrough: Option<serde_json::Value>,
    pub can_undo: Option<bool>,
    pub can_retry: Option<bool>,
    pub retryisclear: Option<bool>,
    pub noextrawidth: Option<bool>,
    pub stride: Option<bool>,
    pub boardwidth: Option<i32>,
    pub boardheight: Option<i32>,
    pub latencypreference: Option<String>,
    pub handling: Option<UserHandling>,
    pub fulloffset: Option<i32>,
    pub fullinterval: Option<i32>,
    pub username: String,
    pub constants_overrides: Option<ObjectOrArrayContainer<Value>>,
    pub boardbuffer: Option<i32>,
    pub physical: Option<bool>,
    pub minoskin: Option<EndDataMinoSkin>, 
    pub ghostskin: Option<String>,
    pub boardskin: Option<String>


 }

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EndDataMinoSkin {
    pub z: String,
    pub l: String,
    pub o: String,
    pub s: String,
    pub i: String,
    pub j: String,
    pub t: String,
    pub other: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EndDataStats {

    pub seed: i32,
    pub lines: i32,
    pub level_lines: i32,
    pub level_lines_needed: i32,
    pub inputs: i32,
    pub holds: Option<i32>,
    pub time: EndGameTime,
    pub score: i32,
    pub zenlevel: i32,
    pub zenprogress: i32,
    pub level: i32,
    pub combo: i32,
    pub currentcombopower: i32,
    pub topcombo: i32,
    pub btb: i32,
    pub topbtb: i32,
    pub currentbtbchainpower: Option<i32>,
    pub tspins: i32,
    pub piecesplaced: i32,
    pub clears: EndGameClears,
    pub garbage: EndGameGarbage,
    pub kills: i32,
    pub finesse: EndGameFinesse
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EndGameTime {
    pub start: f64,
    pub zero: bool,
    pub locked: bool,
    pub prev: f64,
    pub frameoffset: f64
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EndGameClears {
    
    pub singles: i32,
    pub doubles: i32,
    pub triples: i32,
    pub quads: i32,
    pub pentas: Option<i32>,
    pub realtspins: i32,
    pub minitspins: i32,
    pub minitspinsingles: i32,
    pub tspinsingles: i32,
    pub minitspindoubles: i32,
    pub tspindoubles: i32,
    pub tspintriples: i32,
    pub tspinquads: i32,
    pub tspinpentas: Option<i32>,
    pub allclear: i32

}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EndGameGarbage {
    pub sent: i32,
    pub received: i32,
    pub attack: i32,
    pub cleared: i32
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EndGameFinesse {
    pub combo: i32,
    pub faults: i32,
    pub perfectpieces: i32
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EndDataGameHold {
    pub piece: Option<String>,
    pub locked: Option<bool>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EndDataGameControlling {
    pub ldas: f64,
    pub ldasiter: f64,
    pub lshift: bool,
    pub rdas: f64,
    pub rdasiter: f64,
    pub rshift: bool,
    pub lastshift: f64,
    pub softdrop: bool
}