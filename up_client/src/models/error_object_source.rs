/*
 * Up API
 *
 * The Up API gives you programmatic access to your balances and transaction data. You can request past transactions or set up webhooks to receive real-time events when new transactions hit your account. It’s new, it’s exciting and it’s just the beginning. 
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ErrorObjectSource : If applicable, location in the request that this error relates to. This may be a parameter in the query string, or a an attribute in the request body. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorObjectSource {
    /// If this error relates to a query parameter, the name of the parameter. 
    #[serde(rename = "parameter", skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    /// If this error relates to an attribute in the request body, a rfc-6901 JSON pointer to the attribute. 
    #[serde(rename = "pointer", skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

impl ErrorObjectSource {
    /// If applicable, location in the request that this error relates to. This may be a parameter in the query string, or a an attribute in the request body. 
    pub fn new() -> ErrorObjectSource {
        ErrorObjectSource {
            parameter: None,
            pointer: None,
        }
    }
}

