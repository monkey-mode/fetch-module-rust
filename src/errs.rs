use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("Request failed: {0}")]
    RequestError(reqwest::Error),

    #[error("JSON parsing error: {0}")]
    JsonParseError(reqwest::Error),
}

#[derive(Serialize, Deserialize, Debug)]
// This will auto convert form `camelCase` to `snake_case`
#[serde(rename_all = "camelCase")]
pub struct AnimalData {
    id: String,
    weight: f32,
    // This will auto convert form "createdAt" to `created_at`
    created_at: String,
}

pub async fn fetch(url: &str) -> Result<Vec<AnimalData>, CustomError> {
    let response = reqwest::get(url).await.map_err(CustomError::RequestError)?;
    let animals = response
        .json::<Vec<AnimalData>>()
        .await
        .map_err(CustomError::JsonParseError)?;

    Ok(animals)
}
