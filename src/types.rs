//! JSON Datatypes

use url::Url;

/// A Json Document.
/// [Spec](https://jsonapi.org/format/#document-structure)
#[derive(Debug, serde::Deserialize)]
pub struct JsonDocument<D> {
    pub data: Option<D>,
    pub errors: Option<serde_json::Value>,
    pub meta: Option<serde_json::Value>,

    pub jsonapi: Option<serde_json::Value>,

    /// [Spec](https://jsonapi.org/format/#document-links)
    pub links: Option<LinksObject>,

    pub included: Option<Vec<serde_json::Value>>, // TODO: Array of resource objects
}

/// A Resource Object.
/// [Spec](https://jsonapi.org/format/#document-resource-objects)
#[derive(Debug, serde::Deserialize)]
pub struct ResourceObject<A> {
    pub id: Option<String>,

    #[serde(rename = "type")]
    pub kind: String,

    /// [Spec](https://jsonapi.org/format/#document-resource-object-attributes)
    pub attributes: Option<A>,
    pub relationships: Option<serde_json::Value>, // TODO: Relationships object

    /// [Spec](https://jsonapi.org/format/#document-links)
    pub links: Option<LinksObject>,
    pub meta: Option<serde_json::Value>, // TODO: Meta Object
}

/// A Links Object.
/// [Spec](https://jsonapi.org/format/#document-links)
#[derive(Debug, serde::Deserialize)]
pub struct LinksObject {
    /// [Spec](https://jsonapi.org/format/#fetching-relationships-responses)
    #[serde(rename = "self")]
    pub current: Option<Url>,

    /// [Spec](https://jsonapi.org/format/#fetching-relationships-responses)
    pub related: Option<Url>,

    /// [Spec](https://jsonapi.org/format/#fetching-pagination)
    pub first: Option<Url>,

    /// [Spec](https://jsonapi.org/format/#fetching-pagination)
    pub last: Option<Url>,

    /// [Spec](https://jsonapi.org/format/#fetching-pagination)
    pub next: Option<Url>,

    /// [Spec](https://jsonapi.org/format/#fetching-pagination)
    pub prev: Option<Url>,
}
