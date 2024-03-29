/*
 * Up API
 *
 * The Up API gives you programmatic access to your balances and transaction data. You can request past transactions or set up webhooks to receive real-time events when new transactions hit your account. It’s new, it’s exciting and it’s just the beginning. 
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionResourceRelationshipsTransferAccount : If this transaction is a transfer between accounts, this relationship will contain the account the transaction went to/came from. The `amount` field can be used to determine the direction of the transfer. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionResourceRelationshipsTransferAccount {
    #[serde(rename = "data", deserialize_with = "Option::deserialize")]
    pub data: Option<Box<crate::models::TransactionResourceRelationshipsTransferAccountData>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::AccountResourceRelationshipsTransactionsLinks>>,
}

impl TransactionResourceRelationshipsTransferAccount {
    /// If this transaction is a transfer between accounts, this relationship will contain the account the transaction went to/came from. The `amount` field can be used to determine the direction of the transfer. 
    pub fn new(data: Option<crate::models::TransactionResourceRelationshipsTransferAccountData>) -> TransactionResourceRelationshipsTransferAccount {
        TransactionResourceRelationshipsTransferAccount {
            data: if let Some(x) = data {Some(Box::new(x))} else {None},
            links: None,
        }
    }
}


