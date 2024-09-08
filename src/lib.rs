pub mod replay;

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub pps: f64,
    pub apm: f64,
    pub vs: f64,
    pub success: bool
}

#[derive(Serialize, Deserialize)]
pub struct Average {
    pub username: String,
    pub pps: f64,
    pub apm: f64,
    pub vs: f64,
    pub score: u32
}


#[derive(Serialize, Deserialize)]
pub struct Round {
    pub left: Stats,
    pub right: Stats,
    pub time: String
}

#[derive(Serialize, Deserialize)]

pub struct Averages {
    pub left: Average,
    pub right: Average
}
// the output to our `create_user` handler
#[derive(Serialize, Deserialize)]
pub struct LeagueRecord {
    pub averages: Averages,
    pub rounds: Vec<Round>
}

#[derive(Serialize, Deserialize)]
pub struct LeagueRecordRequest {
    pub league_record: LeagueRecord,
    pub ts: String
}


use std::fmt::Display;


#[derive(Debug)]
pub struct Error(pub String);

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}



