use crate::Error;
use crate::JsonDocument;
use reqwest::header::HeaderValue;

/// A Json:Api Client
#[derive(Default, Clone)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    /// Make a new Client
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Get a json document
    pub async fn get_json_document<D>(&self, url: &str) -> Result<JsonDocument<D>, Error>
    where
        D: serde::de::DeserializeOwned,
    {
        static ACCEPT_HEADER_VALUE: HeaderValue =
            HeaderValue::from_static("application/vnd.api+json");

        let response = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, ACCEPT_HEADER_VALUE.clone())
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json().await?)
    }
}
