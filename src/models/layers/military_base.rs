use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MilitaryBase {
    pub id: String,
    pub name: String,
    pub coordinates: [f64; 2],
    pub country: String,
    pub operator: String,
    pub personnel: Option<u32>,
    pub notes: Option<String>,
}




