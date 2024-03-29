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
pub struct WebhookEventResourceRelationships {
    #[serde(rename = "webhook")]
    pub webhook: Box<crate::models::WebhookEventResourceRelationshipsWebhook>,
    #[serde(rename = "transaction", skip_serializing_if = "Option::is_none")]
    pub transaction: Option<Box<crate::models::WebhookEventResourceRelationshipsTransaction>>,
}

impl WebhookEventResourceRelationships {
    pub fn new(webhook: crate::models::WebhookEventResourceRelationshipsWebhook) -> WebhookEventResourceRelationships {
        WebhookEventResourceRelationships {
            webhook: Box::new(webhook),
            transaction: None,
        }
    }
}


