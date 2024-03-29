/*
 * Up API
 *
 * The Up API gives you programmatic access to your balances and transaction data. You can request past transactions or set up webhooks to receive real-time events when new transactions hit your account. It’s new, it’s exciting and it’s just the beginning. 
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ErrorObject : Provides information about an error processing a request. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorObject {
    /// The HTTP status code associated with this error. This can also be obtained from the response headers. The status indicates the broad type of error according to HTTP semantics. 
    #[serde(rename = "status")]
    pub status: String,
    /// A short description of this error. This should be stable across multiple occurrences of this type of error and typically expands on the reason for the status code. 
    #[serde(rename = "title")]
    pub title: String,
    /// A detailed description of this error. This should be considered unique to individual occurrences of an error and subject to change. It is useful for debugging purposes. 
    #[serde(rename = "detail")]
    pub detail: String,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<crate::models::ErrorObjectSource>>,
}

impl ErrorObject {
    /// Provides information about an error processing a request. 
    pub fn new(status: String, title: String, detail: String) -> ErrorObject {
        ErrorObject {
            status,
            title,
            detail,
            source: None,
        }
    }
}


