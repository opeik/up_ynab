/*
 * Up API
 *
 * The Up API gives you programmatic access to your balances and transaction data. You can request past transactions or set up webhooks to receive real-time events when new transactions hit your account. It’s new, it’s exciting and it’s just the beginning. 
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookDeliveryLogResourceAttributes {
    #[serde(rename = "request")]
    pub request: Box<crate::models::WebhookDeliveryLogResourceAttributesRequest>,
    #[serde(rename = "response", deserialize_with = "Option::deserialize")]
    pub response: Option<Box<crate::models::WebhookDeliveryLogResourceAttributesResponse>>,
    #[serde(rename = "deliveryStatus")]
    pub delivery_status: crate::models::WebhookDeliveryStatusEnum,
    /// The date-time at which this log entry was created. 
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

impl WebhookDeliveryLogResourceAttributes {
    pub fn new(request: crate::models::WebhookDeliveryLogResourceAttributesRequest, response: Option<crate::models::WebhookDeliveryLogResourceAttributesResponse>, delivery_status: crate::models::WebhookDeliveryStatusEnum, created_at: String) -> WebhookDeliveryLogResourceAttributes {
        WebhookDeliveryLogResourceAttributes {
            request: Box::new(request),
            response: if let Some(x) = response {Some(Box::new(x))} else {None},
            delivery_status,
            created_at,
        }
    }
}


