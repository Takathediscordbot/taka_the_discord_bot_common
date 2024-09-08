use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserShortData {
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String,    
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserHandling {
    pub arr: f64,
    pub das: f64,
    pub dcd: f64,
    pub sdf: f64,
    pub safelock: bool,
    pub cancel: bool
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPoints {
    pub primary: f64,
    pub secondary: f64,
    pub tertiary: f64,
    pub extra: ExtraObject,
    pub secondary_avg_tracking: Option<Vec<f64>>,
    pub tertiary_avg_tracking: Option<Vec<f64>>,
    pub extra_avg_tracking: Option<ExtraAvgTracking>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtraObject {
    pub vs: Option<f64>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtraAvgTracking {
    #[serde(rename="aggregatestats___vsscore")]
    pub aggregate_stats_vs_score: Vec<f64>
}
