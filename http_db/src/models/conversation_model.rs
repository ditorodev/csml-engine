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
pub struct ConversationModel {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "client")]
    pub client: crate::models::ClientModel,
    #[serde(rename = "flow_id")]
    pub flow_id: Option<String>,
    #[serde(rename = "step_id")]
    pub step_id: Option<String>,
    #[serde(rename = "metadata")]
    pub metadata: serde_json::Value,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "last_interaction_at")]
    pub last_interaction_at: String,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl ConversationModel {
    pub fn new(
        id: String,
        client: crate::models::ClientModel,
        flow_id: Option<String>,
        step_id: Option<String>,
        metadata: serde_json::Value,
        status: Status,
        last_interaction_at: String,
    ) -> ConversationModel {
        ConversationModel {
            id,
            client,
            flow_id,
            step_id,
            metadata,
            status,
            last_interaction_at,
            updated_at: None,
            created_at: None,
        }
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
