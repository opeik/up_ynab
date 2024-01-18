/*
 * Up API
 *
 * The Up API gives you programmatic access to your balances and transaction data. You can request past transactions or set up webhooks to receive real-time events when new transactions hit your account. It’s new, it’s exciting and it’s just the beginning. 
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateTransactionTagsRequest : Request to add or remove tags associated with a transaction. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateTransactionTagsRequest {
    /// The tags to add to or remove from the transaction. 
    #[serde(rename = "data")]
    pub data: Vec<crate::models::TagInputResourceIdentifier>,
}

impl UpdateTransactionTagsRequest {
    /// Request to add or remove tags associated with a transaction. 
    pub fn new(data: Vec<crate::models::TagInputResourceIdentifier>) -> UpdateTransactionTagsRequest {
        UpdateTransactionTagsRequest {
            data,
        }
    }
}


