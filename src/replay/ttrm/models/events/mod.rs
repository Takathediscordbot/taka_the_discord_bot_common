pub mod end_game_event;
pub mod ige_events;
pub mod key_down_event;
pub mod key_up_event;

pub mod target_event;

use end_game_event::{EndData, EndDataExport};
use ige_events::IgeData;
use key_down_event::KeyDownData;
use key_up_event::KeyUpData;
use target_event::TargetsData;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Event {
    Full { frame: i32, data: EndDataExport },
    Start { frame: i32 },
    End { frame: i32, data: EndData },
    Targets { frame: i32, data: TargetsData },
    Keydown { frame: i32, data: KeyDownData },
    Keyup { frame: i32, data: KeyUpData },
    Ige { frame: i32, data: IgeData }
}