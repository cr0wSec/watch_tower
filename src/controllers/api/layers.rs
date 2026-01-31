use crate::models::layers::military_base::MilitaryBase;
use crate::models::layers::nuclear_plant::NuclearPlant;
use crate::models::layers::pipeline::Pipeline;
use crate::models::layers::strait::Strait;
use crate::static_data;
use axum::Json;

pub async fn get_straits() -> Json<Vec<Strait>> {
    Json(static_data::get_straits())
}

pub async fn get_pipelines() -> Json<Vec<Pipeline>> {
    Json(static_data::get_pipelines())
}

pub async fn get_military_bases() -> Json<Vec<MilitaryBase>> {
    Json(static_data::get_military_bases())
}

pub async fn get_nuclear_plants() -> Json<Vec<NuclearPlant>> {
    Json(static_data::get_nuclear_plants())
}

#[derive(serde::Serialize)]
pub struct AllLayers {
    pub pipelines: Vec<Pipeline>,
    pub military_bases: Vec<MilitaryBase>,
    pub nuclear_plants: Vec<NuclearPlant>,
    pub straits: Vec<Strait>,
}

pub async fn get_all_layers() -> Json<AllLayers> {
    Json(AllLayers {
        pipelines: static_data::get_pipelines(),
        straits: static_data::get_straits(),
        nuclear_plants: static_data::get_nuclear_plants(),
        military_bases: static_data::get_military_bases(),
    })
}