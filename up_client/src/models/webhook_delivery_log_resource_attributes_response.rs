/*
 * Up API
 *
 * The Up API gives you programmatic access to your balances and transaction data. You can request past transactions or set up webhooks to receive real-time events when new transactions hit your account. It’s new, it’s exciting and it’s just the beginning. 
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WebhookDeliveryLogResourceAttributesResponse : Information about the response that was received from the webhook URL. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookDeliveryLogResourceAttributesResponse {
    /// The HTTP status code received in the response. 
    #[serde(rename = "statusCode")]
    pub status_code: i32,
    /// The payload that was received in the response body. 
    #[serde(rename = "body")]
    pub body: String,
}

impl WebhookDeliveryLogResourceAttributesResponse {
    /// Information about the response that was received from the webhook URL. 
    pub fn new(status_code: i32, body: String) -> WebhookDeliveryLogResourceAttributesResponse {
        WebhookDeliveryLogResourceAttributesResponse {
            status_code,
            body,
        }
    }
}


