use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MilitaryBase {
    pub id: String,
    pub name: String,
    pub coordinates: [f64; 2],
    pub country: String,
    pub operator: String,
    pub branch: MilitaryBranch,
    pub base_type: BaseType,
    pub personnel: Option<u32>,
    pub status: BaseStatus,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MilitaryBranch {
    Army,
    Navy,
    AirForce,
    Marines,
    Combined,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BaseType {
    NavalBase,
    AirBase,
    ArmyBase,
    NavalAirBase,
    SupportBase,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BaseStatus {
    Active,
    Contested,
    UnderConstruction,
    Closed,
}