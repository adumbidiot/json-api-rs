//! A basic client api for [JSON API](https://jsonapi.org/).

mod client;
mod types;

pub use crate::client::Client;
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

#[cfg(test)]
mod tests {
    use super::*;

    const RESOURCE_OBJECT_JSON: &str = include_str!("../test_data/resource_object.json");

    #[test]
    fn parse_resource_object() {
        let _resource_object: ResourceObject<serde_json::Value> =
            serde_json::from_str(RESOURCE_OBJECT_JSON).expect("failed to parse resource object");
        // TODO: Compare values
    }
}
