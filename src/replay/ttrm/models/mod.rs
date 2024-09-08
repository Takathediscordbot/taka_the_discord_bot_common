use std::time::Duration;
use anyhow::anyhow;
use serde::Deserialize;
use serde::Serialize;
pub mod endcontext;
pub mod events;
pub mod user;
use crate::Average;
use crate::Averages;
use crate::LeagueRecord;
use crate::Round;
use crate::Stats;
use crate::replay::ttrm::models::{
    endcontext::Endcontext,
    events::Event,
    user::{UserHandling, UserPoints, UserShortData},
};

use self::events::end_game_event::Reason;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub ismulti: Option<bool>,
    pub data: Vec<Daum>,
    pub endcontext: Vec<Endcontext>,
    pub ts: String,
    pub gametype: Option<String>,
    pub customtype: Option<String>,
    pub mt: Option<u64>,
    pub shortid: Option<String>,
    pub verified: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Daum {
    pub board: Vec<Board>,
    pub replays: Vec<Replay>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Board {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub username: Option<String>,
    pub user: Option<UserShortData>,
    pub handling: Option<UserHandling>,
    pub active: bool,
    pub success: bool,
    pub inputs: Option<i32>,
    pub piecesplaced: Option<i32>,
    pub naturalorder: Option<i32>,
    pub score: Option<i32>,
    pub wins: Option<i32>,
    pub points: Option<UserPoints>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Replay {
    pub frames: i32,
    pub events: Vec<Event>,
}


impl Root {



    pub fn into_league_record(self) -> anyhow::Result<LeagueRecord> {
        let replay_data = self;
        if replay_data.data.len() == 0 {
            return Err(anyhow!("❌ Replay has no rounds"));
        };

        if replay_data.endcontext.len() != 2 {
            return Err(anyhow!("❌ Couldn't find both players in replay or there was more than 2 players"));
        };

        let left = replay_data.endcontext.get(0);
        let right = replay_data.endcontext.get(1);

        let (Some(left), Some(right)) = (left, right) else {
            return Err(anyhow!("❌ Couldn't find both players in replay"));
        };

        let left_username = left.get_username();
        let right_username = right.get_username();

        let (Some(left_username), Some(right_username)) = (left_username, right_username) else {
            return Err(anyhow!("❌ Couldn't find both players in replay"));
        };

        let end_frames = replay_data.data.iter().map(|round| {
            round.replays.iter().map(|replay| replay.events.iter().find(|event|{
                match event {
                    Event::End { .. } => true,
                    _ => false
                }
            }).map(|event| (replay.frames, event)))
        }).flatten();

        let left_frames = end_frames.clone().filter(|data| {
            match data {
                Some((_ , Event::End { data, .. })) => {
                    data.export.options.username == left_username
                },
                _ => false
            }
        });

        let right_frames = end_frames.clone().filter(|event| {
            match event {
                Some((_, Event::End { data, .. })) => {
                    data.export.options.username == right_username
                },
                _ => false
            }
        });

        let rounds = left_frames.zip(right_frames).map(|(left, right)| {
            match (left, right) {
                (Some((left_frames, Event::End { data: left, .. })), Some((right_frames, Event::End { data: right, .. }))) => {
                    let left_stats = &left.export.aggregatestats;
                    let right_stats = &right.export.aggregatestats;

                    let left_pps = left_stats.pps;
                    let right_pps = right_stats.pps;

                    let left_apm = left_stats.apm;
                    let right_apm = right_stats.apm;

                    let left_vs = left_stats.vsscore;
                    let right_vs = right_stats.vsscore;

                    let left_success = match left.reason {
                        Reason::Winner => true,
                        _ => false
                    };
                    let right_success = match right.reason {
                        Reason::Winner => true,
                        _ => false
                    };
                    let left = Stats {
                        pps: left_pps,
                        apm: left_apm,
                        vs: left_vs,
                        success: left_success
                    };

                    let right = Stats {
                        pps: right_pps,
                        apm: right_apm,
                        vs: right_vs,
                        success: right_success
                    };

                    let frames = (left_frames.max(right_frames) as f64) * (1.0 / 60.0);

                    let duration = Duration::from_secs_f64(frames);

                    let minutes = duration.as_secs() / 60;
                    let seconds = duration.as_secs() % 60;
                    

                    Ok(Round {
                        left,
                        right,
                        time: format!("{minutes}:{seconds:02}")
                    })
                },
                _ => {
                    return Err(anyhow::anyhow!("❌ Couldn't find both players in replay"));
                }
            }
        }).collect::<anyhow::Result<Vec<Round>>>();

        let rounds = match rounds {
            Ok(rounds) => rounds,
            Err(err) => {
                return Err(err);
            }
        };

        let mut averages = rounds.iter().fold(Averages {
            left: Average {
                username: left_username.clone(),
                pps: 0.0,
                apm: 0.0,
                vs: 0.0,
                score: 0
            },
            right: Average {
                username: right_username.clone(),
                pps: 0.0,
                apm: 0.0,
                vs: 0.0,
                score: 0
            }
        }, |mut averages, round| {
            averages.left.pps += round.left.pps;
            averages.left.apm += round.left.apm;
            averages.left.vs += round.left.vs;
            averages.left.score += if round.left.success {
                1
            } else {
                0
            };

            averages.right.pps += round.right.pps;
            averages.right.apm += round.right.apm;
            averages.right.vs += round.right.vs;
            averages.right.score += if round.right.success {
                1
            } else {
                0
            };

            averages
        });

        averages.left.pps /= rounds.len() as f64;
        averages.left.apm /= rounds.len() as f64;
        averages.left.vs /= rounds.len() as f64;

        averages.right.pps /= rounds.len() as f64;
        averages.right.apm /= rounds.len() as f64;
        averages.right.vs /= rounds.len() as f64;


        Ok(LeagueRecord {
            averages, 
            rounds 
        })
    }
}