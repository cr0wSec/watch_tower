use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Strait {
    pub id: String,
    pub name: String,
    pub coordinates: [f64; 2],
    pub strait_type: StraitType,
    pub controlled_by: Vec<String>,
    pub connects: [String; 2],
    pub traffic_volume: Option<String>,
    pub strategic_importance: Importance,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StraitType {
    Natural,
    Canal,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Importance {
    Critical,
    High,
    Medium,
    Low,
}