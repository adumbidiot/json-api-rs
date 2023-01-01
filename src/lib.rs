//! A basic client api for [JSON API](https://jsonapi.org/).

mod types;

pub use crate::types::JsonDocument;
pub use crate::types::LinksObject;
pub use crate::types::ResourceObject;

/// The library error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Reqwest HTTP error
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

/// A Json:Api Client
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
    ) -> Result<JsonDocument<D>, Error> {
        let res = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/vnd.api+json")
            .send()
            .await?
            .error_for_status()?;

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
