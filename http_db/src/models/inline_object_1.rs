/*
 * CSML engine microservices
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineObject1 {
    #[serde(rename = "status")]
    pub status: Status,
}

impl InlineObject1 {
    pub fn new(status: Status) -> InlineObject1 {
        InlineObject1 { status }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "OPEN")]
    OPEN,
    #[serde(rename = "SWITCHED")]
    SWITCHED,
    #[serde(rename = "CLOSED")]
    CLOSED,
    #[serde(rename = "EXPIRED")]
    EXPIRED,
    #[serde(rename = "FAILED")]
    FAILED,
}
