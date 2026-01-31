use askama::Template;
use axum::response::Html;
use crate::utils::error::AppError;
use crate::templates::{HomeTemplate, IndexTemplate};

pub async fn get_index() -> Result<Html<String>, AppError> {
    Ok(Html(IndexTemplate.render()?))
}

pub async fn get_home() -> Result<Html<String>, AppError> {
    Ok(Html(HomeTemplate.render()?))
}