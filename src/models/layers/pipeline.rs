use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Pipeline {
    pub id: String,
    pub name: String,
    pub coordinates: Vec<[f64; 2]>,
    pub pipeline_type: PipelineType,
    pub capacity: Option<Capacity>,
    pub owner: String,
    pub operational_since: Option<u16>,
}


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PipelineType {
    Gas,
    Oil,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "unit", content = "value")]
pub enum Capacity {
    #[serde(rename = "bcm_year")]
    BcmPerYear(f64),
    #[serde(rename = "barrels_day")]
    BarrelsPerDay(u64),
}