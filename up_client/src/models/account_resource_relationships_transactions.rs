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
pub struct AccountResourceRelationshipsTransactions {
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::AccountResourceRelationshipsTransactionsLinks>>,
}

impl AccountResourceRelationshipsTransactions {
    pub fn new() -> AccountResourceRelationshipsTransactions {
        AccountResourceRelationshipsTransactions {
            links: None,
        }
    }
}


