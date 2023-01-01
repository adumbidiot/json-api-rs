//! A basic client api for [JSON API](https://jsonapi.org/).

pub mod types;

pub use crate::types::{JsonDocument, LinksObject, ResourceObject};

pub type JsonResult<T> = Result<T, JsonError>;

#[derive(Debug)]
pub enum JsonError {
    Reqwest(reqwest::Error),
    InvalidStatus(reqwest::StatusCode),
}

impl From<reqwest::Error> for JsonError {
    fn from(e: reqwest::Error) -> JsonError {
        Self::Reqwest(e)
    }
}

#[derive(Default)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_json_document<D: serde::de::DeserializeOwned>(
        &self,
        url: &str,
    ) -> JsonResult<JsonDocument<D>> {
        let res = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/vnd.api+json")
            .send()
            .await?;
        let status = res.status();

        if !status.is_success() {
            return Err(JsonError::InvalidStatus(status));
        }

        Ok(res.json().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESOURCE_OBJECT_JSON: &str = include_str!("../test_data/resource_object.json");

    #[test]
    fn parse_resource_object() {
        let _resource_object: ResourceObject<serde_json::Value> =
            serde_json::from_str(RESOURCE_OBJECT_JSON).unwrap();
        // TODO: Compare values
    }
}
