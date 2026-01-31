use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct NuclearPlant {
    pub id: String,
    pub name: String,
    pub coordinates: [f64; 2],
    pub country: String,
    pub capacity_mw: Option<u32>,
    pub reactors: Option<u8>,
    pub operator: Option<String>,
    pub notes: Option<String>,
}
