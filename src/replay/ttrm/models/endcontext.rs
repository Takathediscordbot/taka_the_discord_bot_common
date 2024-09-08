use serde::Deserialize;
use serde::Serialize;
use super::user::UserShortData;
use super::user::UserPoints;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Endcontext {
    pub id: Option<String>,
    pub username: Option<String>,
    pub user: Option<UserShortData>,
    pub active: bool,
    pub success: Option<bool>,
    pub inputs: i32,
    pub piecesplaced: i32,
    pub naturalorder: i32,
    pub score: Option<i32>,
    pub wins: i32,
    pub points: UserPoints
}

impl Endcontext {
    pub fn get_username(&self) -> Option<String> {
        return self.username.clone().or(self.user.clone().map(|user| user.username))
    }
}